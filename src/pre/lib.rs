pub mod config;

pub use clap::arg;
pub use clap::{AppSettings, Arg, ArgAction, ArgMatches, Command};

use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::PathBuf;

pub use config::{Config, ChronicleConfig};
pub use config::{chron_dir, chron_backup_dir, chron_config_path};

pub type Cli = clap::Command<'static>;
pub type CliErr = anyhow::Error;
pub type CliRes = Result<(), CliErr>;

pub fn cmd(name: &'static str) -> Cli {
    Cli::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

pub fn append(path: &PathBuf, line: &String) {
    let mut fd = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let ln = line.to_owned() + "\n";
    fd.write(ln.as_bytes());
}
