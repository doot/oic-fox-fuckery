#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_openapi::prelude::*;
use loco_rs::prelude::*;

use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::{Extension, debug_handler};

use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;
use reqwest::Client;

use crate::annotated_calendar::AnnotatedCalendar;

use crate::common::settings::Settings;

pub fn routes() -> Routes {
    Routes::new().prefix("api/cal/").add(
        "{team_id}/{season_id}",
        openapi(get(annotated_team_cal), routes!(annotated_team_cal)),
    )
}

/// Annotated Team Calendar
///
/// Returns a team calendar for the given {team_id} and {season_id} that has been augmented with
/// overlapping events from the nearby Fox Theater.
#[debug_handler]
#[utoipa::path(
    get,
    path = "/api/cal/{team_id}/{season_id}",
    params(
        ("team_id", description = "ID of the team"),
        ("season_id", description = "ID of the season")
    ),
    responses(
        (
            status = 200,
            description = "Annotated Calendar success",
            body = String,
            content_type = "text/calendar",
            examples(
                ("John" = (summary = "team_id=4907, season=69"))
             )

        ),
    ),
)]
async fn annotated_team_cal(
    Path((team_id, season_id)): Path<(i64, i64)>,
    State(ctx): State<AppContext>,
    Extension(client): Extension<Client>,
    Extension(settings): Extension<Settings>,
) -> Result<Response> {
    // let team_id = 4907; Sofa Kings
    // let season = 69; Most recent season, possibly get this programatically if param not
    // provided?

    AnnotatedCalendar::get_annotated(
        team_id.to_string(),
        season_id.to_string(),
        client,
        ctx.cache.clone(),
        settings,
    )
    .await
    .as_ical_response()
}
