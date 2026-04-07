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

run_setup "setup_base.sh"               "STEP 1/7: Installing base packages"
run_setup "setup_ufw.sh"                "STEP 2/7: Setup Firewall (UFW)"
run_setup "setup_fail2ban.sh"           "STEP 3/7: Setup Fail2ban (brute-force protection)"
run_setup "setup_docker.sh"             "STEP 4/7: Installing Docker"
run_setup "setup_node.sh"               "STEP 5/7: Installing NodeJS"
run_setup "setup_psql.sh"               "SETP 6/7: Installing PostgreSQL"
run_setup "setup_rust.sh"               "STEP 7/7: Installing Rust + Frameworks"

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║              ✅ SETUP COMPLETE!                  ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
echo "📋 Next steps:"
echo "   1. Upload .env to server:  ./scripts/deploy/setup_env.sh  (from local machine)"
echo "   2. Trigger deploy:         gh workflow run prod.yml"
echo ""
echo "🔐 Optional — run after confirming SSH key access:"
echo "   bash setup__remote/security_ssh_hardening.sh"
echo ""