use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AnirustArgs {
    /// first argument
    #[clap(subcommand)]
    pub mode_type: ModeType,
}

#[derive(Debug, Subcommand)]
pub enum ModeType {
    /// Interactive menu
    Menu,
    /// Search for anime
    Search(SearchCommand),
    /// Download single or range of episodes
    Download(DownloadCommand),
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    /// anime title
    pub title: String,
    /// episode number
    pub episode: Option<u32>,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    /// anime title
    pub title: String,
    /// range of episodes; start
    pub from: u32,
    /// range of episodes; end
    pub to: Option<u32>,
}
