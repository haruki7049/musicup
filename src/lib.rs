//! Musicup

pub mod archive;
pub mod webserver;

use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

/// Musicup configuration for Database and Web server.
/// It is created by confy crate and your configuration file, such as `/home/haruki/.config/musicup/musicup.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    address: IpAddr,
    port: u16,
    zip_name: String,
    cache_dir: PathBuf,
    music_dir: PathBuf,
}

impl std::default::Default for Configuration {
    fn default() -> Self {
        let music_dir: PathBuf = match UserDirs::new() {
            Some(user_dirs) => match user_dirs.audio_dir() {
                Some(path) => path.to_path_buf(),
                None => PathBuf::new(),
            },
            None => PathBuf::new(),
        };

        let cache_dir: PathBuf = match ProjectDirs::from("dev", "haruki7049", "musicup") {
            Some(project_dirs) => project_dirs.cache_dir().to_path_buf(),
            None => PathBuf::new(),
        };

        Self {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
            zip_name: String::from("musics"),
            cache_dir,
            music_dir,
        }
    }
}
