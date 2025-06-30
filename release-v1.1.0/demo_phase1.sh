#!/bin/bash

# Hash Forge Fase 1 - Demo Script
# Questo script dimostra tutte le nuove funzionalità implementate nella Fase 1

echo "🔧 Hash Forge - Fase 1 Demo"
echo "============================="
echo

# Test dei nuovi algoritmi SHA-3
echo "🔐 Testando algoritmi SHA-3..."
./target/release/hash-forge text --input "SHA-3 Test" --algorithm sha3-224
echo
./target/release/hash-forge text --input "SHA-3 Test" --algorithm sha3-256
echo
./target/release/hash-forge text --input "SHA-3 Test" --algorithm sha3-384
echo
./target/release/hash-forge text --input "SHA-3 Test" --algorithm sha3-512
echo

# Test delle funzioni SHAKE
echo "🌊 Testando funzioni SHAKE (output estendibile)..."
./target/release/hash-forge text --input "SHAKE Test" --algorithm shake128
echo
./target/release/hash-forge text --input "SHAKE Test" --algorithm shake256
echo

# Test degli algoritmi xxHash per performance
echo "⚡ Testando algoritmi xxHash (ultra-veloce)..."
./target/release/hash-forge text --input "Speed Test" --algorithm xxh32
echo
./target/release/hash-forge text --input "Speed Test" --algorithm xxh64
echo
./target/release/hash-forge text --input "Speed Test" --algorithm xxh3
echo

# Test HMAC
echo "🔑 Testando HMAC Authentication..."
echo "HMAC con SHA-256:"
HMAC_RESULT=$(./target/release/hash-forge hmac --text "Secret message" --key "test_key" --algorithm sha256 | grep "HMAC (hex):" | cut -d' ' -f3)
echo

echo "Verifica HMAC:"
./target/release/hash-forge verify-hmac --text "Secret message" --key "test_key" --expected-hmac "$HMAC_RESULT" --algorithm sha256
echo

echo "HMAC con SHA3-256:"
./target/release/hash-forge hmac --text "Modern crypto" --key "secure_key" --algorithm sha3-256
echo

echo "HMAC con SHA-512:"
./target/release/hash-forge hmac --text "Maximum security" --key "ultra_secure_key" --algorithm sha512
echo

# Test performance comparison
echo "🏁 Confronto Performance (stesso input)..."
TEST_INPUT="Performance comparison test with longer input to better showcase speed differences between algorithms"

echo "Testando velocità algoritmi:"
echo "- BLAKE3 (fastest crypto):"
time ./target/release/hash-forge text --input "$TEST_INPUT" --algorithm blake3 > /dev/null
echo

echo "- xxHash3 (fastest overall):"
time ./target/release/hash-forge text --input "$TEST_INPUT" --algorithm xxh3 > /dev/null
echo

echo "- SHA3-256 (modern):"
time ./target/release/hash-forge text --input "$TEST_INPUT" --algorithm sha3-256 > /dev/null
echo

echo "- SHA-256 (standard):"
time ./target/release/hash-forge text --input "$TEST_INPUT" --algorithm sha256 > /dev/null
echo

# Test caso d'uso blockchain
echo "⛓️  Caso d'uso: Blockchain/Ethereum compatibility..."
./target/release/hash-forge text --input "ethereum_transaction_data" --algorithm sha3-256
echo

# Test caso d'uso API authentication
echo "🔐 Caso d'uso: API Authentication..."
API_PAYLOAD="GET /api/users timestamp:$(date +%s)"
echo "API Request: $API_PAYLOAD"
./target/release/hash-forge hmac --text "$API_PAYLOAD" --key "api_secret_key" --algorithm sha256
echo

echo "✅ Demo Fase 1 completato!"
echo "Tutti i nuovi algoritmi e funzionalità sono operative."
