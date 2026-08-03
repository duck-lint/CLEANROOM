//! Semantic-access planning contracts.
//!
//! These records request represented structure through canonical addresses,
//! typed paths, surface operations, joins, outputs, and coverage obligations.
//! They do not construct, expand, repair, conform, or execute a plan and they do
//! not resolve problem-space tensions.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    model::{Direction, Requirement, RetrievalSurfaceKind, SemanticAddress},
    projection::SurfaceMatchMode,
};

/// Proposed typed access graph over one frozen semantic projection snapshot.
///
/// It preserves problem-space provenance, canonical address bindings,
/// branching traversal paths, joins, output obligations, coverage requirements,
/// and snapshot identity. It has no authority to execute, repair, judge meaning,
/// or mark the underlying problem resolved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticAccessPlan {
    /// Stable plan identity.
    pub plan_id: String,
    /// Frozen projection snapshot against which the plan must conform.
    pub projection_snapshot_id: String,
    /// Accepted problem-space version used to construct the plan.
    pub problem_space_version: u64,
    /// Newest utterance identity that remains the plan focus.
    pub focus_utterance_id: String,
    /// Configuration snapshot governing bounds and enabled surfaces.
    pub configuration_snapshot_id: String,
    /// Thread and contribution provenance for the accepted problem space.
    pub problem_space_provenance: ProblemSpacePlanProvenance,
    /// Bindings from problem regions to canonical projected addresses.
    pub problem_region_bindings: Vec<ProblemRegionBinding>,
    /// Bindings from problem relations to represented paths or transitions.
    pub relation_bindings: Vec<ProblemRelationBinding>,
    /// Bindings from active constraints to operation obligations.
    pub constraint_bindings: Vec<ConstraintBinding>,
    /// Bindings from open tensions to access objectives without resolution.
    pub open_tension_bindings: Vec<OpenTensionBinding>,
    /// Named canonical address bindings used by path operations.
    pub address_bindings: Vec<AddressBinding>,
    /// Directed paths that may branch independently.
    pub traversal_paths: Vec<TraversalPath>,
    /// Explicit joins that reassemble path outputs.
    pub joins: Vec<PlanJoin>,
    /// Required and optional outputs requested from execution.
    pub requested_outputs: Vec<RequestedOutput>,
    /// Measurable execution and coverage obligations.
    pub coverage_requirements: Vec<CoverageRequirement>,
}

/// Provenance of the problem-space state used for planning.
///
/// It supports inspection of why a plan exists. It is not corpus evidence and
/// does not permit the plan to rewrite problem-space state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemSpacePlanProvenance {
    /// Thread identity owning the problem-space state.
    pub thread_id: String,
    /// Boundary contributions represented in the current planning context.
    pub contribution_ids: Vec<String>,
}

/// Binding from one problem region to canonical projected addresses.
///
/// It records exploratory resolution provenance but cannot assert that the
/// binding answers the region or close an attached tension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemRegionBinding {
    /// Thread-local problem-region identity.
    pub problem_region_id: String,
    /// Canonical projected addresses resolved for the region.
    pub address_binding_ids: Vec<String>,
    /// Planning rationale retained for audit.
    pub rationale: String,
}

/// Binding from a problem-space relation to represented paths or transitions.
///
/// It preserves planning provenance only and does not create a corpus relation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemRelationBinding {
    /// Thread-local problem-relation identity.
    pub problem_relation_id: String,
    /// Traversal paths selected to inspect represented structure.
    pub traversal_path_ids: Vec<String>,
    /// Projection transition identities used by those paths.
    pub transition_ids: Vec<String>,
}

/// Binding from an active problem constraint to plan operations.
///
/// It declares required or optional structural work. It cannot enforce the
/// constraint or determine semantic truth.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConstraintBinding {
    /// Thread-local constraint identity.
    pub constraint_id: String,
    /// Operations that operationalize the declared structural obligation.
    pub operation_ids: Vec<String>,
    /// Requirement attached to the binding.
    pub requirement: Requirement,
}

/// Binding from one open tension to an access objective.
///
/// It may preserve multiple candidate routes. It cannot choose an
/// interpretation, resolve the tension, or infer corpus absence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenTensionBinding {
    /// Thread-local open-tension identity.
    pub tension_id: String,
    /// Candidate address bindings or path outputs relevant to the tension.
    pub candidate_binding_ids: Vec<String>,
    /// Requested output identities intended to inform later synthesis.
    pub requested_output_ids: Vec<String>,
}

