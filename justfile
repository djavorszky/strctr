# List the available commands
_default:
    @just --list

# Runs clippy, denies the warnings
check:
	cargo clippy --locked -- -D warnings

# Runs all tests
test:
    cargo test --locked

# Documents the crate and opens it
doc:
    cargo doc --locked --open