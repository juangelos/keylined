# Rust Development Environment Setup

This guide assumes familiarity with Windows development, VSCode, and modern development workflows.

## Required Tools

- Git 2.43+ with Git LFS
- Visual Studio Code 1.86+
- Rust toolchain via rustup (stable channel, minimum 1.75)
- Windows Terminal
- PowerShell 7+ (pwsh)
- LLVM/Clang (for certain dependencies)
- CMake (for certain dependencies)

## Rust Installation

1. Install rustup from https://rustup.rs/
2. Install required components:
```powershell
rustup default stable
rustup component add clippy
rustup component add rustfmt
rustup component add rust-analyzer
rustup component add rust-src
```

## VSCode Setup

Install the following extensions:
- rust-analyzer
- CodeLLDB (Debug support)
- crates (Dependency management)
- Better TOML
- Git Graph
- GitLens
- markdownlint
- Even Better TOML

## Recommended VSCode Settings

The repository includes preconfigured settings in `.vscode/settings.json`. Key Rust-specific settings:
- Format on save
- Rust-analyzer diagnostics
- Inline hint settings
- Test explorer integration

## Code Style & Linting

The repository enforces:
- `rustfmt` for code formatting
- `clippy` for linting
- `cargo deny` for dependency auditing
- markdownlint for documentation

## Pre-commit Hooks

Install pre-commit hooks:

```powershell
cargo install cargo-husky
cargo husky install
```

## Development Workflow

1. Build the project:
```powershell
cargo build
```

2. Run tests:
```powershell
cargo test
cargo test --doc  # Run documentation tests
```

3. Run linting:
```powershell
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Verification

Run the verification script to ensure correct setup:

```powershell
./devenv/verify-setup.ps1
```

## Debugging

1. Use the CodeLLDB extension in VSCode
2. Launch configurations are provided in `.vscode/launch.json`
3. Debug console shows Rust backtraces when `RUST_BACKTRACE=1`

## Documentation

- Generate and view docs: `cargo doc --open`
- All public APIs must include documentation
- Follow Rust documentation best practices

## Recommended Tools

- `cargo-edit` for dependency management: `cargo install cargo-edit`
- `cargo-watch` for development: `cargo install cargo-watch`
- `cargo-audit` for security: `cargo install cargo-audit`
- `cargo-expand` for macro debugging: `cargo install cargo-expand`
