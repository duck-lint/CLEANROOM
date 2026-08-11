# Rust Assessment

## Recommendation

Rust remains a strong candidate for the clean deterministic runtime and the best default choice for reconstruction. The recommendation is conditional: corpus-sensitive contracts must be grounded in the complete authored substrate before Rust makes them expensive to revise.

Rust can make distinctions such as `ProblemSpaceState`, `SemanticSpaceProjection`, `SemanticObject`, `SemanticRegion`, `SemanticUnit`, `TransportSegment`, `SemanticAccessPlan`, `ConformanceResult`, `RetrievalResult`, and `SynthesisInput` difficult to blur. It cannot determine the correct ontology. A successful compile, schema round trip, or synthetic test proves mechanical consistency, not fit with the complete vault.

## Domain-scoped authority

```text
Constitutive system meaning
    Organon
    explicit operator decisions

Runtime semantic authority
    accepted kernel equations
    accepted runtime invariants

Corpus actuality
    raw authored-vault structure and content
    including irregularities, absences, ambiguity,
    malformed structures, and unresolved authoring states

Representational authority
    corpus-validated semantic contracts

Implementation authority
    accepted implementation contracts
    within their explicitly declared responsibility
```

The Organon constrains system meaning; explicit operator decisions resolve constitutive questions; the vault supplies actual instances and irregularities; equations and invariants constrain deterministic authority; corpus-validated contracts define representation; and implementations are authoritative only inside accepted boundaries. No domain silently overrides another. Conflicts require classification, retained evidence, an explicit decision, and a bounded amendment.

## Operator-attested recovery baseline

The operator attests that complete-vault ingest occurred, real corpus questions were answered, exact/lexical/vector/graph/temporal mechanisms operated, multi-turn private UAT exercised the system, and useful whole-corpus behavior resulted. This is an `operator-attested recovery baseline`, not an unqualified architectural fact.

It does not establish Python ontology correctness, exhaustive projected affordances, preservation of CLEANROOM equations or invariants, evidence-admission correctness, absence of accidental semantic/coherence gates, correctness of candidate Rust contracts, correctness of Python boundaries, or corpus-scale suitability of candidate Rust activation.

The recovery-baseline manifest records, where available:

```text
python_repository_commit
corpus_snapshot_identity
inventory_identity
materialization_or_index_identity
private_uat_suite_identities
report_identities
demonstrated_capabilities
known_failures
known_architectural_violations
unmeasured_areas
private_artifact_hashes_or_locations
attestation_timestamp
```

`materialization_or_index_identity` is a historical recovery-manifest field
where available. It does not establish a future CLEANROOM runtime identity;
unknown values remain `unknown`, and no provider/index identity is inferred or
fabricated before Phase 7 supplies concrete implementation evidence.

Unknown or unavailable identities are `unknown`, never inferred or fabricated.

## What Rust helps with

Rust is suitable for typed state shapes, explicit failure boundaries, serialization contracts, canonical identity and hashing, replayable folds, projection validation, graph and temporal traversal, deterministic deduplication, bounded packet assembly, SQLite access, and provider calls.

It is not evidence that the represented distinctions are correct. A type such as `EligibleRelationProposition` remains unjustified if the kernel and corpus supply no authority for it.

## Dynamic projection versus static types

Rust types should define record shape and authority boundaries for problem-space records, objects, regions, units, identifiers, relations, occurrences, retrieval surfaces, access plans, and packets. They should not hard-code actual corpus objects, headings, links, dates, Organon identifier values, or Python-specific intermediate categories.

```text
Rust type system
    record shape, lifecycle, authority boundaries

SemanticSpaceProjection
    corpus-specific schema and canonical instances

whole-corpus observation exchange
    versioned evidence from authored and materialized facts
```

A `SemanticUnitId` must remain distinct from `TransportSegmentId`. A provider limit may divide one authored unit into ordered transport segments without creating additional canonical semantic units. The authored boundary must be checked against the complete vault, not inferred from a Python `chunk_id`.

