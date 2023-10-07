use std::io::{self, Error, ErrorKind};

use crate::{
    api::client::ApiClient, app_state::AppState, config::Config, menu::error_menu,
    player::AppPlayer, utils::is_command_available,
};

#[derive(Debug)]
pub struct App {
    state: AppState,
    client: ApiClient,
    player: AppPlayer,
}

impl App {
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn client(&self) -> &ApiClient {
        &self.client
    }
    pub fn player(&self) -> &AppPlayer {
        &self.player
    }

    pub fn mut_state(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn mut_player(&mut self) -> &mut AppPlayer {
        &mut self.player
    }

    pub async fn initialise_app() -> Result<Self, io::Error> {
        // Load app configuration
        let config = match Config::load_configuration() {
            Ok(config) => config,
            Err(_) => Config::default(),
        };

        let mut app = App {
            state: AppState::from_config(&config)?,
            client: ApiClient::default(),
            player: AppPlayer::from_config(&config),
        };

        let media_player = app.player().media_player();
        // Check if the media player is available
        if !is_command_available(media_player.as_str()) {
            let error_string = format!("Could not find media player: {}", media_player.as_str());
            let error = Box::new(Error::new(ErrorKind::Other, error_string));
            error_menu(&mut app, error).await;
        }

        Ok(app)
    }
}
