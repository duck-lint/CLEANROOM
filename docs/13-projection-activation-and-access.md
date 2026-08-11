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
- \(S\) contains all five canonical retrieval-surface capabilities and
  telemetry contracts;
- \(\sigma\) identifies the immutable snapshot used for the turn.

The five canonical surfaces are structural dimensions of the complete
projection even when executable providers or indexes are not yet present:

```text
exact, lexical, vector, graph, temporal
```

Surface-family existence is distinct from record-level applicability, corpus
relevance, executable provider/index existence, and runtime invocation. An
individual object, region, unit, identifier, occurrence, or anchor may expose
only the structurally applicable subset; a surface or operation may return
zero results; and configuration cannot create, delete, enable, or disable a
canonical surface in \(M_\sigma\).

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

The model does not spend inference selecting routine initial depths or deciding
whether ordinary structurally represented discovery surfaces should fire for a
particular access operation.

The initial activation should retain why each node or edge became visible.

Configuration acts on the already-complete \(M_\sigma\); it does not constitute
or reshape \(M_\sigma\). Initial breadth, relation depth, candidate limits,
continuation limits, and other budgets govern bounded exposure and traversal
only. A runtime configuration change therefore does not by itself create a
new semantic projection snapshot.

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

Every admitted identifier must expose every canonical retrieval surface
represented in the projection that is structurally capable of operating on its
representation. Runtime policy may choose which of those surfaces to invoke
for a particular operation, but it may not remove a surface from \(M_\sigma\).

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

The two identities are carried separately: the projection snapshot identifies
the immutable semantic object, while the configuration snapshot identifies the
later bounded activation policy. The latter is not part of projection
identity.

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
ingest_identity
schema_version
logical_hash
corpus_snapshot_identity
validation_status
```

and, independently, to one runtime configuration snapshot for the bounded
activation/access operations of that turn. This six-field projection binding
is distinct from that runtime configuration binding.

The runtime configuration binding is separate:

```text
projection snapshot identity
+ runtime configuration snapshot identity
```

The configuration governs \(A_{cfg}(M_\sigma, ... )\) as a bounded view/access
operation over the immutable projection; it does not constitute \(M_\sigma\).

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

`ProjectionActivationConfig` is separate from the activated view. It has five configuration groups: unbanded, primary, secondary, tertiary, and background. Unbanded covers newest-utterance, whole-space-constraint, and configured-default seeds. Each group has textual-seed, structural-neighbour, visible-unit, and preview-text bounds. Per-surface limits may be declared for each canonical surface and activation band that the operation invokes.

Future PR 4B validation must enforce that the runtime configuration identity used by
the operation matches the configuration identity carried by its activation,
plan, or continuation context. It must reject unknown or duplicate surface
configuration names and reject missing runtime configuration only when the
accepted operation requires configuration for that canonical surface. A
configured candidate limit must remain within a hard maximum owned by the
runtime configuration, if one is defined; no routine projection hard surface
limit is consulted. A concrete provider/access failure is a runtime access
failure, not removal of the structural surface from \(M_\sigma\). No
identifier-to-surface affordance may be omitted by hardcoded exception.

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

Activated region records expose heading path, heading identity, visible inherited identifier assignment ids, bounded visible unit ids, full contained count, surfaces, and activation provenance. Activated unit records expose authored block type, heading path, inherited and unit-local identifier assignment ids, typed text preview, incidence counts, temporal-anchor count, surfaces, and activation provenance.

Previews are planning material, not retrieved evidence. Full authored prose remains execution material. A truncated preview authorizes no claim about omitted text. Contained-record vectors are bounded previews, while count fields describe the full frozen projection record. Visible identifier, occurrence, and anchor records remain separate typed records.

### Self-describing continuation handles

A continuation handle is serializable and restart-safe. It requires no hidden runtime registry, contains no evidence, and performs no expansion. Its offset has meaning only with the named frozen projection snapshot, activation configuration snapshot, problem-space thread, problem-space version, newest utterance, access mechanism, filters, and ordering. The projection snapshot and runtime configuration snapshot are separate identities and must each match the active continuation context exactly. A stale, cross-thread, cross-version, cross-utterance, cross-projection, or cross-configuration handle is a future typed violation. No provider/index materialization identity is added until Phase 7 concrete access implementation supplies evidence that one is required.

Continuation origin is typed as a text probe, structural neighbourhood, or temporal probe. Continuation access is typed separately: direct projection-structure continuation follows frozen represented topology without inventing a retrieval surface, while retrieval-surface continuation resumes one concrete projected surface. Text and temporal probes require `ContinuationAccess::RetrievalSurface`. Structural neighbourhoods may continue through `ContinuationAccess::ProjectionStructure` or through a declared retrieval surface; the latter is valid only when the frozen projection declares that concrete surface and structural transition relationship. PR 4A records these combinations only and does not validate them.

Filters are typed as transition, source path prefix, object class, identifier, or temporal range. Identifier filters preserve the exact projected `IdentifierValue` union and must not stringify integer, boolean, semantic-address, or collection values. The filter representation does not decide scalar-versus-collection membership semantics, semantic equivalence, value normalization, relevance, or usefulness.

`ContinuationOrdering::ProjectionVectorOrder` is valid for direct projection-structure continuation. `ContinuationOrdering::SurfaceDeclared` is valid for retrieval-surface continuation. Other origin, access, and ordering combinations are subject to Phase 4B typed validation. No ordering field is a semantic rank or relevance score. Expansion execution belongs to Phase 5. Real retrieval execution belongs to Phase 7.

### Typed activation violations

`ProjectionActivationViolation` is the closed Phase 4B error vocabulary. It covers empty required identities, projection validation status, runtime configuration-context mismatch, missing/unknown/duplicate required surface configuration, invalid configuration values, candidate limits exceeding an applicable runtime maximum, concrete surface-access failure, duplicate activated identities, invalid activated references, invalid activation provenance, invalid continuation handles, invalid telemetry, activated-view bound overflow, and count overflow. A required surface-access failure means that the concrete runtime probe could not execute; it does not imply that the canonical surface is absent from \(M_\sigma\). PR 4A defines the vocabulary only; it does not implement validation behavior, `Display`, or `Error`.

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

## 15. Accepted PR 4B deterministic runtime closure

Status: accepted runtime closure to be implemented after the contract amendment.

This amendment records representation and deterministic-runtime rules only. It does not implement `A_cfg(M_sigma, P_t, u_t, Lambda_t)`, activation-access adapters, retrieval, hydration, semantic binding, expansion execution, or partial-success activation.

### Activation-access seam

PR 4B will introduce a synchronous, read-only production trait approximately named `ProjectionActivationAccess`. Its request and result records are runtime-only and must not be exported as JSON schemas.

The trait receives typed deterministic probe requests, returns canonical projected addresses, returns measured candidate counts, preserves deterministic result order, reports continuation facts, and may return a typed access failure. It never returns hydrated semantic-unit prose, relevance, confidence, truth, evidence admission, or problem-space binding.

The test fixture supplies a deterministic scripted implementation. The contract-amendment PR adds no async behavior and no real exact, lexical, vector, graph, or temporal adapters.

### Typed probe dispatch

Future PR 4B dispatches compatible sources as follows:

```text
textual seed
    → Literal
    → Terms
    → NearestNeighbours

