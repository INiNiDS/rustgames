#!/bin/bash

set -e

print_step() {
    echo -e "\n\033[1;34m==>\033[0m \033[1m$1\033[0m"
}

print_step "Checking formatting (cargo fmt)..."
if ! cargo fmt -- --check; then
    echo -e "\033[0;33m[!] Found formatting errors\033[0m"
    # shellcheck disable=SC2162
    read -p "Apply suggestions? (y/n): " confirm
    if [[ "$confirm" == [yY] || "$confirm" == [yY][eE][sS] ]]; then
        cargo fmt
        echo -e "\033[0;32mSuggestions applied\033[0m"
    else
        echo -e "\033[0;31m Checking was closed by user\033[0m"
        exit 1
    fi
fi

print_step "Running cargo check..."
if ! cargo check --quiet; then
    echo "Check failed!"
    exit 1
fi

print_step "Running Clippy (pedantic, nursery)..."
if ! cargo clippy --quiet -- -W clippy::pedantic -W clippy::nursery; then
    echo "Clippy found issues!"
    exit 1
fi

print_step "Running tests..."
if ! cargo test --quiet -- --nocapture 2>&1 | grep -v "Finished"; then
    echo "Tests failed!"
    exit 1
fi

print_step "Running all examples..."
examples=$(cargo run --example 2>&1 | grep -E '^  ' | awk '{print $1}')

if [ -z "$examples" ]; then
    echo "No examples found."
else
    for ex in $examples; do
        echo -e "\033[0;32mRunning example:\033[0m $ex"
        if ! cargo run --quiet --example "$ex" 2>&1 | grep -v "Finished"; then
            echo "Example '$ex' failed!"
            exit 1
        fi
    done
fi

print_step "All checks passed successfully!"