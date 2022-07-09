mod new;
mod config;
mod draft;
mod reword;
mod erase;
mod ink;

use pre::*;

pub fn commands() -> Vec<Cli> {
    vec![
        new::build(),
        config::build(),
        draft::build(),
        reword::build(),
        erase::build(),
        ink::build(),
    ]
}

pub fn find_proc(cmd: &str) -> Option<fn(&mut Config, &ArgMatches)> {
    let proc = match cmd {
        "new" => new::proc,
        "config" => config::proc,
        "draft" => draft::proc,
        "reword" => reword::proc,
        "erase" => erase::proc,
        "ink" => ink::proc,
        _ => return None,
    };
    Some(proc)
}
