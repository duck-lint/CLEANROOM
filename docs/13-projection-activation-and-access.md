# Projection Activation and Access

## Status

Accepted representation closure for initial activation.

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
Marx, Karl — Capital
    canonical identity: source-material UUID
    activated_by:
        region: Capital/Blood Meridian source-material chronology
        referent exposure: Capital
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
Marx, Karl — Capital
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
    Capital/Blood Meridian source-material chronology

referents:
    Marx, Karl — Capital
    McCarthy, Cormac — Blood Meridian

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

## 14. Accepted PR 4A representation closure

Status: accepted representation closure for initial deterministic activation.

PR 4A closes the serializable contracts around:

```text
W_t^(0) = A_cfg(M_sigma, P_t, u_t, Lambda_t)
```

It does not implement `A_cfg`. The frozen semantic projection remains structural corpus authority. The problem space shapes bounded positive exposure but cannot create corpus structure. Visibility is not relevance, omission is not corpus absence, and activation provenance is not confidence, truth, relevance, or evidence admission.

### Candidate exposure versus canonical binding

Activation exposes projected candidates. It does not bind problem-space referents, problem regions, open tensions, or candidate expressions to canonical semantic addresses. `ProblemReferent` provenance means only that a referent expression exposed a projected candidate under the current surface, configuration, and bounds. It does not mean `problem referent == canonical semantic address`.

Canonical problem-region/address binding remains semantic-access inference work in conceptual Phase 5. No activation representation may introduce `referent_binding`, `canonical_binding`, `problem_region_binding`, `ReferentBinding`, `CanonicalBinding`, `ResolvedProblemRegion`, or a problem-space-to-corpus mapping record.

### Activation input identity

The activation input includes an explicit `ActivationUtterance` containing `utterance_id` and complete conversational `text`. The text is input to deterministic activation, not corpus evidence.

`ActivatedProjection` records the projection snapshot, configuration snapshot, problem-space thread id, exact problem-space version, and newest utterance id. It deliberately does not copy the complete problem-space state or utterance text.

### Layered configuration and surface capability

`ProjectionActivationConfig` is separate from the activated view. It has five configuration groups: unbanded, primary, secondary, tertiary, and background. Unbanded covers newest-utterance, whole-space-constraint, and configured-default seeds. Each group has textual-seed, structural-neighbour, visible-unit, and preview-text bounds. Per-surface limits are declared for every available projected surface and every activation band.

Future PR 4B validation must enforce that configuration and projection snapshot configuration identities match; exactly one surface configuration exists for each available projection surface; unavailable, unknown, and duplicate surface configurations are invalid; each configured candidate limit is within the corresponding hard surface limit; and all available structurally capable configured surfaces participate automatically. No identifier-to-surface affordance may be omitted by hardcoded exception.

Zero total bounds, zero band bounds, and zero surface candidate limits are valid mechanical bounds. A zero candidate limit yields no candidates for that surface and band but makes no negative corpus claim. `maximum_initial_relation_depth == 0` permits no structural expansion beyond directly exposed records. `continuation_page_limit == 0` suppresses continuation handles. No configuration value is a relevance score.

### Explicit activated records and dual provenance

The activated view has separate typed vectors for objects, regions, units, identifier assignments, authored occurrences, temporal anchors, edges, telemetry, and continuation handles. Identifier-assignment and temporal-anchor records carry both `record_provenance` and `activation_provenance`.

`record_provenance` says where the projected fact came from in the frozen projection. `activation_provenance` says why it became visible now. These are separate axes and must not be collapsed.

### Referent and tension-candidate exposure

Activation provenance includes `ProblemReferent` and `OpenTensionCandidate`. `ProblemReferent` records the containing problem-region identity and thread-local referent identity. `OpenTensionCandidate` records the thread-local tension identity and zero-based candidate index in the preserved candidate vector. Candidate exposure does not select the candidate or resolve the tension.

### Textual and structural activation sources

Accepted textual source families are: newest utterance text; operational problem-region referent expressions; active constraint expressions; open-tension unresolved expressions; and each open-tension candidate binding in declared vector order.

Accepted structural source families are: active problem-space relations; operational region topology; attention-band membership; and configured defaults. Problem-space relation labels or reasons must not be converted into invented natural-language search text. Relations guide structural exposure and attach `ProblemRelation` provenance.

Future deterministic seed-group order is: newest utterance; active whole-problem-space constraints; primary problem regions; secondary problem regions; tertiary problem regions; background problem regions; configured defaults. Within each attention band, preserve region order from the corresponding `AttentionLens` vector, referent order from each region, active regional-constraint order from `ProblemSpaceState.constraints`, open-tension order from `ProblemSpaceState.open_tensions`, unresolved expression before candidate bindings, candidate-binding vector order, and active problem-relation order from `ProblemSpaceState.relations`.

