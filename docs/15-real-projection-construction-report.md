# Phase 5 Real Projection Construction Report

Status: **current construction closure passed; corrected v2 projection
constructed twice with identical logical and serialized bytes.**

This is the current post-archive Phase 5 evidence baseline. Private observation
and projection artifacts remain outside the repository. Phase 6 promotion is
reported separately in `16-complete-vault-projection-validation-report.md`.

## Current baseline identity

- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `502bc8d83a3681a21f4ab2f2cafb9598074aa24c`
- Observer schema: `vault-observation/v3`
- Corpus snapshot identity: `eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`
- Private observation artifact SHA-256: `4e3b3fd00caaf591afe92e7fa892b66da3f35a3e98fd719447f1649ab4a18849`

The observer correction excludes wikilinks inside fenced code from authored
link occurrences while retaining the raw Markdown block. The authored vault
was not modified by the observer.

## Explicit current field delta

The fresh observation contains exactly 60 fields:

- `analysis_orientation`: observed on 1,063 records; admitted as the current
  successor to historical `vector_direction` with analysis-orientation role,
  carrying-object applicability, inherited unit provenance, and no occurrence
  or temporal semantics.
- `vector_direction`: absent; retained only as historical terminology.
- `headspace`: observed on 211 records under the prior explicit decision.
- `temporal_pace`: absent; retained only as historical terminology.

The current registry therefore contains 55 admitted fields and 5 excluded
fields. No dual current field was created for the retired `vector_direction`.

## Construction census

| Measure | Count |
|---|---:|
| Resident source records | 1085 |
| Resident Markdown | 1085 |
| Admission eligible | 1077 |
| Excluded Markdown | 8 |
| Objects | 1077 |
| Regions | 4504 |
| Semantic units | 17784 |
| Object classes | 14 |
| Identifier descriptors | 55 |
| Identifier assignments | 12627 |
| Authored occurrences | 4924 |
| Object-field occurrences | 501 |
| Semantic-region occurrences | 189 |
| Semantic-unit occurrences | 4234 |
| Resolved occurrences | 4912 |
| Unresolved occurrences | 12 |
| Ambiguous occurrences | 0 |
| Temporal anchors | 549 |
| FullDate anchors | 533 |
| DateTime anchors | 1 |
| ExactYear anchors | 7 |
| MonthDay anchors | 3 |
| ApproximateYear anchors | 5 |
| Present-null temporal assignments | 69 |
| Retrieval surface families | 5 |
| Valid structural transitions | 22 |

Construction closure and contract-contact failure counters were all zero.
Inherited assignment references: 234250. Region inherited-assignment
references: 54577. Fenced-code example links were not admitted as authored
occurrences.

## Determinism

The corrected projection is `semantic-space-projection/v2` with snapshot:

`projection:phase5:v2:eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`

- Run 1 logical hash: `sha256:f44818bb29b77ceeb097d222f81ecacae09d9e859eb57283319de0d0ce0fa2d1`
- Run 2 logical hash: `sha256:f44818bb29b77ceeb097d222f81ecacae09d9e859eb57283319de0d0ce0fa2d1`
- Run 1 projection bytes SHA-256: `b8fe327284263f9a3c944c62a8b853260f22c9146aef580574dc3608aa8364e3`
- Run 2 projection bytes SHA-256: `b8fe327284263f9a3c944c62a8b853260f22c9146aef580574dc3608aa8364e3`
- Logical identity: yes
- Byte identity: yes
- Private artifacts committed: no

## Historical boundary

Pre-archive Phase 5 identities and hashes remain audit context only. They are
not current baseline inputs, outputs, or claims about this run. Phase 7 has
not begun.
