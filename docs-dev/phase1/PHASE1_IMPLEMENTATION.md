# Hash Forge - Fase 1 Extensions

Questo documento documenta le estensioni implementate nella Fase 1 del progetto Hash Forge.

## 🆕 Nuovi Algoritmi Implementati

### SHA-3 Family (Keccak-based)
- **SHA3-224**: Hash crittografico sicuro, output 224-bit
- **SHA3-256**: Alternativa moderna a SHA-256
- **SHA3-384**: Livello di sicurezza medio
- **SHA3-512**: Massimo livello di sicurezza SHA-3

### SHAKE Functions (Extendable Output)
- **SHAKE128**: Funzione di output estendibile con sicurezza 128-bit
- **SHAKE256**: Funzione di output estendibile con sicurezza 256-bit

### xxHash Family (Non-cryptographic, High Performance)
- **xxHash32**: Hash ultra-veloce a 32-bit per controlli di integrità
- **xxHash64**: Hash ultra-veloce a 64-bit per controlli di integrità
- **xxHash3**: Versione moderna di xxHash, ottimizzato per performance

## 🔐 HMAC Support

Implementato supporto completo per HMAC (Hash-based Message Authentication Code) con tutti gli algoritmi crittografici supportati:

- HMAC-SHA1
- HMAC-SHA256
- HMAC-SHA512  
- HMAC-SHA3-224
- HMAC-SHA3-256
- HMAC-SHA3-384
- HMAC-SHA3-512
- HMAC-BLAKE2b
- HMAC-BLAKE2s

## 📝 Esempi di Utilizzo

### Algoritmi SHA-3
```bash
# SHA3-256 per testo
./target/release/hash-forge text --input "Hello SHA-3!" --algorithm sha3-256

# SHA3-512 per file
./target/release/hash-forge file --path document.pdf --algorithm sha3-512

# SHAKE128 con output estendibile
./target/release/hash-forge text --input "Extendable output" --algorithm shake128
```

### xxHash per Performance
```bash
# xxHash3 per file grandi (non crittografico ma velocissimo)
./target/release/hash-forge file --path large_dataset.bin --algorithm xxh3

# xxHash64 per verifica rapida integrità
./target/release/hash-forge file --path backup.tar --algorithm xxh64
```

### HMAC per Autenticazione
```bash
# Compute HMAC per autenticazione messaggio
./target/release/hash-forge hmac --text "Secret message" --key "my_secret_key" --algorithm sha256

# HMAC di un file
./target/release/hash-forge hmac --file sensitive_data.json --key "api_secret" --algorithm sha3-256

# Verifica HMAC
./target/release/hash-forge verify-hmac --text "Secret message" --key "my_secret_key" --expected-hmac "abc123..." --algorithm sha256

# HMAC con algoritmi SHA-3
./target/release/hash-forge hmac --text "Modern crypto" --key "secure_key" --algorithm sha3-384
```

## 🔬 Casi d'Uso Specifici

### 1. Blockchain & Cryptocurrency
```bash
# SHA3-256 per compatibilità Ethereum
./target/release/hash-forge text --input "ethereum_address" --algorithm sha3-256

# SHAKE per proof-of-work personalizzati
./target/release/hash-forge text --input "block_data" --algorithm shake256
```

### 2. High-Performance Data Processing
```bash
# xxHash3 per checksum rapidi su dataset
./target/release/hash-forge file --path big_data.csv --algorithm xxh3

# Batch processing con xxHash per speed
./target/release/hash-forge batch --directory ./data_files --algorithm xxh64
```

### 3. API Authentication
```bash
# HMAC per firma API requests
./target/release/hash-forge hmac --text "GET /api/users timestamp:1234567890" --key "api_secret_key" --algorithm sha256

# Verifica signature API
./target/release/hash-forge verify-hmac --text "API_REQUEST_BODY" --key "shared_secret" --expected-hmac "received_signature" --algorithm sha3-256
```

### 4. Modern Security Standards
```bash
# SHA3 per compliance con nuovi standard
./target/release/hash-forge file --path certificate.pem --algorithm sha3-256

# SHAKE per key derivation
./target/release/hash-forge text --input "master_password:salt:iteration" --algorithm shake128
```

## 🚀 Performance Comparison

| Algorithm | Type | Speed | Security | Use Case |
|-----------|------|-------|----------|----------|
| SHA3-256 | Crypto | Medium | High | Modern standards |
| SHAKE128 | Crypto | Medium | High | Flexible output |
| xxHash3 | Non-crypto | Ultra Fast | None | Checksums |
| HMAC-SHA256 | MAC | Medium | High | Authentication |
| BLAKE3 | Crypto | Fast | High | General purpose |

## 📋 Testing & Validation

Tutti i nuovi algoritmi sono stati testati per:

- ✅ Compatibilità con standard internazionali
- ✅ Performance su file di diverse dimensioni
- ✅ Correttezza crittografica
- ✅ Integrazione con l'interfaccia esistente
- ✅ Cross-platform compatibility (macOS, Linux, Windows)
- ✅ Memory safety e security
- ✅ Constant-time operations per HMAC

## 🔧 Technical Implementation

### Dependency Updates
```toml
# Aggiunte al Cargo.toml
sha3 = "0.10"          # SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE128, SHAKE256
xxhash-rust = { version = "0.8", features = ["xxh32", "xxh64", "xxh3"] }
```

### New Modules
- `hmac_core.rs`: Gestione completa HMAC con constant-time verification
- Estensioni a `algorithms.rs`: Nuovi algoritmi e metadati
- Estensioni a `core.rs`: Implementazione algoritmi per text e file
- Estensioni a `cli.rs`: Comandi `hmac` e `verify-hmac`

### Security Features
- **Constant-time comparison**: Previene timing attacks in HMAC verification
- **Secure memory handling**: Proper cleanup di chiavi sensibili
- **Algorithm validation**: Controlli per combinazioni supportate
- **Error handling**: Gestione robusta errori crittografici

## 🎯 Next Steps (Fase 2)

La Fase 1 pone le basi per:
- Web interface con i nuovi algoritmi
- REST API per servizi di hashing
- Plugin system per algoritmi custom
- Advanced key management
- Performance optimization con SIMD

Questa implementazione mantiene la filosofia di Hash Forge: sicurezza, performance e usabilità professionale.
