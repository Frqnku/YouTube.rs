#!/bin/bash

# * ./scripts/setup_server/_setup
# Run ONCE on a fresh server before first deploy.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

run_setup() {
    local script=$1
    local desc=$2
    echo ""
    echo "┌────────────────────────────────────────────────┐"
    echo "│ $desc"
    echo "└────────────────────────────────────────────────┘"
    bash "$SCRIPT_DIR/$script"
}

echo "⚠️  WARNING: You are about to automatically setup your remote server."
echo "   Cancel if you have already done it"
echo ""
read -p "   Continue? (y/N): " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
    echo "Aborted."
    exit 0
fi

echo "╔══════════════════════════════════════════════════╗"
echo "║         SERVER SETUP - AUTOMATED INSTALL         ║"
echo "╚══════════════════════════════════════════════════╝"

run_setup "setup_base.sh"               "STEP 1/4: Installing base packages"
run_setup "setup_ufw.sh"                "STEP 2/4: Setup Firewall (UFW)"
run_setup "setup_ssh_hardening.sh"      "STEP 3/4: Setup SSH hardening"
run_setup "setup_fail2ban.sh"           "STEP 4/4: Setup Fail2ban (brute-force protection)"
run_setup "setup_docker.sh"             "STEP 1/4: Installing Docker"
run_setup "setup_node.sh"               "STEP 4/4: Installing NodeJS"
run_setup "setup_rust.sh"               "STEP 4/4: Installing Rust + Frameworks"

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║              ✅ SETUP COMPLETE!                  ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
echo "📋 Next steps:"
echo "   1. Upload .env to server:  ./setup__server_env.sh  (from local machine)"
echo "   2. Trigger deploy:         gh workflow run prod.yml"
echo ""
echo "🔐 Optional — run after confirming SSH key access:"
echo "   bash setup__remote/security_ssh_hardening.sh"
echo ""