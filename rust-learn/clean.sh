#!/bin/bash

# Get the script's filename
script_name=$(basename "$0")

# Find all files not ending with "rs" and not the script itself
find . -type f ! -name "*.rs" ! -name ".git" ! -name "*.md" ! -name "$script_name" -delete

# Print a message confirming the deletion
echo "Deleted all files not ending with 'rs' except the script itself."