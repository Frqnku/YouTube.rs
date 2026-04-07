#!/bin/bash
# Run this ONCE to set up the .env file on the remote server.
# Edit .env first, then run: ./scripts/deploy/setup_env.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"

SERVER_IP="46.62.237.230"
SSH_KEY="$HOME/.ssh/id_rsa"

# Expand variables (e.g. DATABASE_URL uses ${POSTGRES_USER} etc.)
set -a && source "$ENV_FILE" && set +a
envsubst < "$ENV_FILE" > /tmp/.env.server

scp -i "$SSH_KEY" /tmp/.env.server root@"$SERVER_IP":~/.env
ssh -i "$SSH_KEY" root@"$SERVER_IP" "chmod 600 ~/.env"

rm /tmp/.env.server
echo "✅ .env deployed to server!"