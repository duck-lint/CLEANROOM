# Clean Implementation Sequence

## Principle

Do not begin by auditing or rewriting the current runtime.

First implement the clean kernel as its own object.

Existing code is consulted only after the new contracts and tests are accepted.

## Phase 0 — Preserve and isolate

1. Preserve the current accepted runtime under a tag or stable branch.
2. Create the clean-room branch from that exact commit.
3. Delete the tracked implementation on the clean branch.
4. Commit the clean state.
5. Add only the clean-room documents and Organon.
6. Freeze legacy development while the clean contracts are being derived.

Deliverable: a clean branch with repository history but no ambient implementation assumptions.

## Phase 1 — Freeze language-neutral contracts

Define data contracts without choosing implementation modules.

Required contracts:

- `ProblemSpaceState`
- `UtteranceBoundary`
- `SemanticSpaceProjection`
- `TraversalPlan`
- `ConformanceResult`
- `RetrievalResult`
- `ExecutionLimits`
- `SynthesisInput`
- `TurnResult`

Names may change before acceptance.

Each contract must state fields, authority, lifecycle, permitted transformations, forbidden transformations, persistence requirements, and deterministic identity.

Deliverable: human-readable specification, machine-readable schema, synthetic examples, and invalid examples.

## Phase 2 — Build a tiny synthetic semantic space

Before touching the real corpus, create a minimal projection containing:

- two book objects;
- two dated journal units;
- one canonical object link;
- one heading-target link;
- one person or animal object;
- one invalid identifier/object combination;
- exact, lexical, graph, and temporal capabilities.

The fixture should demonstrate top-down inheritance, bottom-up occurrence traversal, heading-specific unit addressability, contextual relation participation, and invalid structural paths.

Deliverable: deterministic `SemanticSpaceProjection` fixture and validation tests, with no LLM calls.

## Phase 3 — Implement problem-space state

Implement:

\[
B_t = D(P_{t-1}, u_t)
\]

and:

\[
P_t = U(P_{t-1}, B_t)
\]

Initially use a fake inference provider with deterministic responses.

Prove fresh-thread cleanliness, continuing-thread state, focus shifts, thread-local reference resolution, restart recovery, and cross-thread isolation.

Deliverable: persisted and replayable problem-space state.

## Phase 4 — Implement traversal inference contract

Implement:

\[
T_t = I(P_t, u_t, M)
\]

Start with a fake inference backend.

The traversal plan must reference typed addresses from the projection rather than ambiguous free-form strings wherever a canonical address exists.

Deliverable: traversal schema, fake-provider tests, and provider-neutral inference interface.

## Phase 5 — Implement structural conformance

Implement:

\[
C(T_t, M)
\]

as a pure deterministic function.

It should return a valid traversal or exact structural violations.

It must not rank evidence, infer aliases, judge paraphrases, create semantic roles, or repair meaning.

Prove invalid cases such as:

- Cleo used as a journal-date identifier;
- missing canonical object;
- unsupported heading target;
- unavailable surface;
- invalid relation direction;
- absent transition.

Deliverable: conformance test suite and optional one-repair interface back to inference.

## Phase 6 — Implement execution adapters

Implement:

\[
R_t = E(T_t, M)
\]

Start with in-memory fixture executors.

Then add one real surface at a time:

1. exact;
2. lexical;
3. vector;
4. graph;
5. temporal.

Every result must hydrate to canonical semantic units and retain full provenance.

No result may be filtered by a generated semantic proposition.

Deliverable: retrieval result contract, per-surface tests, and canonical hydration tests.

## Phase 7 — Implement packet assembly and coverage

Packet assembly may deduplicate by canonical identity, rank, cap, preserve breadth, attach provenance, and record execution limits.

Coverage derives only from measured execution.

Prove exhaustive negative authority, total-count readiness, graph-depth limits, unavailable-surface reporting, candidate-cap reporting, and absence of semantic veto after execution.

Deliverable: deterministic synthesis packet and execution-limit contract.

## Phase 8 — Implement synthesis interface

Implement:

\[
A_t = S(P_t, u_t, T_t, R_t, L_t)
\]

Start with a recording backend that proves the exact packet visible to synthesis.

Then add the real frontier provider.

Prove that the newest utterance is the focus, aggregate problem-space state is background, returned semantic units are visible, provenance and execution limits are visible, and no answer-bearing unit disappears.

Deliverable: provider-neutral synthesis interface and end-to-end synthetic tests.

## Phase 9 — Connect to the real semantic substrate

Only after the clean kernel passes the synthetic suite:

1. inspect the Organon-derived ingest representation;
2. build a read-only adapter from persisted corpus facts into `SemanticSpaceProjection`;
3. compare adapter output against projection requirements;
4. add missing object/unit/occurrence addressability;
5. do not import legacy runtime orchestration.

Deliverable: real-corpus projection adapter and validation report.

## Phase 10 — Evaluate legacy reuse

Now inspect the old repository.

For each component, classify it as:

- reusable unchanged behind an adapter;
- reusable after removal of excess authority;
- useful only as migration logic;
- useful only as test evidence;
- having no place in the clean kernel.

No component is preserved for familiarity or sunk cost.

## Phase 11 — Private UAT

Use a new suite identity for every fresh experimental baseline.

Inspect:

```text
problem-space state
→ traversal
→ conformance
→ retrieved canonical units
→ synthesis packet
→ answer
```

Timing is operational telemetry, not acceptance.

## Phase 12 — Replacement decision

Accept the clean runtime only when its conceptual contracts are stable, its synthetic suite passes, the real projection is exhaustive enough, private UAT confirms the architecture, and no legacy semantic veto has reappeared.

Then merge the clean branch as an intentional replacement while preserving repository history.
