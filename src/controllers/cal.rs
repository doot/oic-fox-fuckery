#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

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

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/cal/")
        .add("/", get(index))
        .add("{team_id}/{season_id}", get(annotated_team_cal))
}

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
