use std::{fs, io::Error, path};

pub fn read_file(step_name: &str, filename: &str) -> Result<String, Error> {
    fs::read_to_string(get_fixture_path(step_name, filename))
}

fn get_fixture_path(step_name: &str, filename: &str) -> path::PathBuf {
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(step_name);
    path.push(filename);

    path
}
