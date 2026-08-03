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

Deliverable:

- a clean branch with repository history but no ambient implementation assumptions.

## Phase 1 — Freeze language-neutral contracts

Define data contracts without choosing implementation modules.

Required seed contracts:

- `ProblemSpaceState`
- `ProblemRegion`
- `ProblemRelation`
- `OpenTension`
- `AttentionLens`
- `BoundaryContribution`
- `SemanticSpaceProjection`
- `ActivatedProjection`
- `SemanticAccessPlan`
- `ConformanceResult`
- `RetrievalResult`
- `ExecutionLimits`
- `SynthesisInput`
- `TurnResult`

Names may change before acceptance.

Each contract must state:

- fields;
- authority;
- lifecycle;
- permitted transformations;
- forbidden transformations;
- persistence requirements;
- deterministic identity.

Deliverable:

- human-readable specification;
- machine-readable schema;
- synthetic examples;
- invalid examples.

## Phase 2 — Build a tiny synthetic semantic space

Before touching the real corpus, create a minimal projection containing:

- two book objects;
- two dated journal units;
- one canonical object link;
- one heading-target link;
- one person or animal object;
- one invalid identifier/object combination;
- exact, lexical, graph, vector, and temporal capabilities.

The fixture should demonstrate:

- top-down inheritance;
- bottom-up occurrence traversal;
- incoming and outgoing navigation;
- heading-specific unit addressability;
- contextual relation participation;
- invalid structural paths.

Deliverable:

- deterministic `SemanticSpaceProjection` fixture;
- validation tests;
- no LLM calls.

## Phase 3 — Implement boundary inference and problem-space state

Implement:

\[
B_t = D(P_{t-1},u_t,v_{t-1})
\]

and:

\[
P_t = U(P_{t-1},B_t)
\]

Initially use a fake boundary-inference provider with deterministic responses.

Represent the problem space as a relational gestalt containing:

- regions;
- relations;
- constraints;
- open tensions;
- contribution history;
- attention lens.

Prove:

- fresh-thread cleanliness;
- continuing-thread state;
- explicit perturbation operations;
- semantic continuation without duplicate accumulation;
- merge, split, redirect, supersede, and retire behavior;
- qualitative persistence without numerical confidence;
- explicit unresolved tensions;
- restart recovery;
- cross-thread isolation.

The deterministic fold must apply declared transformations without inferring semantic similarity.

Deliverable:

- persisted, replayable, reconstructible problem-space state.

## Phase 4 — Implement projection activation

Implement:

\[
W_t^{(0)}
=
A_{\mathrm{cfg}}(M_\sigma,P_t,u_t,\Lambda_t)
\]

Start with deterministic activation against the synthetic projection.

Prove:

- activation is positive-only;
- primary, secondary, tertiary, and background are activation bands over one state;
- every activated node records which problem region, relation, constraint, or open tension exposed it;
- all structurally valid identifier-to-surface affordances are available;
- high-degree regions use summaries and continuation handles;
- configured defaults apply without model micromanagement;
- absence from the working projection authorizes no negative claim.

Deliverable:

- bounded `ActivatedProjection` contract and deterministic activation tests.

## Phase 5 — Implement semantic-access inference

Implement:

\[
T_t = I_2(P_t,u_t,W_t)
\]

Use a separate fake provider from the boundary-inference provider.

Allow the second inference session to request typed expansion:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma,W_t^{(k)},q_k,\beta)
\]

The final plan must reference projected canonical addresses wherever they exist.

Prove:

- problem regions bind to canonical semantic addresses;
- open tensions become explicit access objectives;
- plans may branch and rejoin;
- incoming and outgoing directions remain available;
- routine bounds come from configuration;
- required and optional execution obligations remain distinct;
- the plan does not resolve the problem or judge evidence.

Deliverable:

- `SemanticAccessPlan` schema;
- fake-provider tests;
- provider-neutral semantic-access interface.

## Phase 6 — Implement structural conformance

Implement:

\[
C(T_t,M_\sigma)
\]

as a pure deterministic function.

It returns:

