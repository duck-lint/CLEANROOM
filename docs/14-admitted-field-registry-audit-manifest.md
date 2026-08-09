# Admitted-Field Registry Audit Manifest

## Status

**Purpose:** repository-safe provenance linkage for the 60-field admitted-field
registry recorded in `10-organon-vault-substrate-chunking-map.md` §4.1.

**Authority boundary:** this file records evidence identity and safe structural
summaries. It does not create semantic roles, retrieval surfaces,
embedding/index policy, ranking behavior, or transport policy.

**Audit-completeness state:** **COMPLETE REPOSITORY-SAFE LINKAGE.** Public
structural audit linkage is complete for the current v3 observer/specimen pair.
Private authored contents remain intentionally private. This manifest does not
reproduce the private evidence bundle, and does not acquire semantic or
representational authority by linking evidence. Corpus actuality is supplied by
the versioned authored-vault observation; explicit operator decisions resolve
constitutive questions; representational authority resides in the accepted,
corpus-validated CLEANROOM contracts.

## 1. Current v3 observation identity

| Item | Repository-safe value |
|---|---|
| Observer repository | `duck-lint/semantic-traversal` |
| Observer commit | `e9bb2d95c14b1beb334dc2b8d83420f5998b9a53` |
| Observer schema identifier/version | `vault-observation/v3` |
| Authored-vault specimen identity | `f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4` |
| Pinned run-1 artifact byte SHA-256 | `d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9` |
| Resident source records | `1060` |
| Resident Markdown count | `1060` |
| Admission-eligible Markdown count | `1052` |
| Excluded Markdown count | `8` |
| Valid frontmatter count | `1057` |
| Absent frontmatter count | `3` |
| Malformed-frontmatter count | `0` |
| Parseable UUID count | `1057` |
| UUIDv7 count | `1057` |
| Duplicate UUID groups | `0` |
| Whole-resident total authored-link occurrences | `5008` |
| Whole-resident one-candidate occurrences | `4907` |
| Whole-resident zero-candidate occurrences | `101` |
| Whole-resident multiple-candidate occurrences | `0` |
| Observed frontmatter field count | `60` |
| Operator resolution identity/status | 60 field classifications accepted; 55 admitted; 5 excluded; 0 quarantined; 0 unresolved; private decision-matrix resolution `R001`: accepted |

The observer repository/commit is recorded as evidence identity only. CLEANROOM
does not import, imitate, or infer architectural authority from that
implementation.

The v3 observation was run twice. Both runs reproduced the same specimen
identity, and their serialized summary bytes were reported identical. Complete
serialized observation bytes are not expected to be identical because
`generated_at` is an intentional run timestamp. The pinned run-1 digest above
is an artifact-byte identity only; it is not a logical observation identity or
a specimen/source identity. No unrecoverable vault-manifest or logical
observation digest is used as current v3 audit authority.

### 1.1 Historical v2 predecessor boundary

The registry was originally resolved against the following immutable historical
observation:

| Item | Historical repository-safe value |
|---|---|
| Observer commit | `99d0d4556684000f0ed585e47158a5f7fe9ce7e1` |
| Observer schema | `vault-observation/v2` |
| Specimen identity | `25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688` |
| **HISTORICAL** private evidence JSON SHA-256 | `31cf9ba80fb947fc7bbd758d8e47e839cfb41b3d5cfe4fd6b7c9caa8e6c4fbde` |
| **HISTORICAL** private decision-matrix SHA-256 | `1adb15b094fef29a23aebd7308b476e4d3d4489a57d7e1993fc83ca6a963b36d` |
| **HISTORICAL** generator artifact SHA-256 | `11e23f64cbd7004d8ae3f2d4f9dcfb1627987069f13c181c3eaedf75dadfbc0f` |

Exact field-universe reconciliation against v3 established:

```text
historical fields: 60
current fields: 60
added: 0
removed: 0
unchanged: 60
```

The v2 boundary remains historical provenance; it is not the current corpus
actuality boundary. The three historical digests above identify the original
R001 evidence bundle and are not v3 artifact identities. Because the universe
is unchanged, the existing semantic classifications carry forward without a
new field-admission decision.

The historical v2 specimen identity uses the observer-defined source identity
over resident directory observations and the same six restricted resident
file-record fields listed in §1.2. It uses deterministic JSON serialization
with `sort_keys=True`, separators `(",", ":")`, `ensure_ascii=False`, UTF-8,
and SHA-256. This definition is historical and remains bound only to the v2
observer, schema, and specimen above.

