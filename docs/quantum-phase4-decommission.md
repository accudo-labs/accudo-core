# Quantum-Safe Migration – Phase 4 Decommissioning

## Goals
- Remove the ability to submit classical-only transactions or REST payloads.
- Require Dilithium post-quantum attachments for every classical authenticator.
- Surface clear API errors when PQ materials are absent or malformed.
- Capture observability confirming 100 % PQ compliance across the network.

## Implementation Highlights
- REST `SubmitTransaction` / `SubmitTransactionsBatch` pathways now reject
  requests missing a `post_quantum` bundle for single-key authenticators.
- `SingleKeySignature` verification enforces the presence of Dilithium metadata
  and propagates it to `AccountAuthenticator` / `TransactionAuthenticator`.
- Mempool, consensus, and replay tooling emit PQ-focused counters and refuse
  classical-only payloads.
- CLI replay (`accudo move-tool replay`) surfaces PQ digests and telemetry
  that validators can archive for post-migration audits.

## Release Notes Highlights
- OpenAPI artifacts (`api/doc/spec.{yaml,json}`) expose the new
  `signature.post_quantum` envelope so SDK codegen picks up the requirement by
  default.
- REST regression suites (`api/src/tests`) now inject Dilithium sidecars into
  positive submissions and assert the 400 response for classical-only payloads.
- Telemetry docs link the new counters exported from
  `mempool/src/counters.rs` and `consensus/src/counters.rs` for dashboard
  wiring.

## Rollout Checklist
- [x] Reject classical-only submissions at the REST ingress with `VmError`.
- [x] Require Dilithium attachments in API structs (`SingleKeySignature.post_quantum`).
- [x] Emit enforcement counters `accudo_core_mempool_pq_signature_*` and
  `accudo_consensus_pq_signature_blocks_total`.
- [x] Update SDK/test harnesses to attach Dilithium signatures and assert
  for the new fields in JSON responses.

## Observability
- `accudo_core_mempool_pq_signature_present_total` /
  `accudo_core_mempool_pq_signature_missing_total`
  highlight ingestion success vs. filtered payloads.
- `accudo_consensus_pq_signature_blocks_total{status}` records how many blocks
  were compliant vs. rejected due to missing PQ material.
- CLI replay reports include `post_quantum` digests to support ledger audits.
