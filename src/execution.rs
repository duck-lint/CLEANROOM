//! Retrieval-result contracts.
//!
//! These records preserve canonical returned units, execution status, operation
//! outcomes, and complete access provenance. They do not execute retrieval,
//! rank or semantically filter results, adjudicate propositions, or decide which
//! units reach synthesis.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        OccurrenceId, RecordProvenance, Requirement, SemanticAddress, SemanticObjectId,
        SemanticRegionAddress, SemanticUnitId, TemporalAnchorId, TransportSegmentId,
    },
    projection::{IdentifierValue, TemporalValue},
};

/// Provenance-preserving result of executing one conforming semantic-access plan.
///
/// It retains canonical identity, operation outcomes, returned units, and path
/// provenance. It contains no post-retrieval semantic-admission decision and
/// cannot interpret whether a unit answers the current problem.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievalResult {
    /// Semantic-access plan that was executed.
    pub plan_id: String,
    /// Frozen projection snapshot used by execution.
    pub projection_snapshot_id: String,
    /// Overall mechanical execution status.
    pub execution_status: RetrievalExecutionStatus,
    /// Canonical semantic units returned by valid operations.
    pub returned_units: Vec<RetrievedSemanticUnit>,
    /// Per-operation mechanical outcomes.
    pub operation_results: Vec<OperationExecutionRecord>,
}

/// Mechanical completion status of retrieval execution.
///
/// It reports execution state only. Provider or surface failure must not be
/// represented as a semantic conclusion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalExecutionStatus {
    /// Every requested operation completed.
    Complete,
    /// Some operations completed and exact failures are recorded.
    Partial,
    /// No requested operation completed successfully.
    Failed,
}

/// One canonical semantic unit returned by deterministic execution.
///
/// It preserves canonical unit, object, and region identity; authored content;
/// identifiers and provenance; occurrences; anchors; paths; surfaces; and any
/// subordinate transport segments. It cannot be semantically admitted or
/// rejected inside this deterministic contract.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievedSemanticUnit {
    /// Canonical semantic-unit identity.
    pub unit_id: SemanticUnitId,
    /// Canonical parent semantic object identity.
    pub parent_object_id: SemanticObjectId,
    /// Canonical authored region address.
    pub parent_region_address: SemanticRegionAddress,
    /// Raw authored Markdown or source block materialized by execution.
    pub authored_content: String,
    /// Inherited and unit-local identifier values with provenance.
    pub identifier_assignments: Vec<RetrievedIdentifierAssignment>,
    /// Authored outgoing occurrence identities.
    pub outgoing_occurrence_ids: Vec<OccurrenceId>,
    /// Incoming occurrence identities targeting this unit.
    pub incoming_occurrence_ids: Vec<OccurrenceId>,
    /// Temporal anchors preserved with value and provenance.
    pub temporal_anchors: Vec<RetrievedTemporalAnchor>,
    /// Access-path and retrieval-surface provenance.
    pub retrieval_provenance: Vec<RetrievalProvenance>,
    /// Technical segment provenance used to reconstruct this canonical unit.
    pub transport_segment_provenance: Vec<TransportSegmentProvenance>,
}

/// Identifier assignment retained on a retrieved canonical unit.
///
/// It preserves inheritance and source provenance. It cannot retype the unit or
/// parent object beyond the frozen projection assignment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievedIdentifierAssignment {
    /// Projection assignment identity.
    pub assignment_id: String,
    /// Admitted identifier name.
    pub identifier_name: String,
    /// Structured represented value.
    pub value: IdentifierValue,
    /// Original assignment provenance.
    pub provenance: RecordProvenance,
}

/// Temporal anchor retained on a retrieved canonical unit.
///
/// It preserves source identity and value only. It does not evaluate chronology
/// or transfer a contextual anchor to another object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievedTemporalAnchor {
    /// Canonical temporal-anchor identity.
    pub anchor_id: TemporalAnchorId,
    /// Structured represented temporal value.
    pub value: TemporalValue,
    /// Original anchor provenance.
    pub provenance: RecordProvenance,
}

/// Provenance of one retrieval route that returned a canonical semantic unit.
///
/// It records surface, path, operation, traversed addresses, occurrences, and
/// anchors. It cannot score or interpret the returned content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievalProvenance {
    /// Concrete retrieval-surface identity.
    pub surface_id: String,
    /// Traversal path identity.
    pub path_id: String,
    /// Plan operation identity.
    pub operation_id: String,
    /// Canonical addresses traversed in order.
    pub traversed_addresses: Vec<SemanticAddress>,
    /// Authored occurrences traversed by the route.
    pub occurrence_ids: Vec<OccurrenceId>,
    /// Temporal anchors traversed by the route.
    pub temporal_anchor_ids: Vec<TemporalAnchorId>,
}

/// Provenance for a technical transport segment used during execution.
///
/// It remains subordinate to one canonical semantic unit and may not be
/// promoted into an independent evidence identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransportSegmentProvenance {
    /// Technical transport-segment identity.
    pub segment_id: TransportSegmentId,
    /// Zero-based deterministic reconstruction ordinal.
    pub segment_ordinal: u32,
    /// Total segment count used for complete reconstruction.
    pub total_segments: u32,
}

/// Mechanical outcome of one plan operation.
///
/// It records obligation, completion, returned identities, and exact failure
/// when present. It cannot judge semantic adequacy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OperationExecutionRecord {
    /// Plan operation identity.
    pub operation_id: String,
    /// Required or optional execution obligation.
    pub requirement: Requirement,
    /// Mechanical operation status.
    pub status: OperationExecutionStatus,
    /// Canonical unit identities returned by this operation.
    pub returned_unit_ids: Vec<SemanticUnitId>,
    /// Count of candidates inspected mechanically.
    pub inspected_candidate_count: u64,
}

/// Mechanical status of one executed operation.
///
/// Variants distinguish success, zero matches, unavailable surfaces, bounded
/// truncation, and failure without introducing a semantic conclusion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum OperationExecutionStatus {
    /// Operation completed and may have returned one or more units.
    Completed,
    /// Operation completed successfully with zero matches.
    CompletedZeroMatches,
    /// Operation returned a deterministic bounded subset.
    Truncated {
        /// Stable mechanical truncation reason.
        reason: String,
    },
    /// Required retrieval surface was unavailable.
    SurfaceUnavailable {
        /// Concrete unavailable surface identity.
        surface_id: String,
    },
    /// Operation failed for a stated technical reason.
    Failed {
        /// Exact technical failure description.
        reason: String,
    },
}
