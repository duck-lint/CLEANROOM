# Organon → Vault → Semantic Substrate → Unit-Materialization Map

## Status and provenance

**Action state:** Admitted-field/operator review closed; transport semantic invariants fixed; concrete transport mechanics intentionally consumer-bound.
**Source basis:** the supplied Organon; accepted CLEANROOM contracts; the accepted authored-vault observation and repository-safe audit record for the named specimen; explicit operator decisions recorded in this document; and the supplied vault tree, representative examples, semantic identifier list, and unit-materialization example as supporting evidence. The mini-vault, examples, paths, and legacy implementation are not independent semantic authorities.

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
analysis_orientation
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
| Analysis orientation | `analysis_orientation` | Constitutive or critical direction |
| Canonical naming | `title`, `canonical_name`, `creator`, `aliases` | Address and attribution surfaces |
| Temporal anchoring | `journal_entry_date`, `first_met`, `birthday`, publication year | Time-bearing object facts or anchors |
| Contextual relation | `book_read_today`, `dream_motif`, bridge source/target fields | One object’s participation in relation to another |
| Grouping | `tags` | Non-individuating grouping surface |
| Indexical telemetry | `headspace`, `reactivity`, dream fields, etc. | First-person report identifiers on journal objects |

A field descriptor must state its role and applicability. Treating every field as an interchangeable searchable scalar would erase the Organon’s typing discipline.

### 4.1 Admitted-field registry — accepted operator classifications for the current corpus boundary

The whole-corpus admitted-field registry records **accepted operator classifications**
for the current v3 observation:

- observer repository/commit: `duck-lint/semantic-traversal@502bc8d83a3681a21f4ab2f2cafb9598074aa24c`;
- observer schema: `vault-observation/v3`;
- accepted authored-vault specimen: `eb9447aa14e07995b86beb2c92d3c97c725fbdb23f1c210650b029fecd1d2d3d`;
- observed field universe: **60 frontmatter keys**.

The current repository-safe corpus census is:

```text
resident source records: 1085
resident Markdown: 1085
admission-eligible Markdown: 1077
excluded Markdown: 8
frontmatter valid: 1082
frontmatter absent: 3
frontmatter malformed: 0
parseable UUID: 1082
UUIDv7: 1082
duplicate UUID groups: 0
authored links: 4929
one-candidate authored targets: 4924
zero-candidate authored targets: 5
multiple-candidate authored targets: 0
```

The three frontmatter-absent Markdown records are excluded authoring
infrastructure, not admitted semantic objects. The admission boundary remains
`VAULT DESIGN`, attachments, Canvas files, and PDFs excluded; eligible ordinary
Markdown elsewhere remains admitted under the existing policy.

The historical v2 predecessor remains immutable evidence: observer
`duck-lint/semantic-traversal@99d0d4556684000f0ed585e47158a5f7fe9ce7e1`, schema
`vault-observation/v2`, specimen
`25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688`. Reconciliation
established a historical field universe of 60. The current observation is also
60 fields, with explicit operator replacements `temporal_pace` -> `headspace`
and historical `vector_direction` -> `analysis_orientation`; the latter is
absent from the fresh observation. The current registry remains 55 admitted,
5 excluded, and 0 unresolved. `transition_attempted` is not a registry field.

The registry is exhaustive for that accepted specimen. `note_version`,
`schema_version`, and `note_status` are not observed keys in that specimen and
are not registry members. The CLEANROOM-resident Organon copy predates the
accepted authored-vault specimen used for this registry. Its repository
frontmatter is not part of the measured specimen and must not be used to infer
the accepted specimen's field universe.

A future field universe requires new observation, explicit classification and
admission, registry revision, and any affected rematerialization or
projection-identity change. A key is never silently admitted because it resembles
an existing key or family.

This registry decides semantic role, admission, applicability, inheritance,
occurrence semantics, temporal semantics, and preservation/provenance. It does
**not** decide whether a field receives a dedicated embedded representation,
which indexes are built, how ranking works, or which downstream access strategy
should prefer one representation over another. Retrieval-surface affordances are
declared by the projection for the representations that materially exist.

Repository-safe audit linkage for this registry is recorded in
`14-admitted-field-registry-audit-manifest.md`. Private authored values and
sensitive evidence remain outside this repository.

#### Canonical object and Organon-position fields

