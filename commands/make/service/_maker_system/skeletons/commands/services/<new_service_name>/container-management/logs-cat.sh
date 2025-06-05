#!/bin/bash

container_id=$(docker inspect --format="{{.Id}}" "${APP_NAME}-<new_service_name>-container")
log_file="/var/lib/docker/containers/${container_id}/${container_id}-json.log"
sudo cat "$log_file"