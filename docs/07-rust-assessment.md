# Rust Assessment

## Recommendation

Rust is a strong candidate for the clean deterministic runtime, and likely the best default choice if the language-neutral contracts are frozen before implementation begins.

The reason is not raw speed.

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

## What Rust would help with

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
Result<ConformingAccessPlan, ConformanceViolations>
Result<RetrievalResult, ExecutionFailure>
```

This supports the distinction between:

- invalid boundary operation;
- invalid semantic-access plan;
- unavailable retrieval surface;
- successful zero matches;
- partial execution;
- provider failure.

### Serialization contracts

Serde can serialize and deserialize strongly typed runtime objects.

A JSON Schema generator can expose LLM-facing contracts for:

- boundary-inference output;
- persisted problem-space state;
- semantic-access output;
- semantic projection exchange;
- structural violations;
- diagnostics;
- synthesis packets.

### Deterministic core

Rust is well suited to:

- canonical identity and hashing;
- replayable problem-space folds;
- graph and occurrence traversal;
- temporal ordering;
- deterministic deduplication;
- bounded packet assembly;
- SQLite access;
- provider HTTP calls;
- projection snapshot validation.

## What Rust would not solve

Rust cannot determine the correct ontology.

It can make a wrong ontology beautifully explicit and difficult to remove.

A type such as:

```text
EligibleRelationProposition
```

would still be wrong if the kernel has no place for it.

Rust therefore increases the cost of conceptual mistakes as much as it increases structural safety.

## Dynamic projection versus static types

The corpus's actual semantic possibilities remain data.

Rust types should define the shape of:

- problem-space records;
- semantic objects, regions, and units;
- identifiers;
- relations and occurrences;
- retrieval surfaces;
- semantic-access plans;
- retrieval and synthesis packets.

They should not hard-code actual corpus objects, headings, links, dates, or Organon identifier values.

```text
Rust type system
    defines the shape and authority of runtime records

SemanticSpaceProjection
    contains corpus-specific schema and instances
```

## Semantic units versus transport segments

Rust should encode the distinction directly:

```text
SemanticUnitId
TransportSegmentId
```

A provider or embedding limit may divide one unit into transport segments without creating additional canonical semantic units.

The type system should make accidental promotion of a transport segment into an independently authored semantic unit difficult.

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

That membership validation remains a pure runtime operation against the frozen projection snapshot.

## Suggested initial package shape

Begin with one Rust package and strict modules:

```text
model
problem_space
projection
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

Split modules into crates only after the boundaries survive real implementation.

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

`CLEANROOM` is an independent repository.

The real corpus should enter through a versioned, read-only substrate or projection exchange contract.

Legacy runtime orchestration must not become the route by which the new kernel accesses corpus facts.

## Obsidian boundary

The Obsidian plugin remains TypeScript and communicates through a versioned protocol.

Authored wikilinks remain Obsidian-facing syntax. Canonical UUID and region/unit resolution remain substrate and projection responsibilities.

## Python's remaining role

Python may remain useful for:

- offline evaluation;
- corpus inspection;
- migration scripts;
- notebooks;
- report generation;
- compatibility analysis.

Avoid a hybrid production kernel in which Python and Rust each own different versions of problem-space state, semantic-access schemas, or projection authority.

There should be one runtime authority.

## Decision rule

Choose Rust when:

- the clean contracts have been accepted;
- the runtime is being built greenfield;
- explicit type distinctions are more valuable than fastest possible prototyping;
- slower initial iteration is acceptable in exchange for stronger structural discipline.

Choose Python for disposable prototypes while contracts remain unstable, but do not let a prototype become a second production authority.

## Current recommendation

Use a two-step decision:

1. freeze the language-neutral kernel and machine-readable schemas;
2. implement the accepted runtime in Rust.

Do not port the old Python runtime.

First implement the kernel against a tiny synthetic `SemanticSpaceProjection`. Then connect real corpus artifacts through a new versioned adapter boundary.
