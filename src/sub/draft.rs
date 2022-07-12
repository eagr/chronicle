use pre::*;

use chrono::Local;

pub fn build() -> Cli {
    cmd("draft")
        .about("draft chronicle before inking")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("event").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;
    let event = try_get_arg(args, "event")?;

    if !cfg.chronicle.contains_key(name) {
        bail!("no chronicle named '{name}'");
    }

    let path = draft_path(name);

    let chron_cfg = cfg.chronicle.get(name).context(format!("could not get configuration of '{name}'"))?;
    let date_fmt = if chron_cfg.date.is_empty() { cfg.date.to_string() } else { chron_cfg.date.to_string() };
    let time_fmt = if chron_cfg.time.is_empty() { cfg.time.to_string() } else { chron_cfg.time.to_string() };
    let dt_fmt = format!("{date_fmt} {time_fmt}");
    let now = Local::now().format(&dt_fmt).to_string();
    let line = now + " " + &event;

    append(&path, &line)
}
