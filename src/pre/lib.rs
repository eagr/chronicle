pub mod config;

use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::PathBuf;

pub use clap::AppSettings;
pub use clap::Arg;
pub use clap::ArgAction;
pub use clap::ArgMatches;
pub use clap::Command;
pub use clap::arg;
pub use clap::crate_version;
pub type Cli = clap::Command<'static>;

pub use config::{Config, ChronicleConfig, home};

pub fn cmd(name: &'static str) -> Cli {
    Cli::new(name)
        .dont_collapse_args_in_usage(true)
        .setting(AppSettings::DeriveDisplayOrder)
}

pub fn append_line(path: &PathBuf, line: &String) {
    let mut fd = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let ln = line.to_owned() + "\n";
    fd.write(ln.as_bytes());
}
