#!/bin/bash

NETWORK_NAME="${APP_NAME}-docker-network"

! docker network ls | grep -q "$NETWORK_NAME" && docker network create "$NETWORK_NAME"

docker compose -f docker/web/compose.yml --env-file crates/services/web/.env up --build -d