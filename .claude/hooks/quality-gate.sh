#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 0
INPUT=$(cat)

# 無限ループ防止: Stop hook が再発火した場合はスキップ
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

TEST_OUTPUT=$(cargo test --quiet 2>&1)
TEST_RC=$?
CLIPPY_OUTPUT=$(cargo clippy -- -D warnings 2>&1)
CLIPPY_RC=$?

if [ $TEST_RC -ne 0 ] || [ $CLIPPY_RC -ne 0 ]; then
  REASON=""
  [ $TEST_RC -ne 0 ] && REASON="cargo test failed:\n$TEST_OUTPUT\n"
  [ $CLIPPY_RC -ne 0 ] && REASON="${REASON}cargo clippy failed:\n$CLIPPY_OUTPUT"
  jq -n --arg reason "$REASON" '{"decision":"block","reason":$reason}'
  exit 0
fi
exit 0
