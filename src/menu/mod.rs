use std::error::Error;

use crate::{
    app::App,
    app_state::AppState,
    config::{AudioMode, MediaPlayer, Quality},
    utils::skim_menu::skim_menu,
};

use self::{
    handlers::{
        handle_error_option, handle_main_option, handle_player_option, handle_setting_option,
    },
    options::{ErrorOption, MainOption, MenuOption, PlayerOption, SettingOption},
};

pub mod handlers;
pub mod options;

async fn generic_menu<T: MenuOption>(prompt: Option<&str>) -> Option<T> {
    let options = T::all_options();
    let selection_str = skim_menu(&options, prompt);
    T::from_str(&selection_str)
}

pub async fn main_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<MainOption>(None).await {
        handle_main_option(app, selection).await;
    }
}

pub async fn player_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<PlayerOption>(Some(&player_prompt(app.state()))).await {
        handle_player_option(app, selection).await;
    }
}

fn player_prompt(state: &AppState) -> String {
    format!(
        "{:?} Episode: {:?} ({:?})  quality: {:?} ",
        state
            .current_show()
            .expect("Show selected")
            .name()
            .expect("Show has name"),
        state
            .current_episode()
            .expect("Episode selected")
            .ep_number(),
        state.audio_mode(),
        state.quality(),
    )
}

pub async fn settings_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<SettingOption>(Some(&settings_prompt(app))).await {
        handle_setting_option(app, selection).await;
    }
}

fn settings_prompt(app: &App) -> String {
    format!(
        "quality: {:?}  translation: {:?}  player: {:?}",
        app.state().quality(),
        app.state().audio_mode(),
        app.player().media_player(),
    )
}

pub async fn quality_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<Quality>(None).await {
        app.mut_state().set_quality(selection);
        main_menu(app).await;
    }
}

pub async fn audio_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<AudioMode>(None).await {
        app.mut_state().set_audio_mode(selection);
        main_menu(app).await;
    }
}

pub async fn media_player_menu(app: &mut App) {
    if let Some(selection) = generic_menu::<MediaPlayer>(None).await {
        app.mut_player().set_media_player(selection);
        main_menu(app).await;
    }
}

pub async fn error_menu(app: &mut App, error: impl Error) {
    let prompt = format!("{}", error);
    if let Some(selection) = generic_menu::<ErrorOption>(Some(&prompt)).await {
        handle_error_option(app, selection).await;
    }
}
