# Justfile for how-orc-bot

set dotenv-load

CLUSTER_NAME := "minikube-how-orc-bot"
NAMESPACE := "how-orc-bot"

# Default recipe
default: run

[group('dev')]
dev arg:
  just "dev-{{arg}}"

# Spin up development environment
[group('dev')]
dev-up:
  ./scripts/dev_up.sh {{CLUSTER_NAME}} {{NAMESPACE}}

# Spin down development environment
[group('dev')]
dev-down:
  ./scripts/dev_down.sh {{CLUSTER_NAME}} {{NAMESPACE}}

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

# Run pinggy tunnel
pinggy:
    @echo "Starting pinggy tunnel on port $PORT..."
    ssh -p 443 -o StrictHostKeyChecking=no -o ServerAliveInterval=30 -R0:127.0.0.1:$PORT free.pinggy.io
