---
title: Quantum Posture
status: draft
owner: crypto-platform
last_updated: 2024-08-19
---

# Overview

The workspace now compiles in the post-quantum path unconditionally:

- Dilithium3 signing and verification adapters from `pqcrypto-dilithium` are
  part of the default build.
- `accudo-types` routes all canonical hashes (account addresses,
  authentication keys, WebAuthn challenges, etc.) through the lattice-enhanced
  digest.
- Classical-only fallbacks have been removed; disabling the PQ code is no longer
  supported.

# Operational Notes

- Existing data derived with the legacy SHA3-only hash remains valid, but new
  derivations will emit the lattice-augmented digest. Downstream indexers and
  SDKs must update to the latest release to stay in sync.
- Transaction verification still accepts classical signatures for now; future
  work will require dual-signature (classical + PQ) payloads before removing the
  legacy signature types entirely.
- Tooling that previously relied on feature flags (`quantum_hybrid`,
  `quantum_strict`) can drop them—the build configuration is fixed in quantum
  mode.
