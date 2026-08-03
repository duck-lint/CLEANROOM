# Runtime Invariants

These invariants protect the distinctions expressed in the kernel equations.

They are architectural acceptance criteria, not implementation preferences.

## 1. Two semantic roles across three primary model calls

Natural-language meaning may be interpreted only through:

1. boundary inference;
2. semantic-access inference;
3. synthesis.

The first two are separate calls performing the inference role. One additional bounded semantic-access repair call may occur after structural conformance failure.

No deterministic middle layer may become another semantic model.

## 2. The thread is a relational problem gestalt

Each thread owns one evolving problem-space state.

That state contains:

- problem regions;
- relations among regions;
- active constraints;
- open tensions;
- contribution and persistence history;
- a current attention lens.

It is not merely a transcript, topic list, or continually rewritten summary.

## 3. Boundary contributions are perturbations

The newest utterance contributes an explicit transformation of the existing problem space.

Possible transformations include preserve, reinforce, extend, merge, split, connect, constrain, redirect, supersede, retire, open tension, and resolve tension.

The deterministic runtime may apply those operations.

It may not infer them independently.

## 4. Attention bands are views, not containers

Primary, secondary, tertiary, and background activation are different current intensities over one problem space.

Moving a region between bands must not duplicate it.

A background region remains structurally part of the problem gestalt until explicitly superseded or retired.

## 5. Semantic continuity aggregates rather than piles up

A follow-on question about an existing region should reinforce, refine, redirect, or extend that region rather than create an indefinitely growing duplicate.

Deduplication of meaning is issued by boundary inference.

It is not guessed by deterministic similarity heuristics.

## 6. Coherence is qualitative and structural

Problem-space coherence means preservation of identifiable relational structure through revision.

It is not initially represented by a numeric score.

It is not a truth measure, confidence value, automatic decay function, or ranking signal.

## 7. Open tensions remain explicit

Unresolved references, contradictions, missing distinctions, and recurrent unresolved questions remain represented as open tensions.

The runtime must not smooth them into false resolution.

An open tension in the problem space is not a negative claim about the corpus.

## 8. Thread isolation is absolute

- A fresh thread begins clean.
- A continuing thread evolves only from its own state.
- Different threads never share problem-space state.
- Product-level branch cloning remains outside the initial kernel.

## 9. The immediately preceding turn is continuity, not evidence

Synthesis receives the previous completed turn so a referential follow-up does not appear without local conversational context.

That turn is labeled as conversational continuity.

It does not become retrieval evidence merely by being present.

## 10. The projected semantic space is corpus-derived authority

The projected semantic space is derived from the structured semantic substrate.

It defines what is addressable and traversable within the corpus available to the runtime.

The runtime may not invent or hallucinate a nonexistent semantic relation during semantic access, execution, or post-retrieval packet construction.

## 11. Projection activation is positive-only

Activation means that a semantic region is presently loaded or visible.

It never means that unloaded space is irrelevant, absent, or evidentially empty.

Failure to reach a region under a current budget authorizes no negative conclusion.

## 12. Semantic-access inference is bounded by the projection

The second inference call may compose only from identifiers, addresses, relations, occurrences, directions, anchors, retrieval surfaces, and transitions exposed by the projected semantic space.

The problem-space lens guides access.

It does not create corpus structure.

## 13. Conformance performs no inference

Structural conformance checks only whether the semantic-access plan exists within the projected semantic space.

It may reject:

- absent identifiers;
- impossible identifier/object combinations;
- unavailable retrieval surfaces;
- missing canonical targets;
- invalid directions;
- absent relations;
- unresolved heading or block addresses;
- unsupported transitions;
- configuration violations.

It may record exact structural violations for diagnostics and bounded repair.

It may not decide whether the user's meaning or the evidence's meaning is “close enough.”

## 14. Execution performs no semantic adjudication

Once a plan conforms, execution runs it.

A semantic unit returned by a valid plan cannot be rejected because it fails a runtime-generated subject, predicate, proposition, paraphrase, equivalence test, or coherence test.

## 15. No post-retrieval ontology

After execution, the runtime may not introduce generated semantic roles or admission categories absent from the projection or plan.

Diagnostic descriptions may not acquire veto authority.

## 16. Problem-space coherence never gates retrieved evidence

Coherence belongs to the representation of the evolving conversational problem.

It may guide projection activation and semantic access.

It may not be reapplied after retrieval to decide whether evidence reaches synthesis.

## 17. Packet assembly preserves rather than reinterprets

Packet assembly may:

- deduplicate canonical identities;
- rank by declared deterministic rules;
- apply configured bounds;
- preserve breadth;
- attach provenance and anchors;
- record execution limits.

It may not add a natural-language relevance or coherence threshold.

## 18. Returned semantic units reach synthesis

Every semantic unit returned by a valid plan remains eligible for the synthesis packet unless removed by a declared non-semantic bound.

Every removal must be traceable to a mechanical rule.

## 19. Provenance is never erased

The synthesis packet must preserve enough information to distinguish:

- ownership;
- authorship;
- canonical targets;
- access path;
- retrieval surface;
- temporal anchors;
- execution limits.

Unknown association remains unknown.

## 20. Object identity is not contextual participation

Intrinsic or inherited typing remains distinct from contextual relation participation.

- `Marx, Karl — Capital` may be intrinsically typed as `source_material` with `format: book`.
- A dated journal object or unit may establish that this canonical source-material object participated as `book_read_today`.
- `Cleo` is not intrinsically a `journal_entry_date`.
- A dated journal unit may mention or link to `Cleo`.

## 21. Top-down and bottom-up addressability are equally authoritative

The projected space must support object-to-unit, unit-to-object, inherited identifiers, outgoing occurrences, incoming occurrences, heading and block targets, and temporal relations from every represented direction.

## 22. Coverage constrains claims, not meaning

Coverage may restrict corpus-wide absence, exhaustive count, chronology, graph depth, or unavailable-source claims.

It may not reject a retrieved unit for semantic inadequacy.

## 23. Negative claims require measured exhaustive execution

A claim of no matches requires an explicit exhaustive plan, a supported exact surface, complete eligible scope, and completed total-count measurement.

Activation, contextual probes, and problem-space gaps cannot substitute for exhaustive authority.

## 24. Repair returns to inference

When a plan is structurally invalid, repair belongs to a fresh inference call.

The runtime supplies exact structural violations.

No deterministic heuristic repairs natural-language meaning.

## 25. Dynamic semantic content remains data-driven

Static runtime types may define the shape of problem regions, boundaries, semantic objects, units, relations, plans, and packets.

They may not hard-code the corpus's actual semantic contents.

## 26. Absence of a represented connection is meaningful

Within the closed projected semantic space, a represented path may be traversed and an absent path may not be fabricated.

That is structural absence, not post-retrieval semantic judgment.
