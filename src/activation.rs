//! Positive-only activated-projection contracts.
//!
//! These records represent a bounded working view over one frozen projection.
//! Visibility does not mean relevance. Absence from this view does not mean
//! absence from the corpus. An activated projection has no authority to make
//! negative claims, choose evidence, or implement activation behavior.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        Direction, OccurrenceId, RecordProvenance, RetrievalSurfaceKind, SemanticAddress,
        SemanticObjectId, SemanticRegionAddress, SemanticUnitId, SourceSpan, TemporalAnchorId,
    },
    problem_space::ActivationBand,
    projection::{
        AuthoredBlockType, IdentifierValue, OccurrencePresentation, OccurrenceSource,
        ProjectionValidationStatus, SurfaceMatchMode, TemporalValue,
    },
};

/// Newest conversational utterance supplied to deterministic activation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivationUtterance {
    /// Stable identity of the newest user utterance.
    pub utterance_id: String,
    /// Complete newest utterance text supplied to deterministic activation.
    pub text: String,
}

/// Activation bounds and surface limits paired with one projection configuration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionActivationConfig {
    /// Configuration snapshot paired with the projection.
    pub configuration_snapshot_id: String,
    /// Bounds for newest-utterance, whole-space-constraint, and configured-default seeds.
    pub unbanded: ProjectionActivationBandConfig,
    /// Bounds applied to primary problem-region activation.
    pub primary: ProjectionActivationBandConfig,
    /// Bounds applied to secondary problem-region activation.
    pub secondary: ProjectionActivationBandConfig,
    /// Bounds applied to tertiary problem-region activation.
    pub tertiary: ProjectionActivationBandConfig,
    /// Bounds applied to background problem-region activation.
    pub background: ProjectionActivationBandConfig,
    /// Per-surface candidate limits for every available projected surface.
    pub surface_limits: Vec<ProjectionActivationSurfaceConfig>,
    /// Total typed expansion budget available to the later semantic-access session.
    ///
    /// Initial activation records this budget but does not consume it. Expansion
    /// execution belongs to Phase 5.
    pub maximum_expansion_budget: u64,
    /// Degree at which a visible address is represented as a hub summary.
    pub hub_degree_threshold: u64,
    /// Maximum structural-transition depth during initial activation.
    pub maximum_initial_relation_depth: u32,
    /// Maximum page size carried by a continuation handle.
    pub continuation_page_limit: u32,
    /// Total activated-object bound.
    pub maximum_activated_objects: u32,
    /// Total activated-region bound.
    pub maximum_activated_regions: u32,
    /// Total activated-unit bound.
    pub maximum_activated_units: u32,
    /// Total activated-identifier-assignment bound.
    pub maximum_activated_identifier_assignments: u32,
    /// Total activated-occurrence bound.
    pub maximum_activated_occurrences: u32,
    /// Total activated-temporal-anchor bound.
    pub maximum_activated_temporal_anchors: u32,
    /// Total activated-edge bound.
    pub maximum_activated_edges: u32,
    /// Total telemetry-record bound.
    pub maximum_telemetry_records: u32,
    /// Total continuation-handle bound.
    pub maximum_continuation_handles: u32,
}

/// Per-band deterministic activation bounds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionActivationBandConfig {
    /// Maximum textual seeds consumed from this activation group.
    pub maximum_textual_seeds: u32,
    /// Maximum structural neighbours previewed for one visible address.
    pub maximum_structural_neighbors_per_record: u32,
    /// Maximum directly contained units previewed for one activated region.
    pub maximum_visible_units_per_region: u32,
    /// Maximum Unicode scalar values retained in one unit text preview.
    pub text_preview_character_limit: u32,
}

/// Per-surface candidate limits for every activation band.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionActivationSurfaceConfig {
    /// Concrete projected retrieval-surface identity.
    pub surface_id: String,
    /// Candidate limit for unbanded activation.
    pub unbanded_candidate_limit: u32,
    /// Candidate limit for primary activation.
    pub primary_candidate_limit: u32,
    /// Candidate limit for secondary activation.
    pub secondary_candidate_limit: u32,
    /// Candidate limit for tertiary activation.
    pub tertiary_candidate_limit: u32,
    /// Candidate limit for background activation.
    pub background_candidate_limit: u32,
}

