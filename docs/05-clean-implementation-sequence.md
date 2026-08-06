# Clean Implementation Sequence

## Principle

Recover substrate authority without restarting the project. The Python system supplies an operator-attested recovery baseline; the clean-room task is to preserve and improve the demonstrated capability while grounding representations in the complete authored substrate.

The required order is:

```text
whole-corpus observation
→ contract-contact reconciliation
→ bounded contract amendment
→ real projection construction
→ complete-vault projection validation
→ real projection-access implementation
→ candidate activation revalidation
→ semantic-access inference
→ conformance
→ execution
→ packet and synthesis
→ private UAT
```

This does not restart CLEANROOM. Existing contracts, fold, fixture, and PR #9 work are preserved, while corpus-sensitive portions remain provisional.

## Authority domains

```text
Constitutive system meaning
    Organon
    explicit operator decisions

Runtime semantic authority
    accepted kernel equations
    accepted runtime invariants

Corpus actuality
    raw authored-vault structure and content
    including irregularities, absences, ambiguity,
    malformed structures, and unresolved authoring states

Representational authority
    corpus-validated semantic contracts

Implementation authority
    accepted implementation contracts
    within their explicitly declared responsibility
```

The domains are scoped. The Organon constrains system meaning; explicit operator decisions resolve open authoring or constitutive questions; the vault supplies actual authored instances; equations and invariants constrain deterministic runtime authority; contracts become representational authority only after corpus validation; implementations are authoritative only within accepted boundaries. A cross-domain conflict requires classification, retained evidence, an explicit operator or contract decision, and a bounded amendment.

## Evidence taxonomy

```text
raw authored-vault observations
Python materialization observations
runtime traces and measured behavior
private-UAT reports
synthetic fixtures
implementation source and diagnostics
operator attestations
```

Evidence can establish a required capability, concrete consumer, corpus phenomenon, failure mode, contradiction, unsupported assumption, performance or scale condition, or need for amendment. It does not silently become semantic authority. Python success does not prove Python architecture; private UAT validates behavior rather than internal implementation; synthetic fixtures establish local mechanics rather than corpus actuality; source and diagnostics establish what code does rather than what the system ought to mean.

## Phase 0 — Preserve the recovery boundary

1. Preserve `duck-lint/semantic-traversal` and its history as evidence of the Python system.
2. Preserve `duck-lint/CLEANROOM` and its existing contracts, fold, fixture, and PR #9 work.
3. Record exact repository, branch, corpus, suite, report, and implementation identities where available.
4. Do not restart, delete, rewrite, or import legacy runtime orchestration.
5. Freeze further semantic-access, conformance, execution, packet, and synthesis implementation until real projection access exists.

## Phase 1 — Record the operator-attested recovery baseline

The operator attests that complete-vault ingest occurred; real corpus questions were answered; exact, lexical, vector, graph, and temporal mechanisms operated; multi-turn private UAT exercised the system; and useful whole-corpus behavior resulted.

Classify this as an `operator-attested recovery baseline`. It does not establish Python ontology correctness, projected-affordance exhaustiveness, preservation of CLEANROOM equations or invariants, evidence-admission correctness, absence of accidental semantic/coherence gates, correctness of candidate Rust contracts or Python boundaries, or corpus-scale suitability of candidate Rust activation.

The recovery-baseline manifest records, where available:

```text
python_repository_commit
corpus_snapshot_identity
inventory_identity
materialization_or_index_identity
private_uat_suite_identities
report_identities
demonstrated_capabilities
known_failures
known_architectural_violations
unmeasured_areas
private_artifact_hashes_or_locations
attestation_timestamp
```

Unknown or unavailable identities are `unknown`; no historical identity is inferred or fabricated.

## Phase 2 — Observe the complete authored substrate

Observation precedes semantic classification or admission. The observation bundle exposes, as available:

```text
source path
source identity and UUID observation
raw frontmatter keys
raw frontmatter value shapes
frontmatter source provenance
heading hierarchy
authored Markdown block candidates
raw Markdown
wikilinks
aliases
embeds
heading targets
block targets
source spans
parse failures
unsupported syntax
ambiguous targets
unresolved targets
source hashes
corpus snapshot identity
```

Do not call these facts semantic objects, semantic units, identifiers, or occurrences yet. A relevant observation is then classified as:

```text
admitted
quarantined
rejected
unresolved
open_authoring_decision
```

Admission retains the raw observation, classification rule, rule authority, information preserved, and information lost or excluded.

Python chunks, graph nodes, anchors, and indexes are materialization observations. A chunk may correspond to an authored unit, be a transport split, reveal a defect, or remain unresolved. Naming a record does not settle its semantic status.

