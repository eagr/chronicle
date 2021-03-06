use pre::*;

use anyhow::{Result};
use chrono::{Local, TimeZone};

use std::collections::BTreeMap;
use std::fmt::{Write as _};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Lines, BufRead as _, Write as _};
use std::path::Path;

pub fn build() -> Cli {
    cmd("ink")
        .about("Ink draft onto chronicle")
        .arg(Arg::new("name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;

    if !cfg.exists(name) {
        bail!("no chronicle named '{name}'");
    }

    let store_paths = &cfg.chronicle
        .get(name)
        .context(format!("failed to read config of '{name}'"))?
        .stores;

    if store_paths.is_empty() {
        bail!("'{name}' store not set");
    }

    let chron_dir = dir();
    let draft_path = draft_path(name);

    let chron_cfg = cfg.chronicle.get(name).context(format!("could not get config of '{name}'"))?;
    let is_rev = if let Some(reverse) = &chron_cfg.reverse { *reverse } else { cfg.reverse };
    let date_fmt = if let Some(date) = &chron_cfg.date { date } else { &cfg.date };
    let time_fmt = if let Some(time) = &chron_cfg.time { time } else { &cfg.time };

    let map = parse(&draft_path, date_fmt, time_fmt)?;
    let mut new_ink = String::new();
    for (date, events) in &map {
        let mut day = String::new();
        write!(day, "## {date}\n\n")?;

        for ev in events {
            writeln!(day, "- {ev}")?;
        }
        day.push('\n');

        if is_rev {
            new_ink = day + &new_ink;
        } else {
            new_ink.push_str(&day);
        }
    }

    let bytes = new_ink.as_bytes();
    if is_rev {
        // steps to prepend new ink
        // 0. create tmp file in the .chronicle dir
        // 1. write new content to tmp file
        // 2. append entire store state to tmp file
        // 3. rename tmp file to the same name as store file
        for path in store_paths {
            let mut tmp = tempfile::NamedTempFile::new_in(&chron_dir)?;
            tmp.write_all(bytes)?;
            let mut store = File::open(path)?;
            io::copy(&mut store, &mut tmp)?;
            tmp.persist(path)?;
        }
    } else {
        let mut oo = OpenOptions::new();
        let oo = oo.append(true);
        for path in store_paths {
            let mut store = oo.open(path)?;
            store.write_all(bytes)?;
        }
    }

    // wife off draft
    File::create(&draft_path)?;
    Ok(())
}

fn parse<P>(path: P, date_fmt: &str, time_fmt: &str) -> Result<BTreeMap<String, Vec<String>>>
where P: AsRef<Path>,
{
    let lines = read_lines(&path)?;

    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ln in lines.flatten() {
        let (timestamp, event) = ln.split_once(' ').context("failed to split line")?;

        let timestamp = timestamp.parse::<i64>()?;
        let dt = Local.timestamp(timestamp, 0);
        let date = dt.format(date_fmt).to_string();
        let time = dt.format(time_fmt).to_string();
        let event = time + " " + event;

        if let Some(events) = map.get_mut(&date) {
            events.push(event);
        } else {
            map.insert(date, vec![event]);
        }
    }

    for events in map.values_mut() {
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
