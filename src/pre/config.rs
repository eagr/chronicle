use anyhow::{Result, bail};
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ChronicleConfig {
    #[serde(default)]
    pub storage: String,

    #[serde(default)]
    pub date: String,

    #[serde(default)]
    pub time: String,
}

impl ChronicleConfig {
    pub fn new(storage: &String) -> Self {
        Self {
            storage: storage.to_owned(),
            date: String::new(),
            time: String::new(),
        }
    }
}

fn def_date() -> String { String::from("%Y-%m-%d") }

fn def_time() -> String { String::from("%H:%M") }

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default: String,

    #[serde(default = "def_date")]
    pub date: String,

    #[serde(default = "def_time")]
    pub time: String,

    #[serde(default)]
    pub editor: String,

    pub chronicle: HashMap<String, ChronicleConfig>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            default: String::new(),
            date: def_date(),
            time: def_time(),
            editor: String::new(),
            chronicle: HashMap::new(),
        }
    }

    pub fn exists(self: &Self, name: &String) -> bool {
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

pub fn draft_path(name: &String) -> PathBuf {
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
                        .map(|f| f)
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
        .map(|c| c)
        .map_err(|e| e.into())
}

pub fn write_config(cfg: &mut Config) -> Result<()> {
    let p = config_path();
    let b = toml::to_vec(cfg)?;
    fs::write(&p, &b)?;
    Ok(())
}
