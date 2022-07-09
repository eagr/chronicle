use pre::*;

pub fn build() -> Cli {
    cmd("config")
        .about("configure chronicle")
        .args(&[
            arg!(--editor <EDITOR_BIN> "set default editor for rewording"),
        ])
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    let editor = args.get_one::<String>("editor").unwrap();
    cfg.editor = editor.to_string();
    config::write(cfg);
}