/// Bounded positive working view over one frozen semantic-space projection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedProjection {
    /// Frozen projection snapshot from which the view was drawn.
    pub projection_snapshot_id: String,
    /// Configuration snapshot that bounded the view.
    pub configuration_snapshot_id: String,
    /// Thread whose problem space shaped this view.
    pub problem_space_thread_id: String,
    /// Exact problem-space version used for activation.
    pub problem_space_version: u64,
    /// Newest utterance identity used for activation.
    pub newest_utterance_id: String,
    /// Positive object records visible in the current working view.
    pub activated_objects: Vec<ActivatedObjectRecord>,
    /// Positive region records visible in the current working view.
    pub activated_regions: Vec<ActivatedRegionRecord>,
    /// Positive unit records visible in the current working view.
    pub activated_units: Vec<ActivatedUnitRecord>,
    /// Positive identifier-assignment records visible in the working view.
    pub activated_identifier_assignments: Vec<ActivatedIdentifierAssignmentRecord>,
    /// Positive authored-occurrence records visible in the working view.
    pub activated_occurrences: Vec<ActivatedOccurrenceRecord>,
    /// Positive temporal-anchor records visible in the working view.
    pub activated_temporal_anchors: Vec<ActivatedTemporalAnchorRecord>,
    /// Typed visible structural connections.
    pub edges: Vec<ActivatedEdge>,
    /// Measured projection-access telemetry.
    pub telemetry: Vec<ProjectionTelemetry>,
    /// Bounded continuation handles for inspectable omitted structure.
    pub continuation_handles: Vec<ContinuationHandle>,
}

/// Visible summary of one canonical semantic object.
///
/// It exposes identity and bounded structural previews only. It does not hydrate
/// evidence, score relevance, or imply completeness.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedObjectRecord {
    /// Canonical semantic object identity.
    pub object_id: SemanticObjectId,
    /// Canonical or authored title surface.
    pub title: String,
    /// Non-canonical aliases visible for discovery.
    pub aliases: Vec<String>,
    /// Projected semantic-object class.
    pub object_class: String,
    /// Bounded visible authored-region topology.
    pub visible_region_addresses: Vec<SemanticRegionAddress>,
    /// Bounded visible contained units.
    pub visible_unit_ids: Vec<SemanticUnitId>,
    /// Identifier assignment identities currently visible.
    pub visible_identifier_assignment_ids: Vec<String>,
    /// Count of contained semantic regions in the full snapshot.
    pub contained_region_count: u64,
    /// Count of contained semantic units in the full snapshot.
    pub contained_unit_count: u64,
    /// Bounded incoming-occurrence count.
    pub incoming_occurrence_count: u64,
    /// Bounded outgoing-occurrence count.
    pub outgoing_occurrence_count: u64,
    /// Available surface identities visible for planning.
    pub available_surface_ids: Vec<String>,
    /// Provenance explaining why this object became visible.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Visible summary of one authored semantic region.
///
/// It preserves region identity and bounded contained-address previews but does
/// not select units, collapse headings, or assert relevance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedRegionRecord {
    /// Canonical semantic-region address.
    pub address: SemanticRegionAddress,
    /// Complete authored heading path.
    pub heading_path: Vec<String>,
    /// Stable projected heading identity.
    pub heading_identity: String,
    /// Visible inherited identifier assignments.
    pub visible_identifier_assignment_ids: Vec<String>,
    /// Bounded preview of directly contained canonical units.
    pub visible_unit_ids: Vec<SemanticUnitId>,
    /// Full-snapshot count of contained units.
    pub contained_unit_count: u64,
    /// Available surface identities.
    pub available_surface_ids: Vec<String>,
    /// Provenance explaining visibility.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Visible summary of one canonical semantic unit.
///
/// It exposes identity, bounded preview text, incidence summaries, anchors, and
/// surfaces for planning. Full authored prose remains execution material; this
/// record cannot admit or reject evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedUnitRecord {
    /// Canonical semantic-unit identity.
    pub unit_id: SemanticUnitId,
    /// Canonical parent object identity.
    pub parent_object_id: SemanticObjectId,
    /// Canonical authored region address.
    pub parent_region_address: SemanticRegionAddress,
    /// Authored source-block category.
    pub authored_block_type: AuthoredBlockType,
    /// Complete authored heading path.
    pub heading_path: Vec<String>,
    /// Visible inherited identifier assignments.
    pub visible_inherited_identifier_assignment_ids: Vec<String>,
    /// Visible unit-local identifier assignments.
    pub visible_unit_local_identifier_assignment_ids: Vec<String>,
    /// Activation-time text visibility without evidence hydration.
    pub text_preview: ActivatedTextPreview,
    /// Count of incoming authored occurrences.
    pub incoming_occurrence_count: u64,
    /// Count of outgoing authored occurrences.
    pub outgoing_occurrence_count: u64,
    /// Count of represented temporal anchors.
    pub temporal_anchor_count: u64,
    /// Available surface identities.
    pub available_surface_ids: Vec<String>,
    /// Provenance explaining visibility.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Typed edge visible in an activated projection.
