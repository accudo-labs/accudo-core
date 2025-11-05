# Quantum-Safe Migration – Phase 3 Dual-Signature Support

## Goal
Introduce dual-signature support so classical authenticators can carry a companion
post-quantum (PQ) signature and public key, enabling hybrid verification during
the migration window.

## Implementation Notes
- `SingleKeyAuthenticator` now stores optional `PostQuantumPublicKey` +
  `SignatureBundle` pairs. A new helper `with_post_quantum` wires classical and
  PQ signatures together, while `verify_dual` enforces the bundle for classical
  schemes (Ed25519, secp256k1, secp256r1).
- `MultiKey`/`MultiKeyAuthenticator` track PQ public keys and signatures per
  signer. During verification, PQ attachments propagate into the derived
  `SingleKeyAuthenticator`s.
- New `PostQuantumPublicKey` wrapper records the scheme identifier alongside raw
  key bytes, ensuring verifier lookups can enforce algorithm consistency.
- Added regression tests covering:
  1. Classical + Dilithium signature bundles on single-key authenticators.
  2. Multi-key authenticators with per-signer Dilithium attachments.
  3. Failure paths when PQ material is missing but dual verification is requested.

## Compatibility
- Serialization remains backward compatible: PQ fields are optional with Serde
  defaults, so legacy transactions decode unchanged.
- Classical verification still succeeds when PQ data is absent; callers can opt
  into strict dual enforcement via the new `verify_dual` helpers before making
  it mandatory network-wide.

## Rollout Checklist
- [x] Ship PQ-default SDK and wallet builds; publish the partner cut-over notes in `docs/quantum-rollout-playbook.md`.
- [x] Enforce dual-signature verification across API, mempool, consensus, and VM admissions with dedicated telemetry.
- [x] Expose PQ digest summaries via `accudo move-tool replay --digest-output` to unblock migration dry-runs.

## Operational Telemetry
- `accudo_core_mempool_pq_signature_present_total` / `accudo_core_mempool_pq_signature_missing_total`
  track signature mix ratios at the mempool ingress.
- `accudo_consensus_pq_signature_blocks_total{status=compliant|missing}` surfaces consensus-side
  enforcement results for dashboards and alerting.
- Replay runs emit structured PQ digest reports; provide the optional `--digest-output <path>`
  flag to persist JSON payloads for post-processing pipelines.
