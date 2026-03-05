#!/usr/bin/env bash

# Exit on error and on unset variables
set -eu

# ---------- Terminal styles ----------
if [[ -t 1 ]]; then
	BOLD='\033[1m'
	DIM='\033[2m'
	RESET='\033[0m'
	RED='\033[31m'
	GREEN='\033[32m'
	YELLOW='\033[33m'
	BLUE='\033[34m'
	CYAN='\033[36m'
else
	BOLD=''
	DIM=''
	RESET=''
	RED=''
	GREEN=''
	YELLOW=''
	BLUE=''
	CYAN=''
fi

title() {
	echo -e "\n${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
	echo -e "${BOLD}${CYAN}▶ $1${RESET}"
	echo -e "${BOLD}${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
}

info() {
	echo -e "${DIM}• $1${RESET}"
}

ok() {
	echo -e "${GREEN}✔ $1${RESET}"
}

warn() {
	echo -e "${YELLOW}⚠ $1${RESET}"
}

fail() {
	echo -e "${RED}✖ $1${RESET}" >&2
}

on_error() {
	fail "An error occurred. Database reset has been aborted."
}
trap on_error ERR

FORCE=false

while getopts ":fh" opt; do
	case "$opt" in
		f)
			FORCE=true
			;;
		h)
			echo "Usage: ./scripts/reset_db.sh [-f]"
			echo "  -f    skip confirmation prompt"
			exit 0
			;;
		\?)
			fail "Unknown option: -$OPTARG"
			echo "Usage: ./scripts/reset_db.sh [-f]"
			exit 1
			;;
	esac
done

# Load .env file (containing POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DATABASE, etc.)
if [[ ! -f .env ]]; then
	fail ".env file not found in project root."
	exit 1
fi

source .env

title "Reset PostgreSQL"
info "Target database  : ${BOLD}${POSTGRES_DATABASE}${RESET}"
info "User             : ${BOLD}${POSTGRES_USER}${RESET}"
info "Host             : ${BOLD}127.0.0.1${RESET}"

warn "All data in '${POSTGRES_DATABASE}' will be permanently deleted."

if [[ "$FORCE" != true ]]; then
	echo
	read -r -p "Continue? Type y to proceed (y/N): " confirmation
	if [[ "${confirmation,,}" != "y" ]]; then
		warn "Operation cancelled by user."
		exit 0
	fi
fi

title "1/4 Terminate active connections"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$POSTGRES_DATABASE';"
ok "Connections terminated"

title "2/4 Drop database"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE IF EXISTS $POSTGRES_DATABASE;"
ok "Database dropped"

title "3/4 Recreate database"
psql -h 127.0.0.1 -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DATABASE;"
ok "Database recreated"

title "4/4 Apply migrations"
info "Running: sqlx migrate run"

sqlx migrate run
ok "Migrations applied"

echo -e "\n${BOLD}${GREEN}✅ Reset completed successfully for '${POSTGRES_DATABASE}'.${RESET}\n"