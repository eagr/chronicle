use pre::*;

pub fn build() -> Cli {
    Command::new("reword")
        .about("reword draft")
        .arg(Arg::new("chron_name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let chron_name = args.get_one::<String>("chron_name").unwrap();

    // TODO msg: `chronicle config --editor <EDITOR_BIN>`
    if cfg.editor.is_empty() { panic!("") }
    if !cfg.exists(chron_name) { panic!("") }

    let draft_path = home().join(chron_name);
    std::process::Command::new(&cfg.editor)
        .arg(draft_path)
        .spawn()
        .expect("failed to launch vim")
        .wait()
        .expect("editor exited with with non-zero code");
}
