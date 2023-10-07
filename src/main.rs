mod api;
mod app;
mod app_state;
mod cli;
mod config;
mod menu;
mod player;
mod utils;

use app::App;
use clap::Parser;
use cli::{
    args::{AnirustArgs, ModeType},
    commands::{download::download_command, search::search_command},
};
use menu::main_menu;

#[tokio::main]
async fn main() {
    // Parse user input
    let args = AnirustArgs::parse();
    let mut app = match App::initialise_app().await {
        Ok(app) => app,
        Err(e) => panic!("{}", e),
    };
    match args.mode_type {
        ModeType::Menu => main_menu(&mut app).await,
        ModeType::Search(command) => search_command(&mut app, command).await,
        ModeType::Download(command) => download_command(&mut app, command).await,
    }
}
