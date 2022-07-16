use anyhow::{Result, bail};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct ChronicleConfig {
    pub store: String,
    pub reverse: Option<bool>,
    pub date: Option<String>,
    pub time: Option<String>,
}

impl ChronicleConfig {
    pub fn new(store: &str) -> Self {
        Self {
            store: store.to_string(),
            ..Default::default()
        }
    }
}

fn reverse_default() -> bool { true }
fn date_default() -> String { String::from("%Y-%m-%d") }
fn time_default() -> String { String::from("%H:%M") }

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    #[serde(default = "reverse_default")]
    pub reverse: bool,
    #[serde(default = "date_default")]
    pub date: String,
    #[serde(default = "time_default")]
    pub time: String,
    pub chronicle: HashMap<String, ChronicleConfig>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            reverse: reverse_default(),
            date: date_default(),
            time: time_default(),
            ..Default::default()
        }
    }

    pub fn exists(&self, name: &str) -> bool {
        self.chronicle.contains_key(name)
    }
}

pub fn dir() -> PathBuf {
    home::home_dir().unwrap().join(".chronicle")
}

pub fn backup_dir() -> PathBuf {
    dir().join("backup~")
}

pub fn config_path() -> PathBuf {
    dir().join("config.toml")
}

pub fn draft_path(name: &str) -> PathBuf {
    dir().join(name)
}

fn init_config() -> Result<()> {
    let d = dir();
    fs::create_dir_all(&d)?;
    let p = config_path();
    let mut f = File::create(&p)?;

    let c = Config::new();
    let b = toml::to_vec(&c)?;
    f.write_all(&b)?;
    Ok(())
}

pub fn read_config() -> Result<Config> {
    let p = config_path();
    let f = File::open(&p).or_else(|err| {
        match err.kind() {
            io::ErrorKind::NotFound => {
                init_config().and_then(|()| {
                    File::open(&p)
                        .map_err(|e| e.into())
                })
            },
            _ => Err(anyhow::Error::from(err))
        }
    });

    if let Err(e) = f {
        bail!(e);
    }

    let mut f = f?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    toml::from_str(&s)
        .map_err(|e| e.into())
}

pub fn write_config(cfg: &mut Config) -> Result<()> {
    let p = config_path();
    let b = toml::to_vec(cfg)?;
    fs::write(&p, &b)?;
    Ok(())
}
