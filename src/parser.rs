#![allow(unused)]
use std::{collections::HashMap, error::Error};

pub fn parse_json(mut input: String) -> Result<(), Box<dyn Error>> {
    input = input.trim().to_string();

    if input.is_empty() {
        return Err("json invalid".into());
    }

    if !input.starts_with('{') || !input.ends_with('}') {
        return Err("json invalid".into());
    }

    input = input
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .to_string();

    if input.is_empty() {
        return Ok(());
    }

    input = input.trim().replace("\n", "");
    let mut pairs: Vec<&str> = input.split(",").map(|split| split.trim()).collect();

    for pair in pairs.iter() {
        parse_pair(pair)?;
    }

    Ok(())
}

fn parse_pair(input: &str) -> Result<(), Box<dyn Error>> {
    let splits: Vec<&str> = input.split(":").map(|token| token.trim()).collect();
    for t in splits.iter() {
        if !t.starts_with('"') && !t.ends_with('"') {
            if *t == "true" || *t == "false" || *t == "null" {
                continue;
            }
            let numeric = match t.parse::<i32>() {
                Ok(_) => continue,
                Err(_) => return Err("json invalid".into()),
            };
        }
        if !t.starts_with('"') || !t.ends_with('"') {
            return Err("json invalid".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step1_valid() {
        let data = crate::reader::read_file("tests/fixtures/step1/valid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step1_invalid() {
        let data = crate::reader::read_file("tests/fixtures/step1/invalid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_valid() {
        let data = crate::reader::read_file("tests/fixtures/step2/valid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_invalid() {
        let data = crate::reader::read_file("tests/fixtures/step2/invalid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_valid2() {
        let data = crate::reader::read_file("tests/fixtures/step2/valid2.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_invalid2() {
        let data = crate::reader::read_file("tests/fixtures/step2/invalid2.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step3_valid() {
        let data = crate::reader::read_file("tests/fixtures/step3/valid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step3_invalid() {
        let data = crate::reader::read_file("tests/fixtures/step3/invalid.json").unwrap();
        match parse_json(data.to_string()) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }
}
