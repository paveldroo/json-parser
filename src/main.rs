use std::error::Error;

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
    match parser::parse_json("some_input") {
        Ok(data) => println!("json data: {data:?}"),
        Err(err) => return Err(format!("parse error: {err}").into()),
    }

    Ok(())
}
