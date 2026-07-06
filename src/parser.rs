use std::{collections::HashMap, error::Error};

pub fn parse_json(input: &str) -> Result<(), Box<dyn Error>> {
    if input.len() == 0 {
        return Err("json invalid".into());
    }

    let pairs = HashMap::from([('}', '{')]);
    let mut parentheses: Vec<char> = Vec::new();

    for ch in input.chars() {
        if parentheses.len() == 0 {
            parentheses.push(ch);
            continue;
        }

        if parentheses.last() == pairs.get(&ch) {
            parentheses.pop();
        } else {
            parentheses.push(ch);
        }
    }

    match parentheses.len() {
        0 => Ok(()),
        _ => Err("json invalid".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step1_valid() {
        let data = crate::reader::read_file("step1", "valid.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_step1_invalid() {
        let data = crate::reader::read_file("step1", "invalid.json").unwrap();
        match parse_json(&data) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
