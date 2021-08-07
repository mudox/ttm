use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use clap::{app_from_crate, App, Arg};

use crate::logging::*;

type Themes = HashMap<String, String>;

pub fn themes_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".tmux/tmuxline-files");
    path
}

pub fn data_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".tmux/session-themes.toml");
    path
}

#[derive(Debug, Clone)]
pub enum Task {
    Get(Option<String>),
    Set(String, String),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub task: Task,
    pub themes: Themes,
}

/// Program configuration.
impl Config {
    pub fn load() -> Config {
        let mut cfg = Config {
            task: Task::Get(None),
            themes: Self::load_themes(),
        };

        cfg.parse_args();

        cfg
    }

    fn load_themes() -> Themes {
        let mut text = String::new();
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(data_path())
            .unwrap();
        file.read_to_string(&mut text).unwrap();
        toml::from_str::<HashMap<String, String>>(&text).unwrap()
    }

    fn parse_args(&mut self) {
        let get_cmd = App::new("get")
            .visible_alias("g")
            .about("Get (tmuxline) theme of given tmux session")
            .arg(
                Arg::new("SESSION")
                    .about("Session name")
                    .required(true)
                    .index(1),
            );

        let set_cmd = App::new("set")
            .visible_alias("s")
            .about("Set (tmuxline) theme of given tmux session")
            .arg(
                Arg::new("SESSION")
                    .about("Session name")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("THEME")
                    .about("Theme name")
                    .required(true)
                    .index(2),
            );

        let app = app_from_crate!().subcommand(get_cmd).subcommand(set_cmd);

        let matches = app.get_matches();

        self.task = if let Some(matches) = matches.subcommand_matches("set") {
            let session = matches.value_of("SESSION").unwrap().to_string();
            let theme = matches.value_of("THEME").unwrap().to_string();
            Task::Set(session, theme)
        } else if let Some(matches) = matches.subcommand_matches("get") {
            let session = matches.value_of("SESSION").map(String::from);
            Task::Get(session)
        } else {
            Task::Get(None)
        };
    }
}
