use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::to_string;

use crate::{api::models::response_models, config::AudioMode};

use super::{
    graphql_queries::{EPISODES_LIST_GQL, EPISODE_EMBED_GQL, SEARCH_GQL},
    models::request_models::{self, EpisodeVariables, Request, ShowVariables, StreamVariables},
};

const ALLANIME_API_URL: &str = "https://api.allanime.day/api";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 6.1; Win64; rv:109.0) Gecko/20100101 Firefox/109.0";
const REFERER: &str = "https://allanime.to";

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    allanime_api: String,
}

impl Default for ApiClient {
    fn default() -> Self {
        ApiClient {
            client: Self::initialise_client(),
            allanime_api: ALLANIME_API_URL.to_owned(),
        }
    }
}

impl ApiClient {
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn allanime_api(&self) -> &str {
        &self.allanime_api
    }

    fn initialise_client() -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
        headers.insert("Referer", HeaderValue::from_static(REFERER));

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build client")
    }

    pub async fn request_shows(
        &self,
        query: String,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let params = self.build_search_shows_params(query);
        self.request_data(params).await
    }

    fn build_request<T>(query: &str, variables: T) -> Request
    where
        T: serde::Serialize,
    {
        Request {
            variables: to_string(&variables).expect("Failed to serialize variables"),
            query: query.to_owned(),
        }
    }

    fn build_search_shows_params(&self, query: String) -> Request {
        let show = request_models::Show {
            allow_adult: false,
            allow_unknown: false,
            query,
        };

        let variables = ShowVariables {
            search: show,
            limit: 40,
            page: 1,
            translation_type: "sub".to_owned(),
            country_origin: "ALL".to_owned(),
        };

        Self::build_request(SEARCH_GQL, variables)
    }

    async fn request_data(
        &self,
        params: Request,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let request = self
            .client()
            .request(reqwest::Method::GET, self.allanime_api())
            .query(&params);

        let response = request.send().await?.json().await?;

        Ok(response)
    }

    pub async fn request_episodes(
        &self,
        show_id: String,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let params = self.build_search_episodes_params(show_id);
        self.request_data(params).await
    }

    fn build_search_episodes_params(&self, show_id: String) -> Request {
        let variables = EpisodeVariables { show_id };
        Self::build_request(EPISODES_LIST_GQL, variables)
    }

    pub async fn request_streams(
        &self,
        show_id: String,
        audio_mode: &AudioMode,
        episode: u32,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let params = self.build_search_stream_params(show_id, audio_mode, episode);
        self.request_data(params).await
    }

    fn build_search_stream_params(
        &self,
        show_id: String,
        audio_mode: &AudioMode,
        episode: u32,
    ) -> Request {
        let variables = StreamVariables {
            show_id,
            translation_type: *audio_mode,
            episode_string: episode.to_string(),
        };

        Self::build_request(EPISODE_EMBED_GQL, variables)
    }

    pub async fn request_links(
        &self,
        path: &str,
    ) -> Result<response_models::StreamResponse, Box<dyn std::error::Error>> {
        let request = self.client().request(
            reqwest::Method::GET,
            "https://embed.ssbcontent.site".to_owned() + path,
        );

        let response: response_models::StreamResponse = request.send().await?.json().await?;

        Ok(response)
    }
}

impl ApiClient {}
