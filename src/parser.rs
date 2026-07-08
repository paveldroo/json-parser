use std::error::Error;

pub fn parse_json(mut input: &str) -> Result<(), Box<dyn Error>> {
    input = input.trim();

    if input.is_empty() {
        return Err("json invalid".into());
    }

    if !input.starts_with('{') || !input.ends_with('}') {
        return Err("json invalid".into());
    }

    input = input.strip_prefix('{').unwrap().strip_suffix('}').unwrap();

    if input.is_empty() {
        return Ok(());
    }

    let pairs: Vec<&str> = input.split(",").map(|split| split.trim()).collect();

    for pair in pairs.iter() {
        if pair.is_empty() {
            return Err("json invalid".into());
        }
        parse_pair(pair)?;
    }

    Ok(())
}

fn parse_pair(input: &str) -> Result<(), Box<dyn Error>> {
    let (key, value) = input.split_once(":").ok_or("json invalid")?;
    parse_token(key.trim().to_string()).map_err(|_| "json_invalid")?;
    parse_token(value.trim().to_string()).map_err(|_| "json_invalid")?;

    Ok(())
}

fn parse_list(mut input: String) -> Result<(), Box<dyn Error>> {
    if input == "[]" {
        return Ok(());
    }
    input = input
        .strip_prefix("[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .trim()
        .to_string();

    let splits: Vec<&str> = input.split(',').map(|token| token.trim()).collect();
    for s in splits.iter() {
        parse_token(s.to_string()).map_err(|_| "json invalid")?;
    }
    Ok(())
}

fn parse_token(t: String) -> Result<(), Box<dyn Error>> {
    if t.starts_with('{') && t.ends_with('}') {
        return parse_json(&t);
    }
    if t.starts_with('[') && t.ends_with(']') {
        return parse_list(t.to_string());
    }
    if !t.starts_with('"') && !t.ends_with('"') {
        if t == "true" || t == "false" || t == "null" {
            return Ok(());
        }
        match t.parse::<i32>() {
            Ok(_) => return Ok(()),
            Err(_) => return Err("json invalid".into()),
        };
    }
    if !t.starts_with('"') || !t.ends_with('"') {
        return Err("json invalid".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, bool)] = &[
        ("tests/fixtures/step1/valid.json", true),
        ("tests/fixtures/step1/invalid.json", false),
        ("tests/fixtures/step2/valid.json", true),
        ("tests/fixtures/step2/valid2.json", true),
        ("tests/fixtures/step2/invalid.json", false),
        ("tests/fixtures/step2/invalid2.json", false),
        ("tests/fixtures/step3/valid.json", true),
        ("tests/fixtures/step3/invalid.json", false),
        ("tests/fixtures/step4/valid.json", true),
        ("tests/fixtures/step4/valid2.json", true),
        ("tests/fixtures/step4/invalid.json", false),
    ];

    #[test]
    fn fixtures_validate_as_expected() {
        for &(path, want_valid) in CASES {
            let data = crate::reader::read_file(path)
                .unwrap_or_else(|e| panic!("cannot read fixture {path}: {e}"));

            let got_valid = parse_json(&data).is_ok();
            assert_eq!(
                got_valid, want_valid,
                "fixture {path}: expected valid={want_valid}, got valid={got_valid}\ndata: {data:?}"
            )
        }
    }
}
