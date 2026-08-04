# Problem-Space State

## Status

Seed contract for review.

This document defines the thread-local problem space without assuming any existing runtime implementation.

## 1. Core proposition

Each conversation thread is a continuously morphing problem space.

More precisely:

> The problem space is a bounded relational gestalt that preserves continuity by reorganizing under each new utterance.

It is not:

- a flat transcript;
- one continually rewritten summary;
- a stack of unrelated topics;
- an ever-growing pile of semantically duplicate markers;
- a numeric vector of confidence or relevance scores.

## 2. Three distinct thread artifacts

The thread maintains three different forms of state.

### 2.1 Source transcript

The transcript preserves the actual user and assistant surface utterances.

It is immutable historical evidence of what occurred.

### 2.2 Boundary contributions

Each turn produces an append-only boundary contribution linked to its source utterance.

A contribution describes how the newest utterance perturbs the prior problem space.

### 2.3 Derived active problem space

The current problem space is the bounded relational state derived from:

```text
source turns
+
accepted boundary contributions
+
deterministic fold rules
```

It is reconstructible.

It is not an independent source of truth detached from its history.

## 3. Two fresh inference calls per turn

### Call 1 — boundary inference

\[
B_t = D(P_{t-1},u_t,v_{t-1})
\]

The first inference call receives:

- prior problem-space state \(P_{t-1}\);
- newest utterance \(u_t\);
- immediately preceding completed turn \(v_{t-1}\).

It emits a turn-local boundary contribution \(B_t\).

This call interprets:

- what problem region is currently in focus;
- what regions continue from prior turns;
- what referents remain active;
- what relations changed;
- what constraints were introduced;
- what ambiguity or contradiction remains open;
- what prior framing was corrected;
- how attention should be redirected;
- whether a contribution reinforces, extends, merges, splits, supersedes, or retires existing structure.

It does not inspect the semantic-space projection and does not construct the semantic-access plan.

### Deterministic fold

\[
P_t = U(P_{t-1},B_t)
\]

The runtime validates and applies the declared transformation.

### Call 2 — semantic-access inference

The second inference call receives:

- accepted \(P_t\);
- newest utterance \(u_t\);
- an activated view of the projected semantic space.

It connects the problem gestalt to represented semantic addresses and emits the final executable semantic-access plan.

## 4. Conceptual state structure

The problem space may be represented abstractly as:

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

Where:

### \(\mathcal{G}_t\) — problem regions

Individuated relational gestalts currently belonging to the thread's operational problem space.

A region is not merely a topic label.

It may contain:

- anchor referents;
- active distinctions;
- relations presently in question;
- local constraints;
- unresolved tensions;
- source contributions;
- lifecycle state.

### \(\mathcal{E}_t\) — problem-space relations

Connections among regions, such as:

- continuation;
- dependency;
- comparison;
- correction;
- refinement;
- causal question;
- temporal relation;
- shared referent;
- shared constraint.

### \(\mathcal{C}_t\) — active constraints

Examples:

- exact phrase required;
- temporal ordering requested;
- compare these objects;
- exclude a prior interpretation;
- answer from corpus evidence;
- distinguish publication from reading chronology.

### \(\mathcal{O}_t\) — open tensions

Examples:

- unresolved reference;
- contradiction;
- missing comparison dimension;
- competing framing;
- recurrent unresolved question;
- required connection not yet established.

### \(\mathcal{H}_t\) — history and persistence

Records:

- source contributions;
- prior transformations;
- reinforcement;
- recurrence;
- reframing;
- supersession;
- retirement.

### \(\Lambda_t\) — attention lens

Represents current activation over the relational problem space.

The tuple is conceptual.

The implementation may normalize these records across tables or types.

## 5. Boundary contribution as perturbation

For conceptual emphasis:

\[
B_t \equiv \Delta_t
\]

The boundary contribution may issue operations such as:

```text
create region
preserve region
reinforce region
extend region
merge regions
split region
connect regions
disconnect relation
add constraint
replace constraint
open tension
resolve tension
redirect attention
supersede framing
retire region
```

Every semantic transformation originates in boundary inference.

The deterministic fold applies it.

## 6. Coherence

