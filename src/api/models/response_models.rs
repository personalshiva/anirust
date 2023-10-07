use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Episodes {
    pub sub: i32,
    pub dub: i32,
    pub raw: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodesDetails {
    pub sub: Vec<String>,
    pub dub: Vec<String>,
    pub raw: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "availableEpisodes")]
    pub available_episodes: Option<Episodes>,
    #[serde(rename = "availableEpisodesDetail")]
    pub available_episodes_detail: Option<EpisodesDetails>,
    #[serde(rename = "__typename", default)]
    pub typename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceUrl {
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    #[serde(rename = "downloadUrl", default)]
    pub download_url: Option<String>,
    pub priority: Option<f32>,
    #[serde(rename = "sourceName")]
    pub source_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "streamerId")]
    pub streamer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    #[serde(rename = "episodeString")]
    pub episode_string: String,
    #[serde(rename = "sourceUrls")]
    pub source_urls: Vec<SourceUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shows {
    pub edges: Vec<Show>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub shows: Option<Shows>,
    pub show: Option<Show>,
    pub episode: Option<Episode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Segment {
    range: String,
    index_range: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawUrl {
    bandwidth: i32,
    mime_type: String,
    height: i32,
    width: i32,
    frame_rate: String,
    start_with_sap: i32,
    sar: String,
    url: String,
    codecs: String,
    segment_base: Segment,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawUrls {
    vids: Box<[RawUrl]>,
    audios: Box<[RawUrl]>,
    duration: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Subtitles {
    lang: String,
    label: String,
    default: bool,
    paring: String,
    src: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub link: String,
    src: Option<String>,
    #[serde(rename = "resolutionStr", default)]
    resolution_str: Option<String>,
    #[serde(rename = "fromCache", default)]
    from_cache: Option<String>,
    dash: Option<bool>,
    hls: Option<bool>,
    mp4: Option<bool>,
    priority: Option<i32>,
    subtitles: Option<Box<[Subtitles]>>,
    #[serde(rename = "rawUrls", default)]
    raw_urls: Option<RawUrls>,
    trusts: Option<Box<[String]>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResponse {
    pub links: Vec<Link>,
}