canonical projected address
    → Incidence

temporal subject or temporal anchor
    → Temporal
```

Only modes declared by the concrete `RetrievalSurfaceDescriptor` may fire. Preserve seed-group order, seed order within each group, projection retrieval-surface vector order, descriptor match-mode vector order, and surface-returned candidate order. Do not sort modes alphabetically or by enum variant.

`SurfaceMatchMode::Declared { name }` is valid only when the activation-access implementation explicitly supports that exact declared mode. Otherwise activation fails through `SurfaceAccessFailed`. A projection-declared mode must not be silently omitted.

### Probe and candidate limit accounting

Apply `maximum_textual_seeds` before surface fan-out. One accepted textual seed may therefore produce several probe invocations across configured surfaces and compatible modes.

Each configured surface-band candidate limit applies separately to one probe × one surface × one match mode. It is not shared among all modes, all seeds, or the whole attention band. Total activated-view limits remain global across the final view.

A zero candidate limit suppresses returned candidates for that invocation but still permits telemetry describing the bounded invocation. It makes no negative corpus claim.

### Text-preview construction

`SemanticUnitContent::Inline.normalized_text` supplies the activated text preview. Activation never uses `authored_markdown` as a second competing preview source.

The preview bound counts Unicode scalar values, not bytes. Preserve the first configured number of Unicode scalar values. `truncated` is true exactly when additional scalar values were omitted. A zero preview limit produces an empty inline preview and `truncated == true` when normalized text was non-empty. Genuinely empty normalized text produces an empty inline preview and `truncated == false`.

`SemanticUnitContent::HydrationAddress` produces `ActivatedTextPreview::UnavailableWithoutHydration`. Activation never dereferences the hydration address and does not copy the address into the activated preview. The semantic unit's canonical identity is sufficient for later typed hydration planning. An unavailable preview is not evidence absence and is not a negative corpus claim.

### Per-probe telemetry identity

A future `ProjectionTelemetry` record represents exactly one probe × one concrete surface × one declared match mode. It must not aggregate unrelated referents, constraints, tensions, utterance probes, or candidate expressions into one record.

`activation_provenance` explains the problem-space or utterance source that caused the probe. `probe_id` is mechanical runtime identity. It is not a semantic binding, relevance score, or evidence identity.

Future deterministic probe IDs use first-seen invocation order:

```text
activation-probe:0
activation-probe:1
activation-probe:2
```

Telemetry IDs similarly use:

```text
activation-telemetry:0
activation-telemetry:1
activation-telemetry:2
...
```

Neither ID is derived from text hashes, relevance values, or provider output.

### Expansion-budget recording

Initial `ProjectionTelemetry.remaining_expansion_budget` equals `ProjectionActivationConfig.maximum_expansion_budget`. Phase 4 initial activation does not decrement this value. The budget is later consumed only by typed Phase 5 expansion.

A zero expansion budget is valid and means no later expansion is available. It does not limit the initial activation pass beyond the separate initial activation bounds. This amendment adds no expansion request or execution contracts.

### Candidate-context closure

A directly exposed candidate must enter the activated view with enough represented context to avoid dangling visible references.

Required upward closure:

```text
activated unit
    → parent region
    → parent object

