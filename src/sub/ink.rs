use pre::*;

use anyhow::{Result};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::Path;

// TODO --prepend / --append
pub fn build() -> Cli {
    cmd("ink")
        .about("ink onto chronicle")
        .arg(Arg::new("name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;

    if !cfg.exists(name) {
        bail!("no chronicle named '{name}'");
    }

    let storage = &cfg.chronicle
        .get(name)
        .context(format!("failed to read config of '{name}'"))?
        .storage;

    if storage.is_empty() {
        bail!("'{name}' storage not set");
    }

    let chron_dir = dir();
    let draft_path = draft_path(name);
    let map = parse(&draft_path)?;

    let mut new_ink = String::new();
    for (date, events) in &map {
        let mut day = String::new();
        day.push_str(&format!("## {date}\n\n"));
        for ev in events {
            day.push_str(&format!("- {ev}\n"));
        }
        day.push_str("\n");

        new_ink = day + &new_ink;
    }

    // steps to prepend new ink
    // 0. create tmp file in the same dir as storage file
    // 1. write new content to tmp file
    // 2. append entire storage state to tmp file
    // 3. rename tmp file to the same name as storage file
    let mut tmp = tempfile::NamedTempFile::new_in(chron_dir)?;
    tmp.write(new_ink.as_bytes())?;
    let mut store = File::open(storage)?;
    io::copy(&mut store, &mut tmp)?;
    tmp.persist(storage)?;

    // wife off draft
    File::create(&draft_path)?;
    Ok(())
}

fn parse<P>(path: P) -> Result<BTreeMap<String, Vec<String>>>
where P: AsRef<Path>,
{
    let lines = read_lines(&path)?;
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in lines {
        if let Ok(ln) = line {
            let (date, event) = ln.split_once(' ').context("failed to split line")?;
            let event = event.to_string();

            if let Some(events) = map.get_mut(date) {
                events.push(event);
            } else {
                map.insert(date.to_string(), vec![event]);
            }
        }
    }

    for (_, events) in &mut map {
        events.sort();
    }

    Ok(map)
}

fn read_lines<P>(path: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>,
{
    let fd = File::open(path)?;
    Ok(BufReader::new(fd).lines())
}
