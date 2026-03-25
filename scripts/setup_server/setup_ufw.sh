#!/bin/bash

# Firewall setup script (UFW)
# This script configures UFW to only allow SSH (22), HTTP (80), and HTTPS (443)
# CRITICAL: Prevents database ports from being exposed to the internet

set -e

echo "🔥 Setting up UFW firewall..."

echo "📦 Installing UFW..."
sudo apt install -y ufw

echo "🔒 Configuring firewall rules..."

# Reset to default (deny incoming, allow outgoing)
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow only essential ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS

echo "🚀 Enabling UFW..."
sudo ufw --force enable

echo "✅ Firewall setup complete!"
echo ""
echo "📋 Current firewall status:"
sudo ufw status verbose

echo ""
echo "⚠️  IMPORTANT: Database ports (5432, 5433) are now BLOCKED from external access."
echo "   This prevents the attack vector from the November 2025 incident."