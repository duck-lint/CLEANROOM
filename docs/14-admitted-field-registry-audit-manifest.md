# Admitted-Field Registry Audit Manifest

## Status

**Purpose:** repository-safe provenance linkage for the 60-field admitted-field
registry recorded in `10-organon-vault-substrate-chunking-map.md` §4.1.

**Authority boundary:** this file records evidence identity and safe structural
summaries. It does not create semantic roles, retrieval surfaces,
embedding/index policy, ranking behavior, or transport policy.

**Audit-completeness state:** **COMPLETE REPOSITORY-SAFE LINKAGE.** Public
structural audit linkage is complete for the accepted observer/specimen pair.
Private authored contents remain intentionally private. This manifest does not
reproduce the private evidence bundle, and does not acquire semantic or
representational authority by linking evidence. Corpus actuality is supplied by
the versioned authored-vault observation; explicit operator decisions resolve
constitutive questions; representational authority resides in the accepted,
corpus-validated CLEANROOM contracts.

## 1. Observation identity

| Item | Repository-safe value |
|---|---|
| Observer repository | `duck-lint/semantic-traversal` |
| Observer commit | `99d0d4556684000f0ed585e47158a5f7fe9ce7e1` |
| Observer schema identifier/version | `vault-observation/v2` |
| Authored-vault specimen identity | `25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688` |
| Specimen identity algorithm/version | `SHA-256 over the UTF-8 JSON serialization of {directories: resident directory observations, files: resident file records restricted to relative_path, source_kind, extension, byte_size, source_byte_hash, text_decoding_status}; serialization uses sort_keys=true, separators=(',', ':'), ensure_ascii=false; observer schema vault-observation/v2` |
| Whole-resident Markdown/source count | `1059` |
| Admission-eligible Markdown count | `1051` |
| Valid frontmatter count | `1059` |
| Malformed-frontmatter count | `0` |
| Parseable UUID count | `1059` |
| UUIDv7 count | `1059` |
| Duplicate UUID groups | `0` |
| Whole-resident total authored-link occurrences | `4994` |
| Whole-resident one-candidate occurrences | `4893` |
| Whole-resident zero-candidate occurrences | `101` |
| Whole-resident multiple-candidate occurrences | `0` |
| Observed frontmatter field count | `60` |
| Private evidence JSON SHA-256 | `31cf9ba80fb947fc7bbd758d8e47e839cfb41b3d5cfe4fd6b7c9caa8e6c4fbde` |
| Private decision-matrix SHA-256 | `1adb15b094fef29a23aebd7308b476e4d3d4489a57d7e1993fc83ca6a963b36d` |
| Generator artifact SHA-256 | `11e23f64cbd7004d8ae3f2d4f9dcfb1627987069f13c181c3eaedf75dadfbc0f` |
| Operator resolution identity/status | 60 field classifications accepted; 55 admitted; 5 excluded; 0 quarantined; 0 unresolved; private decision-matrix resolution `R001`: accepted |

The observer repository/commit is recorded as evidence identity only. CLEANROOM
does not import, imitate, or infer architectural authority from that
implementation.

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

These totals describe the accepted operator classification. They are linked to
the private evidence bundle by the exact filesystem-byte digests above.

## 3. Repository-safe per-field audit export

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
or reconstructable private paths. The three digests above are SHA-256 values of
the exact final private filesystem bytes used for this accepted audit bundle.

## 5. CLEANROOM Organon snapshot relationship

The CLEANROOM-resident `000-organon-of-finite-inquiry.md` is not the measured
authored-vault specimen for this registry. Its frontmatter visibly includes
`note_version`, `schema_version`, and `note_status`, while the accepted
authored-vault specimen identified above does not include those keys in its
observed 60-field universe.

The repository copy therefore cannot be used to enlarge or contradict the
accepted specimen's field universe. This is a snapshot/provenance distinction,
not a request to rewrite the Organon's source frontmatter in PR #12.