/// Named canonical address binding used by plan operations.
///
/// It connects a planning label to a projected address. It does not establish
/// structural existence; conformance owns that check.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AddressBinding {
    /// Stable plan-local binding identity.
    pub binding_id: String,
    /// Canonical or projected address proposed by semantic-access inference.
    pub address: SemanticAddress,
    /// Problem-space sources that motivated this binding.
    pub problem_space_provenance: Vec<ProblemSpaceReference>,
}

/// Directed route through represented semantic structure.
///
/// A path may be one branch of a larger acyclic plan and may later join another
/// path. It does not execute its operations or guarantee conformance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TraversalPath {
    /// Stable plan-local path identity.
    pub path_id: String,
    /// Canonical address bindings from which the path begins.
    pub start_binding_ids: Vec<String>,
    /// Ordered typed operations in this branch.
    pub operations: Vec<PlanOperation>,
    /// Named output binding materialized by the path when execution succeeds.
    pub output_binding: String,
    /// Problem-space sources that motivated this path.
    pub problem_space_provenance: Vec<ProblemSpaceReference>,
}

/// One typed operation requested by a semantic-access plan.
///
/// It preserves obligation, inputs, structural operation, bounds, and output.
/// It cannot execute, repair itself, or make a semantic judgment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlanOperation {
    /// Stable plan-local operation identity.
    pub operation_id: String,
    /// Whether failure affects support for the requested route.
    pub requirement: Requirement,
    /// Input bindings consumed by the operation.
    pub input_bindings: Vec<String>,
    /// Typed structural or retrieval-surface operation.
    pub operation: PlanOperationType,
    /// Configured or requested mechanical bounds.
    pub constraints: OperationConstraints,
    /// Named output binding emitted by the operation.
    pub output_binding: String,
}

/// Typed operation families available to a semantic-access plan.
///
/// Variants describe represented structural requests. They do not implement
/// traversal, retrieval, temporal interpretation, hydration, or repair.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PlanOperationType {
    /// Follow one represented structural connection.
    FollowConnection {
        /// Projection transition identity to follow.
        transition_id: String,
        /// Explicit incoming or outgoing direction.
        direction: Direction,
    },
    /// Invoke one represented retrieval surface.
    SearchSurface {
        /// Concrete projected retrieval-surface identity.
        surface_id: String,
        /// Surface family for stable inspection.
        surface_kind: RetrievalSurfaceKind,
        /// Match mode supported by the surface descriptor.
        match_mode: SurfaceMatchMode,
        /// Typed query payload.
        query: SurfaceQuery,
    },
    /// Hydrate projected results to canonical semantic units.
    HydrateSemanticUnits,
    /// Evaluate represented temporal anchors mechanically.
    EvaluateTemporal {
        /// Requested temporal operation.
        evaluation: TemporalEvaluation,
    },
}

/// Typed query payload for a retrieval-surface operation.
///
/// It carries requested literals, terms, vectors-by-reference, graph filters,
/// or temporal bounds. It does not decide relevance or synthesize a claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SurfaceQuery {
    /// Exact literal search.
    Literal {
        /// Literal to search without semantic expansion.
        value: String,
    },
    /// Lexical term search.
    Terms {
        /// Terms supplied by semantic-access inference.
        values: Vec<String>,
    },
    /// Vector query referenced by an existing projected address or binding.
    VectorFromBinding {
        /// Binding whose represented vector may be queried.
        binding_id: String,
    },
    /// Graph incidence constrained by represented transitions.
    Graph {
        /// Allowed transition identities.
        transition_ids: Vec<String>,
        /// Explicit direction.
        direction: Direction,
    },
    /// Temporal range query over represented anchors.
    TemporalRange {
        /// Inclusive lower bound in the projection's admitted format.
        start: Option<String>,
        /// Inclusive upper bound in the projection's admitted format.
        end: Option<String>,
    },
}

/// Mechanical operation constraints attached to one plan operation.
///
/// They remain subordinate to the configuration snapshot and do not express
/// semantic confidence or relevance thresholds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OperationConstraints {
    /// Requested maximum structural depth when applicable.
    pub maximum_depth: Option<u32>,
    /// Requested maximum candidate count when applicable.
    pub maximum_candidates: Option<u32>,
    /// Optional exact object or region scope binding identities.
    pub eligible_scope_binding_ids: Vec<String>,
}

