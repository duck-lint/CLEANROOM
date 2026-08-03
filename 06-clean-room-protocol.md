# Clean-Room Protocol

## Purpose

This protocol prevents the new runtime from being shaped by attempts to preserve the current implementation.

The risk is not merely code reuse. It is vocabulary reuse: existing nouns can quietly force the new architecture to contain objects it does not need.

## 1. Clean context

The initial project workspace should contain only:

- the Organon;
- the clean-room document set;
- a tiny synthetic semantic-space fixture;
- accepted language-neutral schemas.

Do not initially include old source files, implementation plans, PR summaries, diagnostics, unit-test names, or component diagrams.

## 2. Vocabulary discipline

During clean design, prefer kernel terms:

- problem space;
- boundary markers;
- semantic-space projection;
- traversal;
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
- proof that it does not duplicate inference or synthesis.

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

- deconstruction updates boundary markers;
- projection exposes semantic possibility;
- traversal connects boundary to possibility;
- conformance validates structural membership;
- execution materializes units;
- packet assembly preserves and bounds;
- synthesis interprets.

A stage that both validates structure and interprets meaning is suspect.

## 7. Mandatory end-to-end traceability

For every test turn, retain:

- prior problem-space state;
- newest utterance;
- new boundary markers;
- updated problem-space state;
- projection identity;
- traversal;
- conformance result;
- executed paths;
- returned unit identities;
- packet removals and mechanical reasons;
- synthesis input;
- answer.

## 8. No invisible removals

Any semantic unit removed after retrieval must record unit identity, stage, exact deterministic rule, rule authority, and remaining invariant.

“Insufficient semantic grounding” is not an acceptable mechanical reason.

## 9. Projection-first debugging

When inference cannot construct a desired traversal, ask first:

- Is the needed semantic possibility present in the corpus?
- Is it materialized?
- Is it projected?
- Is it addressable in both directions?
- Can the inference model access it?

Do not compensate for a missing projection by adding post-retrieval heuristics.

## 10. Fresh-run hygiene

A fresh experimental baseline requires a new suite identity, new thread identities, empty thread state, a recorded projection identity, and no replacement of reports without replacement of experimental identity.

A report directory is not conversation state.

## 11. Branch discipline

Do not merge legacy runtime feature branches into the clean branch.

When old code is examined, do so through a bounded compatibility review rather than by copying files wholesale.

## 12. Review gates

### Gate A — Kernel accepted

Equations and invariants are stable.

### Gate B — Projection accepted

The semantic object/unit/occurrence model is exhaustively represented.

### Gate C — Traversal accepted

Inference output refers to projected addresses and connections.

### Gate D — Conformance accepted

Invalid structure is rejected without semantic interpretation.

### Gate E — Execution accepted

Returned units preserve identity and reach packet assembly.

### Gate F — Synthesis accepted

The model receives continuity, focus, traversal, retrieval, and limits.

### Gate G — Legacy compatibility accepted

Only then may existing artifacts or operators be reused.

## 13. Stop conditions

Stop implementation and return to design if:

- a new semantic role is added between execution and synthesis;
- a runtime heuristic is proposed to decide paraphrase equivalence;
- an old diagnostic becomes a required object without kernel justification;
- a validly retrieved unit is removed for a non-mechanical reason;
- a needed path is absent from the projection and code is proposed instead of projection work;
- intrinsic typing and contextual participation are blurred;
- the newest utterance and aggregate problem-space context are conflated.

## 14. Migration posture

The goal is not to recreate every old feature.

The goal is to implement the clean kernel.

Legacy behavior is preserved only when it satisfies the new contracts.
