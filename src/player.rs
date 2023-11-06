use std::{
    io,
    process::{Command, Stdio},
};

use crate::{
    app_state::AppState,
    config::{Config, MediaPlayer},
    utils::is_command_available,
};

#[derive(Debug)]
pub struct AppPlayer {
    media_player: MediaPlayer,
}

impl AppPlayer {
    pub fn media_player(&self) -> &MediaPlayer {
        &self.media_player
    }

    pub fn set_media_player(&mut self, media_player: MediaPlayer) {
        self.media_player = media_player;
    }

    pub fn from_config(config: &Config) -> Self {
        let media_player = config
            .player()
            .cloned()
            .unwrap_or_default()
            .media_player
            .unwrap_or_default();
        AppPlayer { media_player }
    }
}

impl AppPlayer {
    pub fn play(&self, state: &AppState) {
        let args: Vec<String> = self.args(state);

        Command::new(self.media_player().as_str())
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start process");
    }

    fn args(&self, state: &AppState) -> Vec<String> {
        match self.media_player() {
            MediaPlayer::IINA => {
                vec![
                    "--no-stdin".to_string(),
                    "--keep-running".to_string(),
                    format!(
                        r#"--mpv-force-media-title="{} Episode {}""#,
                        state
                            .current_show()
                            .expect("Show selected")
                            .name()
                            .expect("Show has name"),
                        state
                            .current_episode()
                            .expect("Episode selected")
                            .ep_number()
                    ),
                    state
                        .current_episode()
                        .expect("Episode selected")
                        .url()
                        .clone(),
                    ">/dev/null 2>&1 &".to_string(),
                ]
            }
            MediaPlayer::VLC => {
                vec![
                    "--play-and-exit".to_string(),
                    format!(
                        "--meta-title={} Episode {}",
                        state
                            .current_show()
                            .expect("Show selected")
                            .name()
                            .expect("Show has name"),
                        state
                            .current_episode()
                            .expect("Episode selected")
                            .ep_number()
                    ),
                    state
                        .current_episode()
                        .expect("Episode selected")
                        .url()
                        .clone(),
                    ">/dev/null 2>&1 &".to_string(),
                ]
            }
            MediaPlayer::MPV => {
                vec![
                    format!(
                        r#"--force-media-title="{} Episode {}""#,
                        state
                            .current_show()
                            .expect("Show selected")
                            .name()
                            .expect("Show has name"),
                        state
                            .current_episode()
                            .expect("Episode selected")
                            .ep_number()
                    ),
                    state
                        .current_episode()
                        .expect("Episode selected")
                        .url()
                        .clone(),
                    ">/dev/null 2>&1 &".to_string(),
                ]
            }
        }
    }

    pub async fn is_available(&self) -> Result<(), Box<io::Error>> {
        // Check if the media player is available
        if !is_command_available(self.media_player().as_str()) {
            let error_string = format!(
                "Could not find media player: {}",
                self.media_player().as_str()
            );
            Err(Box::new(io::Error::new(io::ErrorKind::Other, error_string)))
        } else {
            Ok(())
        }
    }
}
