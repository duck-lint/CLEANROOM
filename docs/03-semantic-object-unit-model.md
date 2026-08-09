# Semantic Object and Semantic Unit Model

## 1. Semantic substrate

The semantic substrate contains canonical semantic objects and canonical semantic units.

A semantic object is not merely a Markdown file. A semantic unit is not merely a text fragment or database row.

They are canonical, addressable identities materialized from the authored vault and exposed through the projected semantic space.

## 2. Semantic objects

A semantic object is a canonical corpus identity:

- **individuated by a stable UUID**;
- **materialized through** its topology, admitted identifiers, authored structure, and occurrences;
- **represented in the vault** by a Markdown note or another explicitly admitted source object.

The UUID is the canonical individuation anchor. It is not, by itself, the whole semantic object.

Formally, an authored note may be observed as:

```text
N = (path, filename, frontmatter, Markdown body)
```

and materialized as:

```text
O = object(
    uuid,
    topology,
    admitted identifiers,
    authored structure,
    occurrences
)
```

### Example — two distinct objects sharing the surface word `Capital`

```text
Marx, Karl — Capital.md
    role: the vault's canonical source-material representation
          of Marx's work Capital
    uuid: 019fc58d-42aa-7919-95f8-a69b609aadff
    note_type: source_material
    tags:
      - book
    title: Capital
    format: book
    creator: Karl Marx
    publication_year: 1867
```

and:

```text
Capital.md
    role: the vault owner's canonical lexicon object
          for the concept or word "capital"
    uuid: 019fc58d-8634-7f2d-ae44-65003742c0fb
    note_type: lexicon_entry
```

These are not one object with two filenames. They are separately individuated semantic objects that may share one textual identifier.

An object's searchable or traversable identifiers may include:

- UUID;
- canonical path and filename surfaces;
- title and aliases;
- admitted frontmatter values;
- topology;
- object type;
- format;
- creator;
- publication year;
- authored links and inbound occurrences.

Examples such as `Capital`, `Marx, Karl — Capital`, `book`, `source_material`, `Karl Marx`, `1867`, and `lexicon_entry` are identifier surfaces associated with canonical objects. They are not interchangeable with canonical identity.

## 3. Semantic units

A semantic unit is a canonical, independently addressable authored unit belonging to one semantic object.

A unit identifier or `chunk_id` addresses the semantic unit. It is not identical to the semantic unit's complete meaning or content.

A semantic-unit identity must preserve enough structure to distinguish it deterministically within a projection snapshot, including as applicable:

- parent object UUID;
- heading or section path;
- authored block ordinal;
- block identifier;
- unit-local text;
- unit-local occurrences and anchors.

Conceptually:

```text
unit_address =
    parent_object_uuid
    + authored_region_address
    + authored_block_ordinal
```

The exact serialized or hashed form is an implementation decision.

A semantic unit may materialize authored structures such as:

- paragraph;
- list;
- block quote;
- table;
- code block;
- equation block;
- callout;
- embedded media reference.

## 4. Semantic regions

A semantic region is an addressable authored structural area within a semantic object, usually arising from a heading path.

```text
semantic object
    contains → semantic regions

semantic region
    contains → zero or more semantic units
```

A region is not a third epistemic object class competing with semantic objects and units. It is structural addressability inside an object.

A heading region may contain several semantic units.

A heading region may also contain zero semantic units. If authored occurrence
syntax appears in the heading marker itself, that occurrence remains sourced by
the canonical semantic region and exact source span; it does not force the
creation of a semantic unit.

A block target may resolve to one explicitly addressed unit.

## 5. Top-down structure

The semantic object supplies context to its units:

```text
semantic object
    contains → semantic regions and units
    identifiers/frontmatter → inherited by units
    topology → inherited by units
    authored structure → locates units
```

Top-down inheritance must preserve provenance.

An inherited identifier remains sourced at the parent object or frontmatter field; it does not become a newly authored unit-level claim.

## 6. Bottom-up structure

Every semantic unit points back to its canonical object and authored region:

```text
semantic unit
    belongs_to → semantic object
    situated_in → semantic region
```

A semantic unit, canonical semantic region, or object field may create
addressable occurrences:

```text
semantic unit
    outgoing occurrence → semantic object
    outgoing occurrence → semantic region
    outgoing occurrence → semantic unit

semantic region
    outgoing occurrence → semantic object, region, or unit
```

The target must expose reverse incidence:

