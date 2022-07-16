use pre::*;

pub fn build() -> Cli {
    cmd("new")
        .about("Create new chronicle")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("store").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;
    let store = try_get_arg(args, "store")?;

    if cfg.chronicle.contains_key(name) {
        bail!("chronicle {name} already exits");
    }

    cfg.chronicle.insert(name.to_owned(), ChronicleConfig::new(store));
    write_config(cfg)
}