### 1.2 Current v3 specimen identity canonicalization

The authored-vault specimen identity is the observer-defined
`vault_resident_snapshot_identity`. At observer commit
`e9bb2d95c14b1beb334dc2b8d83420f5998b9a53`, its input is:

```text
{
  "directories": <complete resident directory observation records>,
  "files": [
    {
      "relative_path": ...,
      "source_kind": ...,
      "extension": ...,
      "byte_size": ...,
      "source_byte_hash": ...,
      "text_decoding_status": ...
    },
    ...
  ]
}
```

Only vault-resident directory and file observations participate. Directory
records participate as complete resident records; file records are restricted
to the six fields shown above. The observer supplies deterministic
relative-path ordering. JSON object keys are recursively sorted with
`sort_keys=True`, compact separators are exactly `(",", ":")`,
`ensure_ascii=False` is used, and the serialization is UTF-8. The digest is
SHA-256 rendered as lowercase hexadecimal. Applying this procedure to both
retained v3 runs reproduced the recorded specimen identity exactly.

## 2. Decision totals

The accepted public registry in #10 §4.1 classifies every observed field exactly
once:

```text
observed fields: 60
admitted fields: 55
excluded fields: 5
quarantined fields: 0
unresolved semantic-registry decisions: 0
```

The five excluded fields are:

```text
address
email
phone
likes
dislikes
```

These totals describe the accepted operator classification. Their original
decision provenance is the historical R001 evidence bundle identified by the
three **HISTORICAL** private digests in §1.1; no new v3 decision-matrix or
generator bundle is implied.

## 3. Current v3 repository-safe authored-shape record

The current repaired substrate establishes the following canonical authored
cardinalities. These are structural authoring facts, not new semantic roles;
authored list order remains preserved source form without automatic ranking
significance.

| Canonical list-valued fields | Canonical scalar-valued fields |
|---|---|
| `creator`, `register_mode`, `from_mode`, `to_mode`, `unity_level` | `format`, `layer`, `vector_direction`, `register`, `pillar`, `hypnagogic_resonance`, `reactivity`, `relationship` |

The current temporal field rows preserve the distinction between parser-native
shape and semantic representation:

| Field | Current native shape evidence | Accepted semantic representations |
|---|---|---|
| `birthday` | date or string, as applicable | `FullDate`, `MonthDay` |
| `first_met` | date or datetime, as applicable | `FullDate`, `DateTime` |
| `original_year_published` | number or string, as applicable | `ExactYear`, `ApproximateYear` |
| `journal_entry_date` | date | `FullDate` only |

### 3.1 Current v3 repository-safe per-field audit export

The following complete table contains one row for each current v3 field,
exactly once. Counts are repository-safe structural evidence. Shapes are
mechanical observation; temporal representation is governed by `docs/10`;
wikilink counts do not create canonical relation classifications. No private
values or paths are exposed.

| Field | Whole-resident presence | Admission-eligible presence | Value shapes | Null count | Wikilink occurrence count | Temporal observation | Admission status |
|---|---:|---:|---|---:|---:|---|---|
| `address` | 23 | 23 | null=21, string=2 | 21 | 0 | — | excluded |
| `aliases` | 523 | 518 | array=518, null=5 | 5 | 0 | — | admitted |
| `architect_or_operator` | 80 | 80 | null=47, string=33 | 47 | 0 | — | admitted |
| `birthday` | 25 | 25 | date=1, null=21, string=3 | 21 | 0 | date/string | admitted |
| `book_read_today` | 210 | 210 | array=177, null=33 | 33 | 224 | — | admitted |
| `bridge_applicability_scope` | 12 | 12 | string=12 | 0 | 0 | — | admitted |
| `bridge_applied` | 12 | 12 | boolean=12 | 0 | 0 | — | admitted |
| `bridge_broken` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `bridge_conditions` | 12 | 12 | string=12 | 0 | 0 | — | admitted |
| `bridge_isomorphism` | 1 | 1 | boolean=1 | 0 | 0 | — | admitted |
| `bridge_justification` | 12 | 12 | string=12 | 0 | 0 | — | admitted |
| `bridge_methods` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `bridge_preservation` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `bridge_required` | 12 | 12 | boolean=12 | 0 | 0 | — | admitted |
| `canonical_name` | 25 | 25 | null=3, string=22 | 3 | 0 | — | admitted |
| `cash_out` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `creator` | 29 | 29 | array=29 | 0 | 0 | — | admitted |
| `dislikes` | 23 | 23 | null=23 | 23 | 0 | — | excluded |
| `dream_location` | 210 | 210 | null=31, string=179 | 31 | 0 | — | admitted |
| `dream_lucidity` | 210 | 210 | null=33, string=177 | 33 | 0 | — | admitted |
| `dream_motif` | 210 | 210 | array=194, null=16 | 16 | 277 | — | admitted |
| `dream_motif_valence` | 210 | 210 | null=32, string=178 | 32 | 0 | — | admitted |
| `email` | 23 | 23 | array=1, null=22 | 22 | 0 | — | excluded |
| `entity_type` | 25 | 25 | string=25 | 0 | 0 | — | admitted |
| `first_met` | 25 | 25 | date=8, datetime=1, null=16 | 16 | 0 | date/datetime | admitted |
| `format` | 29 | 29 | null=1, string=28 | 1 | 0 | — | admitted |
| `from_mode` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `from_register` | 12 | 12 | string=12 | 0 | 0 | — | admitted |
| `hypnagogic_resonance` | 210 | 210 | null=29, string=181 | 29 | 0 | — | admitted |
| `interface` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `iso_broken` | 1 | 1 | array=1 | 0 | 0 | — | admitted |
| `iso_justification` | 1 | 1 | string=1 | 0 | 0 | — | admitted |
| `iso_structure` | 1 | 1 | array=1 | 0 | 0 | — | admitted |
| `journal_entry_date` | 533 | 533 | date=523, null=10 | 10 | 0 | date | admitted |
| `layer` | 1043 | 1038 | null=466, string=577 | 466 | 0 | — | admitted |
| `likes` | 23 | 23 | null=23 | 23 | 0 | — | excluded |
| `note_type` | 1043 | 1038 | null=367, string=676 | 367 | 0 | — | admitted |
| `occupation` | 23 | 23 | null=20, string=3 | 20 | 0 | — | admitted |
| `origin` | 29 | 29 | null=19, string=10 | 19 | 0 | — | admitted |
| `original_year_published` | 29 | 29 | null=17, number=7, string=5 | 17 | 0 | year/string | admitted |
| `phone` | 23 | 23 | null=22, number=1 | 22 | 0 | — | excluded |
| `pillar` | 1031 | 1026 | null=465, string=566 | 465 | 0 | — | admitted |
| `publish_studio` | 29 | 29 | null=24, string=5 | 24 | 0 | — | admitted |
| `quarantine_reasons` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `reactivity` | 210 | 210 | null=29, string=181 | 29 | 0 | — | admitted |
| `recall_ability` | 210 | 210 | null=33, string=177 | 33 | 0 | — | admitted |
| `register` | 1043 | 1038 | null=466, string=577 | 466 | 0 | — | admitted |
| `register_mode` | 1038 | 1033 | array=573, null=465 | 465 | 0 | — | admitted |
| `relationship` | 25 | 25 | null=7, string=18 | 7 | 0 | — | admitted |
| `revision_triggers` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `speculation_quarantine` | 12 | 12 | boolean=12 | 0 | 0 | — | admitted |
| `stop_rule` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `tags` | 513 | 508 | array=501, null=12 | 12 | 0 | — | admitted |
| `temporal_pace` | 210 | 210 | null=37, string=173 | 37 | 0 | — | admitted |
| `title` | 29 | 29 | string=29 | 0 | 0 | — | admitted |
| `to_mode` | 12 | 12 | array=12 | 0 | 0 | — | admitted |
| `to_register` | 12 | 12 | string=12 | 0 | 0 | — | admitted |
| `unity_level` | 1027 | 1022 | array=560, null=467 | 467 | 0 | — | admitted |
| `uuid` | 1057 | 1052 | string=1057 | 0 | 0 | — | admitted |
| `vector_direction` | 1038 | 1033 | null=467, string=571 | 467 | 0 | — | admitted |

### 3.2 Historical v2 repository-safe per-field audit export

