export RUST_BACKTRACE := "1"

set dotenv-load
set positional-arguments

run *ARGS:
    cargo run --release -- "$@"

test *ARGS:
    cargo test -- "$@"

build:
    cargo build

install:
    cargo install --path .

check:
    cargo check
