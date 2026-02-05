#!/usr/bin/env bash
#MISE description="Connect to a Postgres container with pgcli"
#MISE quiet=true

# Fail immediately on any unset variable
set -euo pipefail

#______________________________________________________________________________
# Environment variables (from mise.toml)
: "${LOGIN_USER_NAME:?Missing LOGIN_USER_NAME}"
: "${DB_NAME:?Missing DB_NAME}"
: "${DB_HOST:?Missing DB_HOST}"
: "${DB_PORT:?Missing DB_PORT}"
: "${PGCLI_CONFIG_FILE:?Missing PGCLI_CONFIG_FILE}"

#______________________________________________________________________________
# Connect with pgcli
echo
echo "___________________________________________"
echo
echo "🗃️ Connecting to database: $DB_NAME"
echo "  - Host: $DB_HOST"
echo "  - Port: $DB_PORT"
echo "👤 via the user: $LOGIN_USER_NAME"
echo "___________________________________________"
echo

pgcli \
    -U "$LOGIN_USER_NAME" \
    --pgclirc "$PGCLI_CONFIG_FILE" \
    -d "$DB_NAME" \
    -h "$DB_HOST" \
    -p "$DB_PORT"
