use anyhow::Result;
use config::Config;
use directories::ProjectDirs;
use std::{collections::HashMap, fs};
use toml;

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

const DEFAULT_MUTE_KEYS: &'static str = "CMD+SHIFT+M";
pub struct AppVars {
    pub name: String,
    pub shortname: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub repository: &'static str,
    pub license: &'static str,
    pub authors: Vec<String>,
    pub shortcut: String,
}

impl AppVars {
    pub fn new() -> Result<Self> {
        let shortname = env!("CARGO_PKG_NAME");
        let name = shortname
            .split('-')
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ");
        let authors: Vec<String> = env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(|s| s.to_string())
            .collect();

        println!("NAME: {}", name);
        let cfg = load_config()?;

        Ok(Self {
            name,
            shortname,
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            repository: env!("CARGO_PKG_REPOSITORY"),
            license: env!("CARGO_PKG_LICENSE"),
            shortcut: match cfg.get_string("shortcut") {
                Ok(v) => v,
                Err(_) => DEFAULT_MUTE_KEYS.to_owned(),
            },
            authors,
        })
    }
}

fn load_config() -> Result<Config> {
    let project_dir = ProjectDirs::from("", "", "mic-mute").expect("Unable to find home directory");
    let stg_file = project_dir.preference_dir().join("settings.toml");

    // If the config file does not exist yet, create it with default values
    if !stg_file.exists() {
        fs::create_dir_all(project_dir.preference_dir())?;
        fs::write(
            &stg_file,
            toml::to_string(&HashMap::from([("shortcut", DEFAULT_MUTE_KEYS)]))?,
        )?;
    }

    // Load config file
    println!("Loading configuration file {}", stg_file.to_string_lossy());
    Ok(Config::builder()
        .add_source(config::File::from(stg_file))
        .build()
        .unwrap())
}
