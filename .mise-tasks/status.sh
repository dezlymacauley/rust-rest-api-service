#!/usr/bin/env bash
#MISE description="Check the status of the docker engine"
#MISE quiet=true

if systemctl is-active --quiet docker.service; then
    echo
    echo "🐋 Docker is active"
    echo
else
    echo
    echo "✖️ Docker is inactive"
    echo
fi
