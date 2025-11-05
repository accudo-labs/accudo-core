---
title: Post-Quantum Migration Plan
status: draft
owner: crypto-platform
last_updated: 2024-09-21
---

# Objectives
- Provide end-to-end post-quantum (PQ) protection for signatures, key exchange, hashing, and hashing-derived data structures.
- Preserve wire/storage compatibility during a transitional phase while quantifying risks for legacy clients.
- Deliver a migration path that can be validated in staging networks before flipping defaults.

# Current Cryptographic Footprint
- **Signatures**: Ed25519, MultiEd25519, secp256k1 ECDSA, secp256r1 ECDSA, BLS12-381 aggregate signatures (consensus, DKG, keyless).
- **Key Exchange / Encryption**: X25519 (Noise), Curve25519 ElGamal + AES-256-GCM hybrids.
- **Hashing / PRFs**: SHA3-256 (primary), SHA2-256/SHA-512 (Move natives, HKDF, tooling), Keccak-based Merkle (Jellyfish), Poseidon (experimental).
- **Derived formats**: Authentication keys and account addresses derived from Ed25519 public keys, BCS-serialized Merkle paths, transaction envelopes that assume 32-byte hashes and Ed25519 signatures.

# Target PQ Primitives
| Use-Case                 | Proposed Primitive(s)                       | Notes |
| ------------------------ | ------------------------------------------- | ----- |
| Signature (validator)    | CRYSTALS-Dilithium 3 (NIST Level 3)         | Balance between size and verification speed. |
| Signature (clients/L2)   | Falcon-512 or Dilithium 2                   | Evaluate UX vs hardware limits; may reuse Dilithium if Falcon rollout risk too high. |
| Aggregate signatures     | BLS fallback + PQ overlay (research)        | Until MQ-based aggregates mature; maintain classical path with PQ attestations. |
| Key encapsulation        | CRYSTALS-Kyber 4 (for network Noise hybrid) | Hybrid with X25519 until legacy drop. |
| Hashing / Merkle         | SHA3-256 ⊕ lattice accumulator (already prototyped) | Promote combined digest to canonical `HashValue`. |
| Symmetric crypto         | AES-256-GCM + XSalsa20/Poly1305 fallback    | Remains quantum-safe when keys are PQ-originated. |
| PRFs / HKDF              | HKDF-SHA3 or KMAC                           | Reduce SHA2 reliance. |

# Phased Delivery Plan
## Phase 0 – Analysis (current)
- Catalog all signature, hash, KDF, and key-exchange call sites (automated inventory script).
- Document state formats that embed classical crypto (accounts, proofs, snapshots, Move natives).
- Produce threat model addendum describing hybrid vs pure-PQ trade-offs.

## Phase 1 – Abstraction Hardening
- Extend `crates/accudo-crypto` traits to expose scheme families (`SignatureScheme`, `KeyExchangeScheme`, `HashFunction`) with algorithm identifiers.
- Update downstream crates to depend on traits rather than concrete Ed25519/sha3 modules.
- Lock quantum-safe hashing and Dilithium support on by default.
  - ✅ Implemented in `accudo-crypto` / `accudo-types`; see `docs/quantum-build-modes.md` for current posture.

## Phase 2 – Hashing Upgrade
- Promote `HashValue::quantum_safe_of` as the default path; backfill deterministic test vectors.
- Replace direct `sha2`/`sha3` invocations in Move natives, storage, and tooling with the wrapper.
- Version storage formats (Jellyfish Merkle, indexer checkpoints) to support recomputation using the combined digest; provide offline migration tool.
- Publish compatibility guide for nodes replaying old ledgers with legacy hashes.

## Phase 3 – Dual-Signature Transactions
> Status: Completed in core (see `docs/quantum-phase3-dual-signatures.md`).
- Implement Dilithium key generation, signing, and verification in `crates/accudo-crypto/pq`.
- Extend transaction/authentication payloads to carry `{classical_sig, pq_sig}` pairs plus algo metadata.
- Update consensus / mempool verification to enforce PQ signature presence; provide metrics for mismatch detection.
- Build key-rotation tooling for validators, wallets, and Move modules; ship CLI support.

## Phase 4 – API & Network Decommissioning
> Status: Complete; REST, mempool, and consensus now reject classical-only payloads (see `docs/quantum-phase4-decommission.md`).
- Integrate Kyber-based KEM with the Noise handshake (X25519 + Kyber hybrid) and surface negotiation counters.
- Rotate validator network identities; update `config/` templates and keyless operations.
- Evaluate on-chain key distribution (Move module updates) for PQ keys.
- Remove classical-only REST fallbacks and document the `post_quantum` payload requirements for client SDKs and tooling.

## Phase 5 – State & Storage Migration
- Add ledger checkpointing that records hash algorithm version.
- Upgrade secure storage / vault providers to store PQ private keys and metadata.
- Provide replay tool that rehydrates historical states with PQ hashes for auditability.

## Phase 6 – Testing & Rollout
- Expand fuzzers and property tests to cover mixed signature verification paths.
- Benchmark PQ operations (consensus latency, gas costs) using Forge/Move harnesses.
- Stage rollout: devnet (hybrid optional) → testnet (hybrid required) → mainnet (PQ default, classical optional).
- Publish migration timeline, including client SDK deadlines and fallbacks.

## Phase 7 – Decommission Legacy Crypto
- Disable classical signatures/hashes behind feature gates once adoption > 99%.
- Archive compatibility tooling; document recovery procedures for stragglers.

# Immediate Action Items
1. Implement repo-wide crypto inventory script (Rust CLI under `tools/`).
2. Author trait refactor RFC (align with `RUST_SECURE_CODING.md` guidance).
3. Promote lattice-enhanced hash and add regression tests and golden vectors.
4. Prototype Dilithium signing crate integration (gated behind `pq-dilithium` feature). ✅

# Risks & Open Questions
- Transaction size increase and gas economics with dual signatures.
- Availability of audited Dilithium/Kyber Rust crates compatible with `no_std` targets.
- Aggregation and threshold signatures in PQ world remain immature; interim reliance on classical BLS may persist.
- Storage migration cost for rehashing large ledgers; need incremental approach.
- Regulatory / ecosystem expectations for continued Ed25519 support.

# References
- NIST PQC Finalization reports (2024)
- Noise PQ hybrid draft proposals
- Move language Merkle accumulator documentation
