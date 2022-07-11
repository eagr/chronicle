use chrono::Local;

use pre::*;

pub fn build() -> Cli {
    cmd("draft")
        .about("log to cache before inking it onto chronicle")
        .arg(Arg::new("chron_name").required(true))
        .arg(Arg::new("event").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let chron_name = args.get_one::<String>("chron_name").unwrap();
    let event = args.get_one::<String>("event").unwrap();

    if !cfg.chronicle.contains_key(chron_name) {
        panic!("no chronicle named {chron_name}");
    }

    let draft_path = chron_dir().join(chron_name);

    let c = cfg.chronicle.get(chron_name).unwrap();
    let date_fmt = if c.date.is_empty() { cfg.date.to_string() } else { c.date.to_string() };
    let time_fmt = if c.time.is_empty() { cfg.time.to_string() } else { c.time.to_string() };
    let fmt = format!("{date_fmt} {time_fmt}");
    let now = Local::now().format(&fmt).to_string();
    let line = now + " " + event;

    append(&draft_path, &line);
}
