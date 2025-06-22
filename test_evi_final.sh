#!/bin/bash
export RUST_LOG=error
export CARGO_TERM_COLOR=never
cd /Users/user/dev/test/claude_code/hume/hume_rs

echo "🧪 Testing EVI Conversation Example"
echo "==================================="
echo

# First compile
echo "Compiling..."
cargo build --example evi_conversation_mac 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✓ Compiled successfully"
else
    echo "❌ Compilation failed"
    exit 1
fi

echo
echo "Running example (10 second timeout)..."
echo "--------------------------------------"

# Run with timeout and capture output
timeout 10 cargo run --example evi_conversation_mac 2>&1 | while IFS= read -r line; do
    # Skip warning lines
    if [[ ! "$line" =~ "warning:" ]] && \
       [[ ! "$line" =~ "note:" ]] && \
       [[ ! "$line" =~ "^[[:space:]]*[0-9]" ]] && \
       [[ ! "$line" =~ "^[[:space:]]*|" ]] && \
       [[ ! "$line" =~ "^-->" ]] && \
       [[ ! "$line" =~ "^[[:space:]]*$" ]]; then
        echo "$line"
    fi
done

echo
echo "Test complete!"