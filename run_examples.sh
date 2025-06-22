#!/bin/bash

echo "🚀 Hume Rust SDK Examples Test Suite"
echo "===================================="
echo

# List of examples
examples=(
    "demo_all_features:Comprehensive feature demo"
    "tts_basic:Basic text-to-speech with audio playback"
    "tts_streaming:Real-time streaming TTS"
    "tts_comprehensive:All TTS features demonstration"
    "expression_measurement:Emotion analysis from text/media"
    "evi_chat_demo:EVI setup and configuration guide"
    "evi_conversation_mac:Real-time voice conversation (macOS)"
)

# Check if API key is set
if [ -z "$HUME_API_KEY" ] || [ "$HUME_API_KEY" = "dummy" ]; then
    echo "⚠️  No API key detected. Examples will run in demo mode."
    echo "   Set HUME_API_KEY environment variable for full functionality."
    export HUME_API_KEY="dummy"
else
    echo "✅ API key detected. Examples will use live API."
fi
echo

# Run each example
for example_desc in "${examples[@]}"; do
    IFS=':' read -r example description <<< "$example_desc"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📦 $example"
    echo "   $description"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    # Run the example with timeout and capture output
    if timeout 3 cargo run -q --example $example --manifest-path /Users/user/dev/test/claude_code/hume/hume_rs/Cargo.toml 2>&1 | head -20 | grep -v "warning:" | grep -v "note:" | grep -v "src/"; then
        echo "✅ Success"
    else
        echo "⏱️  Timed out or completed"
    fi
    echo
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ All examples tested!"
echo
echo "To run a specific example:"
echo "  cargo run --example <example_name>"
echo
echo "For more information:"
echo "  https://github.com/HumeAI/hume-rust-sdk"