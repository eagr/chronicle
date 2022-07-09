mod cli;
mod sub;

use pre::config;

// TODO logging
// TODO exit code
fn main() {
    let mut cfg = config::read();
    let res = cli::exec(&mut cfg);
}
