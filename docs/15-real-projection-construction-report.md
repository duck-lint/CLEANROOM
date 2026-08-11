# Phase 5 Real Projection Construction Report

Status: construction closure passed; projection status remains `Unvalidated`.
Phase 6 has not begun.

## Accepted input

- Observer repository: `duck-lint/semantic-traversal`
- Observer commit: `e9bb2d95c14b1beb334dc2b8d83420f5998b9a53`
- Observer schema: `vault-observation/v3`
- Specimen / corpus snapshot: `f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4`
- Pinned private input artifact SHA-256: `d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9`

The constructor hashes the exact input bytes before parsing, then requires the
v3 schema, observer commit, and specimen identity above. Any mismatch fails
closed before projection output is written.

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

The observed field universe is checked by exact set equality against the
accepted 60-field registry in docs/14: 55 admitted and 5 excluded. Missing
fields: 0. Unexpected fields: 0. The accepted class applicability source is
docs/10 and docs/14, represented explicitly by the constructor's class table;
`dream_motif` does not inherit journal-entry-only fields. Generated class
descriptors match that table exactly.

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
- Block-fragment occurrences: 0
- Resolved block targets: 0
- Unresolved block targets: 0
- Parent-object fallback degradations: 0
- Class-applicability failures: 0
- Authority class-applicability failures: 0
- Authority occurrence-capability failures: 0
- Final resolution-state/target mismatches: 0
- Retrieval-affordance failures: 0
- All constructor closure failure counters: 0

Ordinary authored-target resolution retains unresolved and ambiguous states;
there is no first-candidate fallback. Graph direct identity is an occurrence
or incidence record, not an automatically hydrated semantic unit.

Final occurrence resolution is coherent: `Resolved` occurs only with a final
target, while `Unresolved` and `Ambiguous` have no final target. Occurrence
capability is sourced from accepted occurrence semantics (`book_read_today`
and `dream_motif`); `relationship` remains `ProfileRelation` metadata without
an occurrence transition.

Semantic-unit hydration hashes are SHA-256 over the exact authored block bytes
addressed by the hydration span. The source-file hash is not reused as a
unit-content hash.

## Capability topology

Phase 5 creates no executable provider or index, but that implementation fact
does not make the five represented surface families unavailable in the
projection. Their structural visibility is nevertheless typed: vector has no
Identifier visibility, graph returns occurrence/incidence identity, and
temporal exposes only licensed temporal subjects/anchors. Provider executability
remains a later access-boundary fact.
Descriptor surface references, transition references, surface IDs, and
from/to visibility are closed.

## Determinism and evidence

- Run 1 logical hash: `sha256:b21a96fdf2951dd8777c72a3acff3ac0ffa16ffb04d697aa6e7ee6855993c7a8`
- Run 2 logical hash: `sha256:b21a96fdf2951dd8777c72a3acff3ac0ffa16ffb04d697aa6e7ee6855993c7a8`
- Run 1 projection bytes SHA-256: `4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a`
- Run 2 projection bytes SHA-256: `4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a`
- Logical hashes identical: yes
- Private serialized projection bytes identical: yes
- Private artifacts committed: no

The projection remains `Unvalidated`; constructor self-consistency is not Phase
6 validation. Historical v2 evidence is superseded pre-repair context only and
is not current construction input.
