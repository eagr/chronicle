use pre::*;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn build() -> Cli {
    Command::new("ink")
        .about("ink logs onto chronicle")
        .arg(Arg::new("chron_name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let chron_name = args.get_one::<String>("chron_name").unwrap();

    if !cfg.exists(chron_name) {
        panic!("");
    }

    let storage = &cfg.chronicle.get(chron_name).unwrap().storage;

    if storage.is_empty() {
        panic!("");
    }

    let chron_home = chron_dir();
    let draft_path = chron_home.join(chron_name);
    let map = parse(&draft_path);
    let mut new_cont = String::new();
    for (date, events) in &map {
        new_cont.push_str(&format!("## {date}\n\n"));
        for ev in events {
            new_cont.push_str(&format!("- {ev}\n"));
        }
        new_cont.push_str(&"\n");
    }

    let mut tmp = tempfile::NamedTempFile::new_in(chron_home).unwrap();
    tmp.write(new_cont.as_bytes()).unwrap();

    let mut src = File::open(storage).unwrap();
    io::copy(&mut src, &mut tmp).unwrap();
    tmp.persist(storage).unwrap();

    File::create(&draft_path).unwrap();
}

fn parse<P>(path: P) -> BTreeMap<String, Vec<String>>
where P: AsRef<Path>,
{
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let lines = read_lines(&path);
    for line in lines {
        let mut dv: &mut Vec<String> = &mut Vec::new();
        if let Ok(ln) = line {
            let (date, rest) = ln.split_once(' ').unwrap();
            let date = date.to_string();
            let rest = rest.to_string();
            if map.contains_key(&date) {
                dv = map.get_mut(&date).unwrap();
                dv.push(rest);
            } else {
                map.insert(date, vec![rest]);
            }
        }
        dv.sort();
    }

    map
}

fn read_lines<P>(path: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>,
{
    let fd = File::open(path).unwrap();
    io::BufReader::new(fd).lines()
}
