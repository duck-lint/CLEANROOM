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

## Bounded amendment record — projection/runtime separation closure

### Prior claim

The superseded projection contract coupled runtime configuration to projection
identity and routine projection surface bounds, while leaving the status of the
five-surface universe insufficiently explicit.

### Evidence

The kernel/runtime architecture expresses activation as later configured access
over a frozen projection. Complete-vault projection construction and validation
treat \(M_\sigma\) as the deterministic representation of the accepted corpus.
Red-team review exposed the remaining projection/runtime-configuration coupling.
Recovery evidence demonstrated operation of five mechanisms but did not itself
establish projection exhaustiveness. The explicit operator decision recorded by
this amendment establishes exact, lexical, vector, graph, and temporal as the
constitutive structural surface families of Semantic Traversal \(M_\sigma\).

### Bridge rule

```text
corpus actuality
    determines represented instances

accepted constitutive contracts + explicit operator decisions
    determine representation architecture

runtime configuration
    governs later bounded use
```

### Smallest correction

Remove runtime configuration from projection identity and routine access
budgets from projection constitutive metadata. Explicitly establish the five
canonical structural surface families, preserve runtime configuration as a
separate later binding, and do not define provider/index materialization
identity before Phase 7 provides concrete implementation evidence.

### Information preserved

The correction preserves complete projection semantics; canonical
object/region/unit/identifier/occurrence/temporal representation; the
five-surface multiplex architecture; bounded runtime access; later runtime
configuration identity; provider/index implementation freedom; provenance; and
deterministic identity requirements.

### Information superseded or lost

It supersedes projection-owned runtime configuration identity, routine
projection candidate default/hard bounds, runtime authority to determine
whether a canonical surface exists, and the interpretation that the five
surfaces exist only where present. It does not remove runtime operating bounds
or the possibility of a future intrinsic provider limitation.

### Affected contracts and tests

Corrected in this PR: the projection requirements, implementation sequence,
clean-room protocol, Rust assessment, semantic-access language,
projection-activation/access documentation, and directly contradictory
runtime/construction-report wording. Known downstream reconciliation:

- `src/projection.rs`: remove projection-owned
  `configuration_snapshot_id` and routine surface candidate-limit fields;
  reconcile `RetrievalSurfaceDescriptor.available` when it means executable
  provider/index availability; classify `CoverageSemantics::AvailabilityOnly`
  rather than assuming it means provider presence; and remove or redefine
  provider-status text in `technical_limitations` so that only structural
  limitations remain projection state;
- `src/construction.rs`: remove the phase-5 construction configuration
  identity and projection-level candidate-limit initialization; reconcile
  `available: false` provider-state initialization, the
  `phase5:construction:no-indexes` identity, `AvailabilityOnly` initialization,
  and provider-status `technical_limitations`;
- `schemas/semantic-space-projection.schema.json`: remove the projection
  configuration field and routine projection-level candidate-limit fields;
  reconcile the serialized `available`, `coverage_semantics`, and
  `technical_limitations` representations under the same structural/provider
  boundary;
- `tests/support/synthetic_projection.rs`, `tests/synthetic_projection.rs`,
  and `tests/contracts.rs`: reconcile projection fixture construction,
  assertions, and serialized expectations for `available`,
  `AvailabilityOnly`, provider-status `technical_limitations`, and the
  removed configuration/limit fields;
- `tests/activation_contracts.rs`: separate runtime configuration-context
  checks from superseded projection-configuration matching;
- `schemas/activated-projection.schema.json`, `schemas/continuation-handle.schema.json`,
  and activation/conformance fixtures: preserve valid runtime configuration
  bindings while removing any projection-owned assumption;
- `src/activation.rs`, `src/semantic_access.rs`,
  `schemas/projection-activation-config.schema.json`,
  `schemas/semantic-access-plan.schema.json`,
  `schemas/synthesis-input.schema.json`, and
  `schemas/conformance-result.schema.json`: verify that configuration remains
  later runtime policy and that no projection-level hard bound is assumed;
- the serialized Phase 5 projection and Phase 6 validation inputs: rebuild and
  re-establish hashes if the corrected representation changes their shape.

These are known downstream consequences, not changes made here. PR #21 is not
modified.

#### Category A — must be reconciled after this amendment, before Phase 6

The projection-layer mismatches are specifically:

- `src/projection.rs`: `RetrievalSurfaceDescriptor.available`,
  `CoverageSemantics::AvailabilityOnly`, and provider-status entries in
  `technical_limitations`, in addition to the removed configuration and
  routine-limit fields;
- `src/construction.rs`: `available: false`,
  `CoverageSemantics::AvailabilityOnly`, provider-status
  `technical_limitations`, `phase5:construction:no-indexes`, and projection
  candidate-limit initialization;
- `schemas/semantic-space-projection.schema.json`: the serialized forms of
  all of those fields and variants;
- `tests/support/synthetic_projection.rs`, `tests/synthetic_projection.rs`,
  and `tests/contracts.rs`: all corresponding fixture values, assertions, and
  serialization expectations;
- the Phase 5 serialized projection and its logical/byte identity records if
  the corrected representation changes their shape.

The following locations require explicit classification during that same
reconciliation rather than automatic deletion:

- `src/activation.rs`,
  `schemas/activated-projection.schema.json`,
  `schemas/activated-identifier-assignment-record.schema.json`,
  `schemas/activated-occurrence-record.schema.json`, and their tests: the
  `available_surface_ids` fields currently describe surfaces structurally
  capable of inspecting a record and may remain at that layer; any use that
  instead means provider executability must be corrected;
- `tests/activation_contracts.rs`,
  `schemas/projection-activation-violation.schema.json`,
  `src/execution.rs`, `src/packet.rs`,
  `schemas/execution-limits.schema.json`, and
  `schemas/conformance-result.schema.json`: unavailable/provider/access
  statuses are valid runtime facts when scoped to a concrete operation or
  probe, but must not be interpreted as removal of a canonical surface from
  \(M_\sigma\).

#### Category B — valid later runtime/access state

`src/activation.rs`, `src/semantic_access.rs`, `src/execution.rs`,
`src/packet.rs`, the activation/access/conformance schemas, and their tests may
continue to carry runtime configuration identity, operating limits, provider
failure, invocation unavailability, unsupported adapter modes, failed probes,
and unavailable continuation. These describe concrete access state, not
projection surface existence.

#### Category C — deliberately deferred Phase 7 materialization questions

Only the following remain deferred: whether concrete provider/index state is
uniquely determined by a fixed derivation contract over (M_\sigma\), or instead
requires an independent frozen identity/handle; and what identity,
immutability, and telemetry semantics the concrete production access machinery
actually demonstrates. Provider-status leakage into projection is not deferred.

The governing classification is:

```text
surface structural existence in M_sigma
    = projection state

record-level surface applicability
    = projected structural fact for that record

provider/index executability and invocation success
    = later access/runtime state
```

Every complete projection structurally contains the exact, lexical, vector,
graph, and temporal surface families. Provider absence must not be represented
as absence of one of those families. `CoverageSemantics::AvailabilityOnly`
requires implementation classification: it may remain only if it expresses a
provider-independent structural coverage guarantee; if it means merely that a
provider exists, it is stale and must be removed or replaced during the
downstream reconciliation. Likewise, projection `technical_limitations` may
retain a limitation constitutive of the projected affordance, but not status
text such as “no executable index or provider.”

### Migration implications

```text
merge this authority amendment
    ↓
bounded Rust/schema/test reconciliation
    ↓
remove/migrate all runtime/provider state from the projection representation
    ↓
reconstruct Phase 5 projection if serialized shape changes
    ↓
re-establish Phase 5 hashes and identity
    ↓
resume Phase 6 validation against corrected authority
```

The existing Phase 5 serialized projection was constructed under the
superseded representation containing `configuration_snapshot_id`, routine
projection-level candidate limits, `RetrievalSurfaceDescriptor.available`
where tied to executable provider/index state,
`CoverageSemantics::AvailabilityOnly` where tied to provider availability, and
provider/index status encoded in projection `technical_limitations`. If
reconciliation changes its serialized shape, its logical hash, byte SHA-256,
and projection snapshot output must be regenerated from the same accepted
observation; preserving the old hashes is not a migration goal.

### Unresolved decisions

Provider/index materialization identity semantics remain deliberately
undecided until Phase 7 creates concrete production access state whose
behavior supplies evidence that such an identity is required. No independent
provider/index identity is added to the projection, activation, access plan,
continuation, conformance, or telemetry contracts by this amendment.

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

`materialization_or_index_identity` is a historical observed recovery-manifest
field where available. It does not establish a required future CLEANROOM
runtime identity; unknown historical values remain `unknown`, and no future
materialization identity is inferred or fabricated.

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

Phase 7 is not complete until real read-only access exists for the exact,
lexical, vector, graph, and temporal surface families over the validated
projection. Each surface retains its own structurally applicable record kinds
and declared match modes. A corpus may legitimately yield zero results for a
surface or operation. Results are computed from actual corpus-derived surfaces
and indexes; they are not hand-authored expected candidates.

The concrete implementation is intentionally undecided in this documentation correction. Permitted future implementations include Rust read-only indexes, a bounded Python process or protocol, deterministic exported query indexes, or another explicit versioned read-only boundary.

This boundary performs probes needed by initial activation. Later semantic-access execution adapters execute conforming traversal plans and hydrate outputs. They are not the same responsibility.

## Phase 8 — Revalidate candidate activation

Only after Phase 7, test the candidate activation implementation against the real projection-access result, including actual counts, empty objects, heterogeneous identifiers, deep headings, duplicate titles, unresolved links, reverse incidence, hydration addresses, zero-result surfaces, intrinsic provider limitations if any, and every declared surface/match-mode combination. Routine runtime budgets remain activation/access policy, not projection bounds. PR #9 remains untouched and provisional.

## Phase 9 — Resume frozen semantic-access work

After activation revalidation, proceed in order through semantic-access inference, structural conformance, execution adapters, packet assembly, synthesis, and private UAT. Execution must preserve canonical identity and provenance; packet removal must be mechanical and measured; synthesis receives results without a post-retrieval semantic veto.

## Phase 10 — Private UAT and integration

Use fresh suite identities and compare requested distinctions, bindings, paths, unit identities, coverage, evidence, and failure classification. Timing is telemetry, not acceptance. Only after corpus validation and UAT may specific Python producers, adapters, or operators be reused across explicit boundaries.

The end state remains one production kernel authority. Python may remain evidence, a local observer, a bounded exporter, or evaluation/migration tooling. No Python runtime port is authorized merely to preserve implementation ancestry.