/// Temporal evaluation requested after represented anchors are materialized.
///
/// It describes a deterministic operation only and does not choose the semantic
/// meaning of an unresolved chronology dimension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TemporalEvaluation {
    /// Order anchors in ascending or descending represented time.
    Order {
        /// Sort direction.
        direction: TemporalOrder,
    },
    /// Group anchors by canonical target binding.
    GroupByBinding {
        /// Binding identities that define groups.
        binding_ids: Vec<String>,
    },
    /// Select earliest represented anchor per binding.
    EarliestPerBinding,
    /// Select latest represented anchor per binding.
    LatestPerBinding,
}

/// Sort direction for deterministic temporal evaluation.
///
/// It controls ordering only and cannot infer chronology semantics.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemporalOrder {
    /// Earliest to latest.
    Ascending,
    /// Latest to earliest.
    Descending,
}

/// Join that reassembles outputs from branching traversal paths.
///
/// It records graph topology and grouping intent only. It does not compare
/// meaning or synthesize a conclusion.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PlanJoin {
    /// Stable plan-local join identity.
    pub join_id: String,
    /// Input output-bindings from one or more branches.
    pub input_bindings: Vec<String>,
    /// Mechanical join operation.
    pub operation: JoinOperation,
    /// Output binding produced by the join.
    pub output_binding: String,
}

/// Mechanical join operations admitted by the plan contract.
///
/// These variants preserve branch topology without interpreting evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum JoinOperation {
    /// Preserve inputs as separate grouped branches.
    Group,
    /// Concatenate outputs while preserving provenance.
    Concatenate,
    /// Order already materialized temporal records.
    TemporalOrder,
}

/// Requested execution output and obligation.
///
/// It controls materialized packet shape but does not prescribe the final
/// answer or judge whether an output is semantically sufficient.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RequestedOutput {
    /// Stable plan-local output identity.
    pub output_id: String,
    /// Required or optional execution obligation.
    pub requirement: Requirement,
    /// Requested materialized shape.
    pub kind: RequestedOutputKind,
    /// Binding from which the output must be materialized.
    pub source_binding: String,
}

/// Output shapes a semantic-access plan may request.
///
/// Variants control execution materialization only and do not define a final
/// natural-language response.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequestedOutputKind {
    /// Canonical semantic units.
    SemanticUnits,
    /// Canonical semantic-object identities.
    SemanticObjectIdentities,
    /// Canonical semantic-unit identities.
    SemanticUnitIdentities,
    /// Canonical authored semantic regions.
    SemanticRegions,
    /// Occurrence paths with provenance.
    OccurrencePaths,
    /// Temporal-anchor records.
    TemporalAnchors,
    /// Evidence grouped by canonical object.
    GroupedByObject,
    /// Mechanically ordered output.
    OrderedEvidence,
    /// Exhaustive exact total count when supported.
    TotalExactCount,
    /// Surface and path provenance.
    SurfaceProvenance,
}

/// Measurable coverage obligation requested by a plan.
///
/// It constrains later claim scope and execution reporting. It cannot interpret
/// returned units or authorize a claim before measurement completes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CoverageRequirement {
    /// Stable plan-local requirement identity.
    pub coverage_requirement_id: String,
    /// Required or optional obligation.
    pub requirement: Requirement,
    /// Measurable coverage condition.
    pub kind: CoverageRequirementKind,
}

/// Coverage conditions available to a semantic-access plan.
///
/// These conditions are mechanical execution requirements, not semantic gates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CoverageRequirementKind {
    /// Complete exact eligible-scope enumeration and total count.
    ExhaustiveExact {
        /// Surface identity that must support exhaustive exact execution.
        surface_id: String,
        /// Binding identities defining eligible scope.
        eligible_scope_binding_ids: Vec<String>,
    },
    /// Required graph depth within configured maxima.
    GraphDepth {
        /// Required completed depth.
        depth: u32,
    },
    /// Required completion of specified operations.
    OperationsCompleted {
        /// Operation identities that must complete.
        operation_ids: Vec<String>,
    },
    /// Required temporal range coverage.
    TemporalRange {
        /// Inclusive lower bound.
        start: Option<String>,
        /// Inclusive upper bound.
        end: Option<String>,
    },
}

/// Thread-local problem-space source referenced by a plan record.
///
/// It is provenance for planning and cannot be treated as corpus evidence or a
/// canonical semantic address.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProblemSpaceReference {
    /// Problem-region identity.
    Region(String),
    /// Problem-relation identity.
    Relation(String),
    /// Problem-constraint identity.
    Constraint(String),
    /// Open-tension identity.
    OpenTension(String),
}
