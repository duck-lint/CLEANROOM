# Clean Implementation Sequence

## Principle

Recover substrate authority without restarting the project.

The Python runtime has already demonstrated whole-corpus feasibility. The clean-room task is to preserve and improve that demonstrated capability without importing accidental Python architecture into the new kernel.

The sequence is therefore:

```text
preserve completed clean-room work
→ expose complete authored and materialized corpus facts
→ reconcile candidate contracts against those facts
→ construct and validate the real projection
→ revalidate existing corpus-sensitive implementation
→ continue the clean runtime
```

Existing clean-room work remains valuable. Its corpus-sensitive portions are provisional until they survive complete-vault contact.

## Authority rule

Use this order throughout implementation:

1. authored vault and Organon;
2. observed whole-corpus behavior and private UAT;
3. semantic distinctions that survive corpus contact;
4. candidate clean-room contracts;
5. Python implementation boundaries;
6. generic engineering conventions.

The Python runtime is admissible evidence, not architectural authority.

No implementation is justified solely by prose coherence. No synthetic fixture substitutes for whole-corpus grounding. No compatibility mechanism is added without a concrete consumer.

## Phase 0 — Preserve the current repositories and recovery baseline

1. Preserve `duck-lint/semantic-traversal` in its existing repository and history as the demonstrated Python system.
2. Preserve `duck-lint/CLEANROOM` as an independent repository.
3. Record the exact `CLEANROOM` base, open branches, and candidate implementations at the start of recovery.
4. Do not delete, rewrite, or restart the accepted contract, fold, fixture, or activation work.
5. Freeze new corpus-sensitive implementation until the real projection boundary is established.

Deliverable:

- a recorded recovery baseline with no loss of completed work.

## Phase 1 — Correct authority and lifecycle declarations

Update the clean-room documents so that:

- whole-corpus feasibility is recorded as demonstrated;
- the immediate question is faithful reconstruction and improvement;
- the authored vault and Organon outrank candidate contracts;
- Python capabilities, failures, corpus observations, and private UAT are admissible evidence;
- synthetic fixtures are regression surfaces rather than architectural substitutes;
- corpus-sensitive contracts are provisional until validated against the complete vault.

Classify existing artifacts as:

```text
accepted and corpus-independent
provisional pending corpus contact
validated against the complete vault
superseded by higher-authority evidence
```

The deterministic problem-space fold is primarily thread-state machinery and may remain accepted unless corpus contact exposes a relevant contradiction.

Projection, unit-materialization, occurrence, retrieval-surface, and activation assumptions remain provisional.

Deliverable:

- an explicit substrate-first recovery posture without code changes.

## Phase 2 — Define a versioned whole-corpus observation exchange

Create a read-only exchange generated locally from the authored vault and the demonstrated Python substrate.

The exchange must keep three evidence layers distinguishable:

```text
authored source observations
Python materialization observations
runtime behavior and private-UAT observations
```

The authored layer should expose, as available:

- source identity and UUID;
- path and filename;
- admitted frontmatter and field provenance;
- heading hierarchy;
- authored Markdown blocks;
- wikilinks, aliases, embeds, heading targets, and block targets;
- source hashes and snapshot identity.

The Python-materialization layer should expose, as available:

- notes and canonicalized chunks;
- section and ordinal structure;
- admitted frontmatter serialization;
- graph nodes, authored occurrences, and incidence;
- temporal anchors and provenance;
- retrieval-surface availability and bounds;
- inventory identity, validation status, and counts.

The behavior layer should expose repository-safe or private-local records of:

- capabilities demonstrated;
- observed failures;
- execution limits;
- private-UAT obligations and outcomes.

Private corpus content remains local and ignored. Public code may define the exporter, schemas, validation, and redacted aggregate reporting.

The exchange must not silently convert Python records into clean-room authority.

Deliverable:

- a versioned, deterministic, read-only observation bundle and local generation procedure.

## Phase 3 — Reconcile candidate contracts against corpus evidence

For each material clean-room contract field or distinction, record one status:

```text
supported
supported_with_revision
contradicted
not_observed
open_authoring_decision
```

Each judgment must retain:

- the clean-room claim being evaluated;
- authored-vault evidence;
- Python materialization or behavior evidence;
- the bridge rule from evidence to judgment;
- preserved distinctions;
- known breaks or losses;
- required contract revision, if any.

