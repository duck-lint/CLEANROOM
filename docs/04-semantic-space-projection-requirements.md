# Semantic-Space Projection Requirements

## 1. Purpose

The frozen semantic projection \(M_\sigma\) is the runtime-accessible representation of the structured corpus for one complete turn.

It is the closed structural bound on semantic-access inference, conformance, and execution.

The second inference call uses bounded activated views of \(M_\sigma\) to connect the current problem space to possible semantic addresses and access paths. Structural conformance uses the full snapshot to reject nonexistent paths. Execution uses it to materialize the requested semantic units.

## 2. Closed-world scope

The projection is closed over the semantic reality admitted and materialized from the indexed corpus snapshot.

This does not claim completeness beyond the corpus.

It means:

- every admitted semantic possibility available in the snapshot is represented;
- every canonical object, unit, region, occurrence, and anchor is addressable;
- the runtime may follow represented possibilities;
- the runtime may not fabricate absent possibilities.

## 3. Frozen snapshot identity

Every turn binds to one immutable projection snapshot:

```text
projection_snapshot_id
ingest_identity
schema_version
logical_hash
corpus_snapshot_identity
configuration_snapshot_id
validation_status
```

The snapshot may not change between:

```text
boundary inference
→ activation
→ semantic-access inference
→ conformance
→ execution
→ synthesis
```

Corpus changes become visible on a later turn through a new snapshot.

## 4. Two simultaneous responsibilities

The projection must contain both schema-level possibility and instance-level actuality.

### 4.1 Schema-level possibility

- semantic object classes;
- semantic unit and region address shapes;
- identifier definitions;
- identifier applicability;
- inheritance rules;
- relation and occurrence definitions;
- valid directions;
- valid transitions;
- retrieval-surface capabilities;
- temporal and graph affordances;
- structural and configuration bounds.

### 4.2 Instance-level actuality

- canonical object UUIDs;
- canonical unit identities;
- canonical region addresses;
- object-to-region and object-to-unit containment;
- unit-to-object and unit-to-region belonging;
- actual identifier assignments;
- actual authored occurrences;
- resolved object, heading, and block targets;
- actual temporal anchors;
- actual incoming and outgoing incidence.

A schema without canonical instances is too abstract for semantic access.

Instances without schema-level possibility are too opaque for structural conformance.

## 5. Exhaustive addressability with bounded access

The authoritative projection must expose every admitted semantic address even when it cannot be inserted into one model prompt.

Bounded access may use:

- typed projection queries;
- identifier lookup;
- hierarchical summaries;
- deterministic paging;
- exact address resolution;
- high-degree hub summaries;
- continuation handles;
- bounded activated working views.

Mandatory semantic structure must not be silently truncated.

A working projection \(W_t\) is an interface into \(M_\sigma\), not a lossy replacement for it:

\[
W_t \subseteq M_\sigma
\]

Absence from \(W_t\) does not establish absence from \(M_\sigma\).

## 6. Required semantic-object representation

For each semantic object, expose:

- canonical UUID;
- source identity and source kind;
- canonical path and filename surfaces;
- title and admitted aliases;
- object type and format identifiers;
- admitted frontmatter with field provenance;
- topology;
- authored heading tree;
- contained semantic-region addresses;
- contained semantic-unit addresses;
- object-field occurrences;
- body occurrences;
- incoming occurrence addresses;
- temporal anchors or temporal relations when materially sourced;
- retrieval-surface affordances.

The projection must preserve the distinction between:

```text
canonical object identity
identifier surfaces used to discover it
```

## 7. Required semantic-region representation

For each addressable authored region, expose:

- parent object UUID;
- canonical region address;
- authored heading path;
- heading identity and source span;
- contained child-region addresses;
- contained semantic-unit addresses;
- block-target mappings where present;
- incoming occurrences that target the region;
- inherited object identifiers;
- retrieval-surface affordances.

A heading target may resolve to a region containing multiple semantic units.

The runtime must not arbitrarily collapse such a region to one unit.

## 8. Required semantic-unit representation

For each semantic unit, expose:

- canonical unit identifier;
- parent object UUID;
- parent region address;
- authored block type;
- heading path;
- block ordinal;
- explicit block identifier when present;
- semantic-unit text or a deterministic hydration address;
- inherited object identifiers with provenance;
- unit-local identifiers;
- authored outgoing occurrences;
- incoming occurrences;
- temporal anchors;
- retrieval visibility;
- source provenance;
- transport-segment descriptors when technical segmentation is required.

## 9. Transport-segment representation

Transport or embedding segments are technical subdivisions of one semantic unit.

The projection may expose them for provider or embedding operations, but each segment must retain:

- parent semantic-unit identity;
- deterministic segment ordinal;
- complete ordering;
- source-span provenance;
- reconstruction metadata.

A transport segment is not independently promoted to a canonical semantic unit merely because a tokenizer or provider requires splitting.

## 10. Identifier descriptors

For every admitted identifier, expose:

- identifier name;
- semantic role;
- value shape;
- scalar or collection form;
- applicable object, region, and unit domains;
- whether it is intrinsic, inherited, local, or relational;
- source surface and provenance;
- whether it may contain canonical links;
- whether it creates or points to a temporal anchor;
- which retrieval surfaces may inspect it;
- which relations or transitions it enables.

Examples of roles include:

```text
individuation
object class
Organon position
register typing
canonical naming
attribution
temporal anchoring
contextual relation
grouping
indexical telemetry
```

The projection must support structural checks such as:

```text
Cleo does not intrinsically carry journal_entry_date
Marx, Karl — Capital carries note_type: source_material and format: book
a dated journal field occurrence links to that canonical object
```