| Field | Admission and role | Applicability / inheritance | Occurrence / temporal semantics |
|---|---|---|---|
| `uuid` | admitted; canonical semantic-object individuation anchor | all admitted objects; retained by units as parent provenance, never unit identity | no authored relation or independent temporal anchor |
| `note_type` | admitted; object/content class | carrying object; visible on materialized units with parent-field provenance | no canonical occurrence or independent temporal anchor |
| `layer`, `pillar`, `unity_level` | admitted; Organon position | carrying object and inherited unit context with provenance | no canonical occurrence or independent temporal anchor |
| `register`, `register_mode` | admitted; register typing | carrying object and inherited unit context with provenance | no canonical occurrence or independent temporal anchor |
| `analysis_orientation` | admitted; analysis orientation | carrying object and inherited unit context with provenance | no canonical occurrence or independent temporal anchor |
| `aliases` | admitted; alternate authored names/shorthand/address surfaces; non-individuating | carrying object and inherited address context; does not create identity | no canonical occurrence |
| `tags` | admitted; grouping/categorization surface; non-individuating | carrying object and inherited grouping context; not an alias substitute | no canonical occurrence |

#### Naming, entity, and source-material fields

| Field | Admission and role | Applicability / inheritance | Occurrence / temporal semantics |
|---|---|---|---|
| `canonical_name`, `entity_type` | admitted; entity naming/classification | entity objects; inherited by contained units with provenance | no canonical occurrence or independent temporal anchor |
| `occupation` | admitted; entity-profile metadata | entity objects; inherited with source-field provenance | no canonical occurrence or independent temporal anchor |
| `relationship` | admitted; relational entity-profile metadata | entity objects; inherited with source-field provenance | does not itself create a canonical link occurrence or temporal anchor |
| `address`, `email`, `phone`, `likes`, `dislikes` | observed but excluded from canonical substrate | raw observation/provenance only; not unit-inherited semantic metadata | no canonical occurrence or temporal affordance in the admitted substrate |
| `title`, `creator`, `format`, `origin`, `publish_studio` | admitted; source-material naming/metadata | source-material objects; inherited with source-field provenance | no canonical occurrence unless separately authored; no independent temporal anchor |
| `original_year_published` | admitted; publication temporal metadata | source-material objects carrying the field; inherited with provenance | `ExactYear` or `ApproximateYear` may materialize the publication temporal affordance; present-null creates no anchor; no year is invented from source chronology or other metadata |
| `birthday` | admitted; entity temporal metadata | entity objects carrying the field; inherited with provenance | `FullDate` or canonical `MonthDay` may materialize the corresponding temporal anchor; present-null creates no anchor; no canonical occurrence by itself |
| `first_met` | admitted; object-carried temporal metadata recording a first-meeting relation/context | entity objects carrying the field; inherited with provenance | `FullDate` or `DateTime` may supply temporal access/anchoring; present-null creates no anchor; it does not itself create a canonical linked occurrence |
| `journal_entry_date` | admitted; temporally capable object-carried journal field | journal-entry objects carrying the field; inherited with provenance | current authored applicability is `FullDate`; present-null creates no anchor and the date does not become intrinsic to the linked target |

#### Temporal capability and assignment actuality

Temporal role in the field registry describes descriptor-level capability. An
actual temporal anchor is materialized only from an authored assignment that
materially supplies an accepted temporal representation applicable to that
field. The current accepted representation categories are `FullDate`,
`DateTime`, `ExactYear`, `MonthDay`, and `ApproximateYear`; field-specific
applicability is defined by the rows above. Present-null preserves the authored
assignment and provenance distinction but creates no temporal anchor. Field
absence creates neither an assignment nor an anchor.

Observer parser-native shape is not semantic temporal representation. A native
YAML date or datetime may correspond to `FullDate` or `DateTime` where the field
contract permits it. The authored string `--MM-DD` is `MonthDay`, and `~N BCE`
is `ApproximateYear`, only under the accepted authored grammar. A parser-native
string is not automatically non-temporal, and a parser-native date/datetime is
not automatically an anchor without field applicability. No generic string coercion, regex parser, field-name guessing, natural-language parsing, or generic label fallback is authorized.

#### Canonical authored cardinalities

The repaired current substrate records these authored cardinalities without
changing field roles or null/absence semantics:

- canonical list-valued fields: `creator`, `register_mode`, `from_mode`,
  `to_mode`, `unity_level`;
- canonical scalar-valued fields: `format`, `layer`, `analysis_orientation`,
  `register`, `pillar`, `hypnagogic_resonance`, `reactivity`, `relationship`.

Single semantic values remain one-item authored lists for list-valued fields;
multiple authored list entries retain source order without automatic semantic
ranking significance. `from_mode` and `to_mode` remain bridge constitutive
metadata, `register_mode` remains register typing, `unity_level` remains
Organon-position metadata, and `relationship` remains relational entity-profile
metadata without creating a canonical graph occurrence merely by presence.

#### Explicit authored-representation operator decisions

The historical 60-field registry classification was revised for the current
observation by explicit operator decision: `headspace` replaces
`temporal_pace`, and `analysis_orientation` replaces historical
`vector_direction`. The fresh observation contains neither historical field.
Separately, during
authored-substrate repair, the operator explicitly fixed the canonical
authored cardinalities above and the temporal representation categories and
field applicability recorded in this section. Those authored-representation
decisions are constitutive operator authority. The repaired v3 observation
supplies corpus-actuality evidence that the substrate conforms to them; the
observer did not infer or create these rules. No new decision matrix or R001
revision is implied.

#### Canonical contextual relation-bearing fields

| Field | Admission and role | Applicability / inheritance | Occurrence / temporal semantics |
|---|---|---|---|
| `book_read_today` | admitted contextual relation-bearing field | dated `journal_entry` objects carrying the field; parent-field provenance retained by units | creates a canonical occurrence only when authored as a canonical link; the dated journal context can supply contextual temporal provenance for the target |
| `dream_motif` | admitted contextual relation-bearing field | journal-entry objects carrying the field; parent-field provenance retained by units | creates a canonical occurrence only when authored as a canonical link; no independent temporal anchor |

#### Inferential-bridge constitutive metadata

The following **22 fields** are admitted as the full observed inferential-bridge
schema. Bridge units inherit the complete object-level bridge schema with field
provenance. Authored graph behavior applies only where a field's authored
structure licenses a canonical occurrence. No field below substitutes for an
authored relation.

`bridge_applicability_scope`, `bridge_applied`, `bridge_broken`,
`bridge_conditions`, `bridge_isomorphism`, `bridge_justification`,
`bridge_methods`, `bridge_preservation`, `bridge_required`, `cash_out`,
`from_mode`, `from_register`, `interface`, `iso_broken`, `iso_justification`,
`iso_structure`, `quarantine_reasons`, `revision_triggers`,
`speculation_quarantine`, `stop_rule`, `to_mode`, `to_register`.

The bridge schema list is exhaustive for the bridge fields observed in this
accepted specimen. `architect_or_operator` is deliberately **not** bridge
metadata.

#### Journal/indexical state fields

The following seven fields retain the contract-defined `indexical telemetry`
role from §4:

`dream_location`, `dream_lucidity`, `dream_motif_valence`,
`hypnagogic_resonance`, `reactivity`, `recall_ability`, `headspace`.

They are admitted as contextual metadata on journal-entry objects carrying the
field, inherited by contained units with parent-field provenance, and preserved
losslessly. They do not create canonical occurrences or independent temporal
anchors.

`architect_or_operator` is separately admitted as **contextual journal-state
classification**. This is an explicit accepted operator decision, not a
retroactive telemetry classification and not bridge metadata. It applies to
journal-entry objects carrying the field, is inherited with parent-field
provenance, and creates no canonical occurrence or independent temporal anchor.
Its observed values do not authorize an inferential-bridge interpretation.

#### Common representational rules

For every admitted frontmatter field, preserve the authored raw value/form, a
normalized or typed representation where mechanically available, absent versus
present-null/blank, source-object identity, source-field/key provenance, and
authored array-element order. Preserving authored order does **not** assert that
the order is semantically significant. Normalization must never replace or
destroy the authored raw form.

Excluded fields retain the raw observation and provenance required by the
observation boundary and are not materialized in the admitted semantic
substrate.

The registry does not own embedding or index policy. If the projection
materializes an embedded representation for an admitted identifier or other
addressable representation, the accepted projection/access contracts determine
the vector-surface affordance of that representation.

## 5. From authored note to canonical semantic object

Let a Markdown note be represented as an authored source record:

```text
N = (path, filename, frontmatter, Markdown body)
```

After admission, a note becomes a canonical semantic object only when it has a stable individuation anchor:

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

### 5.3 Corpus admission boundary

The operator has fixed the following admission policy for the current corpus boundary:

- `VAULT DESIGN/` is excluded from the semantic space;
- attachments are excluded;
- Obsidian Canvas files are excluded;
- PDFs are excluded;
- `INBOX/` has no special quarantine or draft semantics merely because of its path; eligible Markdown there is observed, admitted, and materialized under the same rules as eligible Markdown elsewhere.

These are admission decisions, not claims that excluded material is semantically irrelevant in every possible future corpus. Raw observation should retain enough path and exclusion provenance to show what was omitted and by which rule.

An excluded source does not become a semantic object, region, unit, occurrence source, or retrieval candidate in the resulting projection. A later admission-policy change requires a new corpus/projection identity and rematerialization of the affected sources.

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

Quote-plus-commentary adjacency does not create a special fused semantic unit. The operator's book-note convention places the material under a page heading such as:

```text
### P. 008
> quoted source passage
- commentary item
- commentary item
```

The page heading defines one semantic region. Within that region, the block quote and the following commentary retain their authored Markdown block boundaries. Adjacency supplies local structural context but does not merge the quote and commentary into one canonical unit. A contiguous Markdown list remains one authored list block unless the authored structure itself establishes additional block boundaries.

### 6.3.1 Block-owned occurrence provenance and thematic breaks

The parser-owned Markdown block is the canonical body-occurrence source. The
factual observation records its exact `source_block_span` and a deterministic
`source_occurrence_ordinal`; an inline `source_span` is retained only as
finer-grained evidence and may be explicitly unavailable. Phase 5 therefore
requires one and only one semantic unit for an ordinary body source block,
and one and only one canonical region for a heading-marker source block.
Frontmatter occurrences remain sourced by their object field and retain a
field-local occurrence ordinal. Repeated identical authored markup must not
collapse occurrence identity.

The observed `hr` block is an authored thematic separator. It consumes the
region-local authored block ordinal but materializes as no semantic unit,
region, retrieval target, or new `AuthoredBlockType`. The resulting semantic
unit ordinals may contain gaps, and repository-safe Phase-5 evidence reports
the number of observed non-materialized `hr` blocks.

This preserves both requirements:

```text
shared page-heading context
≠
shared semantic-unit identity
```

### 6.4 Oversized semantic units and transport segments

Provider, tokenizer, and embedding limits are technical constraints rather than authored semantic boundaries.

An oversized authored semantic unit remains one canonical semantic unit. When a technical operation cannot accept it whole, the runtime may derive ordered transport segments:

```text
semantic unit U
    → transport segment U.1
    → transport segment U.2
```

A transport segment must:

- be subordinate to exactly one canonical semantic unit;
- never create a new canonical semantic unit;
- never create authored ontology or authored boundaries;
- retain the parent semantic-unit identity;
- retain deterministic segment order and ordinal;
- retain source-span provenance;
- preserve deterministic reconstruction;
- avoid breaking complete Markdown constructs where possible;
- remain technical/provider transport only and non-canonical as an independently authored semantic unit.

A token limit therefore cannot silently create new semantic units or alter authored ontology.

No universal transport-segmentation algorithm is part of the semantic-substrate contract. Canonical semantic-unit identity and authored boundaries are materialized independently of transport segmentation. Unit content may remain inline or deterministically hydratable under the projection contract. Technical transport segmentation cannot define, divide, merge, or replace canonical semantic-unit identity or authored boundaries. A concrete transport-segmentation policy is instantiated only when a named technical operation or provider cannot accept the complete canonical semantic unit under a demonstrated hard input constraint.

Until such a consumer exists:

- no tokenizer is selected;
- no token, byte, or character maximum is selected;
- no overlap size is selected;
- no concrete split-preference algorithm is selected;
- no provider-specific transport representation is granted representational authority.

When a concrete consumer requires segmentation, its bounded technical policy must record at least:

- **Trigger:** the named operation, the demonstrated hard whole-input constraint, and proof that the complete unit exceeds it;
- **Measurement basis:** the exact deterministic measurement used by that consumer, such as a named/versioned tokenizer, bytes, characters, or another explicitly identified measurement;
- **Maximum segment size:** an operation-specific configured technical bound that does not redefine canonical semantic identity;
- **Boundary handling:** preservation of complete authored/Markdown constructs where possible, with exact provenance/source spans for any forced split, and a deterministic preference order belonging to that consumer policy;
- **Overlap:** the concrete consumer policy must state whether overlap is used and why; any overlap is transport duplication only and cannot create duplicate semantic/evidence identity;
- **Reconstruction:** ordered segments must deterministically reconstruct the exact parent representation supplied to that operation;
- **Segment identity:** deterministic technical identity resolving to the canonical parent semantic-unit identity, with deterministic ordinal/order, while remaining non-canonical;
- **Consumer/provider specificity:** different technical consumers may require different policies; no universal corpus segment size is inferred from one provider;
- **Versioning:** every concrete transport policy is versioned. If transport-segment descriptors are included in a frozen `SemanticSpaceProjection`, a policy change requires a new projection identity; a runtime-only technical transport-policy change does not alter the canonical parent semantic-unit identity.

This is an accepted consumer-bound deferral contract, not an unresolved operator worksheet. No new provider-policy type or schema is introduced here, and any future need for an explicit policy identity in `TransportSegmentRecord` requires a separate evidence-backed contract amendment when the consumer exists.

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

### 10.5 Book notes and source-material connectivity

The operator has fixed that a canonical source-material object such as:

```text
book name.md
```

is explicitly linked to its distinct book-notes object such as:

```text
BOOK NOTES — book name.md
```

and the book-notes object contains the authored quotations and commentary about that source.

The constitutive connection is the authored canonical link occurrence, not filename similarity and not quote-text matching. In projection terms:

```text
source-material object S
    authored canonical occurrence O
    → book-notes object N

book-notes object N
    ← reverse incidence for O
    source-material context remains traceable
```

If the authored link is placed in the opposite direction or in both directions, the projection preserves the actual occurrence direction(s) and exposes reverse incidence. The runtime does not invent a separate `same_work`, `notes_for`, or equivalent relation merely because the filenames correspond.

Quotations inside `N` remain semantic units of the book-notes object. They do not become canonical source-material units merely because their text was quoted from `S`. A source-unit identity or region identity is available only when the authored substrate supplies an explicit canonical target/address or another later-admitted relation that licenses it.

Thus:

```text
explicit object link
    establishes canonical object-to-object connectivity

quoted text
    preserves authored note content
    but does not by itself establish source-unit identity
```

### 10.6 Inferential bridge object → bridge units

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

This substrate/chunking map owns the recovery sequence only through complete-vault projection validation:

```text
whole-corpus observation
→ contract-contact reconciliation
→ bounded contract amendment
→ real projection construction
→ complete-vault projection validation
```

Everything downstream is governed by `05-clean-implementation-sequence.md`. This document does not restate or compress the later projection-access, activation, semantic-access, conformance, execution, packet/synthesis, or private-UAT gates.

Semantic-unit materialization is therefore not a generic preprocessing utility. It is the boundary at which authored internal structure becomes the addressable unit layer of the semantic substrate. Technical segmentation occurs later and remains subordinate to canonical unit identity.

## 16. Operator-review resolution

The original review points are no longer an undifferentiated open list.

### Resolved by explicit operator decision

1. **Quote-plus-commentary materialization:** page headings provide shared semantic-region context; quote and commentary Markdown blocks are not fused merely by adjacency. Section 6.3 states the materialization rule.
2. **Book notes and source material:** distinct canonical objects are connected by authored canonical links with preserved authored direction and reverse incidence. Filename similarity and quote-text matching do not create the relation. Section 10.5 states the mapping.
3. **Corpus admission:** `VAULT DESIGN`, attachments, Canvas files, and PDFs are excluded; `INBOX` Markdown has no special path-based quarantine and follows ordinary admission/materialization rules. Section 5.3 states the boundary.
4. **Admitted-field registry:** the 60 observed fields in the accepted specimen have accepted operator classifications for admission, role, applicability, inheritance, occurrence semantics, temporal semantics, and preservation/provenance. Section 4.1 records those classifications. Repository-safe audit linkage is recorded separately and does not grant the registry authority over embedding or index design.

### Closed review surface

5. **Transport mechanics:** the non-semantic invariants are fixed and concrete segmentation is intentionally consumer-bound. There is no unresolved operator choice requiring completion before substrate materialization. Future segmentation parameters are technical policy decisions activated only by demonstrated consumer constraints; Section 6.4 records the contract.

No other retrieval, embedding, ranking, index, or provider architecture is introduced by this review resolution.
