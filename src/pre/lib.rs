pub mod config;

pub use anyhow::{Context, Error, anyhow, bail};
pub use clap::arg;
pub use clap::{AppSettings, Arg, ArgAction, ArgMatches, Command};

use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::PathBuf;

pub use config::{Config, ChronicleConfig};
pub use config::{dir, backup_dir, config_path, draft_path};
pub use config::{read_config, write_config};

pub type Cli = clap::Command<'static>;
pub type CliErr = Error;
pub type CliRes = Result<(), CliErr>;

pub fn cmd(name: &'static str) -> Cli {
    Cli::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

pub fn try_get_arg<'a>(args: &'a ArgMatches, name: &str) -> anyhow::Result<&'a String> {
    args.get_one::<String>(name)
        .ok_or_else(|| anyhow!("could not get arg `{name}`"))
}

pub fn append(path: &PathBuf, line: &String) -> CliRes {
    let mut fd = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let ln = line.to_owned() + "\n";
    fd.write_all(ln.as_bytes())?;
    Ok(())
}
