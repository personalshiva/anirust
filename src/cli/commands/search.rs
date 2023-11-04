use std::{collections::HashMap, error::Error, fmt};

use indexmap::IndexMap;

use crate::{
    api::{
        client::ApiClient,
        models::response_models::{Link, Show, Shows, SourceUrl},
        url_processor::{decrypt::decrypt_url, handle_source},
    },
    app::App,
    app_state::{AppState, CurrentEpisode, CurrentShow},
    cli::args::SearchCommand,
    config::{AudioMode, Quality},
    menu::{error_menu, player_menu},
    utils::fzf::{prompt_user, skim_menu},
};

#[derive(Debug)]
pub enum ApiError {
    NoShows(String),
    NoEpisodes(String),
    NoStream(String),
    BadUrl(String),
    MissingField(String),
    ClientError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiError::*;

        let (emoji, desc) = match self {
            NoShows(message) => ("(╯°□°）╯︵ ┻━┻", format!("No shows found: {}", message)),
            NoEpisodes(message) => ("(._.)", format!("No episodes available: {}", message)),
            NoStream(message) => ("t(-_-t)", format!("No stream: {}", message)),
            BadUrl(message) => ("(╬ ಠ益ಠ)", format!("Bad URL: {}", message)),
            MissingField(message) => ("(´･_･`)", format!("Missing field in response: {}", message)),
            ClientError(message) => (
                "(҂◡_◡)",
                format!("Client encountered an error: {}", message),
            ),
        };

        write!(f, "{} {}", emoji, desc)
    }
}

impl Error for ApiError {}

struct SearchResult {
    show: CurrentShow,
    episode: CurrentEpisode,
}

pub async fn search_command(app: &mut App, command: SearchCommand) {
    app.mut_state()
        .set_show_query(Some(command.title), command.episode);
    search_then_menu(app).await;
}

#[async_recursion::async_recursion]
pub async fn search_then_menu(app: &mut App) {
    match search(app).await {
        Ok(result) => {
            app.mut_state().set_show(result.show);
            app.mut_state().set_episode(result.episode);

            player_menu(app).await;
        }
        Err(e) => error_menu(app, e).await,
    };
}

async fn search(app: &mut App) -> Result<SearchResult, ApiError> {
    let show = fetch_show(app.state(), app.client()).await?;
    let episode_num = app.state().show_query().episode();
    let episode = fetch_episode(app.state(), app.client(), &show, episode_num).await?;
    Ok(SearchResult { show, episode })
}

pub async fn fetch_show(state: &AppState, client: &ApiClient) -> Result<CurrentShow, ApiError> {
    let query = match state.show_query().title() {
        Some(query) => query.to_owned(),
        None => enter_query(),
    };
    let shows = search_shows(client, query).await?;
    let show = select_show(state.audio_mode(), shows)?;
    let available_episodes = search_episodes(client, show.id.clone(), state.audio_mode()).await?;

    Ok(CurrentShow::new(show.id, show.name, available_episodes))
}

pub async fn fetch_episode(
    state: &AppState,
    client: &ApiClient,
    show: &CurrentShow,
    ep_number: Option<u32>,
) -> Result<CurrentEpisode, ApiError> {
    let ep_number = match ep_number {
        Some(episode) => episode,
        None => select_episode(show.available_episodes()),
    };

    let sources =
        fetch_sources(client, show.id().to_owned(), state.audio_mode(), ep_number).await?;
    let source = select_source(state.known_providers(), &sources)?;
    let url = fetch_url(client, state.quality(), source).await?;

    Ok(CurrentEpisode::new(ep_number, url))
}

fn enter_query() -> String {
    prompt_user("Search: ")
}

async fn search_shows(client: &ApiClient, query: String) -> Result<Shows, ApiError> {
    let response = client
        .request_shows(query.clone())
        .await
        .map_err(|_| ApiError::ClientError("Failed to request shows".to_owned()))?;

    match response.data.shows {
        Some(shows) if !shows.edges.is_empty() => Ok(shows),
        Some(_) => Err(ApiError::NoShows(format!(
            "Could not find any shows matching {:?}",
            query
        ))),
        None => Err(ApiError::MissingField("No shows field".to_owned())),
    }
}

fn select_show(audio_mode: &AudioMode, mut shows: Shows) -> Result<Show, ApiError> {
    if shows.edges.len() > 1 {
        let show_options = build_show_results(audio_mode, shows)?;
        let display_strings: Vec<&str> = show_options.iter().map(|(s, _)| s.as_ref()).collect();
        let selection = skim_menu(&display_strings, Some("Select show: "));
        Ok(show_options
            .into_iter()
            .find_map(|(s, show)| if s == selection { Some(show) } else { None })
            .expect("Selection from input collection"))
    } else {
        Ok(shows
            .edges
            .pop()
            .ok_or(ApiError::MissingField("Show is empty".to_owned()))?)
    }
}

