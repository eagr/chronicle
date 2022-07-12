use pre::*;

use std::fs::{self, OpenOptions};
use std::io;

pub fn build() -> Cli {
    cmd("erase")
        .about("wipe off draft buffer")
        .arg(Arg::new("name").required(true))
}

pub fn proc(_: &mut Config, args: &ArgMatches) -> CliRes {
    let name = try_get_arg(args, "name")?;

    let bd = backup_dir();
    fs::create_dir_all(&bd)?;

    let dp = draft_path(name);
    let mut draft = OpenOptions::new().create(true).read(true).write(true).open(&dp)?;

    let bp = bd.join(name);
    let mut backup = OpenOptions::new().create(true).append(true).open(&bp)?;

    io::copy(&mut draft, &mut backup)?;
    draft.set_len(0)?;

    Ok(())
}
