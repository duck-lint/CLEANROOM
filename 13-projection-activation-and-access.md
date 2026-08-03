# Projection Activation and Access

## Status

Seed contract for review.

This document defines how the full semantic projection becomes a bounded, positive, expandable working view for the second inference call.

## 1. Core proposition

The full semantic space is exhaustive within the admitted corpus.

Per-turn access is constrained by the current relational problem space.

The problem space acts as a lens over the semantic space, activating a bounded region from which semantic units can be collected.

The lens does not declare the rest of the semantic space irrelevant.

## 2. Full projection

Let the frozen semantic projection be:

\[
M_\sigma = (V,E,I,S)
\]

Where:

- \(V\) contains canonical semantic objects, units, regions, occurrences, and anchors;
- \(E\) contains typed connections among them;
- \(I\) contains identifier assignments and inheritance;
- \(S\) contains retrieval-surface capabilities and telemetry contracts;
- \(\sigma\) identifies the immutable snapshot used for the turn.

## 3. Problem-space lens

The problem space is:

\[
P_t =
(
\mathcal{G}_t,
\mathcal{E}_t,
\mathcal{C}_t,
\mathcal{O}_t,
\mathcal{H}_t,
\Lambda_t
)
\]

Projection activation may be shaped by:

- active problem regions \(\mathcal{G}_t\);
- relations among them \(\mathcal{E}_t\);
- active constraints \(\mathcal{C}_t\);
- open tensions \(\mathcal{O}_t\);
- persistence and contribution history \(\mathcal{H}_t\);
- current attention lens \(\Lambda_t\);
- newest utterance \(u_t\).

Primary, secondary, tertiary, and background are activation bands in \(\Lambda_t\).

They are not separate topic stores.

## 4. Two inference calls

### Call 1

\[
B_t = D(P_{t-1},u_t,v_{t-1})
\]

\[
P_t = U(P_{t-1},B_t)
\]

The first call interprets how the problem gestalt changes.

### Call 2

The second call is one tool-using inference session.

It:

1. receives \(P_t\) and \(u_t\);
2. observes the initial activated projection;
3. queries or expands the projection when needed;
4. sees surface telemetry;
5. resolves projected addresses;
6. emits the final semantic-access plan.

## 5. Initial deterministic activation

\[
W_t^{(0)}
=
A_{\mathrm{cfg}}(M_\sigma,P_t,u_t,\Lambda_t)
\]

The runtime applies configured defaults automatically.

The model does not spend inference selecting routine initial depths or deciding whether ordinary enabled discovery surfaces should fire.

The initial activation should retain why each node or edge became visible.

## 6. Activation provenance

Every activated record should be traceable to one or more of:

```text
problem region
problem-space relation
active constraint
open tension
attention band
newest utterance
configured default
expansion request
```

Example:

```text
Capital
    activated_by:
        region: Capital/Blood Meridian chronology
        referent binding: Capital
        attention band: primary
```

Activation provenance is navigational explanation.

It is not a relevance score.

## 7. Surface visibility across identifiers

Every admitted identifier must expose every enabled retrieval surface structurally capable of operating on its representation.

No identifier may be hidden from a surface by an undocumented hardcoded omission.

Examples:

- exact representations should be exact-searchable;
- textual identifier values should be lexical-searchable;
- embedded representations should be vector-searchable;
- canonical links and occurrences should be graph-navigable;
- temporal identifiers and anchors should be temporally navigable.

A technical limitation must be declared explicitly in the projection.

All valid identifier-to-surface affordances must be projected rather than added through case-by-case patches.

## 8. Positive activation only

Activation means:

```text
this region is presently loaded and visible
```

It does not mean:

```text
all other regions are irrelevant
all other regions contain no evidence
```

Failure to reach a region under the current path and budget means only:

```text
not reached under this lens and budget
```

It authorizes no negative evidentiary conclusion.

## 9. Attention bands

The attention lens may assign problem regions to:

```text
primary activation
secondary activation
tertiary activation
background activation
```

These bands may influence:

- initial breadth;
- ordering of projection summaries;
- which continuation handles are foregrounded;
- how much descriptive preview is initially loaded.

They must not:

- alter semantic identity;
- erase background relations;
- produce a truth score;
- become an evidence-admission threshold.

## 10. Activated region contents

The activated working projection contains:

```text
activated nodes
typed edges
identifier assignments
incoming and outgoing occurrence summaries
available retrieval surfaces
surface telemetry
expansion handles
activation provenance
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
- bounded representative neighbours;
- activation provenance.

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
- available retrieval surfaces;
- activation provenance.

Full semantic-unit prose is hydrated during execution.

Preview size remains configurable.

## 11. Incoming and outgoing navigation

Projection access supports:

```text
source → outgoing occurrence → target
target → incoming occurrence → source
```

This applies to object and unit targets.

No direction is globally privileged.

The second inference call chooses the useful direction for the current problem-space lens.

## 12. High-degree regions

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

## 13. Expansion

The second inference call may request targeted expansion:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma,W_t^{(k)},q_k,\beta)
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

Expansion may be motivated by:

- an unresolved problem-space referent;
- an open tension;
- a missing structural binding;
- a comparison requiring parallel paths;
- incomplete surface telemetry.

Expansion is exploratory projection access.

It is not retrieval evidence admission.

## 14. Configured budgets

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

## 15. Telemetry

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
activation provenance
```

Telemetry describes projection access.

It is not:

- a semantic relevance score;
- a problem-space coherence score;
- a confidence value.

## 16. Frozen turn snapshot

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
→ problem-space fold
→ activation
→ semantic-access inference
→ conformance
→ execution
→ synthesis
```

## 17. Working projection versus full projection

The full projection remains authoritative:

\[
M_\sigma
\]

The activated projection is a bounded working view:

\[
W_t \subseteq M_\sigma
\]

The final plan must conform to \(M_\sigma\), even when constructed through \(W_t\).

An address resolved through expansion becomes part of the visible working projection and may appear in the final plan.

## 18. Open tensions and activation

An open tension may positively activate multiple candidate regions.

Example:

```text
open tension:
    "before" may mean reading chronology or publication chronology
```

The activated projection may expose both:

```text
dated reading occurrences
publication metadata
```

Activation does not decide which interpretation is correct.

The second inference call may preserve both routes in the plan.

## 19. Persistence without scoring

A recurrent or reinforced problem region may remain visible across turns because its history and current attention lens preserve it.

No numerical persistence score is required.

History may record:

```text
introduced
reinforced
recurrent
reframed
superseded
retired
```

These labels describe thread history.

They do not change corpus truth.

## 20. Visualization

A three-dimensional node graph is a useful representation of activation:

```text
semantic-space nodes and edges
activated regions
attention bands
incoming and outgoing paths
surface-specific depth
surface telemetry
problem-space activation provenance
```

Spatial coordinates are not automatically epistemic facts.

The authoritative runtime structure is:

```text
typed graph topology
+
activation state
+
surface telemetry
+
problem-space provenance
```

A UI may render that in two or three dimensions.

The visualization must not be reified into the semantic ontology unless coordinates later receive explicit admitted meaning.

## 21. Example — book chronology lens

Problem-space state:

```text
region:
    Capital/Blood Meridian chronology

referents:
    Capital
    Blood Meridian

constraint:
    compare temporal relation

attention:
    primary
```

Initial activation exposes:

```text
canonical objects
incoming and outgoing occurrence counts
temporal-anchor counts
lexical and vector neighbourhood summaries
available graph directions
activation provenance
```

An open chronology-dimension tension may activate publication metadata and dated reading occurrences in parallel.

The second inference call chooses or preserves the represented routes.

## 22. Example — vegan transition

A `vegan transition` problem region may activate:

- exact identifier matches;
- lexical matches across unit text and admitted identifier values;
- vector neighbours across embedded unit and identifier representations;
- dated contextual units;
- temporal anchors.

No arbitrary rule may expose lexical access while hiding vector access to the same represented identifier without an explicit capability reason.

## 23. Acceptance conditions

Projection activation is acceptable when:

1. the full projection remains exhaustive within the admitted corpus;
2. each turn uses one frozen projection snapshot;
3. activation is shaped by the relational problem space and attention lens;
4. focus bands are views over one state;
5. every activation has inspectable provenance;
6. initial activation uses deterministic configured defaults;
7. the second inference call may perform targeted expansion;
8. all valid identifier-to-surface affordances are exposed;
9. incoming and outgoing navigation are both available;
10. high-degree regions remain inspectable through summaries and continuation;
11. short previews support planning while full prose remains execution material;
12. activation is positive-only;
13. absence from the working projection cannot authorize an absence claim;
14. open tensions may activate multiple candidate routes without forced resolution;
15. telemetry never becomes semantic judgment or coherence scoring;
16. the final plan references resolved projected structure.