```text
semantic object, region, or unit
    incoming occurrence → source semantic unit, semantic region, or object field
```

An occurrence authored in a heading marker uses the region's canonical address
plus its exact source span as provenance. The region remains structural
addressability, not a third epistemic object class.

## 7. Canonical authored links

Obsidian wikilinks are authored using note names, paths, aliases, headings, or block identifiers. Ingest resolves those authored targets to canonical semantic addresses.

The user does not need to author UUID syntax in the vault.

### Object target

```text
[[Marx, Karl — Capital]]
```

Materialized occurrence:

```text
source object or unit
→ authored target: Marx, Karl — Capital
→ resolved target UUID
→ canonical source-material object
```

The occurrence preserves:

- source object;
- source unit or frontmatter field;
- authored target text;
- display alias, when present;
- resolved canonical target;
- occurrence identity;
- authored direction;
- source location and provenance surface.

### Heading-region target

```text
[[Marx, Karl — Capital#Chapter 2]]
```

This resolves to a canonical semantic-region address inside the target object.

The region may contain one or more semantic units. The runtime may not silently degrade the link to the parent object or arbitrarily choose one contained paragraph.

### Block target

```text
[[Marx, Karl — Capital#^block-id]]
```

This resolves to one canonical block-addressed semantic unit.

### Embed

An embed preserves canonical target identity while recording presentation mode. It does not create a second semantic object.

## 8. Reverse incidence and temporal context

A dated journal unit or journal object may contain an authored link to `[[Marx, Karl — Capital]]`.

The guaranteed canonical path is:

```text
Capital source-material object
→ incoming occurrence
→ dated journal object or unit
→ temporal anchor
```

This makes the book object discoverable from the contextual date without making that date an intrinsic identifier of the book.

An implementation may materialize indexed shortcuts for performance, but shortcut edges must preserve and remain reducible to the authoritative occurrence path.

## 9. Intrinsic typing versus contextual participation

These are separate axes.

### Intrinsic or inherited typing

```text
Marx, Karl — Capital
    note_type: source_material
    format: book
    creator: Karl Marx
```

```text
Cleo
    note_type: entity
    entity_type: cat
```

### Contextual participation

```text
dated journal object or unit
    journal_entry_date: 2026-07-02
    book_read_today: [[Marx, Karl — Capital]]
```

The book participates as `book_read_today` in that dated journal context.

This does not mean:

```text
Capital is intrinsically a journal entry
Capital intrinsically carries journal_entry_date
```

Likewise, Cleo is not a `journal_entry_date`, but a dated journal unit may mention or link to Cleo.

## 10. Identifier roles

Identifiers may arise from:

- canonical individuation;
- object class;
- topology;
- Organon placement;
- register typing;
- title, alias, and attribution surfaces;
- temporal anchors;
- contextual relation fields;
- unit-local structure;
- canonical authored occurrences.

The projection must state for every admitted identifier:

- its value shape;
- what object or unit may carry it;
- whether it is intrinsic, inherited, local, or relational;
- where its provenance originates;
- which retrieval surfaces may inspect it;
- which relations or transitions it enables.

Every structurally valid identifier-to-surface affordance must be projected rather than added through case-specific runtime patches.

## 11. Semantic units versus transport segments

A tokenizer, embedding model, or provider context limit must not redefine authored semantic identity.

If one semantic unit is too large for a technical operation, it may be transmitted or embedded through non-semantic transport segments:

```text
semantic unit U
    → transport segment U.1
    → transport segment U.2
```

Transport segments:

- retain the same parent semantic-unit identity;
- carry deterministic segment ordinals;
- preserve complete reconstruction order;
- are not independently promoted to canonical semantic units;
- do not create new ontology or authored boundaries.

A true new semantic unit arises from authored structure or an explicitly accepted materialization rule, not merely from a token ceiling.

## 12. Identity preservation during retrieval

Every retrieved unit must retain:

- canonical unit identity;
- parent object identity;
- region address;
- inherited identifiers and their provenance;
- unit-local identifiers;
- authored occurrences;
- temporal anchors;
- access-path provenance;
- retrieval-surface provenance.

Retrieval may not flatten this into anonymous text.

## 13. No runtime replacement ontology

If a conforming semantic-access plan reaches a unit, the authoritative structural facts are that the unit:

- exists;
- belongs to its object;
- is situated in an authored region;
- carries materialized identifiers and occurrences;
- was reached through represented semantic connections.

Whether that unit answers the current problem is synthesis work.
