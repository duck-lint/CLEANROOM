# Behavioral Examples

These examples test the clean architecture without requiring old runtime concepts.

## 1. Fresh thread

### Input

```text
When did I go vegan?
```

### Expected stages

```text
P0: empty
B1: current subject, vegan transition, temporal focus
P1: aggregate contains those markers
T1: retrieve vegan-related semantic units with temporal anchors
C(T1, M): valid
R1: dated semantic units, including the first-day entry
Synthesis: receives all returned units and answers
```

No prior thread state is present.

## 2. Vegan date

### Returned unit

```text
date: April 16
text: today is officially day #1 of going vegan
```

If this unit was returned by a conforming traversal, it reaches synthesis.

The runtime does not require it to match a generated phrase such as `transition to veganism`.

That equivalence is synthesis work.

## 3. Invalid Cleo/date path

### Proposed traversal

```text
object: Cleo
identifier: journal_date
surface: temporal
```

### Projection

```text
Cleo
    does not carry: journal_date

dated journal unit
    may mention or link to: Cleo
```

### Expected result

The direct assignment or path is structurally invalid because it is absent from the projection.

A different traversal through dated contextual units may be valid.

## 4. Capital intrinsic type

```text
Capital
    object_type: book
```

A traversal using `book` to address Capital is structurally possible.

## 5. Capital contextual participation

```text
journal unit J1
    temporal anchor: July 2
    authored link: [[Capital]]
```

This supports a traversal from dated journal unit to canonical occurrence to Capital.

It may establish the contextual relation `book_read_today`.

That does not make `book_read_today` an intrinsic type of Capital.

## 6. Heading-specific target

```text
I reviewed [[Capital#Chapter 2]] today.
```

Expected projection:

```text
source unit
→ authored heading occurrence
→ canonical semantic-unit address in Capital
```

Execution must preserve the target unit identity.

## 7. Two-book chronology

### Input

```text
Which did I start first, Capital or Blood Meridian?
```

Expected traversal:

```text
dated contextual units
→ canonical occurrences
→ Capital or Blood Meridian
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

A valid exhaustive traversal requires an explicit literal, supported exact mode, complete eligible scope, and completed total count.

Zero matches after completed execution is different from invalid traversal, unavailable surface, or incomplete execution.

## 10. Multi-turn continuation

### Turn 1

```text
What did the calf eat?
```

### Turn 2

```text
When did that change?
```

Turn 2 uses the existing problem-space state to resolve “that.”

A new thread would not inherit the calf reference.

## 11. Mechanically bounded packet

If execution returns 500 units and the synthesis packet permits 50:

- all 500 remain in the retrieval result;
- 50 are selected by a declared deterministic rule;
- every removed unit records the mechanical reason;
- packet limits are visible to synthesis;
- no unit is removed because it failed a generated semantic proposition.

## 12. Provider failure

If inference fails, no traversal is fabricated.

If synthesis fails, the retrieval result remains persisted.

Provider failure must never be represented as a semantic conclusion.
