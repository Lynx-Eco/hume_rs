#!/bin/bash

echo "Testing EVI Connection"
echo "====================="
echo

# Check API key
if [ -z "$HUME_API_KEY" ]; then
    echo "❌ HUME_API_KEY is not set"
    exit 1
fi

echo "✓ API key is set"
echo

# Run the test
export RUST_BACKTRACE=1
cargo run -q --example evi_test_simple --manifest-path /Users/user/dev/test/claude_code/hume/hume_rs/Cargo.toml 2>&1 | while IFS= read -r line; do
    # Filter out warnings
    if [[ ! "$line" =~ "warning:" ]] && [[ ! "$line" =~ "note:" ]] && [[ ! "$line" =~ "src/" ]] && [[ ! "$line" =~ "|" ]]; then
        echo "$line"
    fi
done