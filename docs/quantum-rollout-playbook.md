# Quantum Rollout Playbook

This checklist captures the partner-facing tasks required once the dual-signature
enforcement is live in the codebase.

## Partner Communication
- Publish release notes summarizing the enforcement changes (API rejects missing PQ
  signatures, mempool/consensus strict mode, replay CLI updates).
- Distribute the refreshed OpenAPI spec (`api/doc/spec.yaml`) so partners can
  regenerate clients that understand `signature.post_quantum`.
- Share SDK / wallet upgrade links and recommended minimum versions with exchanges,
  custodians, and node operators.
- Distribute the PQ telemetry dashboard URLs and explain the new metrics that should
  be monitored (`accudo_core_mempool_pq_signature_present_total`,
  `accudo_core_mempool_pq_signature_missing_total`,
  `accudo_consensus_pq_signature_blocks_total`).

## Network Rollout
- Schedule devnet → testnet → mainnet upgrade windows and confirm validator availability.
- Ensure validators import the release tag that contains the enforced PQ checks.
- Add a dry-run checkpoint by replaying a recent block with
  `accudo move-tool replay --digest-output <path>` and storing the digest report.

## Post-Rollout Verification
- Monitor the PQ signature dashboards for 100 % compliance and alert on any non-zero
  `missing` counts.
- Confirm downstream indexers and explorers are displaying the correct hash digests.
- Gather partner acknowledgements confirming SDK and signing services are upgraded.
