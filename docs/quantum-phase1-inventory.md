# Quantum-Safe Migration – Phase 1 Inventory & Risk Baseline

## Scope & Methodology
- Reviewed workspace crypto modules and configuration surfaces (`crates/accudo-crypto`, `types`, `network`, `config`, `consensus`, `sdk`) using static inspection.
- Consulted existing posture docs (`docs/pq-migration-plan.md`, `docs/quantum-build-modes.md`) to align terminology and current defaults.
- Attempted to run the automated inventory tool (`cargo run -p pq-inventory`), but the build requires fetching `dudect-bencher` from GitHub and failed under restricted network access; results below are manual.

## Current Cryptographic Footprint

### Signatures & Attestation
| Primitive | Primary Callers | PQ Posture | Notes |
| --- | --- | --- | --- |
| Ed25519 / MultiEd25519 | Transaction authenticators (`types/src/transaction/authenticator.rs:20`), wallets, SDK (`sdk/Cargo.toml:25`) | Classical only | Still default for user transactions; PQ registry wraps it for coexistence. |
| secp256k1 ECDSA | Keyless APIs, transaction filters (`crates/accudo-transaction-filters/src/transaction_filter.rs:522`) | Classical only | Required for compatibility with existing Web3 tooling. |
| secp256r1 ECDSA | WebAuthn keyless flow (`api/src/tests/webauthn_secp256r1_ecdsa.rs:11`) | Classical only | Backed by RustCrypto implementation; subject to Shor once at scale. |
| BLS12-381 | Consensus voting (`consensus/consensus-types/src/block.rs:12`), DKG | Classical only | Critical path for safety rules; no PQ aggregate alternative yet. |
| Dilithium3 | PQ registry baseline (`crates/accudo-crypto/src/pq/mod.rs:326`) | Post-quantum (NIST L3) | Available for dual-signature payloads; mostly exercised in tests today. |

### Key Exchange, KEM & Encryption
| Primitive | Location | PQ Posture | Notes |
| --- | --- | --- | --- |
| X25519 Diffie-Hellman | Network Noise handshake (`network/framework/src/transport/mod.rs:525`) | Classical only | Mandatory path; vulnerable to retrospective decryption under Shor. |
| Kyber768 KEM | Crypto wrappers (`crates/accudo-crypto/src/pq/kyber.rs:13`) & config plumbing (`config/src/config/identity_config.rs:38`) | PQ-capable (optional) | Keys generated for nodes, but network addresses still permit legacy-only peers. |
| ElGamal + AES-256-GCM hybrid | Data encryption tooling (`crates/accudo-crypto/src/asymmetric_encryption/elgamal_curve25519_aes256_gcm.rs:25`) | Hybrid (classical DH + PQ-strong symmetric) | Symmetric layer safe if keys originate from PQ source; outer ElGamal is Curve25519. |

### Hashing, Merkle, PRFs
- `HashValue::quantum_safe_of` XORs SHA3-256 with a lattice accumulator and backs canonical hashes (`crates/accudo-crypto/src/hash.rs:204`, `types/src/hash_utils.rs:16`).
- HKDF utility still relies on classical hash primitives but supports SHA3-based instantiations (`crates/accudo-crypto/src/hkdf.rs:5`).
- Poseidon and other experimental hashes live under `crates/accudo-crypto/src/poseidon_bn254`, currently opt-in research.

### Key Management & Storage Surfaces
- Validator identity blobs now carry optional Kyber private keys alongside x25519 (`config/src/config/identity_config.rs:36`).
- Safety rules still persist BLS private keys for consensus signing (`consensus/safety-rules/src/persistent_safety_storage.rs:37`).
- SDK derives Ed25519 keys via BIP32 (`sdk/Cargo.toml:25`), implying mnemonic and derivation tools remain classical.

### Dependent Tooling & SDKs
- `accudo-crypto/Cargo.toml:15` lists classical dependencies (ed25519-dalek, blst, libsecp256k1, p256) alongside PQ crates (pqcrypto-dilithium, pqcrypto-kyber).
- TypeScript/Javascript utilities do not presently ship cryptography; all signing flows delegate to Rust crates or external wallets.

## Asset Classification & Quantum Risk
- **Consensus safety keys** (BLS12-381, Ed25519) – high criticality; long-lived; immediate PQ replacement/overlay required before quantum adversaries emerge.
- **Network transport secrets** (x25519) – high exposure; passive capture today enables future decryption; hybrid X25519+Kyber must be enforced.
- **On-chain authentication keys & addresses** – medium; canonical digest already PQ-hardened, but historical data hashed pre-upgrade needs replay tooling.
- **Client wallet keys** (Ed25519/BIP32) – high; rely on third-party wallets; PQ signature rollout needs ecosystem coordination.
- **Keyless/WebAuthn credentials** (webauthn P-256) – medium; browser ecosystem lagging on PQ, fallback plans required.
- **Internal tooling secrets** (ElGamal+AES, HKDF) – medium; safe once fed PQ entropy; ensure seed material migrates alongside key upgrades.

## Known Gaps & Classical Dependencies
- Transaction verification still accepts classical-only payloads; Dilithium signatures optional (`types/src/transaction/authenticator.rs:36`).
- Network layer treats Kyber identity keys as optional extension (`network/framework/src/transport/mod.rs:523`), so pure-classical peers remain allowed.
- Safety-rules and DKG stack exclusively BLS12-381 with no PQ attestations yet (`consensus/consensus-types/src/block.rs:55`).
- Wallet tooling and SDK depend on `ed25519-dalek-bip32` and legacy mnemonic standards (`sdk/Cargo.toml:25`, `sdk/Cargo.toml:34`).
- Automated inventory tooling blocked by restricted network; results must be periodically regenerated once access is restored.

## Threat Model & Guardrails
- **Adversary assumptions**: Large-scale quantum attacker capable of Shor (breaking ECC/finite-field DH) and Grover (square-root speedup on symmetric/hash); retrospective decryption risk assumed for data in transit captured today.
- **Security targets**: ≥ 128-bit PQ security for signatures/KEMs (Dilithium3, Kyber768), ≥ 256-bit classical equivalent for hashes (SHA3+lattice), defend against side-channel leakage per `RUST_SECURE_CODING.md`.
- **Guardrails**:
  - Maintain dual-signature envelopes until classical support can be disabled; reject payloads missing PQ signature after migration gate flips.
  - Enforce hybrid Noise handshakes (X25519 + Kyber) with telemetry on downgrade attempts.
  - Track algorithm identifiers via `SchemeId` to ensure protocol negotiations cannot silently fallback (`crates/accudo-crypto/src/pq/mod.rs:37`).
  - Require PQ-capable storage of private keys (HSM/KMS firmware upgrades) before cutting classical keys.

## Migration Success Metrics (Baseline Targets)
1. 100 % of transaction authenticators include a validated Dilithium (or approved PQ) signature in addition to any classical signature.
2. Network handshake metrics show 0 classical-only peers for validator and fullnode roles.
3. Ledger checkpoint pipelines emit only lattice-augmented hashes; replay tooling verified against historical snapshots.
4. SDK and wallet ecosystem expose PQ key derivation and signing APIs with end-to-end tests.
5. Automated inventory (tools/pq-inventory) runs cleanly in CI to flag regressions in classical dependency usage.

## Recommended Phase 2 Prep
- Prioritize enabling Kyber enforcement in handshake negotiation and collect downgrade telemetry.
- Draft SDK ergonomics plan for dual-signature transactions and mnemonic migration.
- Restore inventory automation under controlled network access or vendor mirror to keep dependency list current.
