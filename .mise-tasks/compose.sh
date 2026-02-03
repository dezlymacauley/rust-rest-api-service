#!/usr/bin/env bash
#MISE description="Run the docker-compose.yaml file"
#MISE quiet=true

# -d means detached. 
# The service `db` (defined in compose.yml) 
# will be started in the background.
docker compose up -d db
