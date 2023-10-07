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

pub async fn handle_main_option(app: &mut App, option: MainOption) {
    match option {
        MainOption::Search => search_show_handler(app).await,
        MainOption::Settings => settings_handler(app).await,
        MainOption::Quit => quit_handler(),
    }
}

async fn search_show_handler(app: &mut App) {
    app.mut_state().set_show_query(None, None);
    search_then_menu(app).await;
}

async fn settings_handler(app: &mut App) {
    settings_menu(app).await;
}

fn quit_handler() {
    std::process::exit(0);
}

#[async_recursion::async_recursion]
pub async fn handle_player_option(app: &mut App, option: PlayerOption) {
    match option {
        PlayerOption::Play => play_handler(app).await,
        PlayerOption::Next => match next_handler(app).await {
            Ok(_) => (),
            Err(e) => error_menu(app, e).await,
        },
        PlayerOption::Previous => match previous_handler(app).await {
            Ok(_) => (),
            Err(e) => error_menu(app, e).await,
        },
        PlayerOption::Download => download_handler(app).await,
        PlayerOption::Select => match select_handler(app).await {
            Ok(_) => (),
            Err(e) => error_menu(app, e).await,
        },
        PlayerOption::Menu => {
            main_menu(app).await;
        }
        PlayerOption::Quit => quit_handler(),
    }
}

#[async_recursion::async_recursion]
async fn play_handler(app: &mut App) {
    if let Err(e) = app.player().is_available().await {
        error_menu(app, e).await
    }
    app.player().play(app.state());
    player_menu(app).await;
}

#[async_recursion::async_recursion]
async fn next_handler(app: &mut App) -> Result<(), ApiError> {
    let next = app.state().next_episode();
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

#[async_recursion::async_recursion]
async fn previous_handler(app: &mut App) -> Result<(), ApiError> {
    let prev = app.state().previous_episode();
    let current_episode = match app.state().current_show() {
        Some(current_show) => {
            fetch_episode(app.state(), app.client(), current_show, Some(prev)).await?
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
    match download(app) {
        Ok(_) => player_menu(app).await,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
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
        SettingOption::Audio => audio_handler(app).await,
        SettingOption::Quality => quality_handler(app).await,
        SettingOption::Player => player_handler(app).await,
    }
}

#[async_recursion::async_recursion]
pub async fn quality_handler(app: &mut App) {
    quality_menu(app).await;
}

#[async_recursion::async_recursion]
pub async fn audio_handler(app: &mut App) {
    audio_menu(app).await;
}

#[async_recursion::async_recursion]
pub async fn player_handler(app: &mut App) {
    media_player_menu(app).await;
}

pub async fn handle_error_option(app: &mut App, option: ErrorOption) {
    match option {
        ErrorOption::Menu => {
            main_menu(app).await;
        }
        ErrorOption::Quit => quit_handler(),
    }
}
