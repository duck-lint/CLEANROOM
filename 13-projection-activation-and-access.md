# Projection Activation and Access

## Status

Seed contract for review.

This document defines how the full semantic projection becomes a bounded, positive, expandable working view for the second inference call.

## 1. Core proposition

The full semantic space is exhaustive within the admitted corpus.

Per-turn access is constrained by the current problem space.

The problem space acts as a lens over the semantic space, activating a bounded region from which semantic units can be collected.

The lens does not declare the rest of the semantic space irrelevant.

## 2. Full projection

Let the frozen semantic projection be:

\[
M_\sigma = (V, E, I, S)
\]

Where:

- \(V\) contains canonical semantic objects, units, regions, occurrences, and anchors;
- \(E\) contains typed connections among them;
- \(I\) contains identifier assignments and inheritance;
- \(S\) contains retrieval-surface capabilities and telemetry contracts;
- \(\sigma\) identifies the immutable snapshot used for the turn.

## 3. Two inference calls

### Call 1

\[
B_t = D(P_{t-1}, u_t)
\]

\[
P_t = U(P_{t-1}, B_t)
\]

The first call deconstructs the newest utterance and updates the problem space.

### Call 2

The second call is one tool-using inference session.

It:

1. receives \(P_t\) and \(u_t\);
2. observes the initial activated projection;
3. queries or expands the projection when needed;
4. sees surface telemetry;
5. resolves projected addresses;
6. emits the final semantic-access plan.

This is the `B` interaction model from the design discussion, used as the second fresh inference call of the turn.

## 4. Initial deterministic activation

The runtime computes the initial working projection:

\[
W_t^{(0)} = A_{\text{cfg}}(M_\sigma, P_t, u_t)
\]

Activation is shaped by:

- primary focus;
- secondary focus;
- tertiary focus;
- background markers;
- active referents;
- active relations;
- constraints;
- the newest utterance;
- configured activation budgets.

The runtime applies configured defaults automatically.

The model does not need to spend inference selecting routine initial depths or deciding whether ordinary enabled discovery surfaces should fire.

## 5. Surface visibility across identifiers

Every admitted identifier must expose every enabled retrieval surface that is structurally capable of operating on its representation.

No identifier may be hidden from a surface by an undocumented hardcoded omission.

Examples:

- exact representations should be exact-searchable;
- textual identifier values should be lexical-searchable;
- embedded representations should be vector-searchable;
- canonical links and occurrences should be graph-navigable;
- temporal identifier or anchor relations should be temporally navigable.

A surface that cannot technically operate on a representation must declare that limitation explicitly in the projection.

The requirement is not that every engine performs the same operation.

The requirement is that all valid identifier-to-surface affordances are projected rather than discovered through whack-a-mole patches.

## 6. Positive activation only

Activation has positive semantics.

It means:

```text
this region is presently loaded and visible
```

It does not mean:

```text
all other regions are irrelevant
all other regions contain no evidence
```

Failure to reach a region under the current budget means only:

```text
not reached under this activation path and budget
```

It never authorizes a negative evidentiary conclusion.

## 7. Activated region contents

The activated working projection contains:

```text
activated nodes
typed edges
identifier assignments
incoming and outgoing occurrence summaries
available retrieval surfaces
surface telemetry
expansion handles
projection snapshot identity
configuration snapshot identity
```

### Semantic object view

An activated object should initially expose:

- canonical identity;
- object type identifiers;
- aliases;
- topology;
- available relations;
- contained-region and unit counts;
- incoming occurrence counts;
- outgoing occurrence counts;
- available retrieval surfaces;
- bounded representative neighbours.

### Semantic unit view

An activated unit should initially expose:

- canonical identity;
- parent object;
- heading or region address;
- inherited identifiers;
- unit-local identifiers;
- incoming and outgoing occurrence summaries;
- temporal-anchor summaries;
- short text preview;
- available retrieval surfaces.

Full semantic-unit prose is hydrated during execution.

The exact preview size remains a configuration decision.

## 8. Incoming and outgoing navigation

Projection access must support the navigation methods already implicit in canonical authored links:

```text
source → outgoing occurrence → target
target → incoming occurrence → source
```

This applies to object and unit targets.

No single direction is privileged globally.

The second inference call selects the useful direction for the current problem-space lens.

## 9. High-degree regions

A high-degree node is represented by:

```text
hub summary
counts by relation and target type
bounded first page of neighbours
surface-specific distributions
continuation handles
```

Example:

