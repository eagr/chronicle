use pre::*;

pub fn build() -> Cli {
    cmd("new")
        .about("create new chronicle")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("storage").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;
    let storage = try_get_arg(args, "storage")?;

    if cfg.chronicle.contains_key(name) {
        bail!("chronicle {name} already exits");
    }

    cfg.chronicle.insert(name.to_owned(), ChronicleConfig::new(storage));
    write_config(cfg)
}
