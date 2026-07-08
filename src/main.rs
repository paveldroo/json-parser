use std::{
    error::Error,
    io::{self, IsTerminal, Read},
};

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
    let content = match args::parse_filepath() {
        Ok(filename) => reader::read_file(&filename)?,
        Err(_) => {
            if io::stdin().is_terminal() {
                return Err("missing filename as argument".into());
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            if buffer.is_empty() {
                return Err("no stdin data and no filename was provided".into());
            }
            buffer
        }
    };
    match parser::parse_json(content.clone()) {
        Ok(_) => {
            println!("json data: {content}");
            Ok(())
        }
        Err(err) => Err(format!("parse error: {err}").into()),
    }
}
