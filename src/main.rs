mod cli;
mod sub;

// TODO logging
// TODO exit code
fn main() {
    match pre::read_config() {
        Ok(mut cfg) => {
            cli::exec(&mut cfg);
        },
        Err(e) => eprintln!("{}", e),
    }
}
