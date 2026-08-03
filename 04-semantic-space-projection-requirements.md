# Semantic-Space Projection Requirements

## 1. Purpose

The projected semantic space \(M\) is the runtime-accessible representation of the structured corpus.

It is the bound on traversal inference.

The inference model uses it to connect the current problem space to possible semantic units and paths. Structural conformance uses it to reject nonexistent paths. Execution uses it to materialize the selected units.

## 2. Closed-world scope

The projection is closed over the semantic reality accessible to the runtime.

This does not claim completeness beyond the corpus.

It means:

- every admitted semantic possibility available in the indexed corpus is represented;
- the runtime may traverse represented possibilities;
- the runtime may not fabricate absent possibilities.

## 3. Two simultaneous responsibilities

The projection must contain both:

### Schema-level possibility

- kinds of semantic objects;
- kinds of semantic units;
- identifier definitions;
- identifier applicability;
- relation definitions;
- valid directions;
- valid transitions;
- retrieval-surface capabilities;
- temporal and graph affordances;
- structural bounds.

### Instance-level actuality

- canonical object identities;
- canonical unit identities;
- object-to-unit membership;
- unit-to-object belonging;
- actual identifier assignments;
- actual occurrences;
- actual link targets;
- actual temporal anchors;
- actual inbound and outbound incidence;
- actual heading and block target resolution.

A schema without canonical instances is too abstract for traversal.

Canonical instances without the schema are too opaque for inference.

## 4. Exhaustive addressability

The authoritative projection must expose all semantic possibilities and addresses.

If the inference model cannot receive the entire projection in one prompt, the solution must preserve exhaustive access through mechanisms such as:

- indexed projection queries;
- deterministic paging;
- typed lookup;
- bounded summaries paired with exact address-resolution tools;
- hierarchical projection;
- schema-first projection followed by instance lookup.

Mandatory semantic structure must not be silently truncated.

A bounded prompt representation may be an interface to \(M\). It is not permitted to become a lossy replacement for \(M\).

## 5. Required object representation

For each semantic object, expose:

- canonical object identifier;
- source UUID or stable source identity;
- canonical path;
- title and admitted aliases;
- object type identifiers;
- admitted frontmatter;
- topology;
- contained semantic-unit addresses;
- authored occurrences;
- inbound occurrence addresses;
- temporal relations, when materialized.

## 6. Required unit representation

For each semantic unit, expose:

- canonical unit identifier;
- parent object identifier;
- section and heading path;
- paragraph and split ordinals;
- block identifier when present;
- semantic-unit text;
- inherited object identifiers;
- unit-local identifiers;
- authored occurrences;
- inbound occurrences;
- temporal anchors;
- retrieval visibility;
- source provenance.

## 7. Identifier descriptors

For every admitted identifier, expose:

- identifier name;
- value shape;
- scalar or collection form;
- applicable object and unit domains;
- whether inherited;
- whether local;
- whether relational;
- whether it may contain canonical links;
- whether it may create a temporal anchor;
- which retrieval surfaces may inspect it;
- which transitions it enables.

The projection must support structural checks such as:

```text
Cleo does not carry journal_date
Capital carries or inherits book
this dated journal unit links to Capital
```

No semantic judge is needed. The assignments and relations either exist or do not.

## 8. Relations and occurrences

The projection must represent:

- contains;
- belongs_to;
- inherits;
- object-to-object authored link;
- unit-to-object authored link;
- unit-to-unit authored link;
- has_temporal_anchor;
- inbound contextual occurrence;
- heading target;
- block target;
- embed target;
- relation direction;
- occurrence source surface.

An occurrence must be an addressable object with its own provenance, not merely an untyped target pair.

## 9. Heading and block resolution

The projection must provide deterministic mappings from authored targets to canonical units:

```text
Capital#Chapter 2 → canonical semantic-unit address
Capital#^block-id → canonical semantic-unit address
```

Ambiguity must be represented explicitly at ingest or projection time.

Runtime execution must not invent target units from string heuristics.

## 10. Retrieval-surface projection

For every retrieval surface, expose:

- whether it is available;
- which semantic components it can inspect;
- accepted match or query modes;
- candidate and depth bounds;
- identity returned by the surface;
- coverage semantics;
- whether exhaustive count is possible;
- whether results hydrate to canonical units;
- whether graph or temporal chaining is possible.

Possible surfaces may include exact, lexical, vector, graph, and temporal retrieval.

The projection describes capabilities. It does not choose the traversal.

## 11. Valid transitions

The projection must state valid transitions such as:

```text
problem-space marker → identifier lookup
identifier or canonical address → retrieval surface
retrieval result → canonical semantic unit
semantic unit → parent object
semantic object → contained units
source occurrence → target object or unit
target object or unit → inbound occurrences
unit or object → temporal anchor
anchored contextual unit → temporal evaluation
retrieved candidates → deterministic deduplication
deduplicated candidates → bounded packet assembly
```

The exact notation may change.

The requirement is that traversal validity is checkable as structural membership rather than natural-language interpretation.

## 12. Structural conformance support

The projection must make it possible to reject a traversal because:

- an identifier is absent;
- the identifier cannot apply to the proposed object or unit;
- the canonical object or unit does not exist;
- a relation does not exist;
- the requested direction is unavailable;
- a heading or block target is unresolved;
- a surface cannot inspect the proposed component;
- a required evaluator is unavailable;
- a requested path exceeds declared bounds.

The projection must not be designed around post-retrieval semantic filtering.

## 13. Projection versioning and identity

The projection should carry:

- schema version;
- source-ingest identity;
- logical projection hash;
- corpus snapshot identity;
- validation status;
- compatibility policy hash;
- deterministic serialization.

Inference, conformance, execution, and synthesis diagnostics must identify the exact projection snapshot used for the turn.

## 14. Projection acceptance tests

A valid projection must prove:

1. every canonical object is addressable;
2. every canonical unit is addressable;
3. every unit resolves to its object;
4. every object resolves to its units;
5. every admitted identifier has an applicability descriptor;
6. every occurrence resolves to its target;
7. every inbound occurrence can be enumerated;
8. every heading and block target resolves deterministically;
9. every temporal anchor identifies its source object or unit;
10. every retrieval surface reports its visible components and bounds;
11. every valid transition is represented;
12. no mandatory semantic structure was discarded by prompt-size projection.
