use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Settings {
    pub tm_api_base: String,
    pub tm_api_key: String,
    pub tm_venue_id: String,
    pub tm_page_size: u64,
    pub oic_cal_base_url: String,
    pub cache_duration: u64,
    #[serde(default = "default_overlap_window_hours")]
    pub overlap_window_hours: i64,
}

fn default_overlap_window_hours() -> i64 {
    3
}

impl Settings {
    pub fn from_json(value: &serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
