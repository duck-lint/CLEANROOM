# Phase 6 Complete-Vault Projection Validation Report

Status: **current Phase 6 validation passed twice; promoted outputs and
validation reports are byte-identical.**

This is the current private-evidence baseline. It records complete-vault
correspondence validation only; it does not begin Phase 7.

## Current evidence tuple

- CLEANROOM validator source commit: `005f75f`
- Observation schema: `vault-observation/v3`
- Observer version: `cleanroom-parser-observer/v2`
- Observer repository: `CLEANROOM`
- Observer commit: `32957a3ff467ee57d7a76d4c4321753ac018d054`
- Parser salvage commit: `72ef99219fd260ba71365005273f6d9f68cab939`
- Corpus snapshot identity: `8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`
- Observation artifact SHA-256: `cc179d31f4035f84742312bab363a1504173f034e11625022a2142294c6eb000`
- Phase 5 schema: `semantic-space-projection/v2`
- Phase 5 snapshot: `projection:phase5:v2:8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`
- Phase 5 projection SHA-256: `925e9437696a4728df4db2be26a0c8aeafbb2304421de007998731bd579b1cea`
- Phase 5 report SHA-256: `a1d4ff78789cdb6f7ad99350760bd003cc478e33bf3112d88691f97d1ec2838c`
- Phase 5 logical hash: `sha256:91352761f4403343de3c293ef78bc9c087011cbdd39f957d789ae7200ad720f0`

The observation, frozen Phase 5 projection, Phase 5 report, and both Phase 6
outputs/reports remain in a durable private location outside Git and outside
the authored vault. No private artifact or corpus content is committed.

## Snapshot delta audit

The prior candidate snapshot was
`8bc26fff58572bc2991f08b216b9974cc1e7c5a45fc9df19e5a7e2703f357293`.
The current and prior observations contain the same 109 directories and 1129
files: no path was added or removed. Exactly seven existing Markdown source
records changed their source hashes. The snapshot function uses resident paths,
source hashes, and topology; it does not include observer implementation
identity. The change is therefore a corpus-state/source-hash change only, not
an observer-induced snapshot identity change.

## Independent validation runs

Both runs consumed the same observation and frozen Phase 5 projection. The
validator independently derived correspondence and did not call the Phase 5
constructor or use its closure result as an oracle.

| Measure | Run 1 | Run 2 |
|---|---|---|
| Validator status | `Validated` | `Validated` |
| Violations | 0 | 0 |
| Failure counts | empty | empty |
| Promoted logical hash | `sha256:ce157f412d51e4ec6c9d34c54e6708862836d9bb5f8b58d6ff212335cb6050ec` | same |
| Promoted projection SHA-256 | `0f60da7b48638f5f494e2691f3be64aaad38b4808bdd53b1a77bad29c000d68a` | same |
| Validation report SHA-256 | `78df04d63c33e333a2dbb9c6db57c9075aeac47282e549848d6078de167ed794` | same |
| Promoted output size | 52,948,513 bytes | 52,948,513 bytes |

The promoted snapshot is
`projection:phase6:8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`.

## Validated census

| Measure | Count |
|---|---:|
| Resident Markdown sources | 1085 |
| Admitted sources / objects | 1077 |
| Excluded Markdown | 8 |
| Regions | 4492 |
| Semantic units | 14108 |
| Identifier descriptors | 55 |
| Identifier assignments | 12627 |
| Occurrences | 4920 |
| Resolved occurrences | 4907 |
| Unresolved occurrences | 13 |
| Ambiguous occurrences | 0 |
| Temporal anchors | 549 |
| Retrieval surfaces | 5 |
| Structural transitions | 22 |
| Transport segments | 0 |

## Contract-contact accounting

- Observed frontmatter fields: 60; admitted: 55; excluded: 5.
- `analysis_orientation` and `headspace` are present; `vector_direction` and
  `temporal_pace` are absent.
- Observed block kinds/counts: paragraph 12727, heading 3482, list 1158,
  `hr` 350, blockquote/callout 237, table 20, code fence 41.
- Admitted block kinds/counts: paragraph 12705, heading 3415, list 1115,
  `hr` 323, blockquote/callout 231, table 19, code fence 38.
- `hr` is observed and non-materialized; all 323 admitted `hr` blocks consume
  authored ordinal positions and are absent from semantic-unit materialization.
- Occurrence sources: object field 501, semantic region 189, semantic unit
  4230. Body ownership was independently checked from exact parser-owned
  `source_block_span`; source occurrence ordinals were checked for deterministic
  collision-free identity. Exact inline spans were checked when present and
  unavailable spans remained unavailable.
- Heading-fragment occurrences: 10; resolved 9; unresolved 1; ambiguous 0.
- Block-fragment occurrences: 0; resolved 0; unresolved 0.
- Zero-candidate links remain explicit unresolved occurrences; no target or
  block-target fallback degradation was observed.

## Closure results

The independent validator closed object, region, unit, identifier, occurrence,
target, reverse-incidence, temporal, retrieval-surface, provenance, transport,
bounds, deterministic-identity, admitted-fact, and unresolved/ambiguous
representation domains. Every failure counter is zero and the violation list
is empty. In particular:

- projection-closure violations: 0;
- occurrence source-attribution failures: 0;
- occurrence identity collisions: 0;
- target-incidence failures: 0;
- block-target fallback degradations: 0;
- transport-segmentation failures: 0;
- bounds failures: 0;
- deterministic-identity failures: 0;
- admitted-fact representation failures: 0;
- unresolved/ambiguous preservation failures: 0.

Historical pre-archive Phase 5/6 identities remain audit context only. They
were not reused as current evidence. Phase 7 remains out of scope.
