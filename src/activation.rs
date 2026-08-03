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
        Direction, RetrievalSurfaceKind, SemanticAddress, SemanticObjectId, SemanticRegionAddress,
        SemanticUnitId,
    },
    problem_space::ActivationBand,
};

/// Bounded positive working view over one frozen semantic-space projection.
///
/// It retains visible records, edges, telemetry, continuation descriptors, and
/// activation provenance. Visible does not mean relevant; absent from this view
/// does not mean absent from the corpus; this type has no negative-claim
/// authority and does not perform activation or expansion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActivatedProjection {
    /// Frozen projection snapshot from which the view was drawn.
    pub projection_snapshot_id: String,
    /// Configuration snapshot that bounded the view.
    pub configuration_snapshot_id: String,
    /// Positive object records visible in the current working view.
    pub activated_objects: Vec<ActivatedObjectRecord>,
    /// Positive region records visible in the current working view.
    pub activated_regions: Vec<ActivatedRegionRecord>,
    /// Positive unit records visible in the current working view.
    pub activated_units: Vec<ActivatedUnitRecord>,
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
    /// Bounded text preview used only for access planning.
    pub text_preview: String,
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

/// Bounded continuation descriptor for a high-degree or truncated view.
///
/// It permits later typed expansion within configuration limits. It does not
/// perform expansion or imply that omitted records are irrelevant.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContinuationHandle {
    /// Stable handle identity.
    pub handle_id: String,
    /// Canonical address whose neighbourhood or content may continue.
    pub subject: SemanticAddress,
    /// Surface family used for continuation.
    pub surface_kind: RetrievalSurfaceKind,
    /// Optional incidence direction for graph-like continuation.
    pub direction: Option<Direction>,
    /// Remaining measured records when known.
    pub remaining_count: Option<u64>,
    /// Maximum records permitted in the next bounded page.
    pub next_page_limit: u32,
    /// Provenance of the activation that produced this handle.
    pub activation_provenance: Vec<ActivationProvenance>,
}
