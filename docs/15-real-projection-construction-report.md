# Phase 5 Real Projection Construction Report

Status: **current construction closure passed; corrected v2 projection
constructed twice with identical logical and serialized bytes.**

This is the current post-archive Phase 5 evidence baseline. Private observation
and projection artifacts remain outside the repository. Phase 6 promotion is
not part of this baseline.

## Current baseline identity

- Observer repository: `CLEANROOM`
- Observer commit: `32957a3ff467ee57d7a76d4c4321753ac018d054`
- Parser salvage commit: `duck-lint/semantic-traversal@72ef99219fd260ba71365005273f6d9f68cab939`
- Observer schema: `vault-observation/v3`
- Corpus snapshot identity: `8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`
- Private observation artifact SHA-256: `cc179d31f4035f84742312bab363a1504173f034e11625022a2142294c6eb000`

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
| Regions | 4492 |
| Semantic units | 14108 |
| Object classes | 14 |
| Identifier descriptors | 55 |
| Identifier assignments | 12627 |
| Authored occurrences | 4920 |
| Object-field occurrences | 501 |
| Semantic-region occurrences | 189 |
| Semantic-unit occurrences | 4230 |
| Resolved occurrences | 4907 |
| Unresolved occurrences | 13 |
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

Observed authored block kinds across the whole resident Markdown corpus:
`paragraph=12727`, `heading=3482`, `list=1158`, `hr=350`,
`blockquote_or_callout=237`, `table=20`, `code_fence=41`. The admitted
non-heading distribution was `paragraph=12705`, `list=1115`, `hr=323`,
`blockquote_or_callout=231`, `table=19`, and `code_fence=38`; `hr` remained
observed and ordinal-consuming but non-materialized.

Construction closure and contract-contact failure counters were all zero.
Inherited assignment references: 197865. Region inherited-assignment
references: 54457. Fenced-code example links were not admitted as authored
occurrences. Body source blocks produced 4,230 unit attributions and 189
heading-marker region attributions. The observation recorded 350 `hr` blocks;
323 occurred in admitted Markdown and were preserved as non-materialized
ordinal-consuming structure. Zero occurrence source-attribution failures,
identity collisions, target-incidence failures, and block-target fallback
degradations were observed.

## Determinism

The corrected projection is `semantic-space-projection/v2` with snapshot:

`projection:phase5:v2:8db3ab329d1890890a1fa7eeaec42d700b29420fb71ed052383dbb9ed3b0e8cc`

- Run 1 logical hash: `sha256:91352761f4403343de3c293ef78bc9c087011cbdd39f957d789ae7200ad720f0`
- Run 2 logical hash: `sha256:91352761f4403343de3c293ef78bc9c087011cbdd39f957d789ae7200ad720f0`
- Run 1 projection bytes SHA-256: `925e9437696a4728df4db2be26a0c8aeafbb2304421de007998731bd579b1cea`
- Run 2 projection bytes SHA-256: `925e9437696a4728df4db2be26a0c8aeafbb2304421de007998731bd579b1cea`
- Run 1 report bytes SHA-256: `a1d4ff78789cdb6f7ad99350760bd003cc478e33bf3112d88691f97d1ec2838c`
- Run 2 report bytes SHA-256: `a1d4ff78789cdb6f7ad99350760bd003cc478e33bf3112d88691f97d1ec2838c`
- Projection ingest identity: `observation:vault-observation/v3:cc179d31f4035f84742312bab363a1504173f034e11625022a2142294c6eb000`
- Logical identity: yes
- Byte identity: yes
- Private artifacts committed: no

## Historical boundary

Pre-archive Phase 5 identities and hashes remain audit context only. They are
not current baseline inputs, outputs, or claims about this run. Phase 6 and
Phase 7 have not begun.
