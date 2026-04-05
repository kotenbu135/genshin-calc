#!/bin/bash
# Verify LSP diagnostics with cargo check to filter out rust-analyzer false positives.
# If cargo check passes, report that LSP errors are likely stale cache.
WORKSPACE_ROOT=$(git rev-parse --show-toplevel) || exit 0
cd "$WORKSPACE_ROOT" || exit 0

OUTPUT=$(cargo check --quiet 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "cargo check passed — LSP errors are likely rust-analyzer cache artifacts. Run 'rust-analyzer: Restart Server' in IDE to clear."
fi

# If cargo check fails, output the real errors
if [ $EXIT_CODE -ne 0 ]; then
  echo "$OUTPUT"
fi

exit 0
