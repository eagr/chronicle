use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{PathBuf};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

pub fn home() -> PathBuf {
    home::home_dir().unwrap().join(".chronicle")
}

pub fn read() -> Config {
    let p = cfg_path();
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
    let p = cfg_path();
    let b = toml::to_vec(cfg).unwrap();
    fs::write(p, b).unwrap();
}

fn cfg_path() -> PathBuf {
    cfg_home().join("config.toml")
}

fn cfg_home() -> PathBuf {
    home::home_dir().unwrap().join(".chronicle")
}

fn cfg_init() {
    let ch = cfg_home();
    fs::create_dir_all(&ch).expect("failed to create config dir");
    let cp = ch.join("config.toml");
    let mut fd = File::create(&cp).expect("failed to create config.toml");

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
