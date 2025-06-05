#!/bin/bash

set -euo pipefail

source "$(dirname "$0")/_maker_system/config"
source "$(dirname "$0")/_maker_system/utils.sh"

read -rp "Enter the new service name (snake_case): " new_service_name

if [[ ! $new_service_name =~ ^[a-z][a-z0-9_]*$ ]]; then
  echo "❌ Invalid name. Use snake_case (e.g., my_service)."
  exit 1
fi

if grep -q "\"$SERVICES_DIR/$new_service_name\"" "$CARGO_WORKSPACE_PATH"; then
  echo "❌ Service '$new_service_name' already exists in Cargo.toml."
  exit 1
fi

UPPER_SERVICE_NAME=$(echo "$new_service_name" | awk '{ print toupper($0) }')

# [2] Update Cargo.toml
echo "Updating Cargo.toml members..."
new_entry="    \"crates/services/$new_service_name\","
insert_sorted_entry_in_block "$CARGO_WORKSPACE_PATH" '^members[[:space:]]*=' ']' "$new_entry"

# [3] Update .env (add port)
echo "Adding port to .env..."
matched_ports=$(grep -oE 'SERVICE_[A-Z_]+_PORT=[0-9]+' "$ENV_FILE_PATH" || true)

if [[ -z "$matched_ports" ]]; then
  new_service_port=34700
else
  last_port=$(echo "$matched_ports" | awk -F= '{ print $2 }' | sort -nr | head -n1)
  new_service_port=$((last_port + 1))
fi

# Ensure newline before appending
tail -c1 "$ENV_FILE_PATH" | read -r _ || echo >> "$ENV_FILE_PATH"
echo "SERVICE_${UPPER_SERVICE_NAME}_PORT=${new_service_port}" >> "$ENV_FILE_PATH"

# [4] Copy commands/services skeleton
echo "Creating command scripts..."
recursive_copy_and_fill "$SKELETONS_DIR/commands/services/<new_service_name>" "$COMMANDS_SERVICES_DIR/$new_service_name" "$new_service_name" "$UPPER_SERVICE_NAME" "$new_service_port"

# [5] Update services_manager/lib.rs
echo "Updating $SERVICES_MANAGER_PATH..."
insert_sorted_entry_in_block "$SERVICES_MANAGER_PATH" '^pub enum ServicesName' '}' "    ${UPPER_SERVICE_NAME},"
insert_sorted_entry_in_block "$SERVICES_MANAGER_PATH" 'match self {' '}' "            ServicesName::${UPPER_SERVICE_NAME} => \"${new_service_name}\","

# [6] Copy crates/services skeleton
echo "Creating service crate..."
recursive_copy_and_fill "$SKELETONS_DIR/crates/services/<new_service_name>" "$SERVICES_DIR/$new_service_name" "$new_service_name" "$UPPER_SERVICE_NAME" "$new_service_port"

# [7] Copy docker skeleton
echo "Creating Docker files..."
recursive_copy_and_fill "$SKELETONS_DIR/docker/<new_service_name>" "$DOCKER_DIR/$new_service_name" "$new_service_name" "$UPPER_SERVICE_NAME" "$new_service_port"

echo
echo "✅ Service '$new_service_name' created successfully!"