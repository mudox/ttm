use std::fs;

use crate::config::{data_path, themes_dir, Config, Task};
use crate::logging::*;

pub fn run(config: Config) {
    debug!("config: {:#?}", &config);

    match &config.task {
        Task::Get(session) => get(&config, session),
        Task::Set(session, name) => set(config.clone(), &session, &name),
    }
}

fn get(config: &Config, theme: &Option<String>) {
    match theme {
        Some(name) => {
            if let Some(theme) = config.themes.get(name) {
                println!("{}", theme);
            }
        }
        _ => {
            for (name, theme) in &config.themes {
                println!("{}: {}", name, theme);
            }
        }
    }
}

fn set(mut config: Config, session: &str, theme: &str) {
    let path = themes_dir().join(theme);
    if !path.exists() {
        eprintln!("invalid theme name: {}", theme);
        return;
    }

    config.themes.insert(session.to_string(), theme.to_string());
    let toml = toml::to_string(&config.themes).unwrap();
    fs::write(data_path(), &toml).unwrap();
}
