#!/usr/bin/env bash
#MISE description="Login to Postgres with psql for admin tasks"
#MISE quiet=true

docker exec -it postgres-database \
psql -U user \
-d rust_rest_api_service
