pub mod database;
pub mod webserver;

use serde::{Deserialize, Serialize};

/// Address we use
const ADDRESS: &str = "0.0.0.0:3000";

/// Musicup configuration for Database and Web server.
/// It is created by confy crate and your configuration file, such as `/home/haruki/.config/musicup/musicup.toml`.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Configuration {}
