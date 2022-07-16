use pre::*;

pub fn build() -> Cli {
    cmd("config")
        .about("Configure chron")
        .args(&[
            arg!(--editor <EDITOR_BIN> "set editor for rewording").required(false),
            arg!(--reverse [IS_REVERSE] "set event order: reverse chronological or not")
                .default_missing_value("true"),
            arg!(--date <DATE_FORMAT> "set date format").required(false),
            arg!(--time <TIME_FORMAT> "set time format").required(false),
        ])
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    if let Ok(Some(editor)) = args.try_get_one::<String>("editor") {
        cfg.editor = editor.to_owned();
    }
    if let Ok(Some(reverse)) = args.try_get_one::<String>("reverse") {
        cfg.reverse = !matches!(reverse.as_bytes(), b"f" | b"F" | b"0");
    }
    if let Ok(Some(date_fmt)) = args.try_get_one::<String>("date") {
        cfg.date = date_fmt.to_owned();
    }
    if let Ok(Some(time_fmt)) = args.try_get_one::<String>("time") {
        cfg.time = time_fmt.to_owned();
    }
    write_config(cfg)
}
