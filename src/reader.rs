use std::{fs, io::Error, path};

pub fn read_file(filepath: &str) -> Result<String, Error> {
    fs::read_to_string(get_fixture_path(filepath))
}

fn get_fixture_path(filepath: &str) -> path::PathBuf {
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(path::PathBuf::from(filepath));

    path
}
