# Runtime Invariants

These invariants protect the distinctions expressed in the kernel equations.

They should be treated as architectural acceptance criteria, not implementation preferences.

## 1. Two semantic inference sites

Only two runtime operations may interpret natural-language meaning:

1. inference, which deconstructs the utterance and constructs the traversal;
2. synthesis, which interprets the returned semantic units and produces the answer.

No deterministic middle layer may become a third semantic model.

## 2. The thread is an evolving problem space

Each thread owns its own aggregate problem-space state.

- A fresh thread begins clean.
- A continuing thread preserves its own state.
- Different threads never share problem-space state.
- The newest utterance may reshape, narrow, expand, or redirect the aggregate boundary.
- Conversation continuity is supplied to synthesis as background, while the newest utterance remains the focus.

## 3. The projected semantic space is corpus-derived authority

The projected semantic space is derived from the structured semantic substrate.

It defines what is addressable and traversable within the corpus accessible to the runtime.

The runtime may not invent or hallucinate non-existent semantic relations for retrieval or for rejecting post retrieval semantic units.

## 4. Traversal inference is bounded by the projection

The inference model may compose only from identifiers, object and unit addresses, relations, occurrences, directions, anchors, retrieval surfaces, and valid transitions exposed in the projected semantic space.

## 5. Conformance performs no inference

Structural conformance checks only whether the traversal exists within the projected semantic space.

It may reject absent identifiers, impossible identifier/object combinations, unavailable retrieval surfaces, missing canonical targets, invalid directions, absent relations, unresolved heading or block addresses, and unsupported transitions. It may record these on failure for debugging.

It may not decide whether the user's meaning or the evidence's meaning is “close enough.”

## 6. Execution performs no semantic adjudication

Once a traversal conforms, execution runs it.

A semantic unit returned by a valid traversal cannot be rejected because it fails a runtime-generated subject, predicate, proposition, paraphrase, or equivalence test.

## 7. No post-retrieval ontology

After execution, the runtime may not introduce new semantic objects such as generated predicate residuals, runtime-owned subject roles, proposition-eligibility objects, inferred equivalence classes, or admission categories absent from the semantic projection or traversal.

## 8. Packet assembly preserves rather than reinterprets

Packet assembly may deduplicate canonical identities, rank, apply deterministic bounds, preserve breadth-before-depth, attach provenance, attach anchors, and record execution limits.

It may not add a new relevance threshold based on natural-language interpretation.

## 9. Returned semantic units reach synthesis

Every semantic unit returned by a valid traversal remains eligible for the synthesis packet unless removed by a declared non-semantic bound.

Any removal must be traceable to a declared mechanical rule.

## 10. Provenance is never erased

The synthesis packet must preserve enough information to distinguish ownership, authorship, targets, traversal path, retrieval surface, temporal anchor, and execution limits.

Unknown association remains unknown. It is never silently assigned.

## 11. Object identity is not contextual participation

Intrinsic or inherited typing must remain distinct from contextual relation participation.

- `Capital` may be intrinsically typed as a `book`.
- A dated journal unit may establish that `Capital` participated as `book_read_today`.
- `Cleo` is not intrinsically a `journal_date`.
- A dated journal unit may mention or link to `Cleo`.

## 12. Top-down and bottom-up addressability are equally authoritative

The projected space must support object-to-unit, unit-to-object, inherited identifiers, unit-authored references, inbound incidence, heading and block targets, temporal anchors, and valid traversal from either direction.

## 13. Coverage constrains claims, not evidence meaning

Coverage may restrict corpus-wide absence claims, exhaustive counts, chronology claims when temporal execution failed, graph claims beyond executed depth, and claims from unavailable sources.

Coverage may not reject a retrieved unit for failing an inferred semantic formulation.

## 14. Negative claims require measured exhaustive execution

A claim such as “there are no matches” requires a declared exhaustive exact traversal, structurally valid literals, full eligible-scope execution, and completed total-count measurement.

## 15. Repair returns to inference

When a traversal is structurally invalid, repair belongs to the inference model.

No deterministic heuristic may repair natural-language meaning.

## 16. Dynamic semantic content remains data-driven

Static runtime types may define the shape of objects, units, identifiers, relations, traversals, and packets.

They must not hard-code the corpus's actual semantic contents.

## 17. Absence of a represented connection is meaningful

Within the closed projected space, a represented path may be traversed and an absent path may not be fabricated.

“Cleo is not a journal date” is not answered by a semantic judge. The proposed type assignment or connection simply does not exist in the projection.