A problem region is coherent insofar as it preserves an identifiable relational structure while incorporating new contributions.

Coherence does not mean remaining unchanged.

Examples of coherent transformation:

```text
calf diet
→ temporal change in calf diet
```

```text
book chronology
→ corrected from reading chronology
→ publication chronology
```

```text
implementation concern
→ recurring concern
→ refined constraint
```

A correction may preserve referents while replacing the relation or comparison dimension.

A contradiction may be represented as an open tension rather than erased.

## 7. Coherence non-goals

Problem-space coherence is not initially:

- a numeric score;
- a confidence probability;
- an embedding similarity threshold;
- an automatic decay rate;
- a truth measure;
- a retrieval ranking signal;
- an evidence-admission criterion.

The state history itself records whether a region was reinforced, recurrent, reframed, superseded, or retired.

## 8. Attention lens

The attention lens exposes:

```text
primary activation
secondary activation
tertiary activation
background activation
```

These are not four containers.

They are activation bands over the same relational state.

### Primary activation

The immediate problem region being acted on.

### Secondary activation

A live adjacent region that materially informs the primary region.

### Tertiary activation

A lower-priority connected region that remains available for continuation.

### Background activation

Relevant continuity that remains structurally part of the problem gestalt without occupying the current foreground.

A region retains one identity while moving among bands.

## 9. Natural aggregation and deduplication

Semantically continuous contributions aggregate rather than pile up.

A follow-on utterance may:

- reinforce an existing region;
- extend it with a relation;
- redirect its focus;
- replace one constraint;
- connect it to another region;
- create a new source occurrence in its history.

The first inference call determines whether this is continuity.

The runtime does not perform semantic merging from vector distance or lexical similarity alone.

Deduplication preserves:

- recurrence count or source history;
- corrections;
- relation changes;
- unresolved tension.

It does not flatten all repetition into one anonymous summary.

## 10. Stable and reorganizing structure

### Persisting structure

Examples:

- canonical referents;
- explicit user corrections;
- active distinctions;
- recurring constraints;
- unresolved tensions;
- relations reinforced across turns.

### Reorganizing structure

Examples:

- transient wording;
- discarded framing;
- completed local question;
- redundant paraphrase;
- stale activation;
- explicitly corrected interpretation.

The boundary contribution states what is preserved and what is released.

## 11. Lifecycle

A problem region may be:

```text
active
background
unresolved
superseded
retired
```

Activation band and lifecycle are related but distinct.

Examples:

- a background region remains active in the gestalt;
- a superseded region remains in history but is no longer operational;
- a retired region is no longer part of the current problem gestalt;
- an unresolved region may be primary, secondary, tertiary, or background.

No automatic numerical decay is used initially.

## 12. Open tensions

Open tensions remain attached to:

- their containing region;
- source turn;
- relevant referents;
- candidate interpretations;
- the missing or contradictory relation.

They persist until:

- resolved;
- superseded;
- explicitly abandoned;
- or their containing region is retired.

A problem-space gap means:

```text
the current problem representation lacks a required connection
```

It does not mean:

```text
the corpus contains no answer
```

## 13. Recurrence

A problem may recur across turns without being duplicated.

The state should preserve:

```text
one region identity
multiple source contributions
recurrent status
still-open tension
```

Recurrence is useful structural history.

It must not be erased merely because duplicate topical labels were merged.

## 14. Boundedness

The problem space has configurable upper bounds on:

- active regions;
- relations;
- open tensions;
- background regions;
- retained source excerpts;
- contribution material included in model context.

When a bound is approached, boundary inference must explicitly choose among:

- merge;
- consolidate;
- demote;
- supersede;
- retire;
- preserve as unresolved.

The runtime must not silently drop a semantic region because a count was exceeded.

## 15. Continuity presented to synthesis

Synthesis receives:

1. updated \(P_t\);
2. newest utterance \(u_t\) as current focus;
3. immediately preceding completed turn \(v_{t-1}\);
4. semantic-access plan;
5. retrieval packet;
6. execution limits.

The previous turn prevents a referential continuation from appearing out of the blue.

It is labeled as conversational continuity, not corpus evidence.

