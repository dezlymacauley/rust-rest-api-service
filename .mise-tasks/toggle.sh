#!/usr/bin/env bash
#MISE description="Activate or deactivate the docker engine"
#MISE quiet=true

if systemctl is-active --quiet docker.service; then
    # If Docker is active, deactivate it.
    sudo systemctl stop docker.service docker.socket
    echo
    echo "✖️ Docker has been deactivated"
else
    # If Docker is inactive, activate it.
    sudo systemctl start docker.service docker.socket
    echo
    echo "🐋  Docker has been activated"
fi