Later seeds may be mechanically omitted when a configured seed bound is reached. Such omission means only `not activated under this configured bound`; it does not mean irrelevant, absent, false, or evidentially empty.

### First-seen canonical deduplication and provenance aggregation

Future PR 4B output vectors use deterministic first-seen order. Available surfaces execute in `SemanticSpaceProjection.retrieval_surfaces` vector order. Surface candidates preserve each surface's deterministic returned order. Structural records preserve frozen projection vector order.

Canonical object, region, unit, assignment, occurrence, anchor, and edge identities appear at most once in their respective activated vectors. When a canonical record is exposed through several paths, retain its first-seen position and append all unique activation-provenance entries in first-seen order. Do not merge records by title, alias, text, similarity, or inferred equivalence. Aliases never become duplicate canonical objects. Attention bands affect breadth and ordering but never semantic identity or truth. No numeric relevance or attention score is introduced.

### Richer bounded summaries

Activated object records expose title, aliases, object class, bounded visible region addresses, bounded visible unit ids, visible identifier assignment ids, full contained counts, occurrence counts, available surfaces, and activation provenance. Aliases are discovery surfaces, not canonical identity. Object class is projected typing, not a generated ontology.

Activated region records expose heading path, heading identity, visible inherited identifier assignment ids, bounded visible unit ids, full contained count, surfaces, and activation provenance. Activated unit records expose authored block type, heading path, inherited and unit-local identifier assignment ids, text preview, truncation flag, incidence counts, temporal-anchor count, surfaces, and activation provenance.

Previews are planning material, not retrieved evidence. Full authored prose remains execution material. A truncated preview authorizes no claim about omitted text. Contained-record vectors are bounded previews, while count fields describe the full frozen projection record. Visible identifier, occurrence, and anchor records remain separate typed records.

### Self-describing continuation handles

A continuation handle is serializable and restart-safe. It requires no hidden runtime registry, contains no evidence, and performs no expansion. Its offset has meaning only with the named frozen projection snapshot, activation configuration snapshot, problem-space thread, problem-space version, newest utterance, access mechanism, filters, and ordering. The projection snapshot, activation configuration snapshot, problem-space thread, problem-space version, and newest utterance identity must match the active continuation context exactly. A stale, cross-thread, cross-version, cross-utterance, cross-projection, or cross-configuration handle is a future typed violation.

Continuation origin is typed as a text probe, structural neighbourhood, or temporal probe. Continuation access is typed separately: direct projection-structure continuation follows frozen represented topology without inventing a retrieval surface, while retrieval-surface continuation resumes one concrete projected surface. Text and temporal probes require `ContinuationAccess::RetrievalSurface`. Structural neighbourhoods may continue through `ContinuationAccess::ProjectionStructure` or through a declared retrieval surface; the latter is valid only when the frozen projection declares that concrete surface and structural transition relationship. PR 4A records these combinations only and does not validate them.

Filters are typed as transition, source path prefix, object class, identifier, or temporal range. Identifier filters preserve the exact projected `IdentifierValue` union and must not stringify integer, boolean, semantic-address, or collection values. The filter representation does not decide scalar-versus-collection membership semantics, semantic equivalence, value normalization, relevance, or usefulness.

`ContinuationOrdering::ProjectionVectorOrder` is valid for direct projection-structure continuation. `ContinuationOrdering::SurfaceDeclared` is valid for retrieval-surface continuation. Other origin, access, and ordering combinations are subject to Phase 4B typed validation. No ordering field is a semantic rank or relevance score. Expansion execution belongs to Phase 5. Real retrieval execution belongs to Phase 7.

### Typed activation violations

`ProjectionActivationViolation` is the closed Phase 4B error vocabulary. It covers empty required identities, projection validation status, configuration snapshot mismatch, missing/unknown/unavailable/duplicate surface configuration, invalid configuration values, candidate limits exceeding hard limits, duplicate activated identities, invalid activated references, invalid activation provenance, invalid continuation handles, invalid telemetry, activated-view bound overflow, and count overflow. PR 4A defines the vocabulary only; it does not implement validation behavior, `Display`, or `Error`.

### Exact PR boundary

```text
PR 4A
    contracts, schemas, representation tests

PR 4B
    deterministic activation implementation
    scripted surface access
    telemetry
    hub summaries
    candidate exposure
    no semantic binding
```
