# Behavioral Examples

These examples test the clean architecture without requiring old runtime concepts.

## 1. Fresh thread

### Input

```text
When did I go vegan?
```

### Expected stages

```text
P0:
    empty relational problem space

B1:
    create region: vegan transition
    relation: current subject → transition
    constraint: temporal answer requested
    attention: primary

P1:
    one bounded problem region with temporal orientation

W1:
    positively activated semantic region around vegan-related units,
    contextual occurrences, and temporal anchors

T1:
    collect vegan-related semantic units with temporal anchors

C(T1, Mσ):
    valid

R1:
    dated semantic units, including the first-day entry

Synthesis:
    receives all returned units and answers
```

No prior thread state is present.

## 2. Vegan date

### Returned unit

```text
date: April 16
text: today is officially day #1 of going vegan
```

If this unit was returned by a conforming plan, it reaches synthesis.

The runtime does not require it to match a generated phrase such as:

```text
transition to veganism
```

That equivalence is synthesis work.

## 3. Invalid Cleo/date path

### Proposed path

```text
object: Cleo
identifier: journal_entry_date
surface: temporal
```

### Projection

```text
Cleo
    does not carry: journal_entry_date

dated journal unit
    may mention or link to: Cleo
```

### Expected result

The direct assignment or path is structurally invalid because it is absent from the projection.

A different path through dated contextual units may be valid.

## 4. Canonical source-material identity

```text
Marx, Karl — Capital
    canonical identity: UUID
    note_type: source_material
    format: book
    creator: Karl Marx
```

A semantic-access plan may discover this object through `Capital`, `Karl Marx`, `source_material`, or `book`, but execution binds to its canonical UUID rather than treating any discovery surface as the object itself.

## 5. Capital contextual participation

```text
journal object or unit J1
    temporal anchor: July 2
    authored link: [[Marx, Karl — Capital]]
```

This supports access from the dated journal context through the outgoing occurrence to the canonical source-material object, or from that object through its incoming occurrence to the dated journal context and temporal anchor.

It may establish the contextual relation `book_read_today`.

That does not make `book_read_today` or the journal date an intrinsic identifier of the source-material object.

## 6. Heading-specific target

```text
I reviewed [[Marx, Karl — Capital#Chapter 2]] today.
```

Expected projection:

```text
source unit
→ authored heading occurrence
→ canonical semantic region in the Capital source-material object
→ one or more contained semantic units
```

Execution must preserve the region or unit identity resolved by the authored target.

## 7. Two-book chronology

### Input

```text
Which did I start first, Capital or Blood Meridian?
```

Expected semantic-access structure:

```text
Marx, Karl — Capital ───────────────┐
                                      ├→ contextual dated occurrences → chronology
McCarthy, Cormac — Blood Meridian ───┘
```

Execution returns dated units while preserving which canonical target each unit references.

Synthesis performs the comparison.

## 8. Exact count

### Input

```text
How many cats do I have?
```

Coverage may authorize an exhaustive count only when total-count execution completed.

Returned units still reach synthesis.

## 9. Exact absence

### Input

```text
Does the exact phrase X occur anywhere?
```

A valid exhaustive plan requires:

- an explicit literal;
- supported exact mode;
- complete eligible scope;
- completed total count.

Zero matches after completed execution is different from:

- invalid plan;
- unavailable surface;
- incomplete execution;
- a problem-space gap;
- failure to activate a region.

## 10. Multi-turn continuation as coherent redirection

### Turn 1

```text
What did the calf eat?
```

Problem region:

```text
calf diet
    referent: calf
    relation: consumed food
```

### Turn 2

```text
When did that change?
```

Boundary contribution:

```text
preserve:
    calf referent

reinforce:
    calf diet region

redirect:
    relation from consumed food
    to temporal transition in diet

resolve:
    "that" → calf diet

attention:
    temporal transition → primary
    prior diet description → supporting relation
```

The second turn does not create a duplicate `calf diet` topic.

It transforms the same identifiable problem region.

A new thread would not inherit the reference.

## 11. Explicit correction and supersession

### Turn 1

```text
Which did I start first, Capital or Blood Meridian?
```

### Turn 2

```text
No, I meant which was published first.
```

Expected boundary contribution:

```text
supersede:
    reading chronology

preserve:
    Capital
    Blood Meridian
    comparative relation

replace constraint:
    publication chronology
```

The old reading chronology remains in history but is no longer an active competing interpretation.

## 12. Open tension without false resolution

### Input

```text
Was Capital before Blood Meridian?
```

If `before` could mean reading chronology or publication chronology and the thread does not resolve the dimension, boundary inference may create:

```text
open tension:
    chronology dimension unresolved
```

The runtime does not choose one by lexical heuristic.

The tension may guide clarification or projection exploration.

It does not imply that no answer exists in the corpus.

## 13. Recurrent unresolved region

A conversation repeatedly returns to the same unresolved implementation question.

Expected behavior:

```text
one persistent problem region
+
several source contributions
+
recurrent status
+
one still-open tension
```

Not:

```text
five near-duplicate topic markers
```

Recurrence is preserved as history rather than erased by deduplication.

## 14. Attention bands as one lens

A problem region may move:

```text
primary
→ secondary
→ background
→ primary
```

The system retains one region identity.

It does not copy the region into four stores.

## 15. Positive-only activation

A working projection activates:

```text
canonical source-material objects for Capital and Blood Meridian
their dated contextual occurrences
temporal surfaces
```

Material outside that working view is merely not loaded under the current lens and budget.

Its absence from the activated view cannot authorize:

```text
there is no relevant evidence elsewhere
```

## 16. Mechanically bounded packet

If execution returns 500 units and the synthesis packet permits 50:

- all 500 remain in the retrieval result;
- 50 are selected by a declared deterministic rule;
- every removed unit records the mechanical reason;
- packet limits are visible to synthesis;
- no unit is removed because it failed a generated proposition or coherence test.

## 17. Previous turn as conversational continuity

### Previous turn

```text
User: What did the calf eat?
Assistant: It first drank milk and later ate hay.
```

### Current utterance

```text
When did that change?
```

Synthesis receives the previous turn as labeled conversational continuity.

It does not treat the previous assistant answer as corpus evidence.

## 18. Oversized unit transport

One authored semantic unit exceeds an embedding or provider input limit.

Expected behavior:

```text
semantic unit U
→ transport segment U.1
→ transport segment U.2
```

Both segments retain the same parent semantic-unit identity and reconstruction order. They do not become two independently authored semantic units.

## 19. Provider failure

If boundary inference fails:

- no problem-space transformation is fabricated.

If semantic-access inference fails:

- no executable plan is fabricated.

If synthesis fails:

- the retrieval result remains persisted.

Provider failure must never be represented as a semantic conclusion.
