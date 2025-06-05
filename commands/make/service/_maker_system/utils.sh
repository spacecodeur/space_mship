insert_sorted_entry_in_block() {
    local file_path="$1"
    local start_pattern="$2"
    local end_pattern="$3"
    local new_entry="$4"

    # Extract the block then remove the first and last line
    members_block=$(sed -n "/$start_pattern/,/$end_pattern/p" "$file_path" | sed '1d;$d')

    # Verify block extraction
    if [[ -z "$members_block" ]]; then
        echo "Error: block not found between patterns '$start_pattern' and '$end_pattern' in file '$file_path'." >&2
        return 1
    fi

    # Fix the last line if it doesn't end with a comma (possibly with spaces)
    members_block=$(echo "$members_block" | sed '$s/\([^,[:space:]]\)[[:space:]]*$/\1,/' )

    original_count=$(echo "$members_block" | wc -l)

    # Add the new entry at the end
    members_block="$members_block"$'\n'"$new_entry"

    members_block=$(echo "$members_block" | sort)

    # Step 1: Find the line where the block starts
    member_begin_line=$(grep -n "$start_pattern" "$file_path" | head -n1 | cut -d: -f1)

    # Step 2: Count the number of lines in the block (without opening/closing lines)
    original_members_block=$(sed -n "/$start_pattern/,/$end_pattern/p" "$file_path" | sed '1d;$d')
    original_count=$(echo "$original_members_block" | wc -l)

    # Calculate the line after the block
    lines_after_member_array=$((member_begin_line + original_count + 1))  # +1 for the line containing "]"

    # Step 3: Capture all lines after this line
    lines_after_member=$(tail -n +"$((lines_after_member_array))" "$file_path")

    # Read the entire file content
    cargo_content=$(cat "$file_path")

    # Remove everything after line $member_begin_line
    cargo_content=$(echo "$cargo_content" | head -n "$member_begin_line")

    # Add the sorted members block
    cargo_content="$cargo_content
$members_block"

    # Add the lines after the array
    cargo_content="$cargo_content
$lines_after_member"

    echo "$cargo_content" > "$file_path"
}

recursive_copy_and_fill() {
  local source_dir="$1"
  local target_dir="$2"
  local service_name="$3"
  local upper_name="$4"
  local service_port="$5"

  cp -r "$source_dir" "$target_dir"

  find "$target_dir" -type f -exec sed -i \
    -e "s/<new_service_name>/${service_name}/g" \
    -e "s/<NEW_SERVICE_NAME>/${upper_name}/g" \
    -e "s/<new_service_port>/${service_port}/g" {} +
}