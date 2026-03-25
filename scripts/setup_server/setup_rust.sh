#!/bin/bash

# Rust setup script
# This script installs Rust toolchain and required dependencies

set -e

# Variables
LEPTOS_VERSION="0.2.47"

echo "📦 Installing Rust toolchain..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo "🔧 Sourcing Rust environment..."
source "$HOME/.cargo/env"

echo "📦 Installing rustup via apt..."
sudo apt install -y rustup

echo "🌙 Setting nightly as default toolchain..."
rustup default nightly

echo "🎯 Adding compilation targets..."
echo "   → wasm32-unknown-unknown (for WebAssembly)"
rustup target add wasm32-unknown-unknown
echo "   → x86_64-unknown-linux-musl (for static linking)"
rustup target add x86_64-unknown-linux-musl

echo "📦 Installing Rust dependencies..."
echo "   → cargo-binstall"
cargo install cargo-binstall

echo "   → sqlx-cli (PostgreSQL only)"
cargo install sqlx-cli --no-default-features --features postgres

echo "   → wasm-bindgen-cli"
cargo install wasm-bindgen-cli

echo "   → cargo-update"
cargo install cargo-update

echo "   → cargo-leptos"
cargo binstall cargo-leptos -y

echo "🔧 Configuring PATH..."
# Add to bashrc if not already present
if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' ~/.bashrc 2>/dev/null; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
fi

# Also add to profile for login shells
if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' ~/.profile 2>/dev/null; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.profile
fi

echo "✅ Rust setup complete!"
echo ""
echo "📋 Installed versions:"
rustc --version
cargo --version