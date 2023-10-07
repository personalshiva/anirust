use reqwest::header::HeaderMap;
use serde_json::to_string;

use crate::{api::models::response_models, config::AudioMode};

use super::models::request_models::{
    self, EpisodeVariables, Request, ShowVariables, StreamVariables,
};

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    allanime_api: String,
}

impl Default for ApiClient {
    fn default() -> Self {
        let client = Self::initialise_client();
        ApiClient {
            client,
            allanime_api: "https://api.allanime.day/api".to_owned(),
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
        let headers = Self::create_default_headers();
        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build client")
    }

    fn create_default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 6.1; Win64; rv:109.0) Gecko/20100101 Firefox/109.0"
                .to_owned()
                .parse()
                .expect("Failed to parse header value"),
        );
        headers.insert(
            "Referer",
            "https://allanime.to"
                .to_owned()
                .parse()
                .expect("Failed to parse header value"),
        );

        headers
    }

    pub async fn request_shows(
        &self,
        query: String,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let params = self.build_search_shows_params(query);
        self.request_data(params).await
    }

    fn build_search_shows_params(&self, query: String) -> Request {
        let search_gql = r#"
        query(
            $search: SearchInput
            $limit: Int
            $page: Int
            $translationType: VaildTranslationTypeEnumType
            $countryOrigin: VaildCountryOriginEnumType
        ) {
            shows(
                search: $search
                limit: $limit
                page: $page
                translationType: $translationType
                countryOrigin: $countryOrigin
            ) {
                edges {
                    _id name
                    availableEpisodes __typename
                }
            }
        }"#
        .to_owned();
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

        Request {
            variables: to_string(&variables).expect("Failed to serialise show variables"),
            query: search_gql,
        }
    }

    async fn request_data(
        &self,
        params: Request,
    ) -> Result<response_models::Response, Box<dyn std::error::Error>> {
        let request = self
            .client()
            .request(reqwest::Method::GET, self.allanime_api())
            .query(&params);

        let response = request
            .send()
            .await?
            .json()
            .await
            .expect("Failed to deserialise response");

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
        let episodes_list_gql = r#"
        query ($showId: String!) {
            show(_id: $showId) {
                _id availableEpisodesDetail
            }
        }"#
        .to_owned();

        let variables = EpisodeVariables { show_id };

        Request {
            variables: to_string(&variables).expect("Failed to serialise episode variables"),
            query: episodes_list_gql,
        }
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
        let episode_embed_gql = r#"
        query(
            $showId: String!,
            $translationType: VaildTranslationTypeEnumType!,
            $episodeString: String!
        ) {
            episode(
                showId: $showId
                translationType: $translationType
                episodeString: $episodeString
            ) {
                episodeString sourceUrls
            }
        }"#
        .to_owned();

        let variables = StreamVariables {
            show_id,
            translation_type: *audio_mode,
            episode_string: episode.to_string(),
        };

        Request {
            variables: to_string(&variables).expect("Failed to serialise variables"),
            query: episode_embed_gql,
        }
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
