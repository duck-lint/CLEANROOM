# Clean-Room Protocol

## Purpose

This protocol prevents the new runtime from being shaped by attempts to preserve the current implementation.

The risk is not merely code reuse.

Existing nouns can quietly force the new architecture to contain objects and authorities it does not need.

## 1. Clean context

The initial project workspace should contain only:

- the Organon;
- the clean-room document set;
- a tiny synthetic semantic-space fixture;
- accepted language-neutral schemas.

Do not initially include:

- old source files;
- implementation plans;
- PR summaries;
- diagnostics;
- legacy unit-test names;
- legacy component diagrams.

## 2. Vocabulary discipline

During clean design, prefer:

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

A legacy term may be mentioned only during the later compatibility audit.

## 3. No salvage presumption

The default question is not:

> How can this existing component fit?

It is:

> Does the clean kernel require this responsibility?

Only after the responsibility is established should existing code be considered.

## 4. Evidence requirement

Every proposed component or rule must include:

- the kernel equation or invariant it implements;
- an example requiring it;
- a counterexample showing what fails without it;
- its exact authority boundary;
- proof that it does not duplicate boundary inference, semantic-access inference, or synthesis.

A proposal based on inference must identify the evidence supporting that inference.

## 5. No implementation before contract acceptance

The sequence is:

```text
define
→ provide examples
→ provide invalid examples
→ review authority
→ freeze schema
→ implement
```

## 6. One responsibility per stage

- boundary inference describes how the problem gestalt changes;
- deterministic folding applies the declared perturbation;
- projection exposes semantic possibility;
- activation presents a bounded positive view through the current attention lens;
- semantic-access inference connects problem regions and open tensions to projected addresses;
- conformance validates structural membership;
- execution materializes units;
- packet assembly preserves and bounds;
- synthesis interprets.

A stage that performs more than one semantic role is suspect.

## 7. Mandatory end-to-end traceability

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
- projection identity;
- activated projection and activation provenance;
- expansion requests and telemetry;
- semantic-access plan;
- conformance result;
- executed paths;
- returned unit identities;
- packet removals and mechanical reasons;
- synthesis input;
- answer.

## 8. No invisible semantic consolidation

When boundary inference merges, splits, supersedes, or retires a problem region, the operation must retain:

- source region identity;
- resulting region identity;
- source turn;
- declared semantic reason;
- preserved relations;
- broken or retired relations.

The deterministic runtime may apply the operation.

It may not generate the semantic reason.

## 9. No invisible evidence removals

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

## 10. Projection-first debugging

When semantic-access inference cannot construct a desired plan, ask first:

- Is the needed semantic possibility present in the corpus?
- Is it materialized?
- Is it projected?
- Is it addressable in every represented direction?
- Is the relevant identifier coupled to every valid retrieval surface?
- Can the attention lens activate or expand toward it?

Do not compensate for a missing projection with post-retrieval heuristics.

## 11. Problem-space-first debugging

When conversational continuity fails, ask:

- Did boundary inference preserve or redirect the correct problem region?
- Was a follow-on contribution merged instead of duplicated?
- Was the prior turn supplied as local continuity?
- Was an unresolved reference represented as an open tension?
- Did a correction supersede the old framing rather than silently coexist with it?
- Did the current attention lens expose the intended region?

Do not compensate with transcript dumping or automatic numerical decay.

## 12. Fresh-run hygiene

A fresh experimental baseline requires:

- a new suite identity;
- new thread identities;
- empty thread state;
- a recorded projection identity;
- no replacement of reports without replacement of experimental identity.

A report directory is not conversation state.

## 13. Repository and import discipline

`duck-lint/CLEANROOM` remains independent from the legacy runtime repository during clean implementation.

Do not import legacy runtime source, orchestration, diagnostics, or tests into `CLEANROOM` merely to accelerate development.

When old artifacts are examined, do so through a bounded compatibility review. Reuse is accepted only when a specific artifact implements a clean responsibility without excess authority and crosses an explicit package, process, or protocol boundary.

## 14. Review gates

### Gate A — Kernel accepted

Equations and invariants are stable.

### Gate B — Problem-space contract accepted

Boundary contributions, relational regions, open tensions, persistence, and the attention lens are explicit.

### Gate C — Projection accepted

The semantic object/unit/occurrence model is exhaustively represented.

### Gate D — Activation accepted

The problem-space lens produces a bounded positive view without negative inference.

### Gate E — Semantic-access plan accepted

Inference output refers to projected addresses and connections.

### Gate F — Conformance accepted

Invalid structure is rejected without semantic interpretation.

### Gate G — Execution accepted

Returned units preserve identity and reach packet assembly.

### Gate H — Synthesis accepted

The model receives continuity, focus, problem-space context, retrieval, and limits.

### Gate I — Legacy compatibility accepted

Only then may specific existing artifacts, substrate producers, adapters, or operators be reused across an explicit boundary.

## 15. Stop conditions

Stop implementation and return to design if:

- focus bands are implemented as unrelated topic containers;
- the deterministic runtime infers semantic merge, split, or supersession;
- a numeric coherence score is introduced without demonstrated necessity;
- coherence is used to rank or admit retrieved evidence;
- an open tension is converted into a corpus absence claim;
- a new semantic role is added between execution and synthesis;
- a runtime heuristic decides paraphrase equivalence;
- a validly retrieved unit is removed for a non-mechanical reason;
- a needed path is absent from the projection and code is proposed instead of projection work;
- intrinsic typing and contextual participation are blurred;
- the newest utterance and background problem-space context are conflated.

## 16. Legacy terminology and migration posture

Current classes, diagnostics, files, tests, and component names are not design authorities.

A legacy term may describe observed behavior during compatibility review, but it does not earn a place in the clean kernel merely because it already exists.

The goal is not to recreate every old feature.

The goal is to implement the clean kernel.

Legacy behavior or code is preserved only when it satisfies the new contracts without additional semantic authority.
