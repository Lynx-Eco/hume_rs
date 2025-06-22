#!/bin/bash
cd /Users/user/dev/test/claude_code/hume/hume_rs
echo "Running EVI Conversation Example..."
echo "=================================="
echo
# Run for 15 seconds then timeout
timeout 15 cargo run -q --example evi_conversation_mac 2>&1 | grep -v "warning:" | grep -v "  |" | grep -v "note:" | head -50