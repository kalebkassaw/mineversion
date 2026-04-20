_:
    just -l

# Check if rustc is in the PATH
rust-installed := `rustc --version || echo "not installed"`

# Setup project dependencies
[unix]
setup:
    @[ "{{rust-installed}}" != "not installed" ] || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

[windows]
setup:
    @echo "\033[1mPlease perform the following actions:\033[0m"
    @echo "Install C++ tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
    @echo "Install Rust: https://rust-lang.org/tools/install/"
    @echo "Restart your terminal for these changes to take effect."
    @echo ""
    @echo "Alternatively, `just _win-links` to open all sites."

_win-links:
    start https://visualstudio.microsoft.com/visual-cpp-build-tools/
    start https://rust-lang.org/tools/install/

# Validate YAML file
validate file_name:
    @echo "✅ Validating YAML-formatted mod file:"
    cargo run --bin validator {{file_name}}

# Run Rust unit tests
test:
    cargo test

# Build Rust application
build:
    cargo build

# Build Rust crate (go time!)
release:
    cargo release