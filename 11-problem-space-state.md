# Problem-Space State

## Status

Seed contract for review.

This document defines the thread-local problem space without assuming any existing runtime implementation.

## 1. Core proposition

Each conversation thread is a continuously morphing problem space.

The runtime does not treat the thread as:

- a flat transcript;
- one continually rewritten summary;
- a stack of independent topics;
- an ever-growing pile of semantically duplicate markers.

Instead, each turn contributes a typed boundary update. The runtime preserves those contributions and derives a bounded current view from them.

## 2. Two fresh inference calls per turn

Each turn contains two distinct inference calls.

### Inference call 1 — boundary deconstruction

\[
B_t = D(P_{t-1}, u_t)
\]

The first inference call receives:

- the prior problem-space state \(P_{t-1}\);
- the newest user utterance \(u_t\);
- the immediately preceding completed turn as continuity context.

It emits a turn-local boundary contribution \(B_t\).

This call is responsible for interpreting:

- the current focus;
- continued or newly introduced referents;
- relevant relations;
- temporal, comparative, exact, or exhaustive orientation;
- constraints and exclusions;
- unresolved references;
- focus changes;
- semantic consolidation;
- supersession or retirement.

It does not inspect the semantic-space projection and does not construct the final semantic-access plan.

### Deterministic fold

\[
P_t = U(P_{t-1}, B_t)
\]

The runtime deterministically applies the contribution to produce the updated problem-space state.

### Inference call 2 — semantic access

The second inference call receives the accepted \(P_t\), the newest utterance, and tool access to the projected semantic space.

It uses the problem space as a lens over the semantic projection and emits the final executable semantic-access plan.

The two calls are separate so that each inference operation can specialize:

```text
call 1
    interpret how the thread's problem space changed

call 2
    connect that problem space to the semantic space
```

## 3. Source artifacts, contributions, and active view

The thread state has three distinct layers.

### 3.1 Source transcript

The transcript preserves the actual user and assistant surface utterances.

It is immutable historical evidence of what occurred.

### 3.2 Boundary contributions

Each \(B_t\) is an append-only inference artifact linked to its source turn.

A contribution may add or modify:

- focus regions;
- referents;
- relations;
- constraints;
- unresolved references;
- evidentiary orientation;
- status transitions;
- consolidation instructions.

Historical contributions are never silently rewritten.

### 3.3 Derived active problem space

\(P_t\) is the current bounded operational view derived from the contribution history.

It is reconstructible from:

```text
source turns
+
accepted boundary contributions
+
deterministic fold rules
```

The active view is not treated as a new source of truth independent of that history.

## 4. Focus topology

The current problem space supports a bounded hierarchy:

```text
primary focus
secondary focus
tertiary focus
background aggregate
```

### Primary focus

The immediate problem being acted on.

### Secondary focus

A live adjacent problem region that materially informs the primary focus.

### Tertiary focus

A live but lower-priority branch that remains available for continuation.

### Background aggregate

Relevant continuity that no longer belongs in the active top three but remains useful to the thread.

This lets a conversation blossom without flattening every turn into one topic or keeping every past topic equally active.

## 5. Natural aggregation and deduplication

Semantically continuous contributions should aggregate rather than pile up.

Examples:

- a follow-on question about the same object;
- a reformulation of the same comparison;
- a new constraint on an existing problem;
- a reference to a relation already active;
- a semantically close continuation that narrows rather than creates a new region.

The first inference call may emit operations such as:

```text
add marker
update marker
merge into existing marker
move focus tier
supersede marker
retire marker
resolve reference
```

The runtime applies those operations deterministically.

The runtime does not independently infer semantic similarity.

Semantic consolidation remains inside the allowed inference site.

## 6. Bounded aggregation

Each focus tier and the background aggregate have configurable upper bounds.

The bounds exist to prevent indefinite accumulation.

When a limit is reached, the first inference call must explicitly choose among:

- merge semantically overlapping markers;
- consolidate a region into a more general boundary marker;
- demote a marker;
- supersede a marker;
- retire a marker;
- preserve it as unresolved.

The runtime must not silently drop a marker because a count was exceeded.

No numeric relevance score is required.

## 7. Marker lifecycle

A marker may occupy one of these states:

```text
primary
secondary
tertiary
background
unresolved
superseded
retired
```

These states integrate focus management and lifecycle management.

### Active states

- primary;
- secondary;
- tertiary;
- background;
- unresolved.

### Historical states

- superseded;
- retired.

No automatic numerical decay is used initially.

A marker changes state only through an explicit boundary contribution.

## 8. Unresolved references

Unresolved references remain attached to:

- their source turn;
- candidate problem regions;
- the expressions that require resolution.

They persist until:

- resolved;
- explicitly abandoned;
- superseded;
- or their containing problem region is retired.

The runtime does not resolve references by lexical guessing.

Resolution is performed by the first inference call using the prior problem-space state and recent conversational continuity.

## 9. Continuity presented to synthesis

Synthesis receives:

1. the updated problem-space state \(P_t\);
2. the newest user utterance \(u_t\) as the current focus;
3. the immediately preceding completed turn:
   - previous user utterance;
   - previous assistant answer;
4. the semantic-access plan;
5. the retrieval packet;
6. execution limits.

The immediately preceding turn is included so a referential utterance does not appear out of the blue.

It is labeled as conversational continuity, not retrieval evidence.

The full transcript remains persisted and auditable but is not automatically inserted into every synthesis call.

## 10. Seed data contract

```text
ProblemSpaceState
    thread_id
    version
    contribution_ids[]
    primary_focus
    secondary_focus
    tertiary_focus
    background_regions[]
    active_referents[]
    active_relations[]
    active_constraints[]
    unresolved_references[]
    superseded_markers[]
    retired_markers[]
    source_turn_range
```

```text
BoundaryContribution
    contribution_id
    source_turn_id
    source_utterance_id
    focus_operations[]
    referent_operations[]
    relation_operations[]
    constraint_operations[]
    reference_resolutions[]
    consolidation_operations[]
    lifecycle_operations[]
```

Exact field names remain provisional.

## 11. Deterministic fold invariant

The fold function may:

- validate referenced marker identities;
- apply declared adds, merges, moves, and retirements;
- enforce configured upper bounds;
- reject malformed state operations;
- preserve history.

It may not:

- infer whether two concepts are semantically equivalent;
- generate a replacement summary;
- resolve ambiguous language;
- decide which topic matters.

Those are responsibilities of inference call 1.

## 12. Branching

Conversation branching behavior is outside the initial kernel contract.

The kernel requires only:

- each runtime thread has one isolated problem-space state;
- a fresh thread begins clean;
- a continuing thread evolves only from its own history.

Product behavior for cloning state at a UI branch point may be specified later.

## 13. Example

### Turn 1

```text
What did the calf eat?
```

Boundary contribution:

```text
primary focus:
    calf diet

active referent:
    calf

active relation:
    consumed food
```

### Turn 2

```text
When did that change?
```

Boundary contribution:

```text
primary focus:
    temporal transition in calf diet

continued referent:
    calf

resolved reference:
    "that" → calf diet

prior focus transition:
    calf diet → secondary/background support
```

The second turn does not create a duplicate independent marker for `calf diet`.

It updates and redirects the existing problem region.

## 14. Acceptance conditions

The problem-space contract is acceptable when:

1. contributions are append-only;
2. the active view is reconstructible;
3. semantically continuous turns can merge rather than pile up;
4. focus tiers remain bounded;
5. no numerical confidence system is required;
6. no automatic decay silently removes context;
7. unresolved references remain explicit;
8. synthesis receives the current utterance, \(P_t\), and the immediately preceding turn;
9. separate threads cannot share state;
10. no deterministic component performs semantic consolidation without an inference-issued operation.