The full transcript remains persisted and auditable but is not automatically inserted into every synthesis request.

## 16. Seed data contract

```text
ProblemSpaceState
    thread_id
    version
    region_ids[]
    relation_ids[]
    constraint_ids[]
    open_tension_ids[]
    contribution_ids[]
    attention_lens
    source_turn_range
```

```text
ProblemRegion
    region_id
    anchor_referents[]
    relation_ids[]
    local_constraint_ids[]
    open_tension_ids[]
    source_contribution_ids[]
    persistence_state
    activation_band
    supersedes_region_id?
```

```text
ProblemRelation
    relation_id
    source_region_id
    relation_type
    target_region_id?
    source_contribution_id
    lifecycle
```

```text
OpenTension
    tension_id
    region_id
    tension_type
    unresolved_expression?
    candidate_bindings[]
    source_turn_id
    lifecycle
```

```text
AttentionLens
    primary_region_ids[]
    secondary_region_ids[]
    tertiary_region_ids[]
    background_region_ids[]
```

```text
BoundaryContribution
    contribution_id
    source_turn_id
    source_utterance_id
    region_operations[]
    relation_operations[]
    constraint_operations[]
    tension_operations[]
    attention_operations[]
    preservation_declarations[]
    release_declarations[]
```

Exact names remain provisional.

## 17. Deterministic fold authority

The fold may:

- validate referenced identities;
- apply declared operations;
- enforce schema;
- enforce configured bounds;
- reject malformed transformations;
- preserve history;
- rebuild the active view.

It may not:

- infer semantic equivalence;
- decide which region matters;
- resolve ambiguous language;
- generate a replacement summary;
- calculate a coherence score;
- close an open tension on its own.

## 18. Branching

Conversation-branch behavior remains outside the initial kernel contract.

The kernel requires only:

- each runtime thread has isolated problem-space state;
- a fresh thread begins clean;
- a continuing thread evolves only from its own history.

## Accepted deterministic-fold contract closure

This section closes the representation and authority boundaries required before
the deterministic fold is implemented. It is normative for the future fold,
but it does not define an executor.

### Constraint applicability and derived incidence

Every constraint canonically declares one of two applicability forms:

- `WholeProblemSpace` applies to all operational regions. It never appears in a
  region's `local_constraint_ids`.
- `Regions { region_ids }` explicitly targets one or several regions. List
  order carries no precedence or priority.

Active, background, and unresolved region persistence states are operational.
Superseded and retired states are not. An active regional constraint may target
only operational regions. Duplicate, empty, or unresolved regional target sets
are invalid fold input.

`ProblemConstraint.applicability` is the canonical source.
`ProblemRegion.local_constraint_ids` is only a derived active
regional-incidence index. The future fold rebuilds it so that an active regional
constraint appears in every operational region it explicitly targets. A shared
regional constraint remains one canonical record. Whole-problem-space,
superseded, and retired constraints are absent from every regional index.

Superseding or retiring a region does not automatically transfer, narrow, or
retire its constraints. Boundary inference must explicitly replace or retire
affected constraints. A replacement constraint declares its complete
applicability; the fold never inherits applicability by convenience. Historical
superseded or retired constraints retain their authored applicability for audit.

### Accepted contribution log and replay metadata

`BoundaryContributionLog` belongs to exactly one thread and holds ordered
`AcceptedBoundaryContribution` entries. The source transcript is a separate
artifact: the log contains no transcript copy, timestamp, storage path, provider
metadata, or state snapshot. `ProblemSpaceState.contribution_history` remains a
compact derived audit summary and is not the replay source.

A fresh thread begins with `ProblemSpaceState.version == 0` and an empty accepted
log. The first accepted entry has sequence `1`. Sequence is contiguous and
unique, but vector order itself is the authoritative replay order. Each entry's
`prior_state_version` is the state version before application. Every successful
fold increments the version exactly once. A failed contribution is not appended
and does not increment the version. Within one thread, a contribution ID,
source-turn ID, or source-utterance ID cannot be accepted twice.

### Fixed fold phase order

The future fold executes these phases exactly:

