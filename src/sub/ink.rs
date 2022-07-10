use pre::*;

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

    // fs::read_to_string
}