```text
Capital
    incoming journal occurrences: 300
    incoming book-note occurrences: 12
    outgoing lexicon occurrences: 8
```

The model may request a filtered continuation by:

- relation;
- direction;
- source path;
- object type;
- identifier;
- date range;
- retrieval surface.

The full neighbourhood is not required in one prompt.

## 10. Expansion

The second inference call may request targeted expansion:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma, W_t^{(k)}, q_k, \beta)
\]

Where:

- \(q_k\) is a typed expansion request;
- \(\beta\) is the configured hard budget.

Expansion may:

- follow incoming occurrences;
- follow outgoing occurrences;
- expand contained regions or units;
- query identifier assignments;
- request lexical candidates;
- request vector candidates;
- request exact matches;
- request graph neighbours;
- request temporal incidence;
- continue a high-degree result page.

Expansion is exploratory projection access.

It is not retrieval evidence admission.

## 11. Configured budgets

Configuration defines:

- initial object count;
- initial unit count;
- initial relation depth;
- hard maximum relation depth;
- per-surface candidate limits;
- continuation-page size;
- identifier expansion depth;
- temporal expansion range;
- total inference-session expansion budget.

Defaults fire deterministically.

The model may request targeted continuation within the hard ceiling.

It may not override the ceiling.

## 12. Telemetry

The initial telemetry set is:

```text
surface availability
exact or estimated candidate count
current depth
maximum depth
returned count
remaining expansion budget
truncation state
identifier/type distribution
temporal-anchor count
unresolved-target count
continuation availability
```

Telemetry is descriptive of projection access.

It is not a semantic relevance score.

Additional telemetry may be added when operational evidence demonstrates a need.

## 13. Frozen turn snapshot

The semantic projection is immutable for the duration of a turn.

Each turn binds to:

```text
projection_snapshot_id
ingest identity
schema version
logical hash
configuration snapshot
```

Ingest changes become visible on the next turn.

They never alter the projection halfway through:

```text
boundary inference
→ activation
→ semantic-access inference
→ conformance
→ execution
→ synthesis
```

## 14. Working projection versus full projection

The full projection remains authoritative:

\[
M_\sigma
\]

The activated projection is a bounded working view:

\[
W_t \subseteq M_\sigma
\]

The final plan must conform to \(M_\sigma\), even when it was constructed through \(W_t\).

An address resolved through an expansion becomes part of the visible working projection and may then appear in the final plan.

## 15. Visualization

A three-dimensional node graph is a useful representation of activation:

```text
inactive semantic space
activated regions
highlighted objects and units
incoming and outgoing paths
surface-specific depth
surface telemetry
```

Spatial coordinates are not automatically epistemic facts.

The authoritative runtime structure is:

```text
typed graph topology
+
activation state
+
surface telemetry
```

A UI may render that in two or three dimensions.

The visualization must not be reified into the semantic ontology unless coordinates are later given an explicit, admitted meaning.

## 16. Example — current book chronology lens

Problem-space state activates:

```text
Capital
Blood Meridian
book identifiers
reading context
comparative chronology
```

The initial deterministic pass exposes:

```text
canonical objects
incoming occurrence counts
temporal-anchor counts
lexical and vector neighbourhood summaries
available graph directions
```

The second inference call may request:

```text
incoming dated journal occurrences
```

The expansion returns bounded occurrence summaries and continuation handles.

The final plan then addresses the resolved occurrences, source units, canonical targets, and temporal anchors.

## 17. Example — lexical and vector access to identifiers

A focus on `vegan transition` may activate:

- exact identifier matches;
- lexical matches across unit text and admitted identifier values;
- vector neighbours across embedded unit and identifier representations;
- dated contextual units;
- temporal anchors.

No arbitrary rule may expose lexical access while hiding vector access to the same represented identifier without an explicit capability reason.

## 18. Acceptance conditions

Projection activation is acceptable when:

1. the full projection remains exhaustive within the admitted corpus;
2. each turn uses one frozen projection snapshot;
3. activation is shaped by the current problem space;
4. initial activation uses deterministic configured defaults;
5. the second inference call may perform targeted expansion;
6. all valid identifier-to-surface affordances are exposed;
7. incoming and outgoing navigation are both available where represented;
8. high-degree regions remain inspectable through summaries and continuation;
9. short previews support planning while full prose remains execution material;
10. activation is positive-only;
11. absence from the working projection cannot authorize an absence claim;
12. telemetry reports access conditions without becoming semantic judgment;
13. the final plan references resolved projected structure.
