#!/bin/bash
# Run this ONCE to set up the .env file on the remote server.
# Edit .env first, then run: ./_setup_env.sh

set -e

SERVER_IP="46.62.237.230"
SSH_KEY="$HOME/.ssh/id_rsa"

# Expand variables (e.g. DATABASE_URL uses ${POSTGRES_USER} etc.)
set -a && source .env && set +a
envsubst < .env > /tmp/.env.server

scp -i "$SSH_KEY" /tmp/.env.server root@"$SERVER_IP":~/.env
ssh -i "$SSH_KEY" root@"$SERVER_IP" "chmod 600 ~/.env"

rm /tmp/.env.server
echo "✅ .env deployed to server!"