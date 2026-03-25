#!/bin/bash

# SSH hardening script
# This script disables root login and password authentication
# HIGH PRIORITY: Prevents brute-force and unauthorized access

set -e

SSHD_CONFIG="/etc/ssh/sshd_config"

echo "🔐 Hardening SSH configuration..."

# Safety check: Ensure SSH key exists
if [ ! -f ~/.ssh/authorized_keys ] || [ ! -s ~/.ssh/authorized_keys ]; then
    echo "❌ ERROR: No SSH keys found in ~/.ssh/authorized_keys"
    echo "   You will be LOCKED OUT if you proceed!"
    echo ""
    echo "   First, add your public key:"
    echo "   echo 'your-public-key' >> ~/.ssh/authorized_keys"
    exit 1
fi

echo "✅ SSH key found in ~/.ssh/authorized_keys"
echo ""
echo "⚠️  WARNING: This will disable password authentication."
echo "   Make sure you can connect with: ssh -i ~/.ssh/your_key user@server"
echo ""
read -p "   Continue? (y/N): " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
    echo "Aborted."
    exit 0
fi

# Backup original config
echo "📦 Backing up original sshd_config..."
sudo cp "$SSHD_CONFIG" "${SSHD_CONFIG}.backup.$(date +%Y%m%d)"

echo "🔧 Disabling root login..."
sudo sed -i 's/^#*PermitRootLogin.*/PermitRootLogin no/' "$SSHD_CONFIG"

echo "🔧 Disabling password authentication..."
sudo sed -i 's/^#*PasswordAuthentication.*/PasswordAuthentication no/' "$SSHD_CONFIG"

echo "🔧 Enabling public key authentication..."
sudo sed -i 's/^#*PubkeyAuthentication.*/PubkeyAuthentication yes/' "$SSHD_CONFIG"

echo "🔄 Restarting SSH service..."
sudo systemctl restart sshd

echo "✅ SSH hardening complete!"
echo ""
echo "📋 Applied settings:"
echo "   → PermitRootLogin no"
echo "   → PasswordAuthentication no"
echo "   → PubkeyAuthentication yes"
echo ""
echo "⚠️  IMPORTANT: Make sure you have SSH key access BEFORE logging out!"
echo "   Test with: ssh -i ~/.ssh/your_key user@server"