use std::error::Error;

pub fn parse_filepath() -> Result<String, Box<dyn Error>> {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.len() != 2 {
        return Err("no filename specified".into());
    }

    Ok(raw_args[1].clone())
}