```text
0. Preflight envelope and declared-identity uniqueness
1. Region operations
2. Relation operations
3. Constraint operations
4. Tension operations
5. Attention operations
6. Preservation/release declaration validation
7. Rebuild derived incidence indexes and the attention lens
8. Validate final referential and lifecycle closure
9. Enforce configured bounds
10. Atomically commit state, history, accepted log entry, and version increment
```

Operations within each category vector execute in declared vector order. The
fold does not sort, semantically consolidate, or reinterpret operations. Newly
declared regions may be referenced by later phases. Intermediate working-copy
incompleteness does not authorize a partial commit. Preservation and release
declarations are audit declarations, not a second mutation mechanism.
Contradictory terminal operations are rejected rather than semantically
reconciled. Excess over a configured bound is rejected rather than resolved by
silent removal.

### Atomicity

```text
valid complete contribution
→ new state
  + contribution-history update
  + accepted-log entry
  + exactly one state-version increment

any violation
→ unchanged prior state
  + no history mutation
  + no accepted-log entry
  + no state-version increment
```

No partial state may become observable.

### Attention orthogonality

Attention is a view over one state. Activation changes neither identity,
constraint applicability, lifecycle, nor semantic strength. In a valid future
folded state, each operational region occupies exactly one attention band, and
`ProblemRegion.activation_band` agrees with `AttentionLens` after rebuilding.
An unresolved region may occupy any attention band; an active region may occupy
background activation. No numeric attention, persistence, confidence, decay,
or coherence score is introduced.

### Implemented deterministic realization

Conceptual PR 3B realizes the accepted fold contract as a pure API:

- callers pass either no prior state (for an empty accepted log) or the exact
  prior derived state, plus explicit `ProblemSpaceFoldLimits`;
- zero is a valid limit and excess rejects the complete contribution;
- a closed, typed violation surface distinguishes envelope, identity,
  lifecycle, declaration, closure, bound, and overflow failures;
- fresh threads have no public version-zero state: the empty log supplies the
  thread identity and the first accepted fold constructs version one;
- active regional incidence and the single attention lens are rebuilt from
  authoritative top-level vector order after every contribution;
- accepted logs replay from no state, in vector order, through the same fold
  mechanics, including after a Serde serialize/deserialize restart boundary.

The realization adds no storage adapter and no production boundary-inference
provider. Boundary inference remains outside the deterministic authority
boundary and is represented only by a scripted test fixture. The accepted
Serde unit-variant exception is unchanged.

## 19. Examples

### 19.1 Calf continuation

Turn 1:

```text
What did the calf eat?
```

State:

```text
region:
    calf diet

referent:
    calf

relation:
    consumed food
```

Turn 2:

```text
When did that change?
```

Boundary contribution:

```text
preserve:
    calf
    calf diet

redirect:
    primary relation → temporal transition

resolve tension:
    "that" → calf diet

attention:
    temporal change → primary
    prior diet state → supporting relation
```

One region evolves.

No duplicate `calf diet` marker is created.

### 19.2 Correction

Turn 1:

```text
Which book did I start first?
```

Turn 2:

```text
Sorry, I meant which was published first.
```

Boundary contribution:

```text
preserve:
    compared books
    comparison structure

supersede:
    reading chronology

replace constraint:
    publication chronology
```

### 19.3 Unresolved dimension

Input:

```text
Was Capital before Blood Meridian?
```

Possible state:

```text
region:
    Capital/Blood Meridian chronology

open tension:
    "before" dimension unresolved
    candidates:
        publication
        reading
```

No false resolution is generated.

## 20. Acceptance conditions

The problem-space contract is acceptable when:

1. the state is relational rather than list-like;
2. boundary contributions are append-only perturbation records;
3. the active view is reconstructible;
4. semantic continuation can transform an existing region;
5. recurrence is preserved without duplicate accumulation;
6. attention bands remain views over one state;
7. no numerical confidence or coherence score is required;
8. no automatic decay silently removes context;
9. open tensions remain explicit;
10. synthesis receives \(P_t\), the newest utterance, and the previous turn;
11. separate threads cannot share state;
12. deterministic folding performs no semantic interpretation;
13. problem-space coherence cannot become a post-retrieval evidence gate.
