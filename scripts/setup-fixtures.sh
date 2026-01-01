#!/bin/bash
# Setup script to install dependencies and generate test fixtures
set -e

echo "=== Compactr.rs Cross-Compatibility Test Setup ==="
echo

# Check for Node.js
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is not installed"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

echo "✓ Node.js $(node --version) found"

# Check for npm
if ! command -v npm &> /dev/null; then
    echo "Error: npm is not installed"
    exit 1
fi

echo "✓ npm $(npm --version) found"
echo

# Install npm dependencies
echo "Installing compactr.js..."
npm install

echo
echo "✓ compactr.js installed"
echo

# Generate fixtures
echo "Generating test fixtures..."
node compactr/tests/fixtures/generate_fixtures.js

echo
echo "=== Setup Complete ==="
echo
echo "You can now run cross-compatibility tests:"
echo "  cargo test --test cross_compatibility"
echo "  cargo test --test binary_format_compatibility"
echo
