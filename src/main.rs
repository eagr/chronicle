mod cli;
mod sub;

// TODO logging
// TODO exit code
fn main() {
    match pre::read_config() {
        Ok(mut cfg) => {
            match cli::exec(&mut cfg) {
                Ok(_) => (),
                Err(e) => eprintln!("{e}"),
            }
        },
        Err(e) => eprintln!("{e}"),
    }
}
