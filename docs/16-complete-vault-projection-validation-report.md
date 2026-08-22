# Phase 6 Complete-Vault Projection Validation Report

Status: **current Phase 6 validation passed twice; promoted outputs are byte
identical.**

This report records the current private-evidence baseline after the explicit
field and observer corrections. It does not begin Phase 7.

## Current pinned input

- CLEANROOM validation source baseline: `9f952cf5eeb3f80132aab2a8ca8582cc8c3bde42`
- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `502bc8d83a3681a21f4ab2f2cafb9598074aa24c`
- Observation schema: `vault-observation/v3`
- Corpus snapshot identity: `eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`
- Observation artifact SHA-256: `4e3b3fd00caaf591afe92e7fa892b66da3f35a3e98fd719447f1649ab4a18849`
- Phase 5 schema: `semantic-space-projection/v2`
- Phase 5 snapshot: `projection:phase5:v2:eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`
- Phase 5 projection SHA-256: `b8fe327284263f9a3c944c62a8b853260f22c9146aef580574dc3608aa8364e3`
- Phase 5 logical hash: `sha256:f44818bb29b77ceeb097d222f81ecacae09d9e859eb57283319de0d0ce0fa2d1`

## Independent validation runs

Both runs consumed the same pinned observation and Phase 5 projection.

| Measure | Run 1 | Run 2 |
|---|---|---|
| Validator status | `Validated` | `Validated` |
| Deterministic violations | 0 | 0 |
| Failure counts | empty | empty |
| Promoted logical hash | `sha256:e27b96a58626c885475d6b7713191429cd4078ef6e3bae316210acbd0198aa65` | same |
| Promoted output SHA-256 | `b6af0c032ab51c8010d729949998b29ceb002cf7f33bd9f4b56102d88710b135` | same |

The promoted output is `semantic-space-projection/v2` with:

`projection:phase6:eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`

The two promoted output files are byte-identical at 60,971,634 bytes.

## Validated census

| Measure | Count |
|---|---:|
| Admitted sources | 1077 |
| Excluded Markdown | 8 |
| Objects | 1077 |
| Regions | 4504 |
| Semantic units | 17784 |
| Identifier descriptors | 55 |
| Identifier assignments | 12627 |
| Occurrences | 4924 |
| Resolved occurrences | 4912 |
| Unresolved occurrences | 12 |
| Ambiguous occurrences | 0 |
| Temporal anchors | 549 |
| Retrieval surfaces | 5 |
| Structural transitions | 22 |

The current registry admits `analysis_orientation` and does not recognize
the absent historical `vector_direction`. `headspace` remains the separately
authorized current successor to `temporal_pace`.

## Provenance boundary

Private observation, projection, and promoted-output artifacts remain outside
Git. Repository-safe reporting exposes only identities, hashes, counts, and
contract-level evidence. Historical pre-archive Phase 5/6 hashes remain audit
context and were not reused as current evidence.

Phase 7 remains out of scope.
