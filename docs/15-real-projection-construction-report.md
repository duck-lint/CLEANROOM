# Phase 5 Real Projection Construction Report

Status: construction closure passed; projection status remains `Unvalidated`.
Phase 6 has not begun.

This report records the corrected Phase 5 projection representation after the
merged PR #22 authority. The reconciliation removes runtime configuration,
provider availability, and routine candidate policy from the frozen semantic
projection. It does not change admitted corpus semantics.

## Accepted input

- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `e9bb2d95c14b1beb334dc2b8d83420f5998b9a53`
- Observer schema: `vault-observation/v3`
- Specimen / corpus snapshot: `f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Pinned private input artifact SHA-256: `d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9`

The constructor hashes the exact input bytes before parsing, then requires the
v3 schema, observer commit, and specimen identity above. Both runs consumed
that exact retained artifact. The authored vault and observer repository were
not accessed or mutated, and no live-vault hydration occurred.

## Corrected projection identity

- Schema version: `semantic-space-projection/v2`
- Projection snapshot: `projection:phase5:v2:f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Validation status: `Unvalidated`
- Configuration snapshot identity: not part of `SemanticSpaceProjection`

The version-qualified snapshot handle distinguishes this corrected immutable
representation from the superseded Phase 5 artifact. It depends only on the
projection representation and corpus snapshot, not runtime configuration,
provider/index availability, time, randomness, or local paths.

## Corpus and projection census

| Measure | Count |
|---|---:|
| Resident source records | 1060 |
| Resident Markdown | 1060 |
| Admission eligible | 1052 |
| Excluded Markdown | 8 |
| Objects | 1052 |
| Regions | 4356 |
| Semantic units | 17118 |
| Object classes | 14 |
| Identifier descriptors | 55 |
| Identifier assignments | 12351 |
| Authored occurrences | 5003 |
| Object-field occurrences | 501 |
| Semantic-region occurrences | 184 |
| Semantic-unit occurrences | 4318 |
| Resolved occurrences | 4895 |
| Unresolved occurrences | 108 |
| Ambiguous occurrences | 0 |
| Temporal anchors | 548 |
| FullDate anchors | 532 |
| DateTime anchors | 1 |
| ExactYear anchors | 7 |
| MonthDay anchors | 3 |
| ApproximateYear anchors | 5 |
| Present-null temporal assignments | 64 |
| Retrieval surface families | 5 |
| Valid structural transitions | 22 |

These values are unchanged from the accepted Phase 5 census. No corpus
semantic decision was reopened.

## Surface representation

The projection contains exactly five unique structural surface families:

`exact`, `lexical`, `vector`, `graph`, and `temporal`.

The descriptors contain no provider-availability boolean, routine candidate
limits, runtime configuration identity, or provider/index status text. Their
record-level surface vectors remain typed structural applicability facts. The
surface descriptors retain structural coverage semantics, with the corrected
Phase 5 construction using `Bounded` declarations and no executable provider
claim. Vector does not gain identifier visibility; graph returns authored
occurrence/incidence identity; temporal remains limited to licensed temporal
subjects and anchors.

Provider/index executability remains a later access-boundary fact. Provider
absence does not remove any of the five structural families from `M_sigma`.
Runtime configuration identity, activation bounds, candidate limits,
continuation limits, access failures, and provider failures remain later
runtime concepts and were not removed from their owning contracts.

## Construction closure

- Accepted field universe: 60 observed, 55 admitted, 5 excluded
- Assignment count: 12351
- Inherited assignment references: 227573
- Region inherited-assignment references: 53039
- Block-fragment occurrences: 0
- Resolved block targets: 0
- Unresolved block targets: 0
- All constructor closure and authority-contact failure counters: 0

The constructor preserves unresolved authored occurrences and does not apply
first-candidate or parent-object fallback. Semantic-unit hydration hashes are
SHA-256 over the exact authored block bytes addressed by the hydration span.

## Determinism and evidence

The exact pinned observation was constructed twice into separate private output
files:

- Run 1 logical hash: `sha256:f931ef244e85b206d5c5d3b487b698c8373f9463776b09a09da00d6622b3b73f`
- Run 2 logical hash: `sha256:f931ef244e85b206d5c5d3b487b698c8373f9463776b09a09da00d6622b3b73f`
- Run 1 projection bytes SHA-256: `f7423f494d905799e44b2c98f470429ae960ae682b43158f90d8b8f6fa9e39d2`
- Run 2 projection bytes SHA-256: `f7423f494d905799e44b2c98f470429ae960ae682b43158f90d8b8f6fa9e39d2`
- Logical hashes identical: yes
- Serialized projection bytes identical: yes
- Private artifacts committed: no

## Superseded historical evidence

The prior representation was incompatible and is retained here only for audit
comparison:

- Old schema version: `semantic-space-projection/v1`
- Old logical hash: `sha256:b21a96fdf2951dd8777c72a3acff3ac0ffa16ffb04d697aa6e7ee6855993c7a8`
- Old projection bytes SHA-256: `4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a`
- Old projection snapshot: `projection:phase5:f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`

Those values are superseded and are not current Phase 5 identities.

The corrected projection remains `Unvalidated`; constructor self-consistency
is not Phase 6 validation. The next step after this reconciliation is to
return to PR #21 and reconcile its independent validator/evidence against the
corrected Phase 5 projection. Phase 6 and Phase 7 have not begun.
