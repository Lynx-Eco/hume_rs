#!/bin/bash

# Generate documentation for Hume Rust SDK

set -e

echo "Generating documentation for Hume Rust SDK..."

# Clean previous docs
rm -rf target/doc

# Generate documentation with all features
cargo doc --all-features --no-deps

# Generate documentation for examples
cargo doc --examples

# Copy custom assets
if [ -d "docs/assets" ]; then
    cp -r docs/assets target/doc/
fi

# Open documentation in browser
if command -v open &> /dev/null; then
    open target/doc/hume/index.html
elif command -v xdg-open &> /dev/null; then
    xdg-open target/doc/hume/index.html
else
    echo "Documentation generated at: target/doc/hume/index.html"
fi

echo "Documentation generation complete!"