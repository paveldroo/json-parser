use std::error::Error;

mod args;
mod parser;
mod reader;

fn main() {
    match run() {
        Ok(_) => println!("json-parser: json is valid!"),
        Err(err) => {
            eprintln!("json-parser: {err}");
            std::process::exit(1)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let filepath = args::parse_filepath()?;
    let input = reader::read_file(&filepath)?;
    match parser::parse_json(input.to_string()) {
        Ok(_) => {
            println!("json data: {input}");
            Ok(())
        }
        Err(err) => Err(format!("parse error: {err}").into()),
    }
}