## Phase 3 — Reconcile contract contact

For every material corpus-sensitive claim, produce a human-readable mapping containing:

```text
contract claim
authority domain
raw authored example
Python materialization observation
runtime or UAT evidence, when relevant
proposed CLEANROOM representation
bridge rule
positive example
ugly or irregular example
counterexample
information preserved
information lost or excluded
unresolved decision
status
required amendment
```

The report must make inspectable mappings possible, including:

```text
this note
this heading
this frontmatter value
this authored block
this link occurrence
this unresolved target

→

this candidate object
this candidate region
this candidate unit
this candidate identifier assignment
this candidate occurrence
this explicit unresolved representation
```

Private content may remain local. Repository-safe output may contain counts, hashes, redacted paths or stable local references, classification totals, unresolved-decision totals, and structural failure summaries. A polished abstract matrix without inspectable local mappings is insufficient.

For each claim, use the lifecycle states `candidate`, `provisionally accepted`, `corpus-validated`, or `superseded`. Corpus-independent mechanics may become accepted without whole-corpus validation when their boundary genuinely does not depend on corpus structure. Corpus-sensitive contracts remain provisional until raw observation, admission, projection construction, real projection access, and applicable UAT validate them. Validation is scoped to a versioned corpus snapshot and admitted evidence boundary; later corpus changes may require revalidation without retroactively invalidating the prior snapshot result.

## Phase 4 — Amend only what evidence requires

Each bounded amendment identifies the prior claim, evidence, bridge rule, smallest correction, information preserved or lost, affected contracts and tests, migration implications, and unresolved decisions. Do not add abstractions, fallbacks, compatibility paths, or extension points without a concrete consumer.

## Phase 5 — Construct the real projection

Build a versioned read-only boundary:

```text
WholeCorpusObservationBundle
    → SemanticSpaceProjection
```

Construction may map admitted authored facts to canonical objects, regions, units, identifiers, occurrences, anchors, and surfaces. It must retain provenance and represent ambiguity, malformed structures, unsupported syntax, and unresolved targets explicitly. It must not invent mappings, guess equivalence, or silently promote Python chunks to authored semantic units.

## Phase 6 — Validate the complete-vault projection

Validate object, region, unit, identifier, occurrence, target, reverse-incidence, temporal, surface, provenance, transport-segmentation, bound, and deterministic-identity closure. Every admitted fact is represented or explicitly rejected, and every unresolved or ambiguous fact remains explicit. Produce a whole-corpus projection validation report with counts, hashes, statuses, and structural failures.

## Phase 7 — Build the real read-only projection-access boundary

This is a distinct implementation phase after complete-vault projection validation:

```text
validated SemanticSpaceProjection
+ actual corpus-derived surface indexes or providers
+ typed activation probe
→ mechanically computed ProjectionActivationProbeResult
```

The boundary is the real `ProjectionActivationAccess` implementation required before candidate activation revalidation.

The boundary supports every actual available combination of exact, lexical, vector, graph, temporal, and declared match modes, where present. Results are computed from actual corpus-derived surfaces and indexes; they are not hand-authored expected candidates.

The concrete implementation is intentionally undecided in this documentation correction. Permitted future implementations include Rust read-only indexes, a bounded Python process or protocol, deterministic exported query indexes, or another explicit versioned read-only boundary.

This boundary performs probes needed by initial activation. Later semantic-access execution adapters execute conforming traversal plans and hydrate outputs. They are not the same responsibility.

## Phase 8 — Revalidate candidate activation

Only after Phase 7, test the candidate activation implementation against the real projection-access result, including actual counts, empty objects, heterogeneous identifiers, deep headings, duplicate titles, unresolved links, reverse incidence, hydration addresses, zero-result surfaces, bounds, and every actual surface/match-mode combination. PR #9 remains untouched and provisional.

## Phase 9 — Resume frozen semantic-access work

After activation revalidation, proceed in order through semantic-access inference, structural conformance, execution adapters, packet assembly, synthesis, and private UAT. Execution must preserve canonical identity and provenance; packet removal must be mechanical and measured; synthesis receives results without a post-retrieval semantic veto.

## Phase 10 — Private UAT and integration

Use fresh suite identities and compare requested distinctions, bindings, paths, unit identities, coverage, evidence, and failure classification. Timing is telemetry, not acceptance. Only after corpus validation and UAT may specific Python producers, adapters, or operators be reused across explicit boundaries.

The end state remains one production kernel authority. Python may remain evidence, a local observer, a bounded exporter, or evaluation/migration tooling. No Python runtime port is authorized merely to preserve implementation ancestry.
