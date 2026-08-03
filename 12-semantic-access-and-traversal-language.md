# Semantic Access and Traversal Language

## Status

Seed contract for review.

This document defines how the second inference call connects the current problem space to the projected semantic space.

## 1. Terminology

`Traversal` remains useful, but it is not the whole operation.

The problem space presents a lens through which the semantic space is viewed. The runtime activates a bounded region through that lens. The second inference call then specifies how semantic units are to be collected from the activated region.

The proposed top-level term is:

```text
SemanticAccessPlan
```

A plan may contain one or more:

```text
TraversalPath
```

A traversal path is the directed movement through represented semantic connections.

This preserves the project term `Semantic Traversal` without treating the entire inference process as one linear graph walk.

## 2. Formal place in the kernel

Let:

- \(P_t\) be the updated problem space;
- \(u_t\) be the newest utterance;
- \(M_\sigma\) be the immutable semantic projection snapshot for the turn;
- \(W_t\) be the activated working projection;
- \(T_t\) be the final semantic-access plan.

The second inference call operates as:

\[
T_t = I_2(P_t, u_t, W_t)
\]

The final executable plan must conform to the full frozen projection:

\[
C(T_t, M_\sigma)
\]

## 3. Exploration versus execution

The second inference call may begin from descriptive problem-space markers such as:

```text
Capital
Blood Meridian
go vegan
the calf
earliest
exact phrase
```

It may use projection tools to bind those markers to:

- canonical semantic objects;
- canonical semantic units;
- semantic regions;
- identifiers;
- occurrences;
- temporal anchors;
- available retrieval surfaces.

The final executable plan must use resolved projected addresses wherever canonical addresses exist.

In other words:

```text
problem-space language
    guides exploration

projection access
    resolves addresses

final access plan
    references represented nodes, edges, identifiers, and surfaces
```

The runtime does not execute an ambiguous natural-language graph path.

## 4. Canonical semantic addresses

A semantic-access plan may address:

```text
semantic object
semantic unit
semantic region
identifier assignment
authored occurrence
temporal anchor
retrieval surface
```

A language-neutral address model is:

```text
SemanticAddress
    Object(object_uuid)
    Unit(unit_id)
    Region(object_uuid, structural_address)
    Identifier(identifier_name, represented_value)
    Occurrence(occurrence_id)
    TemporalAnchor(anchor_id)
```

The exact serialization remains open.

## 5. Semantic regions

A region is an addressable authored structural area inside a semantic object.

Examples:

```text
Capital#Chapter 2
Capital#^block-id
```

A heading region may contain one or more semantic units.

A block address may resolve to one explicitly identified unit.

A region is structural addressability, not a third epistemic ontology above objects and units.

## 6. Plan topology

A semantic-access plan is a typed directed acyclic graph.

It may:

- branch;
- execute retrieval surfaces in parallel;
- follow multiple objects;
- rejoin at a comparison or grouping operation;
- preserve separate provenance paths.

A simple plan may still serialize as a linear path.

The graph form is required for cases such as:

```text
Capital ──────────────┐
                      ├→ contextual occurrences → dated units → chronology
Blood Meridian ───────┘
```

## 7. Direction is explicit and fluid

Semantic connections may be traversed in any direction represented by the projection.

Examples:

```text
object → contained units
unit → parent object
source unit → outgoing occurrence → target object
target object → incoming occurrence → source unit
source unit → outgoing occurrence → target unit
target unit → incoming occurrence → source unit
unit → temporal anchor
anchor → anchored unit
```

The second inference call chooses the required direction in the plan.

The deterministic runtime executes that declared direction.

The runtime does not force every query through inbound occurrences or any other single route.

This should feel like canonical Obsidian navigation generalized into a typed semantic substrate.

## 8. Two classes of plan operation

### 8.1 Connection operations

These follow represented structure:

```text
follow containment
follow parent relation
follow outgoing occurrence
follow incoming occurrence
follow heading or block target
follow inherited identifier
follow temporal anchor
```

### 8.2 Retrieval-surface operations

These discover, match, rank, count, or order materialized semantic records:

```text
exact
lexical
vector
graph
temporal
```

A plan may combine both classes.

## 9. Configured execution bounds

Configuration defines:

- enabled surfaces;
- default activation depth;
- default candidate bounds;
- hard maximum depth;
- hard maximum candidate bounds;
- packet-size limits;
- exact-count scope;
- per-object or per-region caps.

The model does not spend inference deciding ordinary initial depths and limits.

The runtime applies configured defaults automatically.

