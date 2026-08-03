# Semantic Object and Semantic Unit Model

## 1. Semantic substrate

The semantic substrate contains canonical semantic objects and canonical semantic units.

A semantic object is not merely a file. A semantic unit is not merely a text fragment.

They are addressable identities within an explicitly structured semantic space.

## 2. Semantic object

A semantic object is a canonical corpus entity represented by a note or equivalent source object.

Its identity may include:

- stable object identifier;
- source UUID;
- canonical path;
- canonical title;
- object type identifiers;
- admitted frontmatter;
- topology;
- authored relations;
- addressable internal structure.

Example:

```text
Capital.md
    canonical object identity: Capital
    note_type: book
    contains: semantic units
```

`book` is an intrinsic or inherited object identifier.

## 3. Semantic unit

A semantic unit is a canonical, independently addressable unit belonging to a semantic object.

Its identity may include:

- stable unit identifier;
- parent object identifier;
- section path;
- heading identity;
- paragraph ordinal;
- split ordinal;
- block identifier;
- unit text;
- inherited object identifiers;
- unit-level occurrences and anchors.

## 4. Top-down structure

The object supplies context to its units:

```text
semantic object
    contains → semantic units
    identifiers → inherited by units
    topology → inherited by units
    admitted frontmatter → visible on units
```

Top-down inheritance must preserve provenance.

## 5. Bottom-up structure

The unit points back into the object and may make the object reachable from another context:

```text
semantic unit
    belongs_to → semantic object
    occurrence → canonical object
    occurrence → canonical unit
    temporal anchor → contextualized relation
```

A dated journal unit may contain an authored link to `[[Capital]]`.

That occurrence creates a canonical connection from the contextual unit back to the `Capital` object.

## 6. Lateral relations

The substrate must represent relations across objects and units:

```text
object → unit
unit → object
object → object
unit → object
unit → unit
object or unit → temporal anchor
target object or unit → inbound contextual occurrence
```

A relation is not merely string resemblance. It must be materialized as an addressable occurrence, anchor, inherited identifier, or other admitted corpus structure.

## 7. Canonical links

### Object target

```text
[[Capital]]
```

This is an authored occurrence targeting the canonical `Capital` semantic object.

The occurrence should preserve source object, source unit, target object, authored direction, link text, provenance surface, and occurrence identity.

### Heading target

```text
[[Capital#Chapter 2]]
```

This must resolve to a canonical semantic-unit address represented in the projected space.

The runtime may not silently degrade the target to the parent note.

If a heading maps to more than one physical fragment, ingest or projection must represent the address deterministically rather than leaving runtime code to guess.

### Block target

```text
[[Capital#^block-id]]
```

This must resolve to one canonical semantic unit.

### Embed

Embeds preserve target identity while recording presentation mode. They do not create a different semantic object.

## 8. Reverse incidence

A canonical target must be discoverable from both directions:

```text
source unit → authored occurrence → Capital
Capital → inbound occurrence → source unit
```

Stored reverse edges are optional. Reverse addressability is not.

## 9. Intrinsic typing versus contextual participation

These are separate axes.

### Intrinsic or inherited type

```text
Capital
    note_type: book
```

### Contextual participation

```text
dated journal unit
    links to Capital
    temporal anchor: 2026-07-02
    contextual role: book_read_today
```

`book_read_today` is not necessarily an intrinsic type of `Capital`.

It is a relation instantiated by the dated contextual unit and its canonical occurrence.

Likewise, Cleo is not a `journal_date`, but a dated journal unit may establish a dated contextual relation involving Cleo.

## 10. Semantic-unit identifiers

Identifiers may arise from parent-object inheritance, unit-local admitted metadata, section or heading identity, paragraph or split ordinal, block identity, temporal anchors, canonical authored occurrences, graph relation incidence, and source topology.

The projection must state:

- which identifier exists;
- what object or unit may carry it;
- whether it is intrinsic, inherited, local, or relational;
- which retrieval surfaces may inspect it;
- which relations or transitions it enables.

## 11. Identity preservation during retrieval

Every retrieved unit must retain unit identity, parent object identity, inherited identifiers, unit-local identifiers, occurrences, anchor identity, relation provenance, and traversal provenance.

Retrieval may not flatten all of this into anonymous text.

## 12. No runtime replacement ontology

If a valid traversal reaches a unit, the authoritative facts are that the unit exists, belongs to its object, carries materialized identifiers and relations, and was reached through represented connections.

Whether the unit answers the current question is synthesis work.
