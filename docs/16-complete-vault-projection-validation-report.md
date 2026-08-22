# Phase 6 Complete-Vault Projection Validation Report

Status: **validator reconciled; private Phase 6 validation pending**.

This report records the mechanical reconciliation of the existing Phase 6
validator and evidence contract after merged PR #23. It does not claim current
whole-corpus validation. Phase 7 has not begun.

## Corrected Phase 5 input contract

The validator now requires the corrected projection representation:

- CLEANROOM base: `0179bf75d34c97f7d1adefbaeee869f57e361b0e` (merged PR #23)
- projection schema: `semantic-space-projection/v2`
- projection snapshot: `projection:phase5:v2:<corpus-snapshot>`
- historical corrected Phase 5 projection byte SHA-256:
  `f7423f494d905799e44b2c98f470429ae960ae682b43158f90d8b8f6fa9e39d2`
- projection validation status required before promotion: `Unvalidated`

The validator no longer constructs or validates projection-owned
`configuration_snapshot_id`, `available`, routine candidate limits,
`AvailabilityOnly`, or provider/index status text. Its independently derived
surface descriptors use the five structural families and structural
`Bounded` coverage. Runtime configuration and access state remain outside the
projection contract.

The historical byte hash above is retained evidence from the original
pre-archive PR #23 construction. The 2026-08-22 recovery did not possess or
reconstruct that private artifact.

## Public and synthetic reconciliation

The following are now checked against the v2 source/schema contracts:

- validator surface descriptors use only v2 fields;
- Phase 5 identity checks require the v2 schema and version-qualified snapshot;
- synthetic validator fixtures construct v2 projections without removed
  projection/runtime fields;
- v1 Phase 6 input assumptions are absent from the validator implementation;
- the existing independent correspondence, topology, incidence, provenance,
  temporal, transport, and deterministic-promotion checks remain in place.

The validator remains independent from the Phase 5 constructor and does not
use the constructor's high-level closure result as an oracle.

## Historical pre-archive PR #21 evidence

The former PR #21 evidence described a different input and is retained only as
historical audit context:

- old base: `eda0f76d14c1385cee112a3298ed66c0a1411dc4`
- old schema/input representation: `semantic-space-projection/v1`
- old Phase 5 projection byte SHA-256:
  `4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a`
- old Phase 5 logical hash:
  `sha256:b21a96fdf2951dd8777c72a3acff3ac0ffa16ffb04d697aa6e7ee6855993c7a8`
- old Phase 6 output byte SHA-256:
  `f0f5c0b56895952aeb402b8eaea2a6f73f2124c5584d00942da928dd92b80ef1`
- old Phase 6 logical hash:
  `sha256:ae2ef75756cf4c60e65e1d73da5a20a5e38561b5f1042a50b568eb443de1bf71`

Those values are not evidence that the corrected v2 projection has passed
Phase 6. No current corpus counts, validation status, or promoted output are
asserted here.

## Current validator provenance

The reconciled public implementation is composed of:

- `src/bin/phase6_validate.rs`
- `src/construction.rs` for the byte-level SHA-256 primitive
- `src/validation.rs`

The current branch is based on merged PR #23. Exact file-byte hashes and the
commit containing this reconciliation are established by Git and public local
validation; they do not establish private corpus validation.

## Completion boundary

Actual Phase 6 completion still requires all of the following private evidence:

1. the exact pinned `vault-observation/v3` artifact;
2. the exact corrected v2 Phase 5 projection bytes;
3. two independent validator runs with zero deterministic violations;
4. byte-identical promoted outputs and corresponding hashes.

That evidence is unavailable in this recovery environment. The validator is
therefore reconciled and publicly/synthetically testable, but Phase 6 remains
incomplete. No artifact, hash, corpus count, or validation result has been
fabricated.

Real projection access remains Phase 7 work. PR #9 remains untouched and
provisional.
