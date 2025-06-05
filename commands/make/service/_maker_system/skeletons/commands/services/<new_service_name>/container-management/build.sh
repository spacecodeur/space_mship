#!/bin/bash

NETWORK_NAME="${APP_NAME}-docker-network"

! docker network ls | grep -q "$NETWORK_NAME" && docker network create "$NETWORK_NAME"

docker compose -f docker/<new_service_name>/compose.yml --env-file crates/services/<new_service_name>/.env up --build -d