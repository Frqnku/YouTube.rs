#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEMPLATE_FILE="$ROOT_DIR/deploy/nginx/nginx.conf.template"
OUTPUT_FILE="$ROOT_DIR/deploy/nginx/nginx.conf"
ENV_FILE="$ROOT_DIR/.env"

set -a
source "$ENV_FILE"
set +a
envsubst '$SERVER_IP $DOMAIN' < "$TEMPLATE_FILE" > "$OUTPUT_FILE"