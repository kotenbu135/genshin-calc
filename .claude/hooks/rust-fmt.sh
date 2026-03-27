#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 0
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" == *.rs ]]; then
  cargo fmt 2>&1
fi
exit 0
