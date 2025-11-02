---
title: RFC: Cryptographic Trait Refactor for PQ Support
status: draft
owner: crypto-platform
reviewers: []
last_updated: 2024-06-01
---

# Summary
Introduce algorithm-agnostic traits for signatures, hashing, and key exchange so the Accudo codebase can adopt post-quantum (PQ) primitives alongside—or in place of—existing classical implementations.

# Motivation
- Current modules import concrete algorithms (`ed25519`, `sha3`, `x25519`) directly, making it difficult to slot in Dilithium, lattice hashes, or Kyber without large refactors.
- Many crates assume fixed key and signature sizes (32-byte public keys, 64-byte signatures), which breaks with PQ primitives.
- Hybrid rollouts require the ability to express “verify signature under scheme X” without duplicating business logic.

# Goals
1. Define lightweight traits in `crates/accudo-crypto` that encapsulate signature, hash, and key-encapsulation behaviors with explicit algorithm identifiers.
2. Provide adapter implementations for existing primitives while paving the way for Dilithium/Kyber integrations.
3. Update downstream crates to depend on trait objects/generic wrappers rather than concrete crypto modules, limiting churn when enabling PQ features.
4. Preserve ergonomics for current code paths (minimal generic boilerplate).

# Non-Goals
- Replacing classical primitives immediately (covered by the PQ migration plan).
- Designing new serialization formats (addressed in a separate ADR).
- Eliminating every instance of `Sha3`/`Ed25519` import in a single PR.

# Proposed Design
## Trait Hierarchy
- `SignatureAlgorithm`: exposes `type PublicKey`, `type Signature`, `fn scheme_id() -> SchemeId`, `fn verify(...)`, `fn public_key_length()`, `fn signature_length()`, optional aggregate helpers.
- `KeyExchangeAlgorithm`: exposes encapsulation/decapsulation or Diffie-Hellman style operations with negotiated algorithm metadata.
- `HashAlgorithm`: returns digest length, domain separation strategy, and streaming hasher constructor (wrapping existing `CryptoHasher`).

Each trait returns a stable `SchemeId` enum describing the algorithm family (`Ed25519`, `Dilithium3`, `Bls12381`, etc.) so payloads can carry metadata.

## Wrapper Types
- Introduce `AlgorithmRegistry` that can resolve trait implementations by `SchemeId` at runtime (backed by static map for built-ins).
- Provide `SignatureBundle` struct that stores `{scheme_id, bytes}` for transport.
- Update `accudo-crypto::traits` to re-export the generic traits; existing helper macros produce type-safe wrappers without hard-coding algorithms.

## Transition Strategy
1. Land traits and adapters for existing algorithms.
2. Refactor verification entry points (consensus, mempool, Move VM) to accept `SignatureBundle`.
3. Add feature gate `pq-experimental` to register Dilithium/Kyber once the implementations are ready.

# Impacted Areas
- `crates/accudo-crypto` (new traits/adapters, registry).
- Consumers in `consensus`, `mempool`, `state-sync`, `sdk`, and Move native modules.
- Serialization crates that expose signing keys (`crates/accudo-ledger`, `sdk`, `api`).

# Alternatives Considered
- Blanket generics on concrete algorithms (rejected: explosion of type parameters across the codebase).
- Runtime trait objects without registry (rejected: difficult to configure per-network defaults).

# Drawbacks
- Indirection may introduce minor runtime overhead (virtual dispatch).
- Requires broad but mechanical updates in many crates.
- Additional maintenance burden for registry and scheme metadata.

# Unresolved Questions
- Best location to persist algorithm IDs in on-chain structures.
- How to treat aggregate/threshold signatures in the trait hierarchy.
- Level of support for `no_std` targets once PQ crates are pulled in.

# Next Steps
1. Finalize trait API signatures and collect reviewer feedback.
2. Implement registry and classical adapters behind feature gate.
3. Prepare mechanical refactor PRs for consensus/mempool entry points.
