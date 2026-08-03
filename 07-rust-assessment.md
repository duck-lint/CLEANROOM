# Rust Assessment

## Recommendation

Rust is a strong candidate for the clean deterministic runtime, and likely the best default choice if the clean contracts are frozen before implementation begins.

The reason is not raw speed.

The reason is that Rust can make the distinctions in the kernel difficult to blur accidentally:

```text
ProblemSpaceState
UtteranceBoundary
SemanticSpaceProjection
TraversalPlan
ConformanceResult
RetrievalResult
ExecutionLimits
SynthesisInput
```

These can be distinct types rather than loosely related dictionaries passed through one large orchestration path.

## What Rust would help with

### Explicit state shapes

Structs and enums can represent valid traversal variants, canonical addresses, relation directions, retrieval requests, structural violations, execution outcomes, and synthesis-packet contents.

Pattern matching encourages every variant to be handled explicitly.

### Failure boundaries

Recoverable failures can be represented as typed results:

```text
Result<ConformingTraversal, ConformanceViolations>
Result<RetrievalResult, ExecutionFailure>
```

This supports the distinction between invalid traversal, unavailable surface, successful zero matches, partial execution, and provider failure.

### Serialization contracts

Serde can serialize and deserialize strongly typed runtime objects.

A JSON Schema generator can expose LLM-facing contracts for inference output, persisted thread state, semantic projection exchange, diagnostics, and synthesis packets.

### Deterministic core

Rust is well suited to canonical identity, hashing, graph traversal, temporal ordering, deduplication, ranking, bounded packet assembly, SQLite access, HTTP provider calls, and replayable state transitions.

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

Rust types should define the shape of objects, units, identifiers, relations, occurrences, surfaces, traversals, and packets.

They should not hard-code actual corpus objects, headings, links, or dates.

The clean distinction is:

```text
Rust type system
    defines the shape of epistemic records

SemanticSpaceProjection
    contains corpus-specific records
```

## Pydantic analogy

The rough correspondence is:

```text
Pydantic model
    ↔ Rust structs/enums + Serde + JSON Schema

model instance validation
    ↔ deserialization plus explicit conformance

dynamic corpus schema and instances
    ↔ SemanticSpaceProjection data
```

Traversal conformance is more than JSON shape validation. A traversal may be valid JSON while referencing an absent identifier, object, relation, target, or surface.

That membership validation remains a pure runtime operation against the projection.

## Suggested initial shape

Begin with one Rust package and strict modules:

```text
model
projection
thread
inference
conformance
execution
packet
synthesis
runtime
cli
```

Split modules into crates only after the boundaries survive real implementation.

## Provider boundary

Both LLM roles should sit behind narrow interfaces:

```text
InferenceProvider
SynthesisProvider
```

Provider-specific request formats must not leak into kernel types.

## Obsidian boundary

The Obsidian plugin remains TypeScript and communicates through a versioned protocol.

## Python's remaining role

Python may remain useful for offline evaluation, corpus inspection, migration scripts, notebooks, and report generation.

Avoid a hybrid production kernel in which Python and Rust each own different versions of the traversal schema or projection authority.

There should be one runtime authority.

## Decision rule

Choose Rust when:

- the clean contracts have been accepted;
- the runtime is being rebuilt greenfield;
- explicit type distinctions are more valuable than fastest possible prototyping;
- slower initial iteration is acceptable in exchange for stronger structural discipline.

Choose Python when:

- the contracts are still changing daily;
- the immediate purpose is disposable prototyping;
- runtime types are not stable enough to justify compilation boundaries.

## Current recommendation

Use a two-step decision:

1. freeze the language-neutral kernel and machine-readable schemas;
2. implement the accepted runtime in Rust.

Do not begin by porting the old Python runtime.

First implement the kernel against a tiny synthetic `SemanticSpaceProjection`. Then connect real corpus artifacts through a new adapter.
