# Phase 6 Complete-Vault Projection Validation Report

Status: **Validated**

This report records independent Phase 6 correspondence and closure validation
of the frozen Phase 5 projection. Phase 5 construction evidence remains in
`docs/15-real-projection-construction-report.md`; that report remains the
historical record that the projection was `Unvalidated` and that Phase 6 had
not begun. Phase 7 has not begun.

## Exact evidence identities

- CLEANROOM authoritative base: `eda0f76d14c1385cee112a3298ed66c0a1411dc4`
- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `e9bb2d95c14b1beb334dc2b8d83420f5998b9a53`
- Observer schema: `vault-observation/v3`
- Specimen / corpus snapshot: `f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Observation artifact SHA-256: `d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9`
- Phase 5 input projection byte SHA-256: `4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a`
- Phase 5 input logical hash: `sha256:b21a96fdf2951dd8777c72a3acff3ac0ffa16ffb04d697aa6e7ee6855993c7a8`
- Phase 5 input snapshot: `projection:phase5:f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Phase 6 validated snapshot: `projection:phase6:f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Phase 6 logical hash: `sha256:ae2ef75756cf4c60e65e1d73da5a20a5e38561b5f1042a50b568eb443de1bf71`
- Phase 6 validated projection byte SHA-256: `f0f5c0b56895952aeb402b8eaea2a6f73f2124c5584d00942da928dd92b80ef1`

Input bytes were hashed before parsing. The Phase 5 input was not overwritten.
The Phase 6 output changes only `projection_snapshot_id`, `validation_status`,
and `logical_hash`.

## Validated corpus totals

| Measure | Count |
| --- | ---: |
| Resident source records | 1060 |
| Resident Markdown | 1060 |
| Admitted sources | 1052 |
| Excluded Markdown | 8 |
| Object classes | 14 |
| Objects | 1052 |
| Regions | 4356 |
| Semantic units | 17118 |
| Identifier descriptors | 55 |
| Identifier assignments | 12351 |
| Occurrences | 5003 |
| Object-field occurrences | 501 |
| Semantic-region occurrences | 184 |
| Semantic-unit occurrences | 4318 |
| Resolved occurrences | 4895 |
| Unresolved occurrences | 108 |
| Ambiguous occurrences | 0 |
| FullDate anchors | 532 |
| DateTime anchors | 1 |
| ExactYear anchors | 7 |
| MonthDay anchors | 3 |
| ApproximateYear anchors | 5 |
| Non-null temporal anchors | 548 |
| Present-null temporal assignments | 64 |
| Retrieval surfaces | 5 |
| Structural transitions | 22 |
| Transport segments | 0 |
| Block-fragment occurrences | 0 |
| Resolved block targets | 0 |
| Unresolved block targets | 0 |

## Independent validation domains

Each domain completed with zero deterministic structural violations:

| Domain | Status | Failures |
| --- | --- | ---: |
| Admission | passed | 0 |
| Object | passed | 0 |
| Region | passed | 0 |
| Unit | passed | 0 |
| Identifier | passed | 0 |
| Occurrence | passed | 0 |
| Target | passed | 0 |
| Reverse incidence | passed | 0 |
| Temporal | passed | 0 |
| Surface | passed | 0 |
| Provenance | passed | 0 |
| Transport segmentation | passed | 0 |
| Bounds | passed | 0 |
| Deterministic identity | passed | 0 |

The validator derives admitted source membership, object correspondence,
heading-region topology, authored block units, identifier assignments,
authored occurrence identities, exact occurrence source/target correspondence,
and field-specific temporal anchors directly from the exact v3 observation.
Expected and projected region, unit, assignment, descriptor, class, surface,
transition, and anchor sets are checked in both directions. Typed checks then
close canonical references, forward/reverse incidence, capabilities, bounds,
provenance, and subordinate transport records. It does not call the Phase 5
constructor or Phase 5 high-level closure helper as an oracle.

The completeness correction added correspondence-path falsification coverage
for invented regions and units and canonical assignment-value corruption, in
addition to the existing identity, parent, exclusion, target, incidence,
surface-bound, transition-reference, and deterministic-promotion cases. The
production byte-hash pinning remains at the file boundary; synthetic tests use
the same pure correspondence machinery after parsing.

## Determinism

The exact private inputs were validated twice:

- Run 1 logical hash: `sha256:ae2ef75756cf4c60e65e1d73da5a20a5e38561b5f1042a50b568eb443de1bf71`
- Run 2 logical hash: `sha256:ae2ef75756cf4c60e65e1d73da5a20a5e38561b5f1042a50b568eb443de1bf71`
- Run 1 output bytes SHA-256: `f0f5c0b56895952aeb402b8eaea2a6f73f2124c5584d00942da928dd92b80ef1`
- Run 2 output bytes SHA-256: `f0f5c0b56895952aeb402b8eaea2a6f73f2124c5584d00942da928dd92b80ef1`
- Logical hashes identical: yes
- Output bytes identical: yes

## Evidence limitations and boundaries

- The 108 unresolved authored occurrences are represented unresolved and are not structural failures. Ambiguous count is zero.
- The current corpus's zero block-fragment state validates zero-state fidelity only; it is not positive corpus evidence of block-target resolution.
- Synthetic falsification tests establish local validator mechanics, not corpus actuality.
- Structural retrieval-surface descriptors and transitions were validated; executable retrieval providers and indexes were not implemented or exercised.
- Real projection access remains Phase 7 work.
- Candidate activation remains provisional Phase 8 work.
- PR #9 was not modified.
- No live vault was accessed and the observation was not regenerated.
- No private observation or projection artifact is part of this repository change.
