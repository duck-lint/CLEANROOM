//! Measured execution-limit and coverage contracts.
//!
//! These records constrain claim scope using observed execution facts. They do
//! not assemble packets, rank evidence, interpret meaning, or convert incomplete
//! coverage into a semantic conclusion.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::model::Requirement;

/// Measured execution and coverage facts for one retrieval result.
///
/// It reports completed and failed obligations, coverage measurements, and
/// applied deterministic bounds. It constrains later claim scope but cannot
/// reinterpret evidence, score relevance, or decide what the evidence means.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecutionLimits {
    /// Semantic-access plan whose execution was measured.
    pub plan_id: String,
    /// Number of plan operations requested.
    pub requested_operation_count: u64,
    /// Number of operations completed, including completed zero-match results.
    pub completed_operation_count: u64,
    /// Required operations that did not complete.
    pub failed_required_operation_ids: Vec<String>,
    /// Optional operations that did not complete.
    pub failed_optional_operation_ids: Vec<String>,
    /// Measured coverage facts.
    pub coverage_facts: Vec<CoverageFact>,
    /// Deterministic bounds applied during execution or packet preparation.
    pub applied_bounds: Vec<AppliedExecutionBound>,
}

/// One measured execution or coverage fact.
///
/// Variants distinguish exhaustive counts, completed paths, depth, caps,
/// unavailable surfaces, and temporal scope. They cannot reject a returned unit
/// or become a semantic relevance judgment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CoverageFact {
    /// Exhaustive exact execution completed over the named eligible scope.
    ExhaustiveExactCompleted {
        /// Concrete exact surface identity.
        surface_id: String,
        /// Stable scope description or binding identity.
        eligible_scope: String,
        /// Exact total matches in the completed eligible scope.
        total_matches: u64,
    },
    /// One operation completed with its obligation preserved.
    OperationCompleted {
        /// Plan operation identity.
        operation_id: String,
        /// Required or optional obligation.
        requirement: Requirement,
    },
    /// Graph or structural expansion reached the measured depth.
    GraphDepthReached {
        /// Traversal path identity.
        path_id: String,
        /// Maximum completed depth.
        depth: u32,
    },
    /// Candidate processing stopped at a configured cap.
    CandidateCapApplied {
        /// Plan operation identity.
        operation_id: String,
        /// Configured cap.
        cap: u64,
        /// Total known candidates when the surface reported it.
        total_candidates: Option<u64>,
    },
    /// A named retrieval surface was unavailable.
    SurfaceUnavailable {
        /// Concrete unavailable surface identity.
        surface_id: String,
        /// Operations affected by the unavailability.
        affected_operation_ids: Vec<String>,
    },
    /// Temporal execution completed over a represented range.
    TemporalRangeCovered {
        /// Inclusive lower bound when specified.
        start: Option<String>,
        /// Inclusive upper bound when specified.
        end: Option<String>,
        /// Count of anchors inspected.
        inspected_anchor_count: u64,
    },
}

/// One deterministic execution or packet bound and its measured effect.
///
/// It records configuration and truncation facts only. It cannot encode a
/// semantic removal reason.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AppliedExecutionBound {
    /// Stable configured bound name.
    pub bound_name: String,
    /// Configured maximum value.
    pub configured_limit: u64,
    /// Observed pre-bound count or depth.
    pub observed_value: u64,
    /// Whether the bound mechanically truncated output.
    pub truncated: bool,
}
