#!/bin/bash
WORKSPACE_ROOT=$(git rev-parse --show-toplevel) || exit 0
cd "$WORKSPACE_ROOT" || exit 0
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" == *.rs && "$FILE_PATH" == "$WORKSPACE_ROOT"* ]]; then
  cargo fmt 2>&1
fi
exit 0