activated region
    → parent object

activated identifier assignment
    → represented subject

activated temporal anchor
    → represented subject

activated occurrence
    → represented source
    → represented target
```

“Represented source” means the source object for an object-field occurrence, or the source semantic unit, its parent region, and its parent object for a semantic-unit occurrence.

Downward context remains bounded: object region and unit previews obey view bounds; region unit previews obey the band bound; identifier, occurrence, and temporal-anchor vectors obey their total bounds; structural neighbours obey relation-depth and neighbour bounds.

Any identity listed in a visible preview vector must have a corresponding activated record in the same `ActivatedProjection`. Do not emit dangling visible IDs.

Candidate bundles are atomic for view-bound accounting:

```text
if required upward closure cannot fit
    omit the whole new candidate bundle
    do not partially insert it
    record mechanical truncation or continuation facts
```

Previously activated shared context remains present. Omission caused by a bound means only `not activated under this configured bound`.

### Hub summaries

Do not add a new hub ontology or separate `HubSummary` exchange record. A high-degree summary is represented collectively by the activated canonical record, full mechanically known counts, bounded visible neighbours, per-probe surface telemetry, truncation state, and continuation handles.

Degree means count of unique direct represented edge tuples before activated-view truncation. The unique edge tuple is `(source, transition_id, direction, target)`. The hub threshold is mechanical and not a semantic-importance score.

### Activated-edge identity

Deduplicate visible edges only by the exact tuple `(source, transition_id, direction, target)`. Preserve first-seen order. Aggregate unique activation provenance in first-seen order.

Assign deterministic visible edge IDs by first insertion:

```text
activated-edge:0
activated-edge:1
activated-edge:2
...
```

Do not derive activated-edge identity from title, aliases, text, similarity, or inferred relation equivalence.

### Configured defaults

Configured defaults are deterministic policy, not hidden semantic seed content. There is no configuration-owned list of preferred corpus objects, titles, paths, entities, or addresses. The final configured-default pass does not introduce unrelated new root candidates.

`ActivationProvenance::ConfiguredDefault` is appended when an already-motivated activation path exposes additional structure because a deterministic default policy fired, including exactly these stable configuration keys in PR 4B:

```text
automatic_surface_fan_out
bounded_structural_context
high_degree_summary
```

For example, a referent expression motivates a candidate; automatic invocation of all compatible configured surfaces adds `ConfiguredDefault { configuration_key: "automatic_surface_fan_out" }` alongside the referent provenance. Required parent records or bounded child previews add `ConfiguredDefault { configuration_key: "bounded_structural_context" }`. Telemetry and continuation records emitted for a high-degree address add `ConfiguredDefault { configuration_key: "high_degree_summary" }`.

Configured-default provenance does not replace the originating utterance, referent, constraint, tension, relation, region, or attention provenance. Configured defaults must not inject semantically preferred content.

### Atomic surface-access failure and activation failure

`SurfaceAccessFailed` covers scripted access returning a failure, a declared mode unavailable from the configured activation-access implementation, malformed deterministic surface output, and failure to inspect one required configured available surface.

It does not mean no corpus result exists, the probe is irrelevant, the source expression was wrong, or the problem-space interpretation failed.

Future PR 4B activation is atomic:

```text
required surface-access failure
    → Err(SurfaceAccessFailed)
    → no ActivatedProjection returned
```

Future PR 4B returns either:

```text
Ok(complete bounded ActivatedProjection)
```

or:

```text
Err(ProjectionActivationViolation)
```

No partially accepted working view is returned. Failures do not mutate the projection, problem-space state, utterance, configuration, or scripted-access fixture.
