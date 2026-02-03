#!/usr/bin/env bash
#MISE description="Start the Postgres service in detached mode"
#MISE quiet=true

# -d means detached. 
# The service `db` (defined in compose.yml) will be started in the background.
docker compose up -d db
