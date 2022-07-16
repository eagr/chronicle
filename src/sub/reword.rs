use pre::*;

pub fn build() -> Cli {
    cmd("reword")
        .about("Reword draft")
        .arg(Arg::new("name").required(true))
}

pub fn proc(cfg: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;

    if cfg.editor.is_empty() {
        bail!("Set the editor with `chron config --editor <EDITOR_BIN>` and try again.");
    }

    if !cfg.exists(name) {
        bail!("no chronicle named '{name}'");
    }

    let path = draft_path(name);
    std::process::Command::new(&cfg.editor)
        .arg(path)
        .spawn()?
        .wait()?;

    Ok(())
}
