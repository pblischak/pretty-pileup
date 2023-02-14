//! Module for defining configuration options.

use std::path::{Path, PathBuf};

use dirs::home_dir;
use serde_derive::Deserialize;

use crate::colors::ColorTheme;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub base_colors: TomlColors,
    pub quality_gradient: TomlGradient,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TomlColors {
    pub base_a: String,
    pub base_c: String,
    pub base_g: String,
    pub base_t: String,
    pub base_n: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TomlGradient {
    pub low: String,
    pub high: String,
}

/// Locate config file with program options and return appropriate path.
///
/// The options for specifying config options are as follows (in order of
/// priority):
///
///  - A local file named `theme.toml` within the current working directory.
///  - A hidden file in your `$HOME` directory named `.pretty.toml`.
///  - A `theme.toml` file in your `$HOME/.config` directory with the `pretty/
///    subfolder (full path: `$HOME/.config/pretty/theme.toml`).
fn find_config() -> Option<PathBuf> {
    let home = home_dir().expect("Looking for $HOME directory.");
    let current_dir = Path::new(".");
    let local_path = current_dir.join("theme.toml");
    let home_path = Path::new(&home).join(".pretty.toml");
    let config_path = Path::new(&home)
        .join(".config")
        .join("pretty")
        .join("theme.toml");
    if local_path.exists() {
        return Some(local_path);
    }

    if home_path.exists() {
        return Some(home_path);
    }

    if config_path.exists() {
        return Some(config_path);
    }

    None
}

/// Parse the TOML config file defining a color thme.
///
/// ```toml
/// [base_colors]
/// base_a = "<hex color>"
/// base_c = "<hex color>"
/// base_g = "<hex color>"
/// base_t = "<hex color>"
///
/// [qualitygradiant]
/// low = "<hex color>"
/// high = "<hex color>"
/// ```
pub(crate) fn parse_config() -> ColorTheme {
    if let Some(cfg) = find_config() {
        let contents = match std::fs::read_to_string(cfg) {
            Ok(c) => c,
            Err(_) => return ColorTheme::default(),
        };
        let config: Config = match toml::from_str(contents.as_str()) {
            Ok(c) => c,
            Err(_) => return ColorTheme::default(),
        };
        ColorTheme::from_config(config)
    } else {
        ColorTheme::default()
    }
}
