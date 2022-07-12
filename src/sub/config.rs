use pre::*;

pub fn build() -> Cli {
    cmd("config")
        .about("configure chron")
        .args(&[
            arg!(--editor <EDITOR_BIN> "set default editor for rewording"),
            arg!(--date <DATE_FORMAT> "set date format"),
            arg!(--time <TIME_FORMAT> "set time format"),
        ])
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) {
    if let Ok(Some(editor)) = args.try_get_one::<String>("editor") {
        cfg.editor = editor.to_string();
    }

    if let Ok(Some(date_fmt)) = args.try_get_one::<String>("date") {
        cfg.date = date_fmt.to_string();
    }

    if let Ok(Some(time_fmt)) = args.try_get_one::<String>("time") {
        cfg.time = time_fmt.to_string();
    }

    write_config(cfg);
}
