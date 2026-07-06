# Watch dev files
run:
    cargo run -- tests/fixtures/step1/valid.json

lint:
    cargo fmt
    cargo clippy -- -D warnings

test:
    cargo test

release:
    cargo build --bin json-parser --release
    cp target/release/json-parser .
    chmod +x ./json-parser
    ./json-parser tests/fixtures/step1/valid.json
