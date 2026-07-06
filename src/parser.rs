use std::{collections::HashMap, error::Error};

pub fn parse_json(input: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Err("json invalid".into());
    }

    let pairs = HashMap::from([('}', '{'), ('{', '}'), ('"', '"')]);
    let mut parentheses: Vec<char> = Vec::new();

    for (idx, ch) in input.chars().enumerate() {
        if parentheses.is_empty() && pairs.contains_key(&ch) {
            println!("pushing char {ch}");
            parentheses.push(ch);
            continue;
        }

        if ch == '}' && idx > 0 && input.chars().nth(idx - 1).unwrap() == ',' {
            return Err("json invalid".into());
        }

        if ch == ':' && idx > 0 && input.chars().nth(idx - 1).unwrap() != '"' {
            return Err("json invalid".into());
        }

        match pairs.get(&ch) {
            Some(c) => {
                if parentheses.last().unwrap() == c {
                    parentheses.pop();
                } else {
                    parentheses.push(*c);
                }
            }
            _ => continue,
        }
    }

    match parentheses.len() {
        0 => Ok(()),
        _ => {
            dbg!(parentheses);
            Err("json invalid".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step2_valid() {
        let data = crate::reader::read_file("tests/fixtures/step2/valid.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_invalid() {
        let data = crate::reader::read_file("tests/fixtures/step2/invalid.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_valid2() {
        let data = crate::reader::read_file("tests/fixtures/step2/valid2.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(true, "json data: {data:?}"),
            Err(_) => assert!(false, "json data: {data:?}"),
        }
    }

    #[test]
    fn test_step2_invalid2() {
        let data = crate::reader::read_file("tests/fixtures/step2/invalid2.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(false, "json data: {data:?}"),
            Err(_) => assert!(true, "json data: {data:?}"),
        }
    }
}
