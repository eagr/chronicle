use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ChronicleConfig {
    pub storage: String,

    #[serde(default)]
    pub date: String,

    #[serde(default)]
    pub time: String,
}

impl ChronicleConfig {
    pub fn new(storage: &String) -> Self {
        Self {
            storage: storage.to_string(),
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
    pub fn exists(self: &Self, name: &String) -> bool {
        self.chronicle.contains_key(name)
    }
}

pub fn chron_dir() -> PathBuf {
    home::home_dir().unwrap().join("./chronicle")
}

pub fn chron_backup_dir() -> PathBuf {
    chron_dir().join("backup~")
}

pub fn chron_config_path() -> PathBuf {
    chron_dir().join("config.toml")
}

pub fn read() -> Config {
    let p = chron_config_path();
    let mut fd = File::open(&p).unwrap_or_else(|err| {
        match err.kind() {
            io::ErrorKind::NotFound => {
                cfg_init();
                File::open(&p).unwrap()
            },
            _ => panic!("failed to open config.toml: {err:?}"),
        }
    });

    let mut s = String::new();
    fd.read_to_string(&mut s).expect("failed to read config.toml");

    let c: Config = toml::from_str(&s).unwrap();
    c
}

pub fn write(cfg: &mut Config) {
    let p = chron_config_path();
    let b = toml::to_vec(cfg).unwrap();
    fs::write(p, b).unwrap();
}

fn cfg_init() {
    let dir = chron_dir();
    fs::create_dir_all(&dir).expect("failed to create config dir");
    let p = chron_config_path();
    let mut fd = File::create(&p).expect("failed to create config.toml");

    let c = Config {
        default: String::new(),
        date: def_date(),
        time: def_time(),
        editor: String::new(),
        chronicle: HashMap::new(),
    };
    let b = toml::to_vec(&c).unwrap();
    fd.write_all(&b).unwrap();
}
