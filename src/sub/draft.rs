use pre::*;

use chrono::Local;

pub fn build() -> Cli {
    cmd("draft")
        .about("Draft chronicle")
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

    let timestamp = Local::now().timestamp();
    let line = timestamp.to_string() + " " + event;

    append(&path, &line)
}
