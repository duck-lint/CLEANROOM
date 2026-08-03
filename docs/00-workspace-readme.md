---
uuid: 019fc54e-73d0-7cc6-93d1-8a734e7c1f30
note_version: v0.1.0
schema_version: v0.1.2
note_type: 
note_status: 
aliases: []
tags: []
layer: 
unity_level: 
vector_direction: 
register: 
register_mode: 
pillar: 
---
# Semantic Traversal Clean-Room Workspace

## Kernel statement

> The system maintains a continuously morphing, thread-local problem gestalt. One LLM inference call deconstructs each new utterance into a boundary contribution; a second LLM inference call uses the resulting problem-space lens to construct a semantic-access plan over an exhaustively addressable semantic projection. Deterministic runtime stages validate and execute that plan, and LLM synthesis interprets the returned semantic units.

This workspace begins from that kernel.

## Source hierarchy

The clean-room project should use this order of authority:

1. The Organon and the corpus-structuring rules it establishes.
2. The kernel equations and invariants in this package.
3. The semantic object/unit model.
4. The semantic-space projection requirements.
5. Explicit behavioral examples and acceptance tests.
6. Existing implementation artifacts, consulted only after the clean contracts are frozen and only through a bounded compatibility review.

When two sources conflict, the higher source wins until the conflict is deliberately resolved.

## Documents

- `000-organon-of-finite-inquiry.md` — authoritative Organon source, preserving authored wikilinks and terminology.
- `00-workspace-readme.md` — this document.
- `01-kernel-equations.md` — formal representation of boundary inference, problem-space evolution, projection activation, semantic access, execution, and synthesis.
- `02-runtime-invariants.md` — non-negotiable runtime authority boundaries.
- `03-semantic-object-unit-model.md` — the bidirectional and lateral ontology of semantic objects, semantic units, identifiers, occurrences, links, regions, and contextual relations.
- `04-semantic-space-projection-requirements.md` — what the frozen semantic projection and its activated working views must expose.
- `05-clean-implementation-sequence.md` — greenfield implementation order for the independent `CLEANROOM` repository.
- `06-clean-room-protocol.md` — context hygiene, review gates, repository discipline, and later compatibility rules.
- `07-rust-assessment.md` — language decision note aligned with the accepted clean contracts.
- `08-behavioral-examples.md` — minimal scenarios that test the architecture.
- `09-vault-topology-and-authored-conventions.md` — evidence-grounded description of the vault's physical topology and authored conventions.
- `10-organon-vault-substrate-chunking-map.md` — mapping from the Organon through vault structure, semantic objects/units, and unit materialization.
- `11-problem-space-state.md` — two-call boundary inference, relational problem regions, open tensions, attention bands, persistence, and continuity.
- `12-semantic-access-and-traversal-language.md` — canonical addressing, directed access paths, execution obligations, outputs, conformance, and repair.
- `13-projection-activation-and-access.md` — positive-only activated regions, deterministic surface access, expansion, telemetry, and frozen turn snapshots.

## Initial workspace contents

The new project workspace should initially contain only:

- these documents;
- the Organon;
- a corpus-schema example or tiny synthetic fixture;
- later, accepted language-neutral data contracts and their clean implementation.

The legacy runtime remains outside this repository and is not ambient design context.

## Immediate next decision

Before code begins, freeze:

- the equations;
- the invariants;
- the problem-space contract;
- the object/unit ontology;
- the projection and activation contracts;
- the semantic-access plan contract;
- the retrieval-result and synthesis-input contracts;
- the semantic-unit versus transport-segment rule.

Only then choose concrete module boundaries and implementation language.
