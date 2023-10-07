use serde::{Deserialize, Serialize};

use crate::config::AudioMode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub variables: String,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowVariables {
    pub search: Show,
    pub limit: u32, // 40
    pub page: u32,  // 1
    #[serde(rename = "translationType")]
    pub translation_type: String, // "sub"
    #[serde(rename = "countryOrigin")]
    pub country_origin: String, // "ALL"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamVariables {
    #[serde(rename = "showId")]
    pub show_id: String,
    #[serde(rename = "translationType")]
    pub translation_type: AudioMode,
    #[serde(rename = "episodeString")]
    pub episode_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    #[serde(rename = "allowAdult")]
    pub allow_adult: bool, //False
    #[serde(rename = "allowUnknown")]
    pub allow_unknown: bool, //False
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodeVariables {
    #[serde(rename = "showId")]
    pub show_id: String,
}
