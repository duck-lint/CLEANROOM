# Clean-Room Protocol

## Purpose

This protocol prevents the new runtime from being shaped by accidental attempts to preserve the Python implementation while also preventing clean-room prose from outranking the authored substrate.

The risks are symmetrical:

- existing nouns can force the new architecture to contain objects and authorities it does not need;
- internally coherent clean-room abstractions can become insulated from the complete vault they claim to represent.

Clean-room discipline therefore means authority hygiene, not corpus avoidance.

## 1. Authority order

Use this order:

1. authored vault and Organon;
2. observed whole-corpus behavior and private UAT;
3. semantic distinctions that survive corpus contact;
4. candidate clean-room contracts;
5. Python implementation boundaries;
6. generic engineering conventions.

The Python runtime is not architectural authority.

Its demonstrated capabilities, failures, corpus observations, execution traces, and private-UAT results are admissible evidence.

A candidate contract remains revisable when higher-authority evidence contradicts it.

## 2. Clean context

The kernel repository remains independent and must not receive ambient legacy orchestration, implementation plans, diagnostics, component diagrams, or old tests as untyped design context.

The project may receive bounded evidence through explicit artifacts:

- the Organon;
- authored vault observations;
- versioned substrate-observation bundles;
- whole-corpus projection validation reports;
- redacted behavior and private-UAT summaries;
- accepted language-neutral schemas;
- synthetic fixtures used as regression surfaces.

Legacy source may be inspected only for a declared evidence or compatibility question.

The distinction is:

```text
bounded observed artifact
≠ ambient architectural authority
```

## 3. Vocabulary discipline

During clean implementation, prefer:

- problem gestalt;
- problem region;
- boundary contribution;
- perturbation;
- relation;
- constraint;
- open tension;
- attention lens;
- semantic-space projection;
- activated projection;
- semantic-access plan;
- traversal path;
- conformance;
- execution;
- retrieval result;
- execution limits;
- synthesis packet.

A Python term may describe an observed artifact or behavior. It does not automatically name a clean-kernel responsibility.

## 4. No salvage presumption

The default question is not:

> How can this existing component fit?

It is:

> Which demonstrated responsibility or corpus fact requires representation, and what is the narrowest clean authority that can represent it?

Only after the responsibility is established should existing code be considered.

A reusable artifact must cross an explicit package, process, or protocol boundary and must not carry excess semantic authority.

## 5. Evidence requirement

Every proposed component, field, rule, or compatibility path must include:

- the kernel equation or invariant it implements;
- the authored-vault, whole-corpus, private-UAT, or accepted behavioral evidence requiring it;
- an example requiring it;
- a counterexample showing what fails without it;
- its exact authority boundary;
- proof that it does not duplicate boundary inference, semantic-access inference, or synthesis;
- the concrete consumer for any compatibility mechanism.

A proposal based on inference must identify the source evidence and bridge rule licensing that inference.

Prose coherence is not evidence.

## 6. Contract lifecycle

Use explicit lifecycle states:

```text
candidate
provisionally accepted
corpus-validated
superseded
```

- `candidate` means represented for review.
- `provisionally accepted` means internally coherent and mechanically tested but not yet validated against the complete vault where corpus-sensitive.
- `corpus-validated` means it survived a versioned whole-corpus projection and relevant private-UAT contact.
- `superseded` means higher-authority evidence required revision.

Do not describe a corpus-sensitive contract as accepted merely because its schema round-trips or its synthetic fixture passes.

## 7. Synthetic-fixture boundary

A synthetic fixture may:

- isolate deterministic behavior;
- prove invalid structural cases;
- preserve regression coverage;
- make edge conditions reproducible;
- test authority boundaries without private data.

A synthetic fixture may not:

- establish whole-corpus exhaustiveness;
- substitute for actual identifier diversity;
- prove actual vault topology;
- prove real occurrence or temporal coverage;
- authorize a projection contract contradicted by the complete vault;
- establish feasibility already demonstrated by the Python system.

Synthetic success is necessary for local mechanics and insufficient for architectural acceptance.

## 8. Implementation sequence

For corpus-sensitive work, use:

```text
observe authored and materialized corpus facts
→ classify evidence and authority
→ define or amend the contract
→ provide examples and invalid examples
→ validate against the complete projection
→ implement
→ retain synthetic regression tests
```

For corpus-independent deterministic mechanics, implementation may proceed once the relevant authority boundary is accepted.

## 9. One responsibility per stage

- boundary inference describes how the problem gestalt changes;
- deterministic folding applies the declared perturbation;
- substrate observation reports authored and materialized facts;
- projection construction maps accepted facts into a frozen structural representation;
- activation presents a bounded positive view through the current attention lens;
- semantic-access inference connects problem regions and open tensions to projected addresses;
- conformance validates structural membership;
- execution materializes units;
- packet assembly preserves and bounds;
- synthesis interprets.

A stage that performs more than one semantic role is suspect.

## 10. Mandatory end-to-end traceability

For every test turn, retain:

- prior problem-space state;
- newest utterance;
- immediately preceding completed turn;
- boundary contribution;
- explicit perturbation operations;
- created, reinforced, merged, split, redirected, superseded, and retired regions;
- open tensions before and after the update;
- updated relational problem space;
- current attention lens;
- authored corpus snapshot identity;
- substrate-observation identity;
- projection identity and validation status;
- activated projection and activation provenance;
- expansion requests and telemetry;
- semantic-access plan;
- conformance result;
- executed paths;
- returned unit identities;
- packet removals and mechanical reasons;
- synthesis input;
- answer.

## 11. No invisible semantic consolidation

When boundary inference merges, splits, supersedes, or retires a problem region, the operation must retain:

- source region identity;
- resulting region identity;
- source turn;
- declared semantic reason;
- preserved relations;
- broken or retired relations.

The deterministic runtime may apply the operation.

It may not generate the semantic reason.

## 12. No invisible evidence removals

Any semantic unit removed after retrieval must record:

- unit identity;
- stage;
- exact deterministic rule;
- rule authority;
- remaining invariant.

The following are not acceptable mechanical reasons:

- insufficient semantic grounding;
- low problem-space coherence;
- proposition mismatch;
- paraphrase mismatch.

## 13. Projection-first debugging

When semantic access cannot construct or execute a desired path, ask in order:

- Is the needed structure authored in the vault?
- Was it admitted?
- Was it materialized by the substrate producer?
- Was it transferred through the observation exchange?
- Was it projected?
- Is it addressable in every represented direction?
- Is the relevant identifier coupled to every valid retrieval surface?
- Can the attention lens activate or expand toward it?

Do not compensate for a missing projection with post-retrieval heuristics.

Do not assume a Python omission proves the authored structure does not exist.

## 14. Problem-space-first debugging

When conversational continuity fails, ask:

- Did boundary inference preserve or redirect the correct problem region?
- Was a follow-on contribution merged instead of duplicated?
- Was the prior turn supplied as local continuity?
- Was an unresolved reference represented as an open tension?
- Did a correction supersede the old framing rather than silently coexist with it?
- Did the current attention lens expose the intended region?

Do not compensate with transcript dumping or automatic numerical decay.

## 15. Fresh-run hygiene

A fresh experimental baseline requires:

- a new suite identity;
- new thread identities;
- empty thread state;
- a recorded authored corpus snapshot;
- a recorded substrate-observation identity;
- a recorded projection identity;
- no replacement of reports without replacement of experimental identity.

A report directory is not conversation state.

## 16. Repository and import discipline

`duck-lint/CLEANROOM` remains independent from `duck-lint/semantic-traversal`.

Do not import Python runtime source, orchestration, diagnostics, or tests into CLEANROOM merely to accelerate development.

Permitted cross-repository contact must use an explicit versioned boundary, such as:

- a read-only substrate-observation bundle;
- a projection exchange;
- an execution-provider protocol;
- a bounded compatibility report.

Generated private corpus artifacts remain local and ignored.

Exporter code, schemas, validation, and repository-safe aggregate reports may be committed in the repository that owns them.

## 17. Review gates

### Gate A — Kernel authority accepted

Equations and invariants are stable enough to guide recovery.

### Gate B — Problem-space mechanics accepted

Boundary contributions, relational regions, open tensions, persistence, attention, fold authority, replay, and thread isolation are explicit.

### Gate C — Whole-corpus substrate contact accepted

The authored vault and Python materialization are exposed through a versioned observation boundary, with discrepancies and unknowns preserved.

### Gate D — Projection accepted

The complete admitted corpus is represented with validated object, region, unit, identifier, occurrence, anchor, surface, transition, and provenance closure.

### Gate E — Activation accepted

The problem-space lens produces a bounded positive view over the real projection without negative inference.

### Gate F — Semantic-access plan accepted

Inference output refers to projected addresses and connections.

### Gate G — Conformance accepted

Invalid structure is rejected without semantic interpretation.

### Gate H — Execution and packet accepted

Returned units preserve identity, provenance, measured coverage, and mechanical removal reasons.

### Gate I — Synthesis accepted

The model receives continuity, focus, problem-space context, retrieval, and limits without a post-retrieval semantic gate.

### Gate J — Behavioral fidelity accepted

Private UAT confirms preservation and improvement of demonstrated whole-corpus capability.

### Gate K — Compatibility accepted

Only then may specific Python substrate producers, adapters, or operators be reused across an explicit boundary.

## 18. Stop conditions

Stop implementation and return to evidence or design if:

- a corpus-sensitive abstraction is added without authored or observed corpus evidence;
- a synthetic fixture is treated as whole-corpus acceptance;
- a candidate contract outranks contradictory vault evidence;
- a Python boundary is copied merely because it already exists;
- a compatibility mechanism has no concrete consumer;
- attention bands are implemented as unrelated topic containers;
- the deterministic runtime infers semantic merge, split, or supersession;
- a numeric coherence score is introduced without demonstrated necessity;
- coherence is used to rank or admit retrieved evidence;
- an open tension is converted into a corpus absence claim;
- a new semantic role is added between execution and synthesis;
- a runtime heuristic decides paraphrase equivalence;
- a validly retrieved unit is removed for a non-mechanical reason;
- a needed path is absent from the projection and post-retrieval code is proposed instead of substrate or projection work;
- intrinsic typing and contextual participation are blurred;
- the newest utterance and background problem-space context are conflated;
- an unknown or unresolved corpus fact is filled with an invented default.

## 19. Legacy terminology and migration posture

Current Python classes, diagnostics, files, tests, and component names are not design authorities.

They may establish that a capability, failure, data shape, or consumer exists.

The goal is not to recreate every old feature or every old intermediate object.

The goal is to reconstruct the demonstrated system under the clean authority boundaries.

Python behavior or code is preserved only when it satisfies the clean contracts without additional semantic authority. Clean-room contracts are preserved only when they survive higher-authority corpus contact.
