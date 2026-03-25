#!/bin/bash

# Node.js setup script
# This script installs Node.js 20.x and project dependencies

set -e

# Variables
NODE_VERSION="20.x"

echo "📦 Adding Node.js ${NODE_VERSION} repository..."
curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION} | sudo -E bash -

echo "🚀 Installing Node.js..."
sudo apt install -y nodejs

echo "📋 Node.js version installed:"
node --version
npm --version

echo "📦 Installing pnpm..."
sudo npm install -g pnpm

echo "📋 pnpm version installed:"
pnpm --version

echo "📦 Installing dependencies from project root..."
cd "$(dirname "$0")/.."
pnpm install

echo "✅ Node.js setup complete!"