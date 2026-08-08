# Organon → Vault → Semantic Substrate → Unit-Materialization Map

## Status and provenance

**Action state:** Drafted for operator review.  
**Source basis:** the supplied Organon, full vault tree, representative mini-vault, semantic identifier list, chunk-representation example, and accepted clean-room kernel documents.

This document specifies the constitutive mapping from the Organon into the authored vault and from the vault into the semantic space available to semantic-access inference.

It does not derive architecture from the legacy runtime.

## 1. Compression

The complete mapping is:

```text
Organon
    constrains admissible typing, relation, scope, and movement

Vault topology and authored conventions
    instantiate those constraints as canonical notes, paths,
    frontmatter identifiers, Markdown regions, and links

Semantic substrate
    materializes canonical semantic objects, semantic units,
    identifiers, occurrences, anchors, and connections

Semantic-unit materialization
    converts authored internal structure into independently
    addressable semantic units without replacing its meaning

Semantic-space projection
    exposes the resulting object/region/unit/identifier/relation space
    to semantic-access inference and structural conformance
```

The runtime does not manufacture an ontology after retrieval. The ontology available to the runtime is the projected form of the authored substrate.

## 2. Organon as constitutive and critical grammar

The Organon expressly rejects world-complete metaphysical closure. It is an evergreen constitutive-and-critical discipline for inquiry under finitude.

For the semantic substrate, the Organon contributes two complementary functions:

1. **Constitutive:** it supplies the axes by which objects and moves are typed and related.
2. **Critical:** it supplies constraints that prevent untyped crossings, scope inflation, and reification.

The Organon’s analysis orientation is bidirectional:

```text
outside-in / constitutive:
manifold → synthesis/unity → object-as-given → concept formation

inside-out / critical:
concepts → expected appearances → experience → concept update
```

The vault topology and object/unit model should preserve this bidirectionality. Objects contain and type units, while units and occurrences make objects discoverable and revisable from contextual evidence.

## 3. Organon layers mapped into vault regions

### 3.1 Layer M — meta-ontology and linting

The Organon defines Layer M as the ground for typing and linting: register integrity, provenance discipline, scope control, bridge requirements, and anti-reification.

The vault instantiates this through:

```text
LAYER-M META-ONTOLOGY/
├── ORGANON FRAMEWORK/
└── INFERENTIAL BRIDGES/
```

Canonical Organon objects declare the framework. Canonical bridge objects materialize permitted or blocked transformations.

A bridge object is therefore not merely prose about a rule. Its frontmatter can encode the rule’s source and target register, methods, conditions, preserved structure, broken content, justification, and cash-out.

### 3.2 Layer 0 — seed axioms

Layer 0 supplies admissibility axioms:

- finitude and asymptotic closure;
- register integrity;
- synthesis/unification precedence.

In the supplied topology, Layer 0 is represented as a canonical Organon object inside `LAYER-M META-ONTOLOGY/ORGANON FRAMEWORK`, not as a separate top-level content directory.

This is meaningful: Layer 0 is a governing constraint object, not a domain of ordinary corpus content.

### 3.3 Layer 1 — pillars

Layer 1 contains:

- Semantic Geometry: public-facing structural ontology;
- Dynamic Coherence: indexical praxis and updating under finitude.

The vault instantiates these pillars through the top-level `LAYER-1 PILLARS` region.

The supplied tree places the Journal and Entity Index under the Dynamic Coherence physical branch. Note-level frontmatter still distinguishes public Semantic Geometry entity objects from indexical Dynamic Coherence journal objects.

Thus:

```text
physical region
    supplies authored operational context

frontmatter identifiers
    supply declared object typing

links and inherited unit identifiers
    supply actual semantic connectivity
```

No one surface is sufficient by itself.

### 3.4 Layer 2 — interface

Layer 2 imports external material, runs tests and mappings, and produces artifacts that cash out the system.

The vault instantiates this as:

```text
LAYER-2 INTERFACE/
├── CODE
├── CREATIVE WRITING
├── LEXICON
├── MUSIC
├── PROBLEM SPACE ENTHUSIAST
├── READING & RESEARCH
└── TRADING
```

This region contains both inputs and outputs:

- imported source material and papers;
- book notes and research;
- canonical lexicon concepts;
- public and indexical creative artifacts;
- technical procedures and code notes;
- applied trading material;
- project production processes.

Layer 2 is therefore a broad interface zone rather than a single object type.

## 4. Orthogonal Organon identifiers

The supplied object/unit identifier list includes:

```text
uuid
note_type
tags
aliases
journal_entry_date
title
creator
book_read_today
layer
unity_level
vector_direction
register
register_mode
pillar
journal telemetry fields
format
publish_studio
original_year_published
origin
entity_type
canonical_name
relationship
first_met
birthday
```

These identifiers are not all the same kind.

A clean projection should describe at least the following roles:

| Role | Examples | Function |
|---|---|---|
| Individuation | `uuid` | Canonical semantic-object identity |
| Object class | `note_type`, `entity_type`, `format` | Declared type or represented content class |
| Organon position | `layer`, `pillar`, `unity_level` | Framework placement and level of unity |
| Register typing | `register`, `register_mode` | Public/indexical target and descriptive/experiential mode |
| Analysis orientation | `vector_direction` | Constitutive or critical direction |
| Canonical naming | `title`, `canonical_name`, `creator`, `aliases` | Address and attribution surfaces |
| Temporal anchoring | `journal_entry_date`, `first_met`, `birthday`, publication year | Time-bearing object facts or anchors |
| Contextual relation | `book_read_today`, `dream_motif`, bridge source/target fields | One object’s participation in relation to another |
| Grouping | `tags` | Non-individuating grouping surface |
| Indexical telemetry | `temporal_pace`, `reactivity`, dream fields, etc. | First-person report identifiers on journal objects |

A field descriptor must state its role and applicability. Treating every field as an interchangeable searchable scalar would erase the Organon’s typing discipline.

## 5. From authored note to canonical semantic object

Let a Markdown note be represented as an authored source record:

```text
N = (path, filename, frontmatter, Markdown body)
```

A note becomes a canonical semantic object only when it has a stable individuation anchor:

```text
O = object(uuid, topology, identifiers, body structure, occurrences)
```

The supplied convention assigns `uuid` this individuation role.

### 5.1 What belongs to the object

The object owns:

- UUID;
- canonical source path;
- filename and title surfaces;
- admitted frontmatter identifiers;
- object-level link occurrences in frontmatter;
- authored heading tree;
- body-level occurrence sources;
- contained semantic-unit addresses.

### 5.2 Empty-body objects

A semantic object can have zero prose units and remain fully real in the substrate.

Examples include many dream-motif objects and some entity objects. They remain addressable through UUID, path, type, aliases, tags, and inbound relations.

Therefore:

```text
object existence ≠ semantic-unit existence
```

The projection inventory must enumerate canonical objects independently of whether unit materialization produced body units.

## 6. From authored Markdown structure to semantic units

The supplied chunk representation defines a note with heading hierarchy and seven paragraph units:

```text
X > Y > chunk 1
X > Y > chunk 2
X > Y > chunk 3
X > Y > Z > chunk 1
X > Y > Z > chunk 2
X > Y-1 > chunk 1
X > Y-1 > chunk 2
```

This yields the primary rule:

> A semantic unit is a prose-bearing authored Markdown block situated under a canonical heading path and ordinal position within one semantic object.

### 6.1 Conceptual unit identity

A deterministic unit identity must preserve at least:

```text
parent object UUID
heading or region address
block ordinal within the authored region
explicit block identifier, when present
```

Conceptually:

```text
unit_address =
    object_uuid
    + authored_region_address
    + authored_block_ordinal
    + explicit_block_id, when present
```

The exact hash or serialized form is an implementation decision. The required invariants are:

- stable under unrelated changes elsewhere in the object;
- changed when the unit’s own address or source content changes in an identity-relevant way;
- always reversible to the parent object and authored region;
- unique within the projection snapshot.

### 6.2 Heading paths

A heading path is inherited context, not disposable decoration.

For example:

```text
Schopenhauer source object
→ Chapter I - Introduction
→ § 2. Application of the Method
→ paragraph ordinal 1
```

The paragraph unit must remain addressable with its complete path.

### 6.3 Paragraph and Markdown-block boundaries

Blank-line-separated authored paragraphs are direct semantic-unit boundaries in the supplied example.

Other compound Markdown blocks should be treated as authored structures:

- list;
- block quote;
- table;
- code fence;
- equation block;
- callout;
- embedded media reference.

They should not be flattened into undifferentiated prose before unit creation.

### 6.4 Oversized semantic units and transport segments

Provider, tokenizer, and embedding limits are technical constraints rather than authored semantic boundaries.

An oversized authored semantic unit remains one canonical semantic unit. When a technical operation cannot accept it whole, the runtime may derive ordered transport segments:

```text
semantic unit U
    → transport segment U.1
    → transport segment U.2
```

A transport segment must:

- retain the parent semantic-unit identity;
- preserve source-span provenance;
- carry a deterministic segment ordinal;
- preserve complete reconstruction order;
- avoid breaking complete Markdown constructs where possible;
- remain non-canonical as an independently authored semantic unit.

A token limit therefore cannot silently create new semantic units or alter authored ontology.

## 7. Top-down inheritance

Every semantic unit inherits the admitted identifiers of its parent object.

Formally, for object `O` with admitted identifier map `I(O)` and unit `u ∈ O`:

```text
I_inherited(u) = I(O)
```

The unit adds local address and content:

```text
I(u) = I(O)
     + heading or region address
     + block ordinal
     + local link occurrences
     + local block identity
     + local text
```

Examples:

### 7.1 Journal units

Every unit from the representative daily note inherits:

```text
note_type: journal_entry
journal_entry_date: 2026-05-19
layer: 1
register: indexical
register_mode: experiential
pillar: dynamic_coherence
book_read_today: [[Darwin, Charles — Origin of Species]]
dream_motif: [[music_festival]]
```

A `Dream Recall` unit and a `Daily Intent` unit remain different units because their heading paths and ordinal content differ, even though they inherit the same object identifiers.

### 7.2 Source-material units

Every section paragraph from the Schopenhauer source object inherits:

```text
note_type: source_material
title: On the Fourfold Root...
creator: Arthur Schopenhauer
format: book
original_year_published: 1847
```

This makes every paragraph addressable as a unit of that canonical work without duplicating the metadata as a new authored claim.

### 7.3 Entity units

If an entity object contains prose, each prose unit inherits:

```text
note_type: entity
entity_type
canonical_name
relationship
aliases
```

The object remains addressable even if no prose unit exists.

## 8. Bottom-up and lateral addressability

Top-down inheritance is only one direction of the substrate.

### 8.1 Unit → parent object

Every unit resolves to its canonical parent object.

```text
semantic unit
→ belongs_to
→ semantic object
```

This allows a retrieved paragraph to carry its complete object identity and identifiers into synthesis.

### 8.2 Unit → target object

A body link creates an authored occurrence:

```text
source unit
→ occurrence([[Cleo]])
→ Cleo object
```

The occurrence must retain:

- source unit;
- source object;
- target object;
- authored target text;
- display alias;
- source location.

### 8.3 Object field → target object

A frontmatter link creates an object-level typed occurrence:

```text
journal object
→ field occurrence(book_read_today)
→ Darwin source-material object
```

Because units inherit object identifiers, the relation is visible in the context of every journal unit while remaining sourced at the parent object and field path.

### 8.4 Target object → inbound context

The projection must expose reverse incidence:

```text
Darwin source-material object
→ inbound book_read_today occurrence
→ dated journal object
→ journal units
```

This is the bottom-up route needed for questions such as:

```text
When did I read Darwin?
Which canonical source-material object appears in this dated journal context?
```

Stored reverse edges are optional. Reverse addressability is required.

### 8.5 Unit → target unit or region

A heading or block link adds a target below object level:

```text
source unit
→ [[Marx, Karl — Capital#Chapter 2]]
→ heading address / contained unit address(es)
```

```text
source unit
→ [[Marx, Karl — Capital#^block-id]]
→ one canonical unit
```

The target’s parent object identity must remain available. A unit-level link does not sever object membership.

## 9. Intrinsic typing versus contextual participation

This is a core ontological boundary.

### 9.1 Intrinsic or inherited typing

```text
Marx, Karl — Capital source-material object
    note_type: source_material
    format: book
    creator: Karl Marx
```

```text
Cleo object
    note_type: entity
    entity_type: cat
```

These identifiers belong to the objects and are inherited by their units.

### 9.2 Contextual participation

```text
dated journal object
    journal_entry_date: 2026-05-19
    book_read_today: [[Darwin, Charles — Origin of Species]]
```

The relation means the book participates as `book_read_today` in that dated journal context.

It does not mean:

```text
Darwin source-material object is a journal_entry
Darwin source-material object carries journal_entry_date intrinsically
```

Similarly:

```text
dated journal unit
    mentions or links to Cleo
```

can place Cleo in a dated context without making Cleo a date-typed object.

The projected semantic space must encode both the object’s intrinsic identifiers and the contextual relation occurrence. Structural conformance can then determine whether a proposed semantic-access path exists by membership alone.

## 10. Canonical worked mappings

### 10.1 Journal → book

Authored source:

```yaml
journal_entry_date: 2026-05-19
book_read_today:
  - "[[Darwin, Charles — Origin of Species]]"
```

Materialized substrate:

```text
object J
    type: journal_entry
    temporal anchor: 2026-05-19
    field occurrence O1:
        field: book_read_today
        target: canonical Darwin source-material object B

units J.u1 ... J.un
    inherit J identifiers
    belong_to J
```

Reverse projection:

```text
source-material object B
    inbound occurrence O1
    contextual source J
    contextual date 2026-05-19
```

No extra runtime proposition is required.

### 10.2 Journal → dream motif

Authored source:

```yaml
dream_motif:
  - "[[music_festival]]"
```

and body:

```text
Fresh [[music_festival|music festival]] recall...
```

Materialized substrate:

```text
journal object J
    frontmatter occurrence → motif M

Dream Recall unit J.u1
    body occurrence → motif M
```

The two occurrences share a target but retain different sources and provenance surfaces.

### 10.3 Entity collection → entities

Authored source:

```text
My cats [[Cleo]] and [[Toly]].
```

Materialized substrate:

```text
The Cats object C
    unit C.u1
        occurrence → Cleo object
        occurrence → Toly object
```

The canonical links are deterministic structure. The natural-language interpretation of “my cats” remains available to synthesis through the unit text.

### 10.4 Source material → chapter units

Authored source:

```text
## Chapter I - INTRODUCTION
### § 1. The Method.
paragraph 1
paragraph 2
```

Materialized substrate:

```text
source object S
    unit S.u1:
        path: Chapter I > § 1
        ordinal: 1
    unit S.u2:
        path: Chapter I > § 1
        ordinal: 2
```

Both units inherit source title, creator, format, and publication identifiers.

### 10.5 Inferential bridge object → bridge units

Authored object fields define:

```text
indexical → public
via report protocol and measurement
under declared conditions
preserving content
breaking scope
```

Any body unit in that note inherits the full bridge schema. A semantic-access plan can retrieve the bridge by source/target register, method, preserved structure, broken structure, or cash-out field without reconstructing the rule from prose.

## 11. From semantic substrate to semantic-space projection

Let the complete materialized substrate be:

```text
Σ = (O, G, U, I, R, A, S)
```

where:

- `O` = canonical semantic objects;
- `G` = canonical authored semantic regions;
- `U` = canonical semantic units;
- `I` = identifier assignments and descriptors;
- `R` = canonical relation and occurrence records;
- `A` = temporal and internal addresses;
- `S` = retrieval surfaces and their visibility.

The projected semantic space for one frozen turn is a deterministic runtime-accessible representation:

```text
M_σ = project(Σ, corpus_snapshot, schema_version)
```

`M_σ` must expose both:

### 11.1 Possibility grammar

```text
which object, region, and unit kinds exist
which identifiers may apply
which relations and directions exist
which internal addresses exist
which retrieval surfaces can inspect which components
which semantic-access transitions are valid
```

### 11.2 Canonical actuality

```text
which objects actually exist
which regions and units belong to them
which identifiers they actually carry
which occurrences actually connect them
which headings and blocks are addressable
which temporal anchors actually exist
```

A semantic-access plan is conforming when all of its referenced addresses and transitions are members of this projection.

No component needs to “answer” whether Cleo is a date. The invalid assignment or connection is absent from `M_σ`.

## 12. Semantic-unit serialization requirements

Each unit made available to retrieval and synthesis should preserve at least:

```text
unit identity
parent object UUID
parent semantic-region address
parent object path and title surfaces
note_type and admitted inherited identifiers
heading path
block ordinal
explicit block identifier, when present
raw authored Markdown block
normalized searchable representation
canonical outbound occurrences
frontmatter relation provenance inherited from object
local temporal anchor or inherited object anchor
transport-segment descriptors, when technically required
projection snapshot identity
```