## Pre-admission observation boundary

The first corpus-facing implementation is an observation exchange, not semantic admission. As available it exposes source path, source identity and UUID observation, raw frontmatter keys and value shapes, provenance, heading hierarchy, Markdown block candidates and raw Markdown, wikilinks, aliases, embeds, heading/block targets, source spans, parse failures, unsupported syntax, ambiguous/unresolved targets, source hashes, and corpus snapshot identity.

These observations are not yet semantic objects, units, identifiers, or occurrences. Classification as `admitted`, `quarantined`, `rejected`, `unresolved`, or `open_authoring_decision` retains the raw observation, rule, authority, information preserved, and information lost or excluded.

Python chunks, graph nodes, anchors, and indexes are materialization observations. A Python chunk may correspond to an authored unit, transport split, defect, or unresolved record. Rust must not promote it by naming it differently.

## Projection and access boundaries

Projection construction is a versioned read-only mapping from the observation bundle to a validated `SemanticSpaceProjection`. It must retain actual corpus irregularities and explicit unresolved states rather than invent mappings.

After complete-vault projection validation, build the real read-only projection-access boundary:

```text
validated SemanticSpaceProjection
+ actual corpus-derived surface indexes or providers
+ typed activation probe
→ mechanically computed ProjectionActivationProbeResult
```

Phase 7 is not complete until real read-only access exists for the exact,
lexical, vector, graph, and temporal surface families over the validated
projection. Each surface retains its own structurally applicable record kinds
and declared match modes. A corpus may legitimately yield zero results for a
surface or operation. Results are computed from actual corpus-derived surfaces
and indexes. They are not hand-authored expected candidates.

The concrete implementation is not selected by this documentation correction. Future choices may include Rust read-only indexes, a bounded Python process or protocol, deterministic exported query indexes, or another explicit versioned read-only boundary.

Projection access performs probes needed by initial activation. Later semantic-access execution adapters execute conforming traversal plans and hydrate outputs. They are separate responsibilities.

The real `ProjectionActivationAccess` implementation is required before candidate activation revalidation.

## Current project posture

The independent Python system provides an operator-attested recovery baseline, not an unqualified architectural fact. CLEANROOM already contains contracts, schemas, deterministic problem-space folding and replay, a synthetic projection, activation contracts, and candidate activation work. Preserve these artifacts. PR #9 remains provisional; synthetic success does not validate it against the real projection.

Python remains admissible evidence and a possible bounded substrate/access provider. It does not automatically become architectural authority. No Python runtime port is authorized. No compatibility mechanism is introduced without a concrete consumer. The end state remains one production kernel authority.

## Provider and repository boundaries

Keep model providers behind narrow interfaces such as `BoundaryInferenceProvider`, `SemanticAccessInferenceProvider`, and `SynthesisProvider`. Provider-specific formats must not leak into kernel types.

`CLEANROOM` remains independent. A Python exporter may produce the observation bundle because Python owns demonstrated ingest and local corpus access. The exported artifact, provenance, and adapter contract—not Python orchestration—form the boundary. The Obsidian plugin remains an external versioned protocol boundary.

## Decision and continuation rule

Continue with Rust when authority boundaries remain accepted, corpus-sensitive contracts have survived whole-corpus contact, the real projection can be built without invention, and explicit type distinctions justify slower iteration. Use Python for demonstrated behavioral evidence, local substrate observation, bounded export, evaluation, and migration analysis.

The corrected sequence is:

```text
whole-corpus observation
→ contract-contact reconciliation
→ bounded contract amendment
→ real projection construction
→ complete-vault projection validation
→ real projection-access implementation
→ candidate activation revalidation
→ semantic-access inference
→ conformance
→ execution
→ packet and synthesis
→ private UAT
```

Do not restart CLEANROOM, port the old Python runtime, or treat the synthetic projection as sufficient architectural acceptance.
