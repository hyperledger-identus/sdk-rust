# Show available commands
default:
    @just --list --list-submodules

# Build the workspace
[group('sdk-rust')]
build:
    cargo build --all-features

# Run all tests
[group('sdk-rust')]
test:
    cargo test --all-features

# Run tests with code coverage (requires cargo-llvm-cov)
[group('sdk-rust')]
coverage:
    cargo llvm-cov test --all-features --lcov --output-path lcov.info
    cargo llvm-cov report
    echo "Coverage report: lcov.info (use 'cargo llvm-cov report --html' for HTML)"

# Generate HTML coverage report
[group('sdk-rust')]
coverage-html: coverage
    cargo llvm-cov report --html
    echo "HTML report saved to target/llvm-cov/html/index.html"

# Clean all build artifacts
[group('sdk-rust')]
clean:
    cargo clean

# Format all source files (Nix, TOML, Rust)
[group('sdk-rust')]
format:
    echo "Formatting Nix files..."
    find . -name '*.nix' -type f -exec sh -c 'echo "  → {}" && nixfmt {}' \;

    echo "Formatting TOML files..."
    find . -name '*.toml' -type f -exec sh -c 'echo "  → {}" && taplo format {}' \;

    echo "Formatting Rust files..."
    cargo fmt

# Lint text files (markdown, YAML, editorconfig, shell scripts)
[group('checks')]
lint-text:
    #!/usr/bin/env bash
    set -euo pipefail
    EXIT_CODE=0

    echo "=== Markdown Lint ==="
    markdownlint-cli2 "**/*.md" || EXIT_CODE=$?

    echo ""
    echo "=== YAML Lint ==="
    yamllint -c .yamllint.yml . || EXIT_CODE=$?

    echo ""
    echo "=== EditorConfig Check ==="
    editorconfig-checker || EXIT_CODE=$?

    echo ""
    echo "=== ShellCheck ==="
    sh_files=$(find . -name "*.sh" \
      -not -path "*/node_modules/*" \
      -not -path "*/.git/*" \
      -not -path "*/target/*")
    if [[ -n "$sh_files" ]]; then
      echo "$sh_files" | xargs shellcheck --severity=warning || EXIT_CODE=$?
    else
      echo "No .sh files found, skipping."
    fi

    exit $EXIT_CODE

# Run fast checks (tests, formatting, lints)
[group('checks')]
check: format build test
    echo "✓ All checks completed successfully."
