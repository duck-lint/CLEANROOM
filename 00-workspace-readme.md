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

> The system is a continuously morphing problem space projected onto an exhaustively addressable semantic space, with LLM inference constructing the traversal and LLM synthesis interpreting what the traversal returns.

This workspace begins from that kernel rather than from the existing runtime.

The current implementation, its class names, historical patches, diagnostics, and internal abstractions are not design authorities here. Existing code may later be evaluated as reusable material, but nothing is presumed worth preserving until it conforms to the clean model.

## Source hierarchy

The clean-room project should use this order of authority:

1. The Organon and the corpus-structuring rules it establishes.
2. The kernel equations and invariants in this package.
3. The semantic object/unit model.
4. The semantic-space projection requirements.
5. Explicit behavioral examples and acceptance tests.
6. Existing implementation code, consulted only after the clean contracts are frozen.

When two sources conflict, the higher source wins until the conflict is deliberately resolved.

## Documents

- `01-kernel-equations.md` — formal representation of boundary inference, problem-space evolution, projection activation, semantic access, execution, and synthesis.
- `02-runtime-invariants.md` — non-negotiable boundaries that prevent semantic roles from blurring.
- `03-semantic-object-unit-model.md` — the bidirectional and lateral ontology of semantic objects, semantic units, identifiers, occurrences, links, and contextual relations.
- `04-semantic-space-projection-requirements.md` — what the projected semantic space must expose.
- `05-clean-implementation-sequence.md` — greenfield implementation order.
- `06-clean-room-protocol.md` — context-hygiene, branch, review, and migration rules.
- `07-rust-assessment.md` — language decision note.
- `08-behavioral-examples.md` — minimal scenarios that test the architecture.
- `09-vault-topology-and-authored-conventions.md` — evidence-grounded description of the vault's physical topology and authored conventions.
- `10-organon-vault-substrate-chunking-map.md` — mapping from the Organon through vault structure, semantic objects/units, and chunk materialization.
- `11-problem-space-state.md` — two-call boundary inference, bounded focus tiers, semantic aggregation, lifecycle, and continuity.
- `12-semantic-access-and-traversal-language.md` — canonical addressing, directed graph plans, execution obligations, outputs, conformance, and repair.
- `13-projection-activation-and-access.md` — positive-only activated regions, deterministic surface access, expansion, telemetry, and frozen turn snapshots.

## Initial workspace contents

The new project workspace should initially contain only:

- these documents;
- the Organon;
- a corpus-schema example or tiny synthetic fixture;
- later, an accepted language-neutral data contract.

Do not initially add:

- the current runtime source;
- Implementation 08 reports;
- old test names;
- old diagnostics;
- summaries that assume existing component boundaries;
- proposals to salvage particular modules.

## Branch strategy

A branch created from the current repository head and then emptied by a deletion commit preserves the full repository ancestry while establishing a clean working tree.

Recommended sequence:

1. Tag or otherwise preserve the currently accepted runtime head.
2. Create a dedicated clean-room branch from that exact commit.
3. Delete the tracked implementation files on the new branch.
4. Commit the empty-root transition explicitly.
5. Add only the clean-room documents and new implementation.
6. Do not merge ongoing changes from the legacy runtime into the clean branch.
7. When the replacement is accepted, merge the clean branch as an intentional repository-wide replacement.

The old implementation remains recoverable through history and its preserved branch or tag. It should not remain present as ambient design context in the clean branch.

## Immediate next decision

Before code begins, freeze:

- the equations;
- the invariants;
- the object/unit ontology;
- the projection contract;
- the traversal contract;
- the synthesis-input contract.

Only then choose concrete module boundaries and implementation language.
