use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    privacy: Privacy,
    ui: UI,
    extensions: Extensions,
    developer: Developer
}

#[derive(Deserialize)]
pub struct Privacy {
    satelite_sync_nodes: bool,
    safer_file_scanning: bool
}

#[derive(Deserialize)]
pub struct UI {
    theme: String
}

#[derive(Deserialize)]
pub struct Extensions {
    enable: bool
}

#[derive(Deserialize)]
pub struct Developer {
    logging: bool,
    cache_dir: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            privacy: Privacy { satelite_sync_nodes: true, safer_file_scanning: true },
            ui: UI { theme: "default".to_string() },
            extensions: Extensions { enable: true },
            developer: Developer { logging: false, cache_dir: ".warp".to_string() }
        }
    }
}

pub fn load_config_or_default() -> Config {
    let config_location = "Config.toml";

    let contents = match fs::read_to_string(config_location) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", config_location);
            // Exit the program with exit code `1`.
            "".to_string()
        }
    };

    // Use a `match` block to return the
    // file `contents` as a `Config struct: Ok(c)`
    // or handle any `errors: Err(_)`.
    let config: Config = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", config_location);
            // Exit the program with exit code `1`.
            Config::default()
        }
    };

    config
}