Interpret the statuses strictly:

- `supported` means supported within the admitted corpus and evidence scope;
- `supported_with_revision` requires an explicit contract correction;
- `contradicted` blocks dependent implementation;
- `not_observed` neither validates nor invalidates the claim;
- `open_authoring_decision` requires operator authority rather than runtime invention.

Priority review areas include:

- object identity versus discovery surfaces;
- heading-region identity;
- authored semantic-unit boundaries;
- empty-body objects;
- intrinsic identifiers versus contextual participation;
- frontmatter and body occurrence provenance;
- heading and block target resolution;
- reverse incidence;
- temporal-anchor sourcing;
- identifier applicability and semantic roles;
- identifier-to-surface affordances;
- Python chunk splits versus clean-room transport segments;
- hydration and continuation assumptions.

Deliverable:

- a complete contract-contact matrix and bounded amendment list.

## Phase 4 — Amend only the contradicted or unsupported contracts

Revise candidate contracts only where Phase 3 supplies evidence.

Each amendment must identify:

- the higher-authority evidence;
- the exact prior assumption;
- the smallest required correction;
- affected schemas and tests;
- migration implications for existing candidate code;
- unresolved questions left open.

Do not add abstractions merely because they appear prudent.

Do not add fallbacks, compatibility paths, or generalized extension points without a concrete consumer.

Deliverable:

- corpus-grounded contracts and schemas.

## Phase 5 — Build the read-only real projection adapter

Define a versioned adapter:

```text
WholeCorpusObservationBundle
    → SemanticSpaceProjection
```

The adapter may:

- map authored UUIDs to canonical object identities;
- materialize accepted heading regions and semantic units;
- attach admitted identifiers with source provenance;
- map authored links and field relations to occurrence records;
- derive reverse incidence from authoritative occurrences;
- map materially sourced temporal facts to temporal anchors;
- expose validated retrieval-surface capabilities;
- calculate deterministic snapshot identity and logical hash.

The adapter may not:

- infer semantic equivalence;
- guess unresolved targets;
- assign corpus-specific semantic roles without accepted authority;
- treat folder placement as the sole object type;
- flatten contextual participation into intrinsic typing;
- silently promote Python chunks to authored semantic units;
- fabricate missing block mappings;
- fill required fields with invented defaults;
- import legacy runtime orchestration.

Unmappable required structure produces a typed projection-construction violation.

Deliverable:

- a read-only adapter and typed construction-failure surface.

## Phase 6 — Validate the complete-vault projection

Validate at least:

1. every admitted canonical object is represented or explicitly rejected;
2. UUID identity remains distinct from path, filename, title, and aliases;
3. empty-body objects remain addressable;
4. every region belongs to one object;
5. every unit belongs to one object and one region;
6. every object and region resolves to its contained units;
7. every identifier assignment has an applicable descriptor and provenance;
8. inherited and local assignments remain distinguishable;
9. contextual relations remain occurrences rather than intrinsic target properties;
10. every resolved occurrence has canonical source and target;
11. every target exposes reverse incidence;
12. heading targets resolve to regions;
13. block targets resolve to units;
14. ambiguity and unresolved targets remain explicit;
15. every temporal anchor identifies its authored source;
16. every retrieval surface declares inspected components, returned identity, bounds, and coverage semantics;
17. every valid identifier-to-surface affordance is represented;
18. transport segmentation preserves one parent semantic-unit identity;
19. prompt-size bounds discard no mandatory projected structure;
20. the projection is deterministic for equivalent input.

The report records counts, hashes, statuses, and structural failures without requiring private prose in the repository.

Deliverable:

- a whole-corpus projection validation report.

## Phase 7 — Revalidate completed clean-room implementation

Retain the tiny synthetic fixture for fast deterministic regression tests.

Then run existing corpus-sensitive implementation against the validated real projection.

### Problem-space fold

Confirm that no substrate evidence contradicts its authority boundary. Do not reopen it merely because the recovery sequence changed.

### Projection activation

Test the candidate activation implementation against whole-corpus conditions including:

- large object and unit counts;
- empty-body objects;
- high-degree nodes;
- heterogeneous identifier shapes;
- deep heading trees;
- duplicate titles with distinct UUIDs;
- unresolved links;
- object-field and body occurrences sharing targets;
- large reverse-incidence sets;
- hydration-address units;
- zero-result surfaces;
- bound-limited candidate bundles;
- every actual surface and match-mode combination.

Every correction must cite the observed corpus or execution failure requiring it.

Deliverable:

- corpus-grounded acceptance, amendment, or rejection of the existing candidate activation implementation.

## Phase 8 — Implement semantic-access inference

Implement:

\[
T_t = I_2(P_t,u_t,W_t)
\]

Use a separate provider from boundary inference.

Allow typed expansion within configured limits:

\[
W_t^{(k+1)}
=
\operatorname{expand}(M_\sigma,W_t^{(k)},q_k,\beta)
\]

Prove against the validated projection that:

- problem regions bind to canonical semantic addresses;
- open tensions remain explicit access objectives;
- plans may branch and rejoin;
- incoming and outgoing directions remain available;
- routine bounds come from configuration;
- required and optional obligations remain distinct;
- the plan neither resolves the problem nor judges evidence.

Deliverable:

- provider-neutral semantic-access inference over the complete projected substrate.

## Phase 9 — Implement structural conformance

Implement:

\[
C(T_t,M_\sigma)
\]

as a pure deterministic function.

It returns a conforming plan or exact structural violations.

It must not rank evidence, infer aliases, judge paraphrases, generate semantic roles, calculate problem-space coherence, or repair meaning.

Validate failures against actual projected possibilities as well as synthetic invalid cases.

Deliverable:

- conformance and bounded repair interfaces grounded in the real projection.

## Phase 10 — Implement execution adapters

Implement:

\[
R_t = E(T_t,M_\sigma)
\]

Add one real surface at a time:

1. exact;
2. lexical;
3. vector;
4. graph;
5. temporal.

Reuse a Python artifact only when a bounded compatibility review shows that it supplies a clean external substrate or operator responsibility without excess authority.

Every result hydrates to canonical semantic units and retains full provenance.

No result is filtered by a generated proposition, paraphrase comparison, or problem-space coherence test.

Deliverable:

- real execution adapters and per-surface whole-corpus tests.

## Phase 11 — Implement packet assembly, coverage, and synthesis

Packet assembly may deduplicate canonical identity, apply declared mechanical ordering and bounds, preserve breadth, attach provenance, and record execution limits.

Coverage derives only from measured execution.

Synthesis receives:

\[
A_t
=
S(P_t,u_t,v_{t-1},T_t,R_t,L_t)
\]

Prove:

- exact exhaustive negative authority;
- total-count readiness;
- graph-depth and candidate-cap reporting;
- unavailable-surface reporting;
- no semantic veto after execution;
- newest-utterance focus;
- previous-turn continuity labeled separately from corpus evidence;
- visible provenance and execution limits;
- no answer-bearing unit removed for a semantic reason.

Deliverable:

- deterministic packet construction and provider-neutral synthesis.

## Phase 12 — Private UAT and behavioral fidelity

Use a new suite identity for every fresh baseline.

Compare CLEANROOM with demonstrated Python behavior at the level of:

```text
requested distinction
canonical bindings
access paths
returned unit identities
coverage authority
answer-supporting evidence
failure classification
```

Do not require identical plans, internal names, or answer wording.

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

Deliverable:

- private whole-corpus evidence that the reconstruction preserves and improves demonstrated capability.

## Phase 13 — Compatibility and integration decision

For each Python artifact, classify it as:

- valid external substrate producer;
- reusable unchanged behind a versioned boundary;
- reusable only after removal of excess authority;
- migration logic only;
- observational evidence only;
- no place in the clean kernel.

Accept the clean runtime only when:

- its contracts survive complete-vault contact;
- the real projection is sufficiently exhaustive;
- synthetic and whole-corpus suites pass;
- private UAT confirms behavioral fidelity;
- no legacy semantic veto or coherence gate has reappeared.

Then choose an explicit integration strategy:

- replace the old runtime package with a released CLEANROOM runtime;
- consume CLEANROOM as a versioned dependency;
- expose CLEANROOM through a stable service boundary;
- archive the legacy runtime after migration.

Repository ancestry need not be merged for the architecture to replace the old runtime.
