use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    general: General,
    privacy: Privacy,
    audiovideo: AudioVideo,
    extensions: Extensions,
    developer: Developer
}

#[derive(Deserialize)]
pub struct Privacy {
    satellite_sync_nodes: bool,
    safer_file_scanning: bool
}

#[derive(Deserialize)]
pub struct General {
    theme: String
}

#[derive(Deserialize)]
pub struct AudioVideo {
    noise_supression: bool
}

#[derive(Deserialize)]
pub struct Extensions {
    enable: bool
}

#[derive(Deserialize)]
pub struct Developer {
    developer_mode: bool,
    cache_dir: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General { theme: String::from("default") },
            privacy: Privacy { satellite_sync_nodes: true, safer_file_scanning: true },
            extensions: Extensions { enable: true },
            audiovideo: AudioVideo { noise_supression: false },
            developer: Developer { developer_mode: false, cache_dir: ".warp".to_string() }
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