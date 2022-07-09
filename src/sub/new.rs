use pre::*;

pub fn build() -> Cli {
    cmd("new")
        .about("create a new chronicle")
        .arg(Arg::new("chron_name").required(true))
        .arg(Arg::new("storage").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let chron_name = args.get_one::<String>("chron_name").unwrap();
    let storage = args.get_one::<String>("storage").unwrap();

    if cfg.chronicle.contains_key(chron_name) {
        panic!("{chron_name} already exits");
    }

    cfg.chronicle.insert(chron_name.to_string(), ChronicleConfig::new(storage));

    config::write(cfg)
}
