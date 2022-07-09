use pre::*;

use crate::sub;

pub fn exec(cfg: &mut Config) -> std::io::Result<()> {
    let matches = build(cfg).get_matches();
    let (cmd, args) = match matches.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => panic!(""),
    };

    try_exec_sub(cfg, cmd, args);
    Ok(())
}

fn build(cfg: &mut Config) -> Cli {
    let mut cli = Command::new("chronicle")
        .version(crate_version!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommands(sub::commands());

    cli
}

fn try_exec_sub(cfg: &mut Config, cmd: &str, args: &ArgMatches) {
    if let Some(proc) = sub::find_proc(cmd) {
        return proc(cfg, args);
    }
}