Normalization may support retrieval, but the authored representation must remain available. Searchable normalization cannot become the canonical semantic unit.

## 13. Invalidation and regeneration

The clean implementation should distinguish changes to:

### Object identity

A UUID change creates a different semantic object.

### Object metadata

A change to an admitted identifier regenerates the object projection and inherited unit projections.

### Topology

A path change modifies topological address but should not erase object identity when UUID is stable.

### Heading structure

A heading rename or reparenting changes descendant unit addresses and heading-target resolution.

### Unit content

A change to an authored block invalidates that block’s unit serialization and embeddings.

### Relation occurrence

A wikilink or linked frontmatter-field change updates outbound and inbound incidence.

### Unit-materialization and transport policy

A policy change that alters authored block boundaries or canonical unit identity requires a projection version change and deterministic unit regeneration.

A transport-segmentation policy change requires new segment descriptors and projection identity when those descriptors are projected, but it does not by itself create new canonical semantic units.

## 14. Boundaries this mapping forbids

The mapping forbids the runtime from:

- treating filename similarity as canonical identity;
- treating physical folder placement as the only semantic type;
- flattening contextual participation into intrinsic object typing;
- discarding empty-body objects;
- collapsing heading or block targets to anonymous parent-note text;
- stripping object identifiers from retrieved units;
- inventing post-retrieval subject/predicate ontology;
- excluding a validly retrieved unit because it fails a runtime-generated paraphrase;
- compensating for missing projection structure with semantic heuristics.

## 15. Clean implementation consequence

The new runtime should be built around this sequence:

```text
1. ingest authored vault structure into canonical objects, units,
   identifiers, occurrences, anchors, and addresses;

2. project the complete accessible semantic space;

3. let semantic-access inference connect the thread problem space to paths in that projection;

4. structurally confirm that the proposed paths exist;

5. execute them;

6. preserve returned units and provenance in the synthesis packet;

7. let synthesis interpret the result.
```

Semantic-unit materialization is therefore not a generic preprocessing utility. It is the boundary at which authored internal structure becomes the addressable unit layer of the semantic substrate. Technical segmentation occurs later and remains subordinate to canonical unit identity.

## 16. Operator-review points

The supplied materials and accepted clean-room contracts support the complete mapping above. These remaining decisions should be resolved explicitly during schema design:

1. Whether quote-plus-commentary adjacency is one authored semantic unit or two related semantic units.
    Pages will be under headings like such so I think it will be okay how the chunking is currently 
    ### P. 008
        > *"Besides this, **certainty** and **clarity** with regard to its **form** are two essential demands that may very properly be made on an author who ventures on so slippery an undertaking."*
        - He's talking on the nature of knowledge itself in seeking **complete and comprehensive** understanding of through critical inquiry of what we might hope to achieve with reason when **all** the material assistance of experience are taken away (*[[a priori]]*) — sounds like I'm reading [[Myself]] from the future earlier this year, searching outside ourself for answers—though I believe we've come to different conclusions about the ability to do this (talk from the perspective he proposes) at all, considering Seed Axiom F, finitude. Our [[Perspective]] allows construction of a 1 sided [[Inferential Bridge (Rule)|bridge]] that only reaches halfway, and requires a leap of faith to cross, of which we're not in the business of doing, we've no firm knowledge of the other side.
        - Through our framework, pages 9 and 10 see him expounding on certainty—splitting registers, and demands [[Register]] typing—with clarity.
2. The explicit relation connecting book-note objects to source-material objects for the same work, if such a relation is desired.
    semantic objects of the source material (`book name.md`) will be linked to notes like → (`BOOK NOTES — book name.md`) with the quotes from source material in the notes
3. The complete admitted-field registry and each field’s applicability, inheritance, relation, and temporal affordances.
    huh? produce a report or decision matrix or something for me to fill for this
4. The admission policy for `VAULT DESIGN`, attachments, canvases, PDFs, and inbox material.
    vault design, attachments, canvases, pdfs can all be skipped from being put into the space. nothing special about inbox material it can be ingested like everything else
5. The exact transport-segmentation algorithm and reconstruction guarantees, which must remain non-semantic.
    huh? produce a report or decision matrix or something for me to fill for this

These decisions belong in the semantic projection and unit-materialization contracts. They must not be deferred to post-retrieval interpretation.
