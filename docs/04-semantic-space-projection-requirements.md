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

### 5.1 Multiplex projection, not one graph surface

The semantic projection is not identical to any one graph, embedding space, index, or retrieval surface.

A useful visualization may render the projection as a high-dimensional or "semantic hyperspace," but that phrase is descriptive shorthand rather than a new runtime ontology. The authoritative structure remains the typed records, identities, relations, identifier assignments, temporal anchors, and retrieval-surface affordances represented in \(M_\sigma\).

An Obsidian-style graph corresponds primarily to an object-level authored-wikilink view once resolved into the projection. The complete projection additionally contains, as applicable:

- object → region → unit containment;
- unit and object-field occurrence provenance;
- incoming as well as outgoing incidence;
- identifier assignments and inheritance;
- contextual relation participation;
- temporal anchors and temporal incidence;
- exact, lexical, vector, graph, and temporal access affordances over the same canonical identities.

No retrieval surface individually defines the semantic space. Graph access is not privileged over exact, lexical, vector, or temporal access. Authored graph incidence remains represented corpus structure where admitted and resolved; a vector neighbourhood does not create a canonical relation merely because two represented items are near one another.

Visual encodings may expose corpus-derived or Organon-derived properties—for example, color from admitted typing or node size from measured mention frequency—but those encodings remain derived views unless the underlying value is itself represented with provenance in the projection. Force-layout coordinates, visual cluster position, color, or node size must not silently acquire semantic, epistemic, relevance, or ranking authority.

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

The object's body-occurrence aggregation includes authored non-frontmatter
occurrences sourced from semantic regions as well as semantic units. This
aggregation does not change the occurrence's canonical source provenance.

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
- authored outgoing occurrence addresses when the heading or region marker
  itself contains authored occurrence syntax;
- incoming occurrences that target the region;
- inherited object identifiers;
- retrieval-surface affordances.

A heading target may resolve to a region containing multiple semantic units.

The runtime must not arbitrarily collapse such a region to one unit.

### 7.1 Canonical heading-region individuation

Each heading-derived region must receive one deterministic canonical address by
processing authored hierarchy root-down: canonical object, canonical parent
region when present, authored structural heading address within that parent,
then a one-based collision-local ordinal only when equivalent sibling
addresses collide. The ordinal is scoped to equivalent siblings under the same
canonical parent; unrelated siblings and prose do not affect it. This is
structural individuation, not semantic inference.

Canonical region identity should remain stable under unrelated prose edits,
content changes in another region, and insertion or removal of a differently
addressed sibling. Identity may change when its own heading address changes, it
moves beneath another parent, or the authored order/cardinality of equivalent
siblings changes. The serialized `authored_structural_address` must encode the
hierarchy and discriminator deterministically and injectively without private
paths, byte offsets, or runtime state. Source span remains exact provenance and
the heading-target correspondence surface; it is not canonical identity.

#### Repository-safe corpus-contact record

Real-corpus contact exposed 62 duplicate canonical region-address groups across
182 region records and 18 objects. The collision categories were 22 repeated
equivalent-sibling groups, 40 flattened-parent groups, and 5 normalization
collision groups; category counts overlap. Six groups contained nested duplicate
regions, with maximum duplicate nesting depth four. A diagnostic remapping
using root-down hierarchical parent identity plus collision-local sibling order
closed duplicate region addresses and the observed unit-parent, containment, and
region-source-incidence failures. The observation supplied the failure evidence;
the CLEANROOM contract defines this representational identity rule.

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
- region-to-object authored occurrence;
- region-to-region authored occurrence;
- region-to-unit authored occurrence;
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

### Corpus-contact evidence — region-sourced occurrence

The accepted factual observation was produced by `duck-lint/semantic-traversal`
at commit `99d0d4556684000f0ed585e47158a5f7fe9ce7e1`, using observer schema
`vault-observation/v2`, against corpus snapshot
`25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688`.
It found 1 affected occurrence in 1 source object, 6 heading observations, 0
non-heading authored block candidates, and 0 unit candidates overlapping the
occurrence span; the occurrence span was fully contained by a heading-marker
span.

These are corpus observations, not representational authority. CLEANROOM's
response for Phase 5 real-corpus projection construction is to represent a
heading-marker occurrence with a `SemanticRegion` source and exact source span.
An occurrence inside a semantic-unit body remains `SemanticUnit`-sourced even
when a heading region contains that unit. This preserves canonical region
address, exact authored span, occurrence identity, authored direction, target
identity, and reverse incidence. The projection must not manufacture a unit,
drop the occurrence, or degrade its provenance to an object-level source.

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
source semantic region
→ outgoing occurrence
→ target object, region, or unit
```

```text
target object, region, or unit
→ incoming occurrence
→ source unit, semantic region, or object field
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
19. absence from an activated view cannot be mistaken for corpus absence;
20. no single retrieval surface is treated as the whole semantic projection;
21. visualization encodings do not create canonical identities, relations, or semantic authority.
