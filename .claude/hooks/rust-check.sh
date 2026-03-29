#!/bin/bash
WORKSPACE_ROOT=$(git rev-parse --show-toplevel) || exit 0
cd "$WORKSPACE_ROOT" || exit 0
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" == *.rs && "$FILE_PATH" == "$WORKSPACE_ROOT"* ]]; then
  OUTPUT=$(cargo check --quiet 2>&1)
  if [ $? -ne 0 ]; then
    echo "$OUTPUT"
  fi
fi
exit 0
