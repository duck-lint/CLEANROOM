# Admitted-Field Registry Audit Manifest

## Status

**Purpose:** repository-safe provenance linkage for the 60-field admitted-field registry recorded in `10-organon-vault-substrate-chunking-map.md` §4.1.

**Authority boundary:** this file is an audit manifest. It records evidence identity and safe structural summaries; it does not create semantic roles, retrieval surfaces, embedding/index policy, ranking behavior, or transport policy.

**Audit-completeness state:** **PARTIAL PUBLIC LINKAGE.** The operator classifications are accepted for the versioned specimen, but several private-bundle digest fields have not been supplied to this CLEANROOM workspace. They are left explicitly unresolved rather than fabricated. This manifest must not be cited as proof of complete public audit linkage until those fields are populated verbatim from the private audit bundle.

## 1. Observation identity

| Item | Repository-safe value |
|---|---|
| Observer repository | `duck-lint/semantic-traversal` |
| Observer commit | `99d0d4556684000f0ed585e47158a5f7fe9ce7e1` |
| Observer schema identifier/version | **not supplied to CLEANROOM** |
| Authored-vault specimen identity | `25fb8f13dd17efb62abbb52c48f526bc0aedd887b29001c1e60f1642d322b688` |
| Specimen identity algorithm/version | **not supplied to CLEANROOM** |
| Whole-corpus source count | **not supplied to CLEANROOM** |
| Admission-eligible Markdown count | **not supplied to CLEANROOM** |
| Observed frontmatter field count | `60` |
| Private evidence JSON SHA-256 | **not supplied to CLEANROOM** |
| Private decision-matrix SHA-256 | **not supplied to CLEANROOM** |
| Generator artifact SHA-256 | **not supplied to CLEANROOM** |
| Operator resolution identity/status | explicit operator review recorded for PR #12; 60 field classifications accepted; zero semantic registry decisions unresolved |

The observer repository/commit is recorded as evidence identity only. CLEANROOM does not import, imitate, or infer architectural authority from that implementation.

## 2. Decision totals

The accepted public registry in #10 §4.1 classifies every observed field exactly once:

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

These totals describe the accepted operator classification, not the missing private evidence counters identified above.

## 3. Required safe per-field audit export

The private audit bundle is expected to export one repository-safe row for every observed field with no authored values, names, prose, contact information, or private paths:

| Column | Required meaning |
|---|---|
| `field` | exact observed frontmatter key |
| `presence_count` | number of observed sources carrying the key |
| `value_shapes` | redacted structural value-shape set only |
| `null_count` | present-null/blank count under the observer schema |
| `wikilink_occurrence_count` | mechanically observed canonical-link syntax count |
| `temporal_shape` | mechanical temporal-shape flag/result, not semantic interpretation |
| `admission_status` | accepted `admitted` or `excluded` classification |

Those rows are **not reproduced here yet** because the private-bundle export and its hashes are not available in this workspace. A later bounded documentation-only amendment may populate this section from the private audit bundle without changing the 60 semantic classifications. The one corpus statistic explicitly attested during review is:

```text
book_read_today
    presence_count: 208
    carrying note_type: journal_entry for all 208 observed carriers
```

That statistic narrows the accepted applicability of `book_read_today` in #10 §4.1; it does not authorize extrapolation to other fields.

## 4. Public/private provenance rule

The repository-safe manifest may expose identities, hashes, counts, redacted value shapes, classification totals, and other non-sensitive structural summaries. It must not expose private authored values or reconstructable private paths.

A missing private-artifact hash is represented as missing. It is never inferred from filenames, commit dates, neighboring hashes, or implementation output.

## 5. CLEANROOM Organon snapshot relationship

The CLEANROOM-resident `000-organon-of-finite-inquiry.md` is not the measured authored-vault specimen for this registry. Its frontmatter visibly includes `note_version`, `schema_version`, and `note_status`, while the accepted authored-vault specimen identified above does not include those keys in its observed 60-field universe.

The repository copy therefore cannot be used to enlarge or contradict the accepted specimen's field universe. This is a snapshot/provenance distinction, not a request to rewrite the Organon's source frontmatter in PR #12.