No semantic judge is needed. The assignments and paths either exist or do not.

## 11. Retrieval-surface affordances across identifiers

Every admitted identifier must expose every enabled retrieval surface structurally capable of operating on its representation.

Possible surfaces include:

- exact;
- lexical;
- vector;
- graph;
- temporal.

A surface descriptor must state:

- whether the surface is available;
- which object, region, unit, identifier, occurrence, or anchor records it can inspect;
- accepted query or match modes;
- default and hard candidate bounds;
- identity returned by the surface;
- whether results hydrate to canonical semantic units;
- coverage semantics;
- whether exhaustive total count is possible;
- whether graph or temporal continuation is possible;
- explicit technical limitations.

No identifier may be omitted from a capable surface through an undocumented case-specific rule.

## 12. Relations and occurrences

The projection must represent at least:

- object contains region;
- object contains unit;
- region contains child region;
- region contains unit;
- unit belongs to object;
- unit situated in region;
- unit inherits object identifier;
- object-field authored occurrence;
- unit-to-object authored occurrence;
- unit-to-region authored occurrence;
- unit-to-unit authored occurrence;
- incoming occurrence;
- outgoing occurrence;
- has temporal anchor;
- heading target;
- block target;
- embed target;
- relation direction;
- occurrence source surface.

An occurrence is an addressable record with its own provenance, not merely an untyped pair of strings.

## 13. Authored-target resolution

The projection must provide deterministic mappings from authored Obsidian targets to canonical addresses.

Examples:

```text
[[Marx, Karl — Capital]]
→ canonical object UUID
```

```text
[[Marx, Karl — Capital#Chapter 2]]
→ canonical semantic-region address
```

```text
[[Marx, Karl — Capital#^block-id]]
→ canonical semantic-unit address
```

The projection preserves the authored target text and resolved canonical address.

Ambiguity must be represented explicitly at ingest or projection time. Runtime execution must not invent a target from string heuristics.

## 14. Reverse incidence

Every resolved target must be discoverable from both represented directions.

```text
source unit
→ outgoing occurrence
→ target object, region, or unit
```

```text
target object, region, or unit
→ incoming occurrence
→ source unit or object field
```

Stored reverse edges are optional. Reverse addressability is mandatory.

A contextual date remains available through its canonical occurrence path rather than being silently converted into an intrinsic target identifier.

## 15. Activated working projection support

The full projection must support deterministic construction of a positive activated view:

\[
W_t^{(0)} = A_{\mathrm{cfg}}(M_\sigma,P_t,u_t,\Lambda_t)
\]

Activated records must expose:

- canonical identity;
- relevant identifiers;
- incoming and outgoing summaries;
- available surfaces;
- bounded previews;
- candidate counts;
- continuation handles;
- activation provenance linking back to problem regions, relations, constraints, open tensions, the newest utterance, or configuration.

Activation means presently visible, not relevant or exhaustive.

## 16. Valid semantic-access transitions

The projection must state valid transitions such as:

```text
problem-region referent → identifier lookup
problem-space relation → represented graph direction
active constraint → required surface operation
open tension → one or more candidate binding routes
identifier or canonical address → retrieval surface
retrieval-surface result → canonical semantic unit hydration
semantic object → contained regions or units
semantic region → contained units
semantic unit → parent object or region
source occurrence → target object, region, or unit
target address → incoming occurrences
object or unit → temporal anchor
anchored contextual unit → temporal evaluation
retrieval result → deterministic deduplication
retrieval result → bounded packet assembly
```

Problem-space records guide semantic-access inference. They are not corpus addresses until bound to projected structure.

## 17. Structural conformance support

The projection must make it possible to reject a semantic-access plan because:

- an identifier is absent;
- an identifier cannot apply to the proposed address kind;
- a canonical object, region, or unit does not exist;
- a relation does not exist;
- the requested direction is unavailable;
- a heading or block target is unresolved;
- a surface cannot inspect the proposed component;
- a required evaluator is unavailable;
- an operation consumes or emits the wrong address type;
- a requested bound exceeds configuration.

Conformance may record exact violations for diagnostics and bounded repair.

It may not decide whether the user's language or retrieved evidence is semantically close enough, coherent enough, or propositionally adequate.

## 18. Projection versioning and invalidation

A new projection snapshot is required when identity-relevant material changes, including:

- admitted object creation or removal;
- UUID or canonical target changes;
- frontmatter identifier changes;
- heading, block, or authored-unit address changes;
- link-target resolution changes;
- transport-segmentation policy changes when segment descriptors are part of the projection;
- identifier applicability changes;
- retrieval-surface capability changes;
- schema changes.

The projection must serialize deterministically enough to produce a stable logical hash for equivalent input state.

## 19. Projection acceptance tests

A valid projection must prove:

1. every canonical semantic object is addressable by UUID;
2. object discovery surfaces remain distinct from canonical identity;
3. every canonical semantic region is addressable;
4. every canonical semantic unit is addressable;
5. every unit resolves to its object and region;
6. every object and region resolves to contained units;
7. every admitted identifier has a role and applicability descriptor;
8. every valid identifier-to-surface affordance is exposed;
9. every occurrence resolves to its canonical target;
10. every target exposes incoming incidence;
11. every heading target resolves to a region deterministically;
12. every block target resolves to a unit deterministically;
13. every temporal anchor identifies its source object or unit;
14. transport segments retain one parent unit identity;
15. every retrieval surface reports visible components and bounds;
16. every valid transition is represented;
17. activation provenance is available;
18. no mandatory semantic structure is discarded by prompt-size constraints;
19. absence from an activated view cannot be mistaken for corpus absence.
