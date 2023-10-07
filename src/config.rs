use std::{
    fs,
    io::{self},
};

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

use crate::menu::options::MenuOption;

#[derive(Debug)]
pub enum ConfigError {
    DirectoryNotFound,
    // FileNotFound(String),
    IoError(io::Error),
    ParseError(String),
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    state: Option<State>,
    pub player: Option<Player>,
}

impl Config {
    pub fn state(&self) -> Option<&State> {
        self.state.as_ref()
    }

    pub fn player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    pub fn load_configuration() -> Result<Self, ConfigError> {
        let default_config = Config::default();
        // Get the configuration directory for the current user
        let mut config_path = dirs::home_dir().ok_or(ConfigError::DirectoryNotFound)?;

        // Append the provided file name to the configuration directory
        config_path.push(".config/anirust/config.toml");

        // Ensure the config file exists
        if !config_path.exists() {
            return Ok(default_config);
        }

        // Read the file
        let contents = fs::read_to_string(&config_path).map_err(ConfigError::IoError)?;

        // Parse the TOML string into the Config struct
        let config: Config = toml::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse TOML: {}", e)))?;

        Ok(config)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct State {
    pub quality: Option<Quality>,
    pub audio_mode: Option<AudioMode>,
    pub download_dir: Option<String>,
}
impl Default for State {
    fn default() -> Self {
        State {
            quality: Some(Quality::Best),
            audio_mode: Some(AudioMode::Sub),
            download_dir: Some("anime".to_owned()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    pub media_player: Option<MediaPlayer>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            media_player: Some(MediaPlayer::IINA),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone, EnumString, AsRefStr, Default)]
pub enum MediaPlayer {
    #[default]
    #[serde(rename = "iina")]
    IINA,
    #[serde(rename = "mpv")]
    MPV,
    #[serde(rename = "vlc")]
    VLC,
}

impl MediaPlayer {
    pub fn as_str(&self) -> &'static str {
        match *self {
            MediaPlayer::IINA => "iina",
            MediaPlayer::MPV => "mpv",
            MediaPlayer::VLC => "vlc",
        }
    }
}

impl MenuOption for MediaPlayer {
    fn all_options() -> Vec<&'static str> {
        vec![Self::IINA.as_ref(), Self::MPV.as_ref(), Self::VLC.as_ref()]
    }
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, EnumString, AsRefStr, Default)]
pub enum AudioMode {
    #[default]
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "dub")]
    Dub,
    #[serde(rename = "raw")]
    Raw,
}

impl MenuOption for AudioMode {
    fn all_options() -> Vec<&'static str> {
        vec![Self::Sub.as_ref(), Self::Dub.as_ref(), Self::Raw.as_ref()]
    }
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone, Copy, EnumString, AsRefStr, Default)]
pub enum Quality {
    #[default]
    #[serde(rename = "best")]
    Best,
    #[serde(rename = "worst")]
    Worst,
}

impl MenuOption for Quality {
    fn all_options() -> Vec<&'static str> {
        vec![Self::Best.as_ref(), Self::Worst.as_ref()]
    }
    fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}
