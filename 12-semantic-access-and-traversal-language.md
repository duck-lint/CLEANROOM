# Semantic Access and Traversal Language

## Status

Seed contract for review.

This document defines how the second inference call connects the current relational problem space to the projected semantic space.

## 1. Terminology

`Traversal` remains useful, but it is not the whole operation.

The problem space presents a lens through which the semantic space is viewed.

The runtime activates a bounded semantic region through that lens.

The second inference call then specifies how semantic units are to be collected from the activated region.

The proposed top-level term is:

```text
SemanticAccessPlan
```

A plan may contain one or more:

```text
TraversalPath
```

A traversal path is directed movement through represented semantic connections.

This preserves the project term `Semantic Traversal` without treating the entire inference process as one linear graph walk.

## 2. Formal place in the kernel

Let:

- \(P_t\) be the updated relational problem space;
- \(\Lambda_t\) be its current attention lens;
- \(u_t\) be the newest utterance;
- \(M_\sigma\) be the immutable semantic projection snapshot for the turn;
- \(W_t\) be the activated working projection;
- \(T_t\) be the final semantic-access plan.

The second inference call operates as:

\[
T_t = I_2(P_t,u_t,W_t)
\]

The final executable plan must conform to:

\[
C(T_t,M_\sigma)
\]

## 3. What guides semantic access

The second inference call may begin from:

- active problem regions;
- anchor referents;
- relations presently in question;
- active constraints;
- open tensions;
- the current attention lens;
- the newest utterance.

Examples:

```text
region:
    Capital and Blood Meridian chronology

referents:
    Capital
    Blood Meridian

constraint:
    compare temporal anchors

open tension:
    chronology dimension unresolved
```

These problem-space records guide projection exploration.

They are not executable corpus addresses.

## 4. Exploration versus execution

The second inference call uses projection tools to bind problem-space structure to:

- canonical semantic objects;
- canonical semantic units;
- semantic regions;
- identifier assignments;
- occurrences;
- temporal anchors;
- available retrieval surfaces.

The final executable plan uses resolved projected addresses wherever canonical addresses exist.

```text
problem-space region, relation, constraint, or tension
    guides exploration

projection access
    resolves semantic addresses and paths

final semantic-access plan
    references represented nodes, edges, surfaces, and outputs
```

The runtime does not execute an ambiguous natural-language graph path.

## 5. Canonical semantic addresses

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

## 6. Semantic regions

A region is an addressable authored structural area inside a semantic object.

Examples:

```text
Capital#Chapter 2
Capital#^block-id
```

A heading region may contain one or more semantic units.

A block address may resolve to one explicitly identified unit.

A semantic region is structural addressability.

It is not a third epistemic ontology above objects and units.

## 7. Plan topology

A semantic-access plan is a typed directed acyclic graph.

It may:

- branch;
- execute retrieval surfaces in parallel;
- follow multiple objects;
- rejoin at a comparison, grouping, or ordering operation;
- preserve separate provenance paths.

A simple plan may serialize as a linear path.

Graph form supports:

```text
Capital ──────────────┐
                      ├→ contextual occurrences → dated units → chronology
Blood Meridian ───────┘
```

## 8. Direction is explicit and fluid

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

The second inference call chooses the direction.

The deterministic runtime executes it.

No single route is globally privileged.

## 9. Two classes of operation

### 9.1 Connection operations

```text
follow containment
follow parent relation
follow outgoing occurrence
follow incoming occurrence
follow heading or block target
follow inherited identifier
follow temporal anchor
```

### 9.2 Retrieval-surface operations

```text
exact
lexical
vector
graph
temporal
```

A plan may combine both classes.

## 10. Problem-space bindings

The plan records how its projected addresses relate back to the problem space.

Examples:

```text
problem region → canonical object bindings
problem relation → traversal paths
active constraint → required operation
open tension → requested output or unresolved plan objective
```

This provenance allows later inspection of why an address or path was selected.

The plan does not mark the problem region resolved.

Resolution remains a synthesis or later boundary-inference outcome.

## 11. Configured execution bounds

Configuration defines:

- enabled surfaces;
- default activation depth;
- default candidate bounds;
- hard maximum depth;
- hard maximum candidate bounds;
- packet-size limits;
- exact-count scope;
- per-object or per-region caps.

The model does not spend inference selecting ordinary initial depths and limits.

The runtime applies configured defaults.

The model may request targeted continuation within the configured hard maxima when projection telemetry shows a need.

The accepted configuration snapshot is attached to the plan.

## 12. Required and optional operations

Each plan operation may be:

```text
required
optional
```

### Required

Failure affects whether the requested claim can be supported.

Examples:

- exact exhaustive count for a corpus-wide absence claim;
- temporal ordering for a chronology question;
- canonical target resolution for an object comparison.

### Optional

Failure is recorded but does not automatically invalidate the primary evidence route.

Examples:

- vector expansion for supporting context;
- secondary graph enrichment;
- lexical recall supporting a successful exact route.

Required or optional status defines execution obligation.

It does not determine semantic truth.

## 13. Requested output shape

The plan declares what execution must materialize.

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

The output declaration controls packet shape.

It does not prescribe the final answer.

## 14. Open tensions and access objectives

An open tension may guide a plan toward information needed for synthesis.

Examples:

```text
unresolved referent
    → retrieve candidate canonical bindings

missing chronology dimension
    → retrieve publication and reading relation surfaces

contradictory prior framing
    → collect evidence for both represented routes
```

The plan may preserve the tension as unresolved.

It may not close the tension merely because one path was easier to activate.

## 15. Seed plan contract

```text
SemanticAccessPlan
    plan_id
    projection_snapshot_id
    problem_space_version
    focus_utterance_id
    configuration_snapshot_id
    problem_region_bindings[]
    relation_bindings[]
    constraint_bindings[]
    open_tension_bindings[]
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
    problem_space_provenance[]
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

## 16. Conformance

Conformance validates structural existence.

It checks that:

- every address exists;
- every relation exists;
- the requested direction is represented;
- the requested surface is available;
- each operation may consume the preceding output type;
- requested outputs can be materialized;
- configuration bounds are respected.

It does not:

- judge problem-space coherence;
- resolve open tensions;
- decide whether prose proves a paraphrase;
- decide whether evidence is semantically close enough.

## 17. Repair

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

## 18. Example — book chronology

Problem-space state:

```text
region:
    Capital and Blood Meridian chronology

referents:
    Capital
    Blood Meridian

constraint:
    temporal comparison

open tension:
    comparison dimension may require binding
```

Projection exploration resolves canonical objects and represented temporal paths.

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

## 19. Possible Rust encoding

Illustrative only:

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

The semantic distinctions come first:

```text
problem region
≠ semantic object
≠ semantic unit
≠ semantic region
≠ identifier assignment
≠ occurrence
≠ anchor
```

Rust's role is to stop code from treating these as interchangeable.

## 20. Acceptance conditions

The semantic-access language is acceptable when:

1. the top-level object is a semantic-access plan;
2. traversal paths remain explicit routes inside it;
3. active problem regions and tensions have traceable bindings;
4. final plans use canonical projected addresses;
5. plans may branch and rejoin;
6. direction may be incoming or outgoing;
7. all operations are typed;
8. configured defaults are deterministic;
9. required and optional obligations remain distinct;
10. output shape is declared;
11. open tensions may guide access without being silently resolved;
12. repair is bounded to one fresh inference call;
13. no operation creates a post-retrieval semantic or coherence veto.
