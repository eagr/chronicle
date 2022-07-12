use pre::*;

use crate::sub;

pub fn exec(cfg: &mut Config) -> CliRes {
    let mut m = build().get_matches();
    let (cmd, args) = match m.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => {
            m = build().get_matches_from(vec!["chron", "help"]);
            m.subcommand().unwrap()
        },
    };

    exec_sub(cfg, cmd, args);
    Ok(())
}

fn build() -> Cli {
    Command::new("chron")
        .version(clap::crate_version!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommands(sub::commands())
}

fn exec_sub(cfg: &mut Config, cmd: &str, args: &ArgMatches) {
    if let Some(proc) = sub::find_proc(cmd) {
        return proc(cfg, args);
    }
}