- a valid plan;
- or exact structural violations.

It must not:

- rank evidence;
- infer aliases;
- judge paraphrases;
- generate semantic roles;
- calculate problem-space coherence;
- repair meaning.

Prove invalid cases such as:

- Cleo used as a journal-date identifier;
- missing canonical object;
- unsupported heading target;
- unavailable surface;
- invalid relation direction;
- absent transition;
- requested bound beyond configuration.

Deliverable:

- conformance suite;
- one-repair interface back to a fresh semantic-access inference call.

## Phase 7 — Implement execution adapters

Implement:

\[
R_t = E(T_t,M_\sigma)
\]

Start with in-memory fixture executors.

Then add one real surface at a time:

1. exact;
2. lexical;
3. vector;
4. graph;
5. temporal.

Every result must hydrate to canonical semantic units and retain full provenance.

No result may be filtered by:

- a generated semantic proposition;
- a paraphrase comparison;
- a problem-space coherence test.

Deliverable:

- retrieval result contract;
- per-surface tests;
- canonical hydration tests.

## Phase 8 — Implement packet assembly and coverage

Packet assembly may:

- deduplicate canonical identity;
- rank by declared mechanical rules;
- apply configured bounds;
- preserve breadth;
- attach provenance;
- record execution limits.

Coverage derives only from measured execution.

Prove:

- exact exhaustive negative authority;
- total-count readiness;
- graph-depth limits;
- unavailable-surface reporting;
- candidate-cap reporting;
- absence of semantic veto after execution.

Deliverable:

- deterministic synthesis packet;
- execution-limit contract.

## Phase 9 — Implement synthesis interface

Implement:

\[
A_t
=
S(P_t,u_t,v_{t-1},T_t,R_t,L_t)
\]

Start with a recording backend that proves the exact packet visible to synthesis.

Then add the real frontier provider.

Prove:

- the newest utterance remains the focus;
- the immediately preceding turn supplies local continuity;
- the relational problem space supplies bounded background;
- previous conversation text is not mislabeled as retrieval evidence;
- returned semantic units are visible;
- provenance and execution limits are visible;
- open tensions remain distinguishable from corpus absence;
- no answer-bearing unit disappears.

Deliverable:

- provider-neutral synthesis interface;
- recorded input fixtures;
- end-to-end synthetic tests.

## Phase 10 — Connect to the real semantic substrate

Only after the clean kernel passes the synthetic suite:

1. inspect the Organon-derived ingest representation;
2. build a read-only adapter from persisted corpus facts into `SemanticSpaceProjection`;
3. compare adapter output against projection requirements;
4. add missing object/unit/occurrence addressability;
5. do not import legacy runtime orchestration.

Deliverable:

- real-corpus projection adapter;
- projection validation report.

## Phase 11 — Evaluate legacy reuse

Now inspect the old repository.

For each component, classify it as:

- reusable unchanged behind an adapter;
- reusable after removal of excess authority;
- useful only as migration logic;
- useful only as test evidence;
- having no place in the clean kernel.

No component is preserved for familiarity or sunk cost.

## Phase 12 — Private UAT

Use a new suite identity for every fresh experimental baseline.

Inspect:

```text
prior problem space
→ boundary perturbation
→ updated relational gestalt
→ current attention lens
→ activated semantic projection
→ semantic-access plan
→ conformance
→ retrieved canonical units
→ synthesis packet
→ answer
```

Test at least:

- semantic continuation without duplicate accumulation;
- explicit correction and supersession;
- recurrent unresolved tension;
- multi-turn referent resolution;
- projection expansion;
- positive-only activation;
- exact count and exact absence;
- multi-object chronology;
- thread isolation;
- restart recovery.

Timing is operational telemetry, not acceptance.

## Phase 13 — Replacement decision

Accept the clean runtime only when:

- its conceptual contracts are stable;
- its synthetic suite passes;
- the real projection is sufficiently exhaustive;
- private UAT confirms the architecture;
- no legacy semantic veto or coherence gate has reappeared.

Then merge the clean branch as an intentional replacement while preserving repository history.
