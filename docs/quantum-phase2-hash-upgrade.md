# Quantum-Safe Migration – Phase 2 Hashing Upgrade

## Overview
Phase 2 promotes the lattice-augmented digest (`HashValue::quantum_safe_of`) as the canonical hash
throughout the workspace. The goal is to remove direct SHA3-256 dependencies from protocol
surfaces so that every new derivation inherits post-quantum collision resistance while preserving
the 32-byte layout used on wire.

## Key Changes
- CLI, governance tooling, SDK builders, consensus proposer election, Move VM metadata, and
  integration tests now route through the quantum-safe digest.
- Transaction emitters and multisig workflows derive payload hashes with the PQ helper, ensuring
  hashes recorded on-chain or passed to Move scripts no longer rely on classical-only SHA3.
- `types::hash_utils::canonical_hash` includes regression coverage to guard against accidental
  regressions back to legacy hash helpers.
- Ledger metadata APIs now surface `HashDigest`/`HashVersion` helpers to make the hash cohort explicit.
- `accudo move-tool replay` accepts `--digest-output` to materialize quantum-safe digest summaries when re-running a transaction locally.

## Compatibility
- Existing data derived with legacy SHA3 remains accepted; historical hashes are untouched.
- PQ and classical digests remain 32 bytes, so serialization formats did not change. Partners can
  continue consuming existing APIs without binary changes.
- Low-level cryptography that depends on legacy digest semantics (e.g., secp256k1 ECDSA signing,
  HKDF test vectors) intentionally retains SHA3-256 usage for standards compliance.

## Next Steps
1. ✅ Version ledger metadata with `HashDigest` / `HashVersion` to record when PQ digests are emitted.
2. ✅ Extend replay tooling to regenerate historical checkpoints with PQ hashes for auditability.
3. Remove remaining direct SHA3 dependencies once upstream crates expose PQ-aware alternatives.
