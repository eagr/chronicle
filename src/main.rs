mod cli;
mod sub;

// TODO logging
// TODO exit code
fn main() {
    let mut cfg = pre::config::read();
    let res = cli::exec(&mut cfg);
}
