use pre::*;

use std::fs;

pub fn build() -> Cli {
    Command::new("erase")
        .about("wipe off draft buffer")
        .arg(Arg::new("chron_name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let chron_name = args.get_one::<String>("chron_name").unwrap();

    let draft_path = chron_dir().join(chron_name);
    let draft_cont = match fs::read_to_string(&draft_path) {
        Ok(cont) => cont,
        Err(_) => String::new(),
    };

    if draft_cont.is_empty() { return; }

    let backup_dir = chron_backup_dir();
    fs::create_dir_all(&backup_dir).unwrap();
    let backup_file = backup_dir.join(chron_name);
    fs::write(&backup_file, &draft_cont).unwrap();
    fs::write(&draft_path, "").unwrap();
}
