<p align="center">
  <img
    src="anirust.png"
    alt="anirust">
</p>

Anirust is a CLI tool to watch anime.

Inspired by (☞ﾟヮﾟ)☞ <a href="https://github.com/pystardust/ani-cli">ani-cli</a>

## Table of Contents
- [Quick Start](#quick-start)
- [Dependencies](#dependencies)
- [Usage](#usage)
- [Installation](#installation)
## Quick Start
**Install Rust and Cargo**:
If you haven't already installed Rust and Cargo, the easiest way is to use `rustup`, the Rust toolchain installer.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
**Install anirust**:
```sh
cargo install anirust
```

**Uninstall**:
```sh
cargo uninstall anirust
```

## Dependencies
Select Video Player:
- mpv
- iina - mpv replacement for MacOS
- vlc

Download manager:
- aria2c - Default
- yt-dlp - m3u8 Downloader
- ffmpeg - m3u8 Downloader (fallback)

<details><summary><b>MacOS</b></summary>

*To install (with Homebrew) the dependencies required on Mac OS, you can run:*

```sh
brew install aria2 ffmpeg git yt-dlp && \
brew install --cask iina
```
*Why iina and not mpv? Drop-in replacement for mpv for MacOS. Integrates well with OSX UI. Excellent support for M1. Open Source.*
</details>

## Usage
For info, type:
```sh
anirust help
```

**Examples:**

open interactive menu:
```sh
anirust menu
```
search for show episodes:
```sh
anirust search berserk
```
download a range of episodes:
```sh
anirust download "chainsaw man" 1 10
```

**Custom Configuration**

Custom settings can be specified in `~/.config/anirust/config.toml` file, such as:
```toml
[state]
quality = "best"
audio_mode = "sub"
download_dir = "Desktop/anime"

[player]
media_player = "iina"
```

## Installation
<details><summary><b>From Source</b></summary>

1. **Clone the Repository**:
   Use `git` to clone the repository:

   ```sh
   git clone https://github.com/personalshiva/anirust.git
   ```

2. **Navigate to the Project Directory**:

   ```bash
   cd anirust
   ```

3. **Build and Install the Project**:
   You can build and install the project using Cargo. To install the binary to a location in your `PATH` (`~/.cargo/bin/`), you can use:

   ```sh
   cargo install --path .
   ```

   Note: The `--path .` argument tells Cargo to install the crate in the current directory.
</details>
