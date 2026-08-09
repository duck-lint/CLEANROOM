# Phase 5 Real Projection Construction Report

Status: completed construction evidence; not Phase 6 validation.

## Accepted input

- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `99d0d4556684000f0ed585e47158a5f7fe9ce7e1`
- Observer schema: `vault-observation/v2`
- Accepted specimen: `25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688`

The accepted observation was used directly. No live-vault hydration was
performed.

## Admission and materialization

- Whole resident source records: 1098
- Resident Markdown records: 1059
- Admission eligible: 1051
- Admitted objects: 1051
- Excluded source records: 8
- Semantic regions: 4346
- Semantic units: 17113
- Identifier descriptors: 55
- Identifier assignments: 12323
- Present-null identifier assignments: 3599
- Temporal anchors: 547

## Occurrences and closure

- Authored occurrences: 4989
- Object-field occurrences: 498
- Semantic-region occurrences: 185
- Semantic-unit occurrences: 4306
- Resolved occurrences: 4882
- Unresolved occurrences: 107
- Ambiguous occurrences: 0
- Heading-fragment targets: 10
- Heading-target exact-span joins: 10/10
- Ambiguous heading-target joins: 0
- Duplicate canonical region addresses: 0
- Duplicate unit-parent keys: 0
- Region containment failures: 0
- Region-source incidence failures: 0
- Region-target incoming-incidence failures: 0

The materializer uses the accepted root-down `canonical_region_identities`
constructor. Its returned addresses are reused for object region lists,
parent/child containment, unit parent regions, region occurrence sources, and
heading-fragment targets. Heading targets join by accepted object identity plus
exact matched heading span to an already materialized region address.

## Determinism and status

- First construction logical hash: `fnv1a:e8f60dea6631a262`
- Second construction logical hash: `fnv1a:e8f60dea6631a262`
- Deterministic equality: yes
- Projection schema: `semantic-space-projection/v1`
- Projection validation status: `Unvalidated`
- Full projection artifact: private and not committed

Phase 5 construction is complete against the accepted observation specimen.
Phase 6 has not begun; complete-vault projection validation remains outside this
report.
