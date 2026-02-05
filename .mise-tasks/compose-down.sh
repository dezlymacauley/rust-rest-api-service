#!/usr/bin/env bash
#MISE description="Stop the container"
#MISE quiet=true

docker compose down --remove-orphans
