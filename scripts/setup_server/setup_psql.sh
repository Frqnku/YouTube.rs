#!/bin/bash

# PostgreSQL setup script
# This script installs PostgreSQL and configures the user
# Database creation is handled by reset_db.sh

set -e

POSTGRES_PASSWORD="password"

echo "📦 Installing PostgreSQL..."
sudo apt install -y postgresql postgresql-contrib libpq-dev

echo "🚀 Starting PostgreSQL service..."
sudo systemctl start postgresql
sudo systemctl enable postgresql

echo "🔐 Configuring PostgreSQL user..."
sudo -u postgres psql <<EOF
ALTER USER postgres WITH PASSWORD '$POSTGRES_PASSWORD';
\q
EOF

echo "✅ PostgreSQL setup complete!"
echo "   User: postgres"
echo "   Password: $POSTGRES_PASSWORD"
echo ""
echo "📌 Run ./reset_db.sh to create the database and run migrations."