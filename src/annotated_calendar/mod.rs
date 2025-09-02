use std::{sync::Arc, time};

use axum::{body::Body, http::HeaderValue};
use loco_rs::{cache::Cache, prelude::*};

use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime};

use chrono::{DateTime, Duration, Local, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{spawn, time::Instant};

use reqwest::{Client, header};

use crate::common::settings::Settings;

/// Struct to hold a Calendar object that has been annotated
pub struct AnnotatedCalendar {
    calendar: Calendar,
}

impl AnnotatedCalendar {
    /// Construct a new AnnotatedCalendar object by fetching and processing relevant data from both
    /// the Oakland Ice calendar and the TicketMaster events API.
    pub async fn get_annotated(
        team_id: String,
        season_id: String,
        client: Client,
        cache: Arc<Cache>,
        settings: Settings,
    ) -> Self {
        let oic_key = format!("oic:{team_id}:{season_id}");
        let tm_venue_id = settings.tm_venue_id.to_string();
        let tm_cache_key = format!("tm:{tm_venue_id}");

        // Start future to request OIC calendar data
        let _cache = cache.clone();
        let _client = client.clone();
        let _settings = settings.clone();
        let oic_handle = spawn(async move {
            AnnotatedCalendar::get_oic_events_cached(
                _client,
                _cache,
                oic_key.to_string(),
                team_id.to_string(),
                season_id.to_string(),
                _settings,
            )
            .await
        });

        // Start future to request TM event data
        let _cache = cache.clone();
        let _client = client.clone();
        let _settings = settings.clone();
        let tm_handle = spawn(async move {
            AnnotatedCalendar::get_venue_events_cached(
                _client,
                _cache,
                tm_venue_id.to_string(),
                tm_cache_key.to_string(),
                _settings,
            )
            .await
        });

        // Block until both responses finish
        let mut oic_calendar = oic_handle.await.unwrap();
        let tm_venue_events = tm_handle.await.unwrap();

        // Iterate over OIC calendar entries, seeing if any games overlap with a show at the Fox.
        // If they do, annotate them.
        for component in oic_calendar.components.iter_mut() {
            if let CalendarComponent::Event(game) = component {
                let game_start = game
                    .get_start()
                    .and_then(|date_perhaps_time| match date_perhaps_time {
                        DatePerhapsTime::Date(date) => Some(date.and_time(NaiveTime::MIN).and_utc()),
                        DatePerhapsTime::DateTime(naive_date_time) => naive_date_time.try_into_utc(),
                    })
                    .unwrap();
                for VenueEventInfo {
                    lower_bound,
                    upper_bound,
                    actual_start,
                    artist_name,
                } in &tm_venue_events
                {
                    // TODO: This needs to be hard coded to either PT or the timezone of the
                    // calendar. It only works locally since the server is in PT
                    let actual_start_local: DateTime<Local> = DateTime::from(*actual_start);
                    if game_start >= *lower_bound && game_start <= *upper_bound {
                        tracing::debug!(
                            "Found a match for event {} at {}",
                            game.get_summary().unwrap(),
                            game_start
                        );
                        game.summary(
                            format!(
                                "[Leave Early - Fox show at {} ({})] {}",
                                actual_start_local,
                                artist_name,
                                game.get_summary().unwrap()
                            )
                            .as_str(),
                        );
                    }
                }
            }
        }
        Self { calendar: oic_calendar }
    }

    /// Return an ical response from an instantiated AnnotatedCalendar
    pub fn as_ical_response(self) -> Result<Response> {
        Ok(format::render()
            .response()
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/Calendar; charset=utf-8"),
            )
            .header(
                header::CONTENT_DISPOSITION,
                HeaderValue::from_static("inline; filename=cal.ics"),
            )
            .body(Body::from(self.calendar.to_string()))?)
    }

    /// Fetch OIC calendar data from either the cache or http request. Stores the result in the
    /// cache before returning.
    async fn get_oic_events_cached(
        client: Client,
        cache: Arc<Cache>,
        oic_key: String,
        team_id: String,
        season: String,
        Settings {
            oic_cal_base_url,
            cache_duration,
            ..
        }: Settings,
    ) -> Calendar {
        let url = format!("{oic_cal_base_url}/team-cal.php?team={team_id}&tlev=0&tseq=0&season={season}&format=iCal");
        let parsed_calendar: Calendar = cache
            .get_or_insert_with_expiry::<String, _>(
                oic_key.as_str(),
                time::Duration::from_secs(cache_duration),
                async {
                    tracing::debug!("Fetching '{oic_key}' from API...");
                    client.get_oic(&url).await
                },
            )
            .await
            .unwrap()
            .parse()
            .unwrap();
        parsed_calendar
    }

    /// Get events from TicketMaster API
    async fn get_venue_events(
        client: Client,
        venue_id: String,
        Settings {
            tm_api_base,
            tm_api_key,
            tm_page_size,
            ..
        }: Settings,
    ) -> Result<Vec<VenueEventInfo>> {
        let url = format!(
            "{tm_api_base}/discovery/v2/events.json?venueId={venue_id}&size={tm_page_size}&sort=date,asc&apikey={tm_api_key}"
        );

        let fox_events = client.get_tm(&url).await?;

        let mut venue_event_infos = Vec::new();
        for fe in &fox_events._embedded.events {
            let st = fe.dates.start.date_time;
            venue_event_infos.push(VenueEventInfo {
                lower_bound: st - Duration::hours(3),
                upper_bound: st + Duration::hours(3),
                actual_start: st,
                artist_name: fe.name.clone(),
            });
        }
        venue_event_infos.sort_by_key(|k| k.actual_start);
        Ok(venue_event_infos)
    }

    /// Fetch TicketMaster event data from either the cache or API request. Stores the result in the
    /// cache before returning.
    async fn get_venue_events_cached(
        client: Client,
        cache: Arc<Cache>,
        venue_id: String,
        tm_cache_key: String,
        settings: Settings,
    ) -> Vec<VenueEventInfo> {
        cache
            .get_or_insert_with_expiry::<Vec<VenueEventInfo>, _>(
                tm_cache_key.as_str(),
                time::Duration::from_secs(settings.cache_duration),
                async {
                    tracing::debug!("Fetching '{tm_cache_key}' from API...");
                    AnnotatedCalendar::get_venue_events(client, venue_id.to_string(), settings).await
                },
            )
            .await
            .unwrap()
    }
}

#[derive(Deserialize, Debug)]
struct TmResponse {
    _embedded: TmEvents,
}

#[derive(Deserialize, Debug)]
struct TmEvents {
    events: Vec<TmEvent>,
}

#[derive(Deserialize, Debug)]
struct TmEvent {
    name: String,
    // id: String,
    // url: String,
    dates: TmEventDate,
}

#[derive(Deserialize, Debug)]
struct TmEventDate {
    start: TmEventDateStart,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TmEventDateStart {
    date_time: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Default, Debug)]
struct VenueEventInfo {
    lower_bound: DateTime<Utc>,
    upper_bound: DateTime<Utc>,
    actual_start: DateTime<Utc>,
    artist_name: String,
}

#[async_trait]
trait TmGetter {
    async fn get_tm(&self, url: &str) -> Result<TmResponse, Error>;
}

#[async_trait]
pub trait OicGetter {
    async fn get_oic(&self, url: &str) -> Result<String, Error>;
}

#[async_trait]
impl TmGetter for reqwest::Client {
    async fn get_tm(&self, url: &str) -> Result<TmResponse, Error> {
        let now = Instant::now();
        let fox_events_resp = self.get(url).send().await;
        tracing::debug!("Fox events fetch took: {:?}", now.elapsed());

        let fox_events = match fox_events_resp {
            Ok(resp) => match resp.json::<TmResponse>().await {
                Ok(events) => events,
                Err(e) => {
                    tracing::error!("Error fetching TM events from {}: {}", url, e);
                    return Err(Error::wrap(e));
                }
            },
            Err(e) => {
                tracing::error!("Error fetching TM events from {}: {}", url, e);
                return Err(Error::wrap(e));
            }
        };
        Ok(fox_events)
    }
}

#[async_trait]
impl OicGetter for reqwest::Client {
    async fn get_oic(&self, url: &str) -> Result<String, Error> {
        let now = Instant::now();
        let oic_resp = self.get(url).send().await;
        tracing::debug!("OIC fetch took: {:?}", now.elapsed());
        match oic_resp {
            Ok(resp) => match resp.text().await {
                Ok(cal) => Ok(cal),
                Err(e) => {
                    tracing::error!("Error fetching OIC calendar from {}: {}", url, e);
                    Err(Error::wrap(e))
                }
            },
            Err(e) => {
                tracing::error!("Error fetching OIC calendar from {}: {}", url, e);
                Err(Error::wrap(e))
            }
        }
    }
}