During the second inference call, targeted continuation or expansion may be requested only when the visible region and telemetry show a reason to expand, and only within the configured hard maxima.

The accepted configuration snapshot is attached to the plan.

## 10. Required and optional operations

Each plan operation may be labeled:

```text
required
optional
```

This is a yes/no design decision about supporting both labels; it is not a choice to make every step one universal category.

### Required

Failure affects whether the requested claim can be supported.

Examples:

- exact exhaustive count for a corpus-wide absence claim;
- temporal ordering for a chronology question;
- canonical target resolution for an object comparison.

### Optional

Failure is recorded but does not automatically invalidate the primary evidence path.

Examples:

- vector expansion for extra context;
- secondary graph enrichment;
- supporting lexical recall when a required exact path succeeded.

Required or optional status does not determine semantic truth. It determines execution obligations.

## 11. Requested output shape

The plan explicitly declares what execution must materialize.

Possible outputs include:

```text
semantic units
canonical object identities
canonical unit identities
semantic regions
occurrence paths
temporal anchors
grouped evidence by object
ordered evidence
total exact count
surface provenance
```

The output declaration controls packet shape, not the final conclusion.

## 12. Seed plan contract

```text
SemanticAccessPlan
    plan_id
    projection_snapshot_id
    problem_space_version
    focus_utterance_id
    configuration_snapshot_id
    address_bindings[]
    traversal_paths[]
    surface_operations[]
    joins[]
    required_outputs[]
    optional_outputs[]
    coverage_requirements[]
```

```text
TraversalPath
    path_id
    start_addresses[]
    operations[]
    output_binding
```

```text
PlanOperation
    operation_id
    requirement: required | optional
    input_bindings[]
    operation_type
    direction
    relation_or_surface
    constraints
    output_binding
```

Exact names remain provisional.

## 13. Conformance

Conformance validates only structural existence.

It checks that:

- every address exists;
- every relation exists;
- the requested direction is represented;
- the requested surface is available;
- each operation may consume the preceding output type;
- requested outputs can be materialized;
- configuration bounds are respected.

It does not judge whether retrieved prose semantically proves a generated proposition.

## 14. Repair

Initial repair policy:

- one repair attempt;
- a fresh inference call;
- same problem-space state;
- same newest utterance;
- same frozen projection snapshot;
- same configuration;
- invalid plan plus exact structural violations.

The repair call may revise the plan.

No deterministic component repairs natural-language meaning.

A second invalid result becomes an explicit inference failure.

## 15. Example — book chronology

Problem-space focus:

```text
Which did I start first, Capital or Blood Meridian?
```

Exploration resolves:

```text
Capital → canonical book object
Blood Meridian → canonical book object
```

Final plan:

```text
path A:
    Capital
    → incoming canonical occurrences
    → source semantic units
    → temporal anchors

path B:
    Blood Meridian
    → incoming canonical occurrences
    → source semantic units
    → temporal anchors

join:
    order anchors by canonical target

outputs:
    grouped contextual units
    canonical target identities
    temporal anchors
    provenance paths
```

Synthesis performs the semantic comparison.

## 16. Possible Rust encoding

This section is illustrative, not normative.

```rust
enum SemanticAddress {
    Object(ObjectId),
    Unit(UnitId),
    Region(RegionAddress),
    Identifier(IdentifierAddress),
    Occurrence(OccurrenceId),
    TemporalAnchor(AnchorId),
}

enum Direction {
    Outgoing,
    Incoming,
}

enum Requirement {
    Required,
    Optional,
}

enum PlanOperation {
    FollowRelation(RelationStep),
    SearchSurface(SearchStep),
    EvaluateTemporal(TemporalStep),
    Hydrate(HydrationStep),
    Join(JoinStep),
}
```

The user's semantic distinction comes first:

```text
object
≠ unit
≠ region
≠ identifier assignment
≠ occurrence
≠ anchor
```

Rust's role is to prevent code from treating those addresses as interchangeable.

## 17. Acceptance conditions

The traversal language is acceptable when:

1. the top-level operation is a semantic-access plan;
2. traversal paths remain explicit internal routes;
3. final executable plans use canonical projected addresses;
4. plans may branch and rejoin;
5. direction is explicit and may be incoming or outgoing;
6. all operations are typed;
7. configured defaults are deterministic;
8. the model does not choose routine initial bounds;
9. required and optional execution obligations are distinct;
10. output shape is declared;
11. repair is bounded to one fresh inference call;
12. no plan operation creates a post-retrieval semantic veto.