The following table contains one row for every observed field, exactly once.
Counts distinguish whole-resident presence from admission-eligible presence;
null means present-null and does not mean absent. Value shapes and temporal
columns are mechanical observations only. Wikilink counts are syntax evidence
and do not create semantic relation classifications.

| Field | Presence count | Value shapes | Null count | Wikilink occurrence count | Temporal-shape observation | Admission status |
|---|---:|---|---:|---:|---|---|
| `address` | whole-resident 23; admission-eligible 23 | null=21, string=2 | 21 | 0 | not mechanically temporal | excluded |
| `aliases` | whole-resident 526; admission-eligible 518 | array=521, null=5 | 5 | 0 | not mechanically temporal | admitted |
| `architect_or_operator` | whole-resident 80; admission-eligible 80 | null=47, string=33 | 47 | 0 | not mechanically temporal | admitted |
| `birthday` | whole-resident 25; admission-eligible 25 | null=21, string=4 | 21 | 0 | date-like observed | admitted |
| `book_read_today` | whole-resident 208; admission-eligible 208 | array=176, null=32 | 32 | 223 | not mechanically temporal | admitted |
| `bridge_applicability_scope` | whole-resident 12; admission-eligible 12 | string=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_applied` | whole-resident 12; admission-eligible 12 | boolean=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_broken` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_conditions` | whole-resident 12; admission-eligible 12 | string=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_isomorphism` | whole-resident 1; admission-eligible 1 | boolean=1 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_justification` | whole-resident 12; admission-eligible 12 | string=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_methods` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_preservation` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `bridge_required` | whole-resident 12; admission-eligible 12 | boolean=12 | 0 | 0 | not mechanically temporal | admitted |
| `canonical_name` | whole-resident 25; admission-eligible 25 | null=3, string=22 | 3 | 0 | not mechanically temporal | admitted |
| `cash_out` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `creator` | whole-resident 29; admission-eligible 29 | array=4, string=25 | 0 | 0 | not mechanically temporal | admitted |
| `dislikes` | whole-resident 23; admission-eligible 23 | null=23 | 23 | 0 | not mechanically temporal | excluded |
| `dream_location` | whole-resident 208; admission-eligible 208 | null=30, string=178 | 30 | 0 | not mechanically temporal | admitted |
| `dream_lucidity` | whole-resident 208; admission-eligible 208 | null=32, string=176 | 32 | 0 | not mechanically temporal | admitted |
| `dream_motif` | whole-resident 208; admission-eligible 208 | array=193, null=15 | 15 | 275 | not mechanically temporal | admitted |
| `dream_motif_valence` | whole-resident 208; admission-eligible 208 | null=31, string=177 | 31 | 0 | not mechanically temporal | admitted |
| `email` | whole-resident 23; admission-eligible 23 | array=1, null=22 | 22 | 0 | not mechanically temporal | excluded |
| `entity_type` | whole-resident 25; admission-eligible 25 | string=25 | 0 | 0 | not mechanically temporal | admitted |
| `first_met` | whole-resident 25; admission-eligible 25 | null=16, string=9 | 16 | 0 | date-like observed | admitted |
| `format` | whole-resident 29; admission-eligible 29 | array=1, null=1, string=27 | 1 | 0 | not mechanically temporal | admitted |
| `from_mode` | whole-resident 12; admission-eligible 12 | array=4, string=8 | 0 | 0 | not mechanically temporal | admitted |
| `from_register` | whole-resident 12; admission-eligible 12 | string=12 | 0 | 0 | not mechanically temporal | admitted |
| `hypnagogic_resonance` | whole-resident 208; admission-eligible 208 | array=1, null=28, string=179 | 28 | 0 | not mechanically temporal | admitted |
| `interface` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `iso_broken` | whole-resident 1; admission-eligible 1 | array=1 | 0 | 0 | not mechanically temporal | admitted |
| `iso_justification` | whole-resident 1; admission-eligible 1 | string=1 | 0 | 0 | not mechanically temporal | admitted |
| `iso_structure` | whole-resident 1; admission-eligible 1 | array=1 | 0 | 0 | not mechanically temporal | admitted |
| `journal_entry_date` | whole-resident 531; admission-eligible 531 | null=9, string=522 | 9 | 0 | date-like observed | admitted |
| `layer` | whole-resident 1045; admission-eligible 1037 | array=1, null=470, string=574 | 470 | 0 | not mechanically temporal | admitted |
| `likes` | whole-resident 23; admission-eligible 23 | null=23 | 23 | 0 | not mechanically temporal | excluded |
| `note_type` | whole-resident 1045; admission-eligible 1037 | null=371, string=674 | 371 | 0 | not mechanically temporal | admitted |
| `occupation` | whole-resident 23; admission-eligible 23 | null=20, string=3 | 20 | 0 | not mechanically temporal | admitted |
| `origin` | whole-resident 29; admission-eligible 29 | null=19, string=10 | 19 | 0 | not mechanically temporal | admitted |
| `original_year_published` | whole-resident 29; admission-eligible 29 | null=17, number=7, string=5 | 17 | 0 | year-like observed | admitted |
| `phone` | whole-resident 23; admission-eligible 23 | null=22, number=1 | 22 | 0 | not mechanically temporal | excluded |
| `pillar` | whole-resident 1033; admission-eligible 1025 | array=1, null=469, string=563 | 469 | 0 | not mechanically temporal | admitted |
| `publish_studio` | whole-resident 29; admission-eligible 29 | null=24, string=5 | 24 | 0 | not mechanically temporal | admitted |
| `quarantine_reasons` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `reactivity` | whole-resident 208; admission-eligible 208 | array=1, null=28, string=179 | 28 | 0 | not mechanically temporal | admitted |
| `recall_ability` | whole-resident 208; admission-eligible 208 | null=32, string=176 | 32 | 0 | not mechanically temporal | admitted |
| `register` | whole-resident 1045; admission-eligible 1037 | array=1, null=470, string=574 | 470 | 0 | not mechanically temporal | admitted |
| `register_mode` | whole-resident 1040; admission-eligible 1032 | array=35, null=469, string=536 | 469 | 0 | not mechanically temporal | admitted |
| `relationship` | whole-resident 25; admission-eligible 25 | array=2, null=7, string=16 | 7 | 0 | not mechanically temporal | admitted |
| `revision_triggers` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `speculation_quarantine` | whole-resident 12; admission-eligible 12 | boolean=12 | 0 | 0 | not mechanically temporal | admitted |
| `stop_rule` | whole-resident 12; admission-eligible 12 | array=12 | 0 | 0 | not mechanically temporal | admitted |
| `tags` | whole-resident 516; admission-eligible 508 | array=505, null=11 | 11 | 0 | not mechanically temporal | admitted |
| `temporal_pace` | whole-resident 208; admission-eligible 208 | null=36, string=172 | 36 | 0 | not mechanically temporal | admitted |
| `title` | whole-resident 29; admission-eligible 29 | string=29 | 0 | 0 | not mechanically temporal | admitted |
| `to_mode` | whole-resident 12; admission-eligible 12 | array=3, string=9 | 0 | 0 | not mechanically temporal | admitted |
| `to_register` | whole-resident 12; admission-eligible 12 | string=12 | 0 | 0 | not mechanically temporal | admitted |
| `unity_level` | whole-resident 1029; admission-eligible 1021 | array=1, null=471, string=557 | 471 | 0 | not mechanically temporal | admitted |
| `uuid` | whole-resident 1059; admission-eligible 1051 | string=1059 | 0 | 0 | not mechanically temporal | admitted |
| `vector_direction` | whole-resident 1040; admission-eligible 1032 | array=12, null=471, string=557 | 471 | 0 | not mechanically temporal | admitted |

No authored field values, private paths, aliases, raw target strings, or
representative private content are included in this export.

## 4. Public/private provenance rule

The repository-safe manifest may expose identities, exact private-artifact
digests, counts, redacted structural shapes, classification totals, and other
non-sensitive structural summaries. It must not expose private authored values
or reconstructable private paths. The three **HISTORICAL** digests in §1.1 are
SHA-256 values of the exact private filesystem bytes used for the original R001
v2 audit bundle; they are not current v3 artifact identities.

## 5. CLEANROOM Organon snapshot relationship

The CLEANROOM-resident `000-organon-of-finite-inquiry.md` is not the measured
authored-vault specimen for this registry. Its frontmatter visibly includes
`note_version`, `schema_version`, and `note_status`, while the accepted
authored-vault specimen identified above does not include those keys in its
observed 60-field universe.

The repository copy therefore cannot be used to enlarge or contradict the
accepted specimen's field universe. This is a snapshot/provenance distinction,
not a request to rewrite the Organon's source frontmatter in PR #12.
