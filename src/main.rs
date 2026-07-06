use std::error::Error;

fn main() {
    if let Err(err) = run() {
        eprintln!("json-parser: {err}");
        std::process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
