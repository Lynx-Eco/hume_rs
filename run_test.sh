#!/bin/bash
cd /Users/user/dev/test/claude_code/hume/hume_rs
export RUST_LOG=error
cargo run -q --example test_connection 2>&1 | grep -v "warning:" | grep -v "  |" | grep -v "note:"