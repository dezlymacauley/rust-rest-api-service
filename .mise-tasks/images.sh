#!/usr/bin/env bash
#MISE description="View downloaded container images"
#MISE quiet=true

docker image ls --format "table {{.ID}}\t{{.Repository}}:{{.Tag}}\t{{.Size}}"
