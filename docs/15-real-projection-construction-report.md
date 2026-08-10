# Phase 5 Real Projection Construction Report

Status: construction closure passed; projection status remains `Unvalidated`.
Phase 6 has not begun.

## Accepted input

- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `e9bb2d95c14b1beb334dc2b8d83420f5998b9a53`
- Observer schema: `vault-observation/v3`
- Specimen / corpus snapshot: `f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Pinned private input artifact SHA-256: `d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9`

The private observation artifact was consumed directly. The authored vault and
observer repository were not mutated, and no live-vault hydration occurred.

## Corpus and projection

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
| Retrieval surfaces | 5 |
| Valid structural transitions | 22 |

Canonical class applicability is sourced from the accepted field registry and
validated against observed assignments. It is not learned from class-member
co-occurrence. Canonical list cardinalities remain collections even for one
authored value.

## Temporal materialization

- FullDate: 532
- DateTime: 1
- ExactYear: 7
- MonthDay: 3
- ApproximateYear: 5
- Total material non-null temporal anchors: 548
- Present-null temporal assignments without anchors: 64
- Present-null assignments incorrectly anchored: 0

Parser-native date/datetime evidence is interpreted only under the accepted
field-specific contract. Arbitrary strings do not become temporal labels.

## Occurrences and closure

- Authored occurrences: 5003
- Object-field occurrences: 501
- Semantic-region occurrences: 184
- Semantic-unit occurrences: 4318
- Resolved: 4895
- Unresolved: 108
- Ambiguous: 0
- Ordinary source-attribution failures: 0
- Target-incidence failures: 0
- Block-target fallback degradations: 0
- Class-applicability failures: 0
- Retrieval-affordance failures: 0
- All constructor closure failure counters: 0

Ordinary authored-target resolution retains unresolved and ambiguous states;
there is no first-candidate fallback. Graph direct identity is an occurrence
or incidence record, not an automatically hydrated semantic unit.

Semantic-unit hydration hashes are SHA-256 over the exact authored block bytes
addressed by the hydration span. The source-file hash is not reused as a
unit-content hash.

## Capability topology

The five represented surfaces remain unavailable because Phase 5 creates no
provider or executable index. Their structural visibility is nevertheless
typed: vector has no Identifier visibility, graph returns occurrence/incidence
identity, and temporal exposes only licensed temporal subjects/anchors.
Descriptor surface references, transition references, surface IDs, and
from/to visibility are closed.

## Determinism and evidence

- Run 1 logical hash: `sha256:aeac1135a7694cb7929f9dac9ced887e2399d3597a53471725274b5f650ccb0e`
- Run 2 logical hash: `sha256:aeac1135a7694cb7929f9dac9ced887e2399d3597a53471725274b5f650ccb0e`
- Run 1 projection bytes SHA-256: `e29256bcaa52bc819f2c194326dfd0418664c74e712e401180031da1f5e035dd`
- Run 2 projection bytes SHA-256: `e29256bcaa52bc819f2c194326dfd0418664c74e712e401180031da1f5e035dd`
- Logical hashes identical: yes
- Private serialized projection bytes identical: yes
- Private artifacts committed: no

The projection remains `Unvalidated`; constructor self-consistency is not Phase
6 validation. Historical v2 evidence is superseded pre-repair context only and
is not current construction input.