///
/// It mirrors represented projection structure and cannot invent a relation,
/// execute traversal, or authorize semantic interpretation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedEdge {
    /// Snapshot-local visible edge identity.
    pub edge_id: String,
    /// Canonical source address.
    pub source: SemanticAddress,
    /// Stable represented transition identity.
    pub transition_id: String,
    /// Explicit direction exposed to planning.
    pub direction: Direction,
    /// Canonical target address.
    pub target: SemanticAddress,
    /// Provenance explaining visibility.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Bounded activation-time visibility of one semantic unit's text.
///
/// Inline previews are planning material taken from represented normalized text.
/// Unavailable content requires later typed hydration and is not read during
/// activation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ActivatedTextPreview {
    /// Bounded normalized text was available directly in the projection.
    Inline {
        /// Mechanically bounded normalized text.
        text: String,

        /// Whether represented normalized text continued beyond this preview.
        truncated: bool,
    },

    /// The projection carries only a deterministic hydration address.
    ///
    /// Activation does not dereference that address.
    UnavailableWithoutHydration,
}

/// Source of positive activation for one visible record or edge.
///
/// Provenance explains navigation only. It is not a relevance, confidence, or
/// truth score and cannot close an open tension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ActivationProvenance {
    /// Exposed by one active problem region.
    ProblemRegion {
        /// Thread-local problem-region identity.
        region_id: String,
    },
    /// Exposed by one specific problem-space referent expression.
    ///
    /// This records the textual source that caused exposure. It does not assert
    /// that the referent is canonically bound to the activated address.
    ProblemReferent {
        /// Containing problem-region identity.
        region_id: String,
        /// Thread-local problem-referent identity.
        referent_id: String,
    },
    /// Exposed by one problem-space relation.
    ProblemRelation {
        /// Thread-local problem-relation identity.
        relation_id: String,
    },
    /// Exposed by one active problem-space constraint.
    Constraint {
        /// Thread-local constraint identity.
        constraint_id: String,
    },
    /// Exposed by one explicit open tension.
    OpenTension {
        /// Thread-local tension identity.
        tension_id: String,
    },
    /// Exposed by one preserved candidate of an open tension.
    ///
    /// Candidate order is the declared vector order in the problem-space state.
    /// Exposure does not select this candidate or resolve the tension.
    OpenTensionCandidate {
        /// Thread-local open-tension identity.
        tension_id: String,
        /// Zero-based candidate index within the preserved candidate vector.
        candidate_index: u32,
    },
    /// Exposed by one region's current attention band.
    AttentionBand {
        /// Region carrying the activation band.
        region_id: String,
        /// Current band in the shared attention lens.
        band: ActivationBand,
    },
    /// Exposed by the newest utterance.
    NewestUtterance {
        /// Current utterance identity.
        utterance_id: String,
    },
    /// Exposed by a deterministic configuration default.
    ConfiguredDefault {
        /// Stable configuration key.
        configuration_key: String,
    },
    /// Exposed by a typed bounded expansion request.
    ExpansionRequest {
        /// Expansion request identity.
        request_id: String,
    },
}

/// Measured access telemetry for an activated surface or region.
///
/// It reports counts, depth, truncation, budget, and continuation. It is not a
/// semantic score and has no evidence-admission authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionTelemetry {
    /// Stable identity of this telemetry record.
    pub telemetry_id: String,
    /// Stable identity of the exact activation probe being measured.
    pub probe_id: String,
    /// Exact projected surface match mode used by the probe.
    pub match_mode: SurfaceMatchMode,
    /// Retrieval surface family measured by this record.
    pub surface_kind: RetrievalSurfaceKind,
    /// Concrete surface identity.
    pub surface_id: String,
    /// Exact or estimated candidate count.
    pub candidate_count: CandidateCount,
    /// Current represented expansion depth.
    pub current_depth: u32,
    /// Configured hard maximum depth.
    pub maximum_depth: u32,
    /// Count returned into the visible view.
    pub returned_count: u64,
    /// Remaining bounded expansion budget.
    pub remaining_expansion_budget: u64,
    /// Mechanical truncation state.
    pub truncation_state: TruncationState,
    /// Bounded counts by represented identifier or type label.
    pub identifier_type_distribution: Vec<CountByLabel>,
    /// Count of visible temporal anchors.
    pub temporal_anchor_count: u64,
    /// Count of structurally unresolved authored targets.
    pub unresolved_target_count: u64,
    /// Whether bounded continuation is available.
    pub continuation_available: bool,
    /// Provenance explaining the measured activation.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Exactness of a projection-access candidate count.
///
/// It preserves measurement status only and does not express confidence in
/// semantic relevance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "count",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CandidateCount {
    /// Exact measured candidate count.
    Exact(u64),
    /// Deterministic estimate supplied by the surface.
    Estimated(u64),
}

/// Mechanical truncation state of an activated result set.
///
/// It describes bounded visibility only and authorizes no conclusion about
/// omitted corpus material.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TruncationState {
    /// No truncation occurred within the measured scope.
    Complete,
    /// Configured view bound truncated the visible result.
    Bounded,
    /// Hard expansion budget truncated the visible result.
    BudgetExhausted,
}

/// Count associated with one represented label.
///
/// This is mechanical telemetry, not a relevance distribution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CountByLabel {
    /// Identifier, object class, or other projected label.
    pub label: String,
    /// Measured count.
    pub count: u64,
}

/// Positive identifier-assignment visible in the working view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedIdentifierAssignmentRecord {
    /// Snapshot-local identifier-assignment identity.
    pub assignment_id: String,
    /// Projected identifier descriptor name.
    pub identifier_name: String,
    /// Canonical projected subject of the assignment.
    pub subject: SemanticAddress,
    /// Represented projected value.
    pub value: IdentifierValue,
    /// Corpus or materialization provenance from the frozen projection.
    pub record_provenance: RecordProvenance,
    /// Retrieval surfaces structurally capable of inspecting this assignment.
    pub available_surface_ids: Vec<String>,
    /// Reasons this assignment became visible in the working view.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Positive authored occurrence visible in the working view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedOccurrenceRecord {
    /// Canonical authored-occurrence identity.
    pub occurrence_id: OccurrenceId,
    /// Authored object-field, semantic-region, or semantic-unit source.
    pub source: OccurrenceSource,
    /// Authored target text retained by the projection.
    pub authored_target_text: String,
    /// Optional authored display alias.
    pub display_alias: Option<String>,
    /// Canonical projected target.
    pub resolved_target: SemanticAddress,
    /// Link or embed presentation form.
    pub presentation_mode: OccurrencePresentation,
    /// Authored incidence direction.
    pub direction: Direction,
    /// Exact represented source span when present.
    pub source_span: Option<SourceSpan>,
    /// Retrieval surfaces structurally capable of inspecting this occurrence.
    pub available_surface_ids: Vec<String>,
    /// Reasons this occurrence became visible.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Positive temporal anchor visible in the working view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedTemporalAnchorRecord {
    /// Canonical temporal-anchor identity.
    pub anchor_id: TemporalAnchorId,
    /// Canonical projected subject carrying the anchor.
    pub subject: SemanticAddress,
    /// Materially represented temporal value.
    pub value: TemporalValue,
    /// Corpus or materialization provenance from the projection.
    pub record_provenance: RecordProvenance,
    /// Retrieval surfaces structurally capable of inspecting this anchor.
    pub available_surface_ids: Vec<String>,
    /// Reasons this anchor became visible.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Original probe or structural neighbourhood described by a continuation handle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuationOrigin {
    /// Continuation of one deterministic textual surface probe.
    TextProbe {
        query_text: String,
        match_mode: SurfaceMatchMode,
    },
    /// Continuation of one structural neighbourhood.
    StructuralNeighbourhood {
        subject: SemanticAddress,
        transition_id: Option<String>,
        direction: Option<Direction>,
    },
    /// Continuation of one temporal surface probe.
    TemporalProbe {
        start: Option<TemporalValue>,
        end: Option<TemporalValue>,
    },
}

/// Typed filter applied to a continuation handle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuationFilter {
    /// Restrict to one projected structural transition.
    Transition { transition_id: String },
    /// Restrict source objects to a canonical path prefix.
    SourcePathPrefix { canonical_path_prefix: String },
    /// Restrict to one projected semantic-object class.
    ObjectClass { object_class: String },
    /// Restrict by one projected identifier and optional represented value.
    Identifier {
        /// Projected identifier descriptor name.
        identifier_name: String,
        /// Optional exact represented projected value.
        represented_value: Option<IdentifierValue>,
    },
    /// Restrict by a temporal range.
    TemporalRange {
        start: Option<TemporalValue>,
        end: Option<TemporalValue>,
    },
}

/// Stable ordering under which a continuation cursor is interpreted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuationOrdering {
    /// Stable order of the corresponding frozen projection vector.
    ProjectionVectorOrder,
    /// Stable order declared by one deterministic surface.
    SurfaceDeclared { ordering_key: String },
}

/// Mechanism through which a continuation page is interpreted.
///
/// Projection-structure continuation follows frozen represented topology
/// directly. Retrieval-surface continuation resumes one concrete projected
/// surface. Neither variant executes continuation by itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuationAccess {
    /// Continue deterministic structure represented directly by the projection.
    ProjectionStructure,
    /// Continue one concrete projected retrieval surface.
    RetrievalSurface {
        /// Concrete snapshot-local surface identity.
        surface_id: String,
        /// Stable surface family declared by the projection.
        surface_kind: RetrievalSurfaceKind,
    },
}

/// Bounded continuation descriptor for a high-degree or truncated view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContinuationHandle {
    /// Stable handle identity.
    pub handle_id: String,
    /// Frozen projection snapshot required by this continuation.
    pub projection_snapshot_id: String,
    /// Activation configuration snapshot required by this continuation.
    pub configuration_snapshot_id: String,
    /// Thread whose accepted problem space shaped the original activation.
    pub problem_space_thread_id: String,
    /// Exact accepted problem-space version used for the original activation.
    pub problem_space_version: u64,
    /// Newest utterance identity used for the original activation.
    pub newest_utterance_id: String,
    /// Original probe or structural neighbourhood.
    pub origin: ContinuationOrigin,
    /// Mechanism required to interpret and continue this handle.
    pub access: ContinuationAccess,
    /// Typed filters already applied to the continuation.
    pub filters: Vec<ContinuationFilter>,
    /// Stable ordering under which the cursor is interpreted.
    pub ordering: ContinuationOrdering,
    /// Zero-based next offset in that stable ordering.
    pub next_offset: u64,
    /// Remaining records when mechanically knowable.
    pub remaining_count: Option<u64>,
    /// Maximum records permitted in the next page.
    pub next_page_limit: u32,
    /// Reasons the truncated neighbourhood or result set became visible.
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Activated-vector identity category used by future validation violations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivatedRecordKind {
    Object,
    Region,
    Unit,
    IdentifierAssignment,
    Occurrence,
    TemporalAnchor,
    Edge,
    Telemetry,
    ContinuationHandle,
}

/// Closed typed error vocabulary reserved for deterministic activation validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "violation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProjectionActivationViolation {
    EmptyRequiredIdentity {
        field: String,
    },
    ProjectionNotValidated {
        status: ProjectionValidationStatus,
    },
    ConfigurationSnapshotMismatch {
        projection_configuration_snapshot_id: String,
        activation_configuration_snapshot_id: String,
    },
    MissingAvailableSurfaceConfiguration {
        surface_id: String,
    },
    UnknownOrUnavailableSurfaceConfiguration {
        surface_id: String,
    },
    DuplicateSurfaceConfiguration {
        surface_id: String,
    },
    InvalidConfigurationValue {
        field: String,
    },
    SurfaceCandidateLimitExceedsHardLimit {
        surface_id: String,
        requested: u32,
        hard_maximum: u32,
    },
    SurfaceAccessFailed {
        /// Concrete projected retrieval-surface identity.
        surface_id: String,
        /// Exact activation probe that failed.
        probe_id: String,
        /// Mechanical failure context.
        context: String,
    },
    DuplicateActivatedIdentity {
        kind: ActivatedRecordKind,
        identity: String,
    },
    InvalidActivatedReference {
        context: String,
    },
    InvalidActivationProvenance {
        context: String,
    },
    InvalidContinuationHandle {
        handle_id: String,
        context: String,
    },
    InvalidTelemetry {
        surface_id: String,
        context: String,
    },
    ActivatedViewBoundExceeded {
        kind: ActivatedRecordKind,
        actual: u64,
        maximum: u32,
    },
    CountOverflow,
}
