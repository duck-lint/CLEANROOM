# Rust Assessment

## Recommendation

Rust remains a strong candidate for the clean deterministic runtime and the best default choice for the reconstruction.

The recommendation is conditional:

> Corpus-sensitive contracts should be grounded in the complete authored substrate before Rust makes them expensive to revise.

The reason to use Rust is not raw speed.

The reason is that Rust can make the distinctions in the kernel difficult to blur accidentally:

```text
ProblemSpaceState
ProblemRegion
ProblemRelation
OpenTension
AttentionLens
BoundaryContribution
SemanticSpaceProjection
ActivatedProjection
SemanticObject
SemanticRegion
SemanticUnit
TransportSegment
SemanticAccessPlan
ConformanceResult
RetrievalResult
ExecutionLimits
SynthesisInput
```

These can be distinct types rather than loosely related dictionaries passed through one large orchestration path.

## Current project posture

The project is not choosing between an unproven Python idea and a Rust prototype.

The Python runtime has already demonstrated whole-corpus feasibility.

CLEANROOM already contains substantial Rust work:

- exchange contracts and schemas;
- strong semantic identities;
- deterministic problem-space folding and replay;
- a synthetic semantic projection;
- projection-activation contracts;
- a candidate deterministic activation implementation.

That work should be preserved.

Its corpus-sensitive abstractions remain provisional until they survive complete-vault projection and private-UAT contact.

## What Rust helps with

### Explicit state shapes

Structs and enums can represent:

- problem-space perturbation operations;
- attention-band transitions;
- canonical object, region, and unit addresses;
- occurrence direction;
- retrieval requests;
- required and optional execution obligations;
- structural violations;
- execution outcomes;
- synthesis-packet contents.

Pattern matching encourages every variant to be handled explicitly.

### Failure boundaries

Recoverable failures can be represented as typed results:

```text
Result<ProblemSpaceState, BoundaryFoldViolation>
Result<SemanticSpaceProjection, ProjectionConstructionViolations>
Result<ConformingAccessPlan, ConformanceViolations>
Result<RetrievalResult, ExecutionFailure>
```

This supports the distinction between:

- invalid boundary operation;
- unmappable or invalid substrate structure;
- invalid semantic-access plan;
- unavailable retrieval surface;
- successful zero matches;
- partial execution;
- provider failure.

### Serialization contracts

Serde can serialize and deserialize strongly typed runtime objects.

A JSON Schema generator can expose contracts for:

- boundary-inference output;
- persisted problem-space state;
- substrate observation and projection exchange;
- semantic-access output;
- structural violations;
- diagnostics;
- synthesis packets.

### Deterministic core

Rust is well suited to:

- canonical identity and hashing;
- replayable problem-space folds;
- projection construction and validation;
- graph and occurrence traversal;
- temporal ordering;
- deterministic deduplication;
- bounded packet assembly;
- SQLite access;
- provider HTTP calls.

## What Rust does not solve

Rust cannot determine the correct ontology.

It can make a wrong ontology beautifully explicit and difficult to remove.

A type such as:

```text
EligibleRelationProposition
```

would still be wrong if the kernel and corpus supply no authority for it.

Rust therefore increases the cost of conceptual mistakes as much as it increases structural safety.

A successful compile, schema round trip, or synthetic test proves mechanical consistency. It does not prove that the represented distinctions fit the complete vault.

## Dynamic projection versus static types

The corpus's actual semantic possibilities remain data.

Rust types should define the shape and authority of:

- problem-space records;
- semantic objects, regions, and units;
- identifiers;
- relations and occurrences;
- retrieval surfaces;
- semantic-access plans;
- retrieval and synthesis packets.

They should not hard-code actual corpus objects, headings, links, dates, Organon identifier values, or Python-specific intermediate categories.

```text
Rust type system
    defines record shape, lifecycle, and authority boundaries

SemanticSpaceProjection
    contains corpus-specific schema and canonical instances

whole-corpus observation exchange
    supplies versioned evidence from authored and materialized substrate facts
```

## Semantic units versus transport segments

Rust should encode the distinction directly:

```text
SemanticUnitId
TransportSegmentId
```

