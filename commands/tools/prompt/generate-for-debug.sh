#!/bin/bash

##############################################
# CONFIGURATION TO BE FILLED
##############################################

INTRODUCTION="Hello! I need your help with a question about a project I'm developing.
I'll first give you the general context of my project before explaining my issue."

APP_GOAL="The application's goal is to create an online role-playing game where the story is told by a Game Master managed by an AI (LLM). This application is architected as microservices; it currently contains 5 main components:
- [frontend] Web component - provides an ergonomic web interface for players
- [backend] Universe component - provides structured information about an imaginary universe (D&D, Warhammer, etc.)
- [backend] Adventure component - manages universe instances, an adventure takes place in a universe and this component stores and exposes the player's progress in the initially chosen universe
- [backend] AI component - generates the next narrative steps for the player based on the chosen universe and adventure progress
- [backend] Game component - serves as the backend entry point after the Web component and orchestrates with other components in my architecture

Each component is managed via a Docker container, docker containers are defined in the \`./dockers\` folder, and finally I use a homemade command system (I can run commands like \`./commands.sh tools/docker/ls\`)"

TECHNOLOGIES="rust | docker | ia (llm ?)"

EXPERIENCE="I am a developer/QA engineer for ~10 years, and I want to learn how to use AI in my projects. This project allows me to train myself to use/integrate AI."

##############################################
# PARAMETERS AND OPTIONS
##############################################

TREE_PATH=""
FILES_TO_PROCESS=()

show_help() {
  echo "Usage: ./generate-prompt.sh [OPTIONS] [files_or_directories...]"
  echo
  echo "This script generates a structured prompt to help describe a technical problem in your project."
  echo
  echo "Options:"
  echo "  --tree <directory>   Display the structure of the given directory using 'tree'."
  echo "                       Overrides the default root-level structure display."
  echo "  -h, --help           Show this help message and exit."
  echo
  echo "Arguments:"
  echo "  files_or_directories   A list of files or directories to include in the"
  echo "                         'RELEVANT FILE CONTENTS' section."
  exit 0
}


# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --tree)
      TREE_PATH="$2"
      shift 2
      ;;
    -h|--help)
      show_help
      ;;
    *)
      FILES_TO_PROCESS+=("$1")
      shift
      ;;
  esac
done

##############################################
# PROMPT GENERATION
##############################################

echo "$INTRODUCTION"
echo
echo "====================================="
echo "### GENERAL PROJECT CONTEXT ###"
echo
echo "Application goal:"
echo "$APP_GOAL"
echo
echo "Technologies used:"

# Display technologies as a list
IFS='|' read -ra TECH_LIST <<< "$TECHNOLOGIES"
for tech in "${TECH_LIST[@]}"; do
  echo "- $(echo "$tech" | xargs)"
done

echo
echo "My experience level:"
echo "$EXPERIENCE"
echo

# Show project structure
if command -v tree &> /dev/null; then
  echo "====================================="
  echo "## GENERAL PROJECT STRUCTURE"
  echo
  echo '```'
  if [ -n "$TREE_PATH" ]; then
    tree -a "$TREE_PATH"
  else
    tree -I 'target|node_modules|.git|dist|build|venv|__pycache__|.vscode' -a
  fi
  echo '```'
  echo
fi

# Function to display file content
display_file_content() {
  local file="$1"
  if [ -f "$file" ]; then
    echo
    echo "Here is the content of the file \`$file\` :"
    echo
    echo '```'
    cat "$file"
    echo '```'
  fi
}

# Function to process files or directories
process_path() {
  local path="$1"
  if [ -f "$path" ]; then
    display_file_content "$path"
  elif [ -d "$path" ]; then
    find "$path" -type f | while read -r file; do
      display_file_content "$file"
    done
  else
    echo "❌ Invalid path: $path"
    exit 1
  fi
}

# If file paths were provided, display their contents
if [ "${#FILES_TO_PROCESS[@]}" -gt 0 ]; then
  echo "====================================="
  echo "## RELEVANT FILE CONTENTS FOR MY ISSUE"
  for path in "${FILES_TO_PROCESS[@]}"; do
    process_path "$path"
  done
  echo
fi

echo "====================================="
echo "### NOW, HERE IS THE DETAIL OF MY PROBLEM"
echo "(To be completed manually)"
echo
