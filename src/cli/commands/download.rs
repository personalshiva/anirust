use std::{fmt, fs, path::Path, process::Command};

use crate::{app::App, cli::args::DownloadCommand, menu::error_menu, utils::is_command_available};

use std::error::Error;

use super::search::{fetch_episode, fetch_show};

#[derive(Debug)]
pub enum DownloadError {
    StreamNotSelected,
    ShowNotSelected,
    EpisodeNotSelected,
    DirectoryCreationFailed,
    RequiredToolMissing(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::StreamNotSelected => write!(f, "Stream has not been selected"),
            Self::ShowNotSelected => write!(f, "Show has not been selected"),
            Self::EpisodeNotSelected => write!(f, "Episode has not been selected"),
            Self::DirectoryCreationFailed => write!(f, "Failed to create download directory"),
            Self::RequiredToolMissing(tool) => write!(f, "`{}` is required for download", tool),
        }
    }
}

impl Error for DownloadError {}

pub async fn download_command(app: &mut App, command: DownloadCommand) {
    app.mut_state().set_show_query(Some(command.title), None);
    let episode_range = match command.to {
        Some(to_ep) => command.from..=to_ep,
        None => command.from..=command.from,
    };
    match fetch_show(app.state(), app.client()).await {
        Ok(show) => app.mut_state().set_show(show),
        Err(err) => error_menu(app, err).await,
    };

    for ep_number in episode_range {
        match fetch_episode(
            app.state(),
            app.client(),
            app.state().current_show().expect("Show selected"),
            Some(ep_number),
        )
        .await
        {
            Ok(episode) => {
                app.mut_state().set_episode(episode);
                let _ = download(app);
            }
            Err(err) => error_menu(app, err).await,
        }
    }
}

pub fn download(app: &App) -> Result<(), DownloadError> {
    let url = app
        .state()
        .current_episode()
        .ok_or(DownloadError::StreamNotSelected)?
        .url();
    let show_name = app
        .state()
        .current_show()
        .ok_or(DownloadError::ShowNotSelected)?
        .name()
        .expect("Show has name");
    let episode = app
        .state()
        .current_episode()
        .ok_or(DownloadError::EpisodeNotSelected)?
        .ep_number();
    let filename = format!("{} Episode {}", show_name, episode);
    let mut download_dir = app.state().download_dir().clone();
    download_dir.push(show_name);

    // Ensure download directory exists
    if !Path::new(&download_dir).exists() {
        fs::create_dir_all(&download_dir).map_err(|_| DownloadError::DirectoryCreationFailed)?;
    }

    if url.contains("m3u8") {
        if is_command_available("yt-dlp") {
            yt_dlp_download(&download_dir, &filename, url);
            return Ok(());
        } else if is_command_available("ffmpeg") {
            ffmpeg_download(&download_dir, &filename, url);
            return Ok(());
        } else {
            return Err(DownloadError::RequiredToolMissing(
                "yt-dlp or ffmpeg".to_string(),
            ));
        }
    }
    // default
    aria2c_download(filename, url, &download_dir);
    Ok(())
}

fn aria2c_download(filename: String, url: &str, download_dir: &std::path::Path) {
    let output_path = format!("{}.mp4", filename);
    let _ = Command::new("aria2c")
        .args([
            "--enable-rpc=false",
            "--check-certificate=false",
            "--continue",
            "--summary-interval=0",
            "-x",
            "16",
            "-s",
            "16",
            url,
            "--dir",
            download_dir
                .to_str()
                .expect("Could not convert download_dir to &str"),
            "-o",
            &output_path,
            "--download-result=hide",
        ])
        .spawn()
        .expect("Failed to run aria2c")
        .wait();
}

fn ffmpeg_download(download_dir: &std::path::PathBuf, filename: &String, url: &str) {
    let output_path = format!("{:?}/{}.mp4", download_dir, filename);
    let _ = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "error",
            "-stats",
            "-i",
            url,
            "-c",
            "copy",
            &output_path,
        ])
        .spawn()
        .expect("Failed to run ffmpeg")
        .wait();
}

fn yt_dlp_download(download_dir: &std::path::PathBuf, filename: &String, url: &str) {
    let output_path = format!("{:?}/{}.mp4", download_dir, filename);
    let _ = Command::new("yt-dlp")
        .args([
            url,
            "--no-skip-unavailable-fragments",
            "--fragment-retries",
            "infinite",
            "-N",
            "16",
            "-o",
            &output_path,
        ])
        .spawn()
        .expect("Failed to run yt-dlp")
        .wait();
}