A provider or embedding limit may divide one unit into transport segments without creating additional canonical semantic units.

The type system should make accidental promotion of a transport segment into an independently authored semantic unit difficult.

The actual authored unit boundary must be checked against the complete vault rather than inferred from the current Python `chunk_id` shape.

## Pydantic analogy

The rough correspondence is:

```text
Pydantic model
    ↔ Rust structs/enums + Serde + JSON Schema

model instance validation
    ↔ deserialization plus explicit validation

dynamic corpus schema and instances
    ↔ SemanticSpaceProjection data
```

Structural conformance is more than JSON shape validation. A plan may be valid JSON while referencing an absent identifier, object, region, unit, relation, target, direction, or surface.

Projection construction is also more than deserialization. It must prove that the exchanged authored and materialized facts can be mapped into the accepted clean-room distinctions without invention.

## Suggested package shape

Begin with one Rust package and strict modules:

```text
model
problem_space
projection
projection_adapter
projection_validation
activation
boundary_inference
semantic_access
conformance
execution
packet
synthesis
runtime
cli
```

Names remain implementation decisions.

Split modules into crates only after their boundaries survive complete-vault and private-UAT contact.

## Provider boundaries

The three primary model calls should sit behind narrow interfaces:

```text
BoundaryInferenceProvider
SemanticAccessInferenceProvider
SynthesisProvider
```

The bounded repair call may reuse the semantic-access provider interface with a distinct request type.

Provider-specific request formats must not leak into kernel types.

## Corpus boundary

CLEANROOM remains an independent repository.

The real corpus enters through a versioned, read-only substrate observation or projection exchange contract.

The exchange should preserve the distinction among:

```text
authored vault fact
Python materialization observation
runtime behavior observation
clean-room interpretation
```

Legacy runtime orchestration must not become the route by which the Rust kernel accesses corpus facts.

A Python exporter may produce the observation bundle because Python currently owns the demonstrated ingest and local corpus access. The exported artifact, its provenance, and the adapter contract—not Python orchestration—form the boundary.

## Obsidian boundary

The Obsidian plugin remains TypeScript and communicates through a versioned protocol.

Authored wikilinks remain Obsidian-facing syntax. Canonical UUID and region/unit resolution remain substrate and projection responsibilities.

The complete vault, not the plugin's convenience representation, remains higher authority.

## Python's continuing role

Python is not merely a disposable prototype.

It currently provides:

- demonstrated whole-corpus runtime behavior;
- local authored-vault access and materialization;
- corpus inventory and inspection;
- private-UAT infrastructure;
- observational evidence about capabilities and failures;
- a possible read-only substrate-observation exporter.

Python may continue to support:

- offline evaluation;
- corpus inspection;
- migration scripts;
- notebooks;
- report generation;
- compatibility analysis.

Python must not remain a second production authority for problem-space state, semantic-access schemas, projection meaning, or evidence admission after migration.

The end state should contain one production kernel authority.

## Decision rule

Continue with Rust when:

- the clean authority boundaries remain accepted;
- the relevant corpus-sensitive contracts have survived whole-corpus contact;
- the real projection can be constructed and validated without invention;
- explicit type distinctions are more valuable than fastest possible iteration;
- slower initial iteration is acceptable in exchange for stronger structural discipline.

Use Python where it already has concrete authority or access as:

- demonstrated behavioral evidence;
- a local substrate observer;
- a bounded exporter;
- evaluation and migration tooling.

Do not port the Python runtime merely to preserve implementation ancestry.

## Current recommendation

Use this sequence:

1. preserve the existing Rust contracts, fold, fixture, and candidate activation work;
2. define a versioned whole-corpus observation exchange;
3. compare candidate contracts against the authored vault, Python materialization, and private UAT;
4. revise only distinctions contradicted or unsupported by that evidence;
5. construct and validate the complete-vault `SemanticSpaceProjection`;
6. re-evaluate the existing activation implementation against the real projection;
7. continue semantic access, conformance, execution, packet assembly, and synthesis in Rust;
8. migrate or retire Python responsibilities only through explicit validated boundaries.

Do not restart CLEANROOM.

Do not treat the synthetic projection as sufficient architectural acceptance.

Do not port the old Python runtime.
