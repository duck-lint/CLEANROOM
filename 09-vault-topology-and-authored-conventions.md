# Vault Topology and Authored Conventions

## Status and provenance

**Action state:** Drafted for operator review.  
**Source basis:**

- the supplied full vault tree;
- the representative mini-vault, which preserves the full directory shape and provides representative notes;
- the supplied `Organon of Finite Inquiry`;
- the supplied semantic object/unit identifier list;
- the supplied chunk-representation example;
- the previously accepted clean-room kernel documents.

This document describes the vault as an authored and topologically organized semantic substrate. It does not treat the legacy runtime as design authority.

Statements are distinguished as follows:

- **Observed** — directly present in the supplied tree or representative notes.
- **Structural inference** — follows from the alignment of topology, frontmatter, authored links, and the Organon.
- **Open requirement** — must be settled by operator review or implementation tests; it is not silently inferred.

## 1. The vault is an instantiated architecture, not a bag of Markdown files

The vault’s top-level topology is organized around the Organon’s layers and functions:

```text
INBOX/
LAYER-1 PILLARS/
LAYER-2 INTERFACE/
LAYER-M META-ONTOLOGY/
VAULT DESIGN/
.gitignore
vault.base
```

The physical layout is meaningful, but it is not the sole source of semantic typing. A note’s semantic position is constituted jointly by:

```text
canonical note identity
+ physical topology
+ admitted frontmatter identifiers
+ authored Markdown structure
+ canonical link occurrences
+ temporal anchors
```

A path places an object in an authored region. Frontmatter declares object-level identifiers. Markdown headings organize its internal semantic units. Wikilinks and admitted relational fields connect it to other canonical objects and units.

The evidence therefore rejects two simplistic readings:

1. **The folder tree alone is the ontology.** It is not. Entity notes are physically under the Dynamic Coherence region while their frontmatter may identify them as `public`, `descriptive`, and `semantic_geometry`.
2. **Frontmatter alone is the ontology.** It is not. The source object’s path, heading structure, and authored occurrences supply relations and addressability not exhausted by scalar fields.

The semantic substrate is the composition of all of these authored structures.

## 2. Top-level regions

### 2.1 `LAYER-M META-ONTOLOGY/`

**Observed contents:**

```text
LAYER-M META-ONTOLOGY/
├── INFERENTIAL BRIDGES/
├── ORGANON FRAMEWORK/
├── Mapping a General Framework Of A Philosophical System onto ours.md
└── Synthesis Of The Organon – Non‑Obvious Implications & Hidden Failure Modes.md
```

The `ORGANON FRAMEWORK` region contains canonical notes for:

- Layer 0 seed axioms;
- Layer 1 pillars;
- Layer 2 interface;
- Layer M meta-ontology;
- Horizon Discipline;
- the integrated Organon.

These are typed as `note_type: organon` where populated. Their `register` and `pillar` are `meta`, while the `layer` identifier records the layer being specified.

The `INFERENTIAL BRIDGES` region contains frontmatter-rich semantic objects such as:

- Indexical Experience to Public Claim;
- Public Model Import to Indexical Commitment;
- Operationalization;
- Isomorphic Mapping;
- Register Coercion Blocker;
- Quarantine for World-Scope Closure.

Their frontmatter materializes bridge structure through fields such as:

```text
from_register
to_register
from_mode
to_mode
bridge_methods
bridge_conditions
bridge_preservation
bridge_broken
bridge_justification
bridge_applicability_scope
cash_out
revision_triggers
stop_rule
```

**Structural inference:** Layer M is the vault region in which typing constraints, admissibility conditions, bridge rules, and failure boundaries are themselves represented as canonical semantic objects. These objects are not merely explanatory prose. Much of their semantic content exists in admitted fields.

### 2.2 `LAYER-1 PILLARS/`

**Observed contents:**

```text
LAYER-1 PILLARS/
├── PILLAR 1-SEMANTIC GEOMETRY/
└── PILLAR 2-DYNAMIC COHERENCE/
    ├── ENTITY INDEX/
    └── JOURNAL/
```

The full tree currently places almost all populated Layer-1 material under `PILLAR 2-DYNAMIC COHERENCE`. The `PILLAR 1-SEMANTIC GEOMETRY` directory is present but appears unpopulated in the supplied tree.

This does not mean all objects under the Dynamic Coherence path are themselves indexical Dynamic Coherence objects. The entity notes supply the clearest counterexample:

```text
physical path:
LAYER-1 PILLARS/PILLAR 2-DYNAMIC COHERENCE/ENTITY INDEX/Cleo.md

frontmatter identifiers:
note_type: entity
register: public
register_mode: descriptive
pillar: semantic_geometry
entity_type: cat
```

**Structural inference:** The Layer-1 topology places the Entity Index within the operational ecology of Dynamic Coherence, while each entity object may still be semantically typed as a public Semantic Geometry model. Physical containment and semantic identification are related but non-identical axes.

#### `ENTITY INDEX/`

The Entity Index contains canonical notes for people, animals, agents, and entity collections. Representative fields include:

```text
note_type: entity
entity_type
canonical_name
relationship
first_met
birthday
aliases
tags
uuid
```

An entity note may carry little or no body prose and remain semantically useful. `Cleo.md`, for example, is primarily an identified metadata-bearing object. `The Cats.md` contains the authored statement:

```text
My cats [[Cleo]] and [[Toly]].
```

That body creates canonical outbound link occurrences from the collection object to two entity objects. The runtime projection must preserve the occurrences without needing to invent a deterministic natural-language relation type beyond what the authored structure provides.

#### `JOURNAL/`

The Journal contains several distinct topological families:

```text
JOURNAL/
├── 2025/
│   ├── 2025-01/
│   ├── ...
│   └── Theme for 2025.md
├── 2026/
│   ├── 2026-01/
│   ├── ...
│   └── Theme for 2026.md
├── CAREER/
├── DREAM MOTIFS/
├── LISTS/
├── MENTAL HEALTH/
└── RANDOM INFORMATION/
```

Daily journal files are nested by year and month, with filenames such as:

```text
2026/2026-05/19_Tuesday.md
```

The representative daily note also carries the explicit field:

```yaml
journal_entry_date: 2026-05-19
```

The supplied identifier description states that `journal_entry_date` records the day the handwritten entry belongs to, rather than the later digitization date. This makes the field the explicit temporal identifier; the directory and filename provide reinforcing topology rather than replacing it.

The daily note template supplies four recurring headings:

```text
# Dream Recall
# Yesterday Review
# Daily Intent
# Freeform Journaling
```

The representative journal note uses those headings to organize authored content. Each heading region contains one or more semantic units.

The Journal also contains canonical motif objects under `DREAM MOTIFS`. Many supplied motif notes have almost no body prose but carry:

```text
uuid
note_type: dream_motif
aliases
tags: dream
```

These are identity-bearing semantic objects used as canonical relation targets. A daily journal note can point to a motif both in frontmatter and in body prose:

```yaml
dream_motif:
  - "[[music_festival]]"
```

```text
Fresh [[music_festival|music festival]] recall...
```

The same daily note can establish a book-consumption relation through:

```yaml
book_read_today:
  - "[[Darwin, Charles — Origin of Species]]"
```

This relation originates in the dated journal object. It does not retype the target book object as a journal date.

Other Journal subregions organize durable contextual objects rather than daily entries: career material, lists, mental-health records, random reference material, and dream motifs. Their path supplies contextual topology, while their note-level identifiers determine the objects’ declared type and register where populated.

### 2.3 `LAYER-2 INTERFACE/`

**Observed major regions:**

```text
LAYER-2 INTERFACE/
├── CODE/
├── CREATIVE WRITING/
├── LEXICON/
├── MUSIC/
├── PROBLEM SPACE ENTHUSIAST/
├── READING & RESEARCH/
└── TRADING/
```

The Organon describes Layer 2 as the region where the system imports external material, runs stress tests, instantiates models, and produces practical, theoretical, creative, and coordination-facing outputs.

The directory structure materially instantiates that function:

- `CODE` contains technical procedures, lessons, local-agent material, and language-specific references.
- `CREATIVE WRITING` contains authored outputs divided into `INDEXICAL` and `PUBLIC` branches, then into project or framework-specific regions.
- `LEXICON` contains canonical concept objects and nested conceptual families.
- `MUSIC` contains music-project artifacts.
- `PROBLEM SPACE ENTHUSIAST` contains project identity, process, production, and publication artifacts.
- `READING & RESEARCH` contains book notes, papers, research, resources, source material, and print-oriented material.
- `TRADING` contains applied procedures, guardrails, resources, and trading journals.

The `CREATIVE WRITING/INDEXICAL` and `CREATIVE WRITING/PUBLIC` split is an explicit topological instantiation of register orientation. It does not eliminate the need for note-level identifiers, but it provides an authored regional constraint.

#### `LEXICON/`

The Lexicon contains canonical notes for concepts such as `Register`, `Scope`, `Object`, `Concept`, `Public`, `Indexical`, `Semantic Geometry`, and many others. It also contains nested conceptual families such as:

```text
LOGIC/
QUANTUM/
SEMANTICS/
SUBJECT-OBJECT DUALITY/
FOURFOLD ROOT/
1ST ORDER CYBERNETIC LOOP/
```

The supplied lexicon template organizes entries under:

```text
# Working definition
# What it is not
# Operational cues / examples
# Common confusions
# Links
```

This structure makes a lexicon note both a canonical concept object and a set of internally addressable semantic units that distinguish definition, exclusion, example, confusion, and relation surfaces.

#### `READING & RESEARCH/`

This region makes an important ontological distinction:

```text
READING & RESEARCH/
├── BOOK NOTES/
└── SOURCE MATERIAL/
```

Representative `BOOK NOTES` objects are typed `book_notes` and contain the vault owner’s quotes, commentary, interpretations, questions, page markers, and links. Their headings are often page-oriented, such as `### P. 005` or `##### p. 13`.

Representative `SOURCE MATERIAL` objects are typed `source_material` and may contain the external text wholesale. The supplied Schopenhauer source object carries identifiers such as:

```text
title
creator
format: book
original_year_published
```

Its body is organized by authored chapter and section headings.

**Structural inference:** A book-notes object and a source-material object about the same work are distinct canonical semantic objects. One is an index of the vault owner’s reading and commentary; the other is a representation of the imported source. Filename similarity does not itself create canonical identity or a typed same-work relation.

**Open requirement:** If the clean runtime needs a guaranteed connection between book-note objects and source-material objects, that connection must be explicitly materialized through an admitted identifier or canonical link. It must not be inferred solely from filenames.

### 2.4 `VAULT DESIGN/`

**Observed contents:**

```text
VAULT DESIGN/
├── (.)MD TEMPLATES/
├── ATTACHMENTS/
├── CSS TEST PAGES/
├── MISC/
└── SECURITY/
```

This region contains vault-support artifacts: templates, images, PDFs, style test pages, security notes, and miscellaneous resources.

**Structural inference:** `VAULT DESIGN` is operational infrastructure for authoring and maintaining the vault rather than one of the Organon’s primary content regions.

**Open requirement:** The supplied material does not establish whether all, some, or none of this region should enter the semantic projection. Admission must be explicit. Physical presence in the vault is not enough to decide corpus inclusion.

### 2.5 `INBOX/`

The supplied tree contains an `INBOX` directory but no listed contents.

No semantic role beyond physical existence is established by the evidence. It should not be automatically typed as quarantine, draft, or capture space without an explicit policy.

## 3. Canonical object families observed in the representative vault

The representative mini-vault contains at least the following populated object families:

| `note_type` | Observed region | Primary represented function |
|---|---|---|
| `organon` | Layer-M / Organon Framework | Framework, layer, axiom, and meta-constraint objects |
| `inferential_bridge` | Layer-M / Inferential Bridges | Explicit cross-register or structural transformation rules |
| `entity` | Layer-1 / Entity Index | Canonical people, animals, agents, or entity collections |
| `journal_entry` | Layer-1 / Journal / year / month | Dated indexical record object |
| `dream_motif` | Layer-1 / Journal / Dream Motifs | Canonical motif identity object |
| `to_do_list` | Layer-1 / Journal / Lists | Operational list object |
| `book_notes` | Layer-2 / Reading & Research / Book Notes | Reading commentary and notes object |
| `source_material` | Layer-2 / Reading & Research / Source Material | Imported source representation |

The full tree shows additional likely families—lexicon entries, creative works, procedures, code notes, papers, research notes, and others—but the supplied representative objects do not provide a complete authoritative `note_type` census.

The user-supplied identifier list is explicitly non-exhaustive. The clean projection must therefore discover and validate the actual admitted schema rather than freeze this sample as the final type universe.

## 4. Identity, typing, and topology

### 4.1 `uuid` individuates the semantic object

Every populated representative note except templates and support pages carries a UUID or is intended to. The supplied identifier description calls `uuid` the semantic-object identification anchor.

Canonical object identity must not depend on:

- filename stability;
- title uniqueness;
- aliases;
- path stability;
- body text.

Those remain address and context surfaces. UUID provides individuation.

### 4.2 `note_type` identifies content class

The supplied identifier description defines `note_type` as the categorical type of content contained within the semantic object and its units.

Because semantic units inherit object identifiers, `note_type: journal_entry` is visible on every unit materialized from that journal object; `note_type: source_material` is visible on each unit materialized from the imported source.

### 4.3 Path is topological context, not a substitute for type

The path records authored placement within the Organon-shaped vault. It can support traversal such as:

```text
Layer 2
→ Reading & Research
→ Source Material
→ a canonical source object
→ its chapter units
```

But path cannot safely override frontmatter. The Entity Index example proves that folder placement and declared pillar/register are not identical axes.

### 4.4 Aliases and tags support addressability and grouping

The supplied identifier list describes:

- `aliases` as shorthands or synonyms of objects/units;
- `tags` as object/unit grouping.

They are not canonical identity anchors.

An alias may help the inference model connect a problem-space marker to a canonical object. The traversal must still bind to the object’s canonical identity before execution.

### 4.5 Intrinsic fields and relational fields are distinct

Examples of intrinsic or object-descriptive identifiers include:

```text
note_type
entity_type
canonical_name
title
creator
format
original_year_published
layer
register
pillar
```

Examples of contextual relation-bearing fields include:

```text
book_read_today
dream_motif
from_register
to_register
bridge_methods
bridge_preservation
bridge_broken
```

The distinction is not reducible to scalar versus wikilink value. A field’s semantic role must be represented in the projection’s identifier descriptor.

## 5. Authored Markdown conventions

### 5.1 Headings define semantic regions

Headings supply hierarchical context within a semantic object.

Observed patterns include:

- journal sections (`Dream Recall`, `Yesterday Review`, `Daily Intent`, `Freeform Journaling`);
- book-note page markers (`P. 005`, `p. 13`);
- source-material chapter and numbered section headings;
- Organon purposes, non-goals, axioms, pillars, imports, exports, and failure modes;
- lexicon definition, exclusion, example, confusion, and links sections.

A unit’s heading path is part of its semantic address.

### 5.2 Blank-line-separated prose establishes authored unit boundaries

The supplied chunk representation explicitly maps three paragraphs under one heading path to three chunks/semantic units:

```text
X > Y > chunk 1
X > Y > chunk 2
X > Y > chunk 3
```

A nested heading changes the path:

```text
X > Y > Z > chunk 1
```

A later peer heading creates a new branch:

```text
X > Y-1 > chunk 1
```

The author’s Markdown hierarchy and paragraph separation are therefore primary semantic-boundary signals.

### 5.3 Compound Markdown structures must retain authored form

Representative notes contain:

- block quotations;
- bulleted and numbered lists;
- tables;
- equations;
- code-like field declarations;
- emphasized spans;
- wikilinks.

The chunker must not flatten these structures before unit materialization. A list, table, code fence, display equation, or block quote must remain structurally intact unless an explicit oversized-unit policy applies.

**Open requirement:** The supplied material does not completely determine whether an adjacent quotation and its immediately following commentary should materialize as one coupled unit or as two separately addressable units. This should be settled with representative acceptance cases rather than an arbitrary token rule.

### 5.4 Headings without prose remain structural

A heading may contain only child headings and no direct prose. In that case it still contributes to descendant unit addresses but need not create a standalone text unit.

### 5.5 Empty-body objects remain semantic objects

Entity and dream-motif notes demonstrate that an object can be semantically significant through:

- canonical identity;
- type identifiers;
- aliases;
- tags;
- inbound and outbound relations;
- topology;

without containing prose units.

The semantic projection must therefore index objects independently of chunk count.

## 6. Wikilink and occurrence conventions

### 6.1 Object links

```text
[[Capital]]
[[Cleo]]
[[music_festival]]
```

These are authored canonical-target occurrences. The projection must preserve:

- source object;
- source unit, when the link occurs in body text;
- source field path, when the link occurs in admitted frontmatter;
- target object;
- display alias, if present;
- occurrence direction and identity.

### 6.2 Aliased links

```text
[[music_festival|music festival]]
```

The display text differs from the canonical target. The occurrence must preserve both. Display text is not target identity.

### 6.3 Heading and block links

The vault can author targets such as:

```text
[[Capital#Chapter 2]]
[[Capital#^block-id]]
```

These are not merely links to `Capital.md`. They include an internal target address.

**Open requirement:** A heading may contain multiple ordinal semantic units. The clean projection must explicitly choose and represent one of the following rather than blur them:

1. a heading address resolves to a canonical heading-region object containing one or more unit addresses; or
2. the authored heading itself is assigned a canonical unit identity distinct from its paragraph units; or
3. an accepted deterministic rule maps the heading target to one unit.

A block ID can resolve directly to one unit. A bare heading link cannot be silently collapsed to the whole parent object.

### 6.4 Frontmatter link occurrences

Fields such as `book_read_today` and `dream_motif` contain wikilink values. These occurrences are as semantically real as body links, but their field path adds typed relational context.

For example:

```text
journal object
    journal_entry_date: 2026-05-19
    book_read_today: [[Darwin, Charles — Origin of Species]]
```

materializes a relation whose source is the dated journal object and whose target is the canonical book object. The target book does not inherit `journal_entry_date` as an intrinsic identifier. It participates in a dated contextual relation.

## 7. Representative object anatomies

### 7.1 Daily journal object

```text
object identity:
    UUID of 19_Tuesday.md

intrinsic/contextual identifiers:
    note_type: journal_entry
    journal_entry_date: 2026-05-19
    layer: 1
    register: indexical
    register_mode: experiential
    pillar: dynamic_coherence

relational fields:
    book_read_today → Darwin book object
    dream_motif → music_festival motif object

internal regions:
    Dream Recall
    Yesterday Review
    Daily Intent
    Freeform Journaling

units:
    paragraph/block units under each heading path
```

### 7.2 Entity object

```text
object identity:
    UUID of Cleo.md

identifiers:
    note_type: entity
    entity_type: cat
    canonical_name
    aliases
    relationship
    first_met

body:
    optional
```

The entity remains addressable even if it has no prose unit.

### 7.3 Entity-collection object

```text
The Cats.md
    authored body link → Cleo
    authored body link → Toly
```

The object and link occurrences are authoritative. A more specific natural-language relation such as “member of collection” is not structurally typed by the supplied frontmatter and should not be fabricated as a deterministic runtime fact unless later materialized.

### 7.4 Source-material object

```text
object identity:
    UUID of the imported work

identifiers:
    note_type: source_material
    title
    creator
    format
    original_year_published

internal topology:
    chapter heading
    section heading
    paragraph units
```

This object may contain hundreds or thousands of semantic units while remaining one canonical semantic object.

### 7.5 Inferential-bridge object

```text
object identity:
    UUID of bridge note

semantic schema:
    source register/mode
    target register/mode
    methods
    conditions
    preserves
    breaks
    justification
    cash-out
    revision and stop rules

body:
    concise explanation, block, allowance, and upgrade conditions
```

Its units inherit the bridge’s full object-level schema.

## 8. Admission and exclusion boundaries

The supplied tree includes Markdown notes, PDFs, images, canvas files, templates, CSS test pages, and operational configuration.

The vault’s physical contents are broader than the semantic corpus necessarily exposed to traversal.

The clean system needs an explicit admission policy covering:

- Markdown content regions;
- templates;
- CSS test pages;
- security notes;
- attachments and PDFs;
- canvases;
- hidden/configuration files;
- inbox material;
- generated artifacts.

No path should be silently admitted or excluded solely because the legacy runtime did so.

## 9. Operator-review points

The supplied evidence is sufficient to draft the topology and conventions. The following points remain intentionally open because the evidence does not uniquely settle them:

1. Whether `VAULT DESIGN` notes participate in the semantic corpus, and under what scope.
2. Whether `INBOX` is semantically admitted before classification.
3. The canonical relation between `BOOK NOTES` and `SOURCE MATERIAL` objects representing the same work.
4. The exact cardinality and identity semantics of `[[Note#Heading]]` targets when a heading contains multiple paragraph units.
5. Whether a quotation and its immediately associated commentary form one unit or two.
6. The final exhaustive admitted-field schema; the supplied list is explicitly provisional.

These are not blockers to the clean-room project. They should become explicit projection and chunking acceptance tests rather than implicit runtime heuristics.
