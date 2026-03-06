# Contributing to rustgames

First off, thank you for considering contributing to rustgames! It's people like you that make building and maintaining this 2D game engine a rewarding experience for everyone involved.
## Getting Started
* Fork the repository on GitHub.

* Clone your fork locally:
```Bash
git clone https://github.com/INiNiDS/rustgames.git
```
* Create a branch for your feature or bug fix:
```Bash
git checkout -b feature/my-awesome-feature
```
## Development Workflow

We use standard Rust tooling to maintain code quality. Before submitting any changes, please ensure your code complies with the following checks. You can use `./test_all.sh` script to run all checks at once. Or you can run them individually:
* Formatting: We use rustfmt to ensure a consistent code style. Run the following command before committing:
```Bash
cargo fmt --all
```
* Linting: We use clippy to catch common mistakes and improve code design. Ensure your code passes without warnings:
```Bash
cargo clippy --all-targets --all-features -- -D warnings
```
* Testing: Make sure all existing tests pass, and write new tests for your features if applicable:
```Bash
cargo test
```
## Code Style & Architecture
* Performance & Safety: As a game engine, performance is critical, but not at the expense of memory safety. Use unsafe blocks only when absolutely necessary (e.g., for specific FFI bindings or proven performance bottlenecks) and always thoroughly document the safety invariants.
* Modularity: Try to keep distinct systems (like 2D rendering, physics, and input handling) clearly separated and decoupled.
* Documentation: Add inline doc comments (///) for all public structs, traits, and functions.
* Tests: We value a robust test suite. Please add unit tests for new features and bug fixes, and consider integration tests for more complex interactions.
## Pull Request Process

When you are ready to share your code, open a Pull Request (PR) against the main branch. You can read PULL_REQUEST_TEMPLATE.md for more details on how to structure your PR description. 

# Reporting Bugs and Requesting Features

If you aren't ready to write code, you can still contribute by submitting issues!
* Search the issue tracker first to see if your bug or feature request has already been reported.
* For bugs, provide a minimal, reproducible code example if possible.
* Clearly explain the expected behavior versus the actual behavior.