use crate::{
    app::App,
    cli::commands::{
        download::download,
        search::{fetch_episode, search_then_menu, select_episode, ApiError},
    },
};

use super::{
    audio_menu, error_menu, main_menu, media_player_menu,
    options::{ErrorOption, MainOption, PlayerOption, SettingOption},
    player_menu, quality_menu, settings_menu,
};

#[async_recursion::async_recursion]
pub async fn handle_main_option(app: &mut App, option: MainOption) {
    match option {
        MainOption::Search => {
            app.mut_state().set_show_query(None, None);
            search_then_menu(app).await;
        }
        MainOption::Settings => settings_menu(app).await,
        MainOption::Quit => std::process::exit(0),
    }
}

#[async_recursion::async_recursion]
pub async fn handle_player_option(app: &mut App, option: PlayerOption) {
    match option {
        PlayerOption::Play => play_or_menu(app).await,
        PlayerOption::Next | PlayerOption::Previous => {
            let result = next_previous_handler(app, option).await;
            if let Err(e) = result {
                error_menu(app, e).await;
            }
        }
        PlayerOption::Download => download_handler(app).await,
        PlayerOption::Select => {
            let result = select_handler(app).await;
            if let Err(e) = result {
                error_menu(app, e).await;
            }
        }
        PlayerOption::Menu => main_menu(app).await,
        PlayerOption::Quit => std::process::exit(0),
    }
}

async fn play_or_menu(app: &mut App) {
    if let Err(e) = app.player().is_available().await {
        error_menu(app, e).await;
    }
    app.player().play(app.state());
    player_menu(app).await;
}

#[async_recursion::async_recursion]
async fn next_previous_handler(app: &mut App, option: PlayerOption) -> Result<(), ApiError> {
    let next = match option {
        PlayerOption::Next => app.state().next_episode(),
        PlayerOption::Previous => app.state().previous_episode(),
        _ => {
            return Err(ApiError::ClientError(
                "Invalid PlayerOption: Expected Next or Previous".to_owned(),
            ))
        }
    };
    let current_episode = match app.state().current_show() {
        Some(current_show) => {
            fetch_episode(app.state(), app.client(), current_show, Some(next)).await?
        }
        None => {
            return Err(ApiError::ClientError(
                "No current show is selected".to_owned(),
            ))
        }
    };
    app.mut_state().set_episode(current_episode);
    player_menu(app).await;
    Ok(())
}

async fn download_handler(app: &mut App) {
    if let Err(e) = download(app) {
        eprintln!("Error: {}", e);
    } else {
        player_menu(app).await;
    }
}

async fn select_handler(app: &mut App) -> Result<(), ApiError> {
    let current_episode = match app.state().current_show() {
        Some(current_show) => {
            let ep_number = select_episode(current_show.available_episodes());
            fetch_episode(app.state(), app.client(), current_show, Some(ep_number)).await?
        }
        None => {
            return Err(ApiError::ClientError(
                "No current show is selected".to_owned(),
            ))
        }
    };
    app.mut_state().set_episode(current_episode);
    player_menu(app).await;

    Ok(())
}

pub async fn handle_setting_option(app: &mut App, option: SettingOption) {
    match option {
        SettingOption::Audio => audio_menu(app).await,
        SettingOption::Quality => quality_menu(app).await,
        SettingOption::Player => media_player_menu(app).await,
    }
}

pub async fn handle_error_option(app: &mut App, option: ErrorOption) {
    match option {
        ErrorOption::Menu => main_menu(app).await,
        ErrorOption::Quit => std::process::exit(0),
    }
}
