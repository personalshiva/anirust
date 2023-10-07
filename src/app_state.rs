use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};

use indexmap::IndexMap;

use crate::config::{AudioMode, Config, Quality};

#[derive(Debug)]
pub struct AppState {
    quality: Quality,
    audio_mode: AudioMode,
    download_dir: PathBuf,
    show_query: ShowQuery,
    current_show: Option<CurrentShow>,
    current_episode: Option<CurrentEpisode>,
    known_providers: IndexMap<String, String>,
}

#[derive(Debug)]
pub struct ShowQuery {
    title: Option<String>,
    episode: Option<u32>,
}
impl ShowQuery {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    pub fn episode(&self) -> Option<u32> {
        self.episode
    }
}

#[derive(Debug)]
pub struct CurrentShow {
    id: String,
    name: Option<String>,
    available_episodes: Vec<u32>,
}
impl CurrentShow {
    pub fn new(id: String, name: Option<String>, available_episodes: Vec<u32>) -> Self {
        CurrentShow {
            id,
            name,
            available_episodes,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn available_episodes(&self) -> &Vec<u32> {
        &self.available_episodes
    }
}

#[derive(Debug)]
pub struct CurrentEpisode {
    ep_number: u32,
    url: String,
}

impl CurrentEpisode {
    pub fn new(ep_number: u32, url: String) -> Self {
        CurrentEpisode { ep_number, url }
    }

    pub fn ep_number(&self) -> u32 {
        self.ep_number
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

impl AppState {
    pub fn from_config(config: &Config) -> Result<Self, io::Error> {
        let state = config.state().cloned().unwrap_or_default();
        // Get the home directory for the current user
        let mut download_dir = dirs::home_dir().ok_or(io::Error::new(
            ErrorKind::NotFound,
            "Home directory not found",
        ))?;

        // Append the provided file name to the configuration directory
        download_dir.push(state.download_dir.unwrap_or("anime".to_owned()));

        let known_providers: IndexMap<String, String> = vec![
            ("Default".to_string(), "wixmp".to_string()),
            ("Sak".to_string(), "dropbox".to_string()),
            ("Kir".to_string(), "wetransfer".to_string()),
            ("S-mp4".to_string(), "sharepoint".to_string()),
            ("Luf-mp4".to_string(), "gogoanime".to_string()),
        ]
        .into_iter()
        .collect();
        Ok(AppState {
            quality: state.quality.unwrap_or_default(),
            audio_mode: state.audio_mode.unwrap_or_default(),
            download_dir,
            show_query: ShowQuery {
                title: None,
                episode: None,
            },
            current_show: None,
            current_episode: None,
            known_providers,
        })
    }
}

impl AppState {
    pub fn set_show(&mut self, show: CurrentShow) {
        self.current_show = Some(show);
    }

    pub fn current_show(&self) -> Option<&CurrentShow> {
        self.current_show.as_ref()
    }

    pub fn set_episode(&mut self, episode: CurrentEpisode) {
        self.current_episode = Some(episode);
    }

    pub fn current_episode(&self) -> Option<&CurrentEpisode> {
        self.current_episode.as_ref()
    }

    pub fn next_episode(&self) -> u32 {
        let current_episode = self.current_episode().expect("episode selected").ep_number;
        let available_episodes_len = self
            .current_show()
            .expect("show selected")
            .available_episodes
            .len() as u32;
        if current_episode < available_episodes_len {
            current_episode + 1
        } else {
            1
        }
    }

    pub fn previous_episode(&self) -> u32 {
        let current_episode = self.current_episode().expect("episode selected").ep_number;
        if current_episode > 1_u32 {
            current_episode - 1
        } else {
            self.current_show()
                .expect("show selected")
                .available_episodes
                .len() as u32
        }
    }

    pub fn quality(&self) -> &Quality {
        &self.quality
    }

    pub fn audio_mode(&self) -> &AudioMode {
        &self.audio_mode
    }

    pub fn download_dir(&self) -> &PathBuf {
        &self.download_dir
    }

    pub fn show_query(&self) -> &ShowQuery {
        &self.show_query
    }

    pub fn known_providers(&self) -> &IndexMap<String, String> {
        &self.known_providers
    }

    pub fn set_quality(&mut self, quality: Quality) {
        self.quality = quality
    }

    pub fn set_audio_mode(&mut self, audio_mode: AudioMode) {
        self.audio_mode = audio_mode
    }

    pub fn set_show_query(&mut self, title: Option<String>, episode: Option<u32>) {
        self.show_query = ShowQuery { title, episode };
    }
}
