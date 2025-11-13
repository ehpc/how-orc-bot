# Justfile for how-orc-bot

set dotenv-load

# Default recipe
default: run

# Format code with rustfmt
fmt:
    cargo fmt

# Lint code with clippy
lint:
    cargo clippy -- -D warnings

# Format and lint
check: fmt lint

# Build the project
build:
    cargo build

# Run the project
run:
    RUST_LOG=DEBUG cargo run

# Clean build artifacts
clean:
    cargo clean

# Run tests
test:
    cargo test