fn build_show_results(
    audio_mode: &AudioMode,
    shows: Shows,
) -> Result<Vec<(String, Show)>, ApiError> {
    let show_references: Vec<(String, Show)> = shows
        .edges
        .into_iter()
        .filter_map(|show| {
            let name = show.name.clone()?;
            let episode_count = match audio_mode {
                AudioMode::Sub => show.available_episodes?.sub,
                AudioMode::Dub => show.available_episodes?.dub,
                AudioMode::Raw => show.available_episodes?.raw,
            };

            if episode_count == 0 {
                return None; // Skip shows with zero episodes.
            }

            Some((format!("{} ({:?} episodes)", name, episode_count), show))
        })
        .collect();

    if show_references.is_empty() {
        Err(ApiError::NoEpisodes(
            format!("No {:?} episodes", audio_mode).to_owned(),
        ))
    } else {
        Ok(show_references)
    }
}

async fn search_episodes(
    client: &ApiClient,
    show_id: String,
    audio_mode: &AudioMode,
) -> Result<Vec<u32>, ApiError> {
    let response = client
        .request_episodes(show_id)
        .await
        .map_err(|_| ApiError::ClientError("Failed to request episodes".to_owned()))?;
    let episodes_details = response
        .data
        .show
        .expect("No show in the response")
        .available_episodes_detail
        .expect("No episodes details");
    let episode_vec = match audio_mode {
        AudioMode::Sub => episodes_details.sub,
        AudioMode::Dub => episodes_details.dub,
        AudioMode::Raw => episodes_details.raw,
    };

    episode_vec
        .iter()
        .map(|x| {
            x.parse().map_err(|_| {
                ApiError::MissingField(format!("Could not parse episode number {}", x).to_owned())
            })
        })
        .collect()
}

pub fn select_episode(available_episodes: &[u32]) -> u32 {
    let display_episodes: Vec<String> = available_episodes
        .iter()
        .rev()
        .map(|x| x.to_string())
        .collect();
    let display_episodes: Vec<&str> = display_episodes.iter().map(|x| x.as_str()).collect();
    skim_menu(&display_episodes, Some("Select episode: "))
        .parse()
        .expect("Selecting from from previously parsed integers")
}

async fn fetch_sources(
    client: &ApiClient,
    show_id: String,
    audio_mode: &AudioMode,
    episode: u32,
) -> Result<Vec<SourceUrl>, ApiError> {
    let response = client
        .request_streams(show_id, audio_mode, episode)
        .await
        .map_err(|_| ApiError::ClientError("Failed to get streams".to_owned()))?;
    let mut sources = response
        .data
        .episode
        .ok_or(ApiError::MissingField("No episode in response".to_owned()))?
        .source_urls;
    sort_streams_by_priority(&mut sources);
    Ok(sources)
}

fn select_source(
    known_providers: &IndexMap<String, String>,
    sources: &[SourceUrl],
) -> Result<SourceUrl, ApiError> {
    known_providers
        .iter()
        .find_map(|(id, _)| {
            sources
                .iter()
                .find(|&source| &source.source_name == id)
                .cloned()
        })
        .ok_or(ApiError::NoStream(
            "No stream URL from known providers".to_owned(),
        ))
}

fn sort_streams_by_priority(streams: &mut [SourceUrl]) {
    streams.sort_by(|a, b| {
        a.priority
            .expect("No priority value")
            .total_cmp(&b.priority.expect("No priority value"))
            .reverse()
    });
}

async fn fetch_url(
    client: &ApiClient,
    quality: &Quality,
    stream: SourceUrl,
) -> Result<String, ApiError> {
    let decrypted_url =
        decrypt_url(stream.source_url).map_err(|e| ApiError::BadUrl(e.to_owned()))?;

    let response = client
        .request_links(&decrypted_url)
        .await
        .map_err(|_| ApiError::ClientError("Failed to request links".to_owned()))?;

    let selected_url = find_first_quality(client.client(), quality, &response.links)
        .await
        .ok_or(ApiError::BadUrl(
            "Failed to produce URL for selected quality".to_owned(),
        ))?;

    Ok(selected_url)
}

async fn find_first_quality(
    client: &reqwest::Client,
    quality: &Quality,
    links: &[Link],
) -> Option<String> {
    for link in links.iter().map(|e| &e.link) {
        let qualities = match handle_source(client, link).await {
            Some(value) => value,
            None => continue,
        };

        let quality = select_quality(quality, &qualities);
        return quality; // return the first occurrence
    }

    None // if no quality was found in any link
}

fn select_quality(quality: &Quality, qualities: &HashMap<u32, String>) -> Option<String> {
    match quality {
        Quality::Best => qualities
            .keys()
            .cloned()
            .max()
            .and_then(|max_key| qualities.get(&max_key))
            .cloned(),
        Quality::Worst => qualities
            .keys()
            .cloned()
            .min()
            .and_then(|min_key| qualities.get(&min_key))
            .cloned(),
    }
}
