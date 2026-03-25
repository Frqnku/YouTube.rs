#!/bin/bash

# Fail2ban setup script
# This script installs and configures fail2ban for brute-force protection
# HIGH PRIORITY: Blocks IPs after repeated failed login attempts

set -e

echo "🛡️ Setting up Fail2ban..."

echo "📦 Installing fail2ban..."
sudo apt install -y fail2ban

echo "🔧 Creating local configuration..."
sudo tee /etc/fail2ban/jail.local > /dev/null <<EOF
[DEFAULT]
bantime = 1h
findtime = 10m
maxretry = 5

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 24h
EOF

echo "🚀 Enabling and starting fail2ban..."
sudo systemctl enable fail2ban
sudo systemctl restart fail2ban

echo "✅ Fail2ban setup complete!"
echo ""
echo "📋 Configuration:"
echo "   → SSH: 3 failed attempts = 24h ban"
echo "   → Default: 5 failed attempts = 1h ban"
echo ""
echo "📊 Check status with:"
echo "   sudo fail2ban-client status"
echo "   sudo fail2ban-client status sshd"