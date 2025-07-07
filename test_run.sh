#!/bin/bash

# Test script for solana-burn-cli
# This uses a test private key for testing (DO NOT USE IN PRODUCTION)

# This is a test keypair - DO NOT USE WITH REAL FUNDS
# You should replace this with your own test keypair
TEST_PRIVATE_KEY="11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"

echo "Testing Solana Burn CLI..."
echo "Note: This uses a test keypair and may not have any token accounts"
echo ""

# Make the script executable
chmod +x "$0"

# Run with devnet to be safe
cargo run -- --private-key "$TEST_PRIVATE_KEY" --rpc-url "https://api.devnet.solana.com"
