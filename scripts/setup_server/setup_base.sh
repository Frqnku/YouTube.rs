#!/bin/bash

# Base system setup script
# This script updates the system and installs essential build dependencies

set -e

echo "📦 Updating system packages..."
sudo apt update

echo "🔧 Installing essential build tools..."
sudo apt install -y build-essential pkg-config libssl-dev musl-tools

echo "✅ Base system setup complete!"
echo "📋 Installed packages:"
echo "   → build-essential (compilers, make, etc.)"
echo "   → pkg-config (package configuration tool)"
echo "   → libssl-dev (OpenSSL development libraries)"
echo "   → musl-tools (static linking tools)"