#!/bin/bash

# Docker setup script
# This script installs Docker Engine and configures it for the current user

set -e

echo "📦 Downloading Docker installation script..."
curl -fsSL https://get.docker.com -o get-docker.sh

echo "🚀 Installing Docker Engine..."
sudo sh get-docker.sh

echo "👤 Adding current user to docker group (to avoid using sudo)..."
sudo usermod -aG docker $USER

echo "🔧 Starting Docker service..."
sudo systemctl start docker
sudo systemctl enable docker

echo "🧹 Cleaning up installation script..."
rm get-docker.sh

echo "✅ Docker setup complete!"
echo "📋 Docker version installed:"
sudo docker --version

echo ""
echo "⚠️  Important: You need to log out and log back in for group changes to take effect."
echo "   After logging back in, you can run 'docker ps' without sudo."