//! Relational problem-space contracts.
//!
//! These records represent inferred thread-local state and declared boundary
//! perturbations. They do not perform boundary inference, semantic merging,
//! lifecycle transitions, or semantic deduplication. Deterministic application
//! of these records is provided by `problem_space_fold`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// One thread-local relational problem-space state.
///
/// It preserves regions, relations, constraints, open tensions, contribution
/// history, and one attention lens over the same state. It may be consumed by
/// later stages but cannot infer its own updates or authorize corpus claims.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemSpaceState {
    /// Runtime thread identity whose state is isolated from every other thread.
    pub thread_id: String,
    /// Monotonic state version after accepted deterministic folding.
    pub version: u64,
    /// Individuated problem regions in the current relational state.
    pub regions: Vec<ProblemRegion>,
    /// Relations among current or historically retained problem regions.
    pub relations: Vec<ProblemRelation>,
    /// Active or historically retained problem constraints.
    pub constraints: Vec<ProblemConstraint>,
    /// Explicit unresolved tensions and their lifecycle state.
    pub open_tensions: Vec<OpenTension>,
    /// Compact derived audit summary; the accepted log is the replay source.
    pub contribution_history: Vec<ContributionHistoryRecord>,
    /// Current activation bands over the one relational state.
    pub attention_lens: AttentionLens,
    /// Inclusive source-turn range represented by this state.
    pub source_turn_range: SourceTurnRange,
}

/// One individuated relational region within a problem space.
///
/// A region is more than a topic label: it retains referents, local structural
/// links, constraints, tensions, contribution provenance, persistence, and
/// activation. It cannot autonomously merge, split, supersede, or retire.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemRegion {
    /// Stable identity of this problem region within the thread.
    pub region_id: String,
    /// Referents that help preserve the region's continuity.
    pub anchor_referents: Vec<ProblemReferent>,
    /// Relation identities incident to this region.
    pub relation_ids: Vec<String>,
    /// Derived active regional-incidence index, rebuilt by the future fold.
    ///
    /// This contains only active regional constraints explicitly targeting this
    /// operational region. Canonical applicability lives on
    /// [`ProblemConstraint::applicability`]; whole-problem-space, superseded,
    /// and retired constraints are absent here.
    pub local_constraint_ids: Vec<String>,
    /// Open-tension identities attached to this region.
    pub open_tension_ids: Vec<String>,
    /// Boundary contributions that created or transformed this region.
    pub source_contribution_ids: Vec<String>,
    /// Operational persistence state distinct from attentional activation.
    pub persistence_state: RegionPersistenceState,
    /// Current activation band in the single attention lens.
    pub activation_band: ActivationBand,
    /// Prior region directly superseded by this region, when declared.
    pub supersedes_region_id: Option<String>,
}

/// One represented relation among problem regions.
///
/// It records a declared relational connection and provenance. It may guide
/// later projection access but is not itself a corpus address or executable
/// traversal edge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemRelation {
    /// Stable relation identity within the thread.
    pub relation_id: String,
    /// Source problem-region identity.
    pub source_region_id: String,
    /// Declared relation category.
    pub relation_type: ProblemRelationType,
    /// Optional target region when the relation is binary.
    pub target_region_id: Option<String>,
    /// Boundary contribution that declared this relation.
    pub source_contribution_id: String,
    /// Current lifecycle of the represented relation.
    pub lifecycle: RecordLifecycle,
}

/// One active or historically retained problem-space constraint.
///
/// It records a declared requirement such as chronology, exclusion, or exact
/// wording. It does not execute or independently interpret the constraint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemConstraint {
    /// Stable constraint identity within the thread.
    pub constraint_id: String,
    /// Human-readable declared constraint expression.
    pub expression: String,
    /// Canonical declaration of where this constraint applies.
    pub applicability: ProblemConstraintApplicability,
    /// Boundary contribution that introduced the constraint.
    pub source_contribution_id: String,
    /// Current lifecycle of the constraint record.
    pub lifecycle: RecordLifecycle,
}

/// Canonical applicability authored for one problem-space constraint.
///
/// In the future fold, [`ProblemConstraintApplicability::WholeProblemSpace`]
/// applies to every operational region and never appears in
/// [`ProblemRegion::local_constraint_ids`]. [`ProblemConstraintApplicability::Regions`]
/// explicitly targets one or several regions; vector order grants no priority.
/// Active regional constraints may target only operational regions: active,
/// background, and unresolved persistence states are operational, while
/// superseded and retired states are not.
///
/// Duplicate, empty, or unresolved target sets are invalid future fold input.
/// Shared applicability remains one canonical constraint, whose identity is
/// included in every targeted operational region's derived active-incidence
/// index. Superseding or retiring a region never transfers, narrows, or retires
/// constraints automatically: boundary inference must explicitly replace or
/// retire affected constraints, and every replacement fully declares its own
/// applicability. The fold must not inherit applicability by convenience.
/// Historical superseded or retired constraints retain their authored
/// applicability for audit. These are fold invariants, not Serde validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProblemConstraintApplicability {
    /// Applies to all operational regions.
    WholeProblemSpace,
    /// Applies exactly to the declared regional identities.
    Regions {
        /// Target regions; ordering carries no precedence or priority.
        region_ids: Vec<String>,
    },
}

/// One explicit unresolved tension in the problem representation.
///
/// It preserves ambiguity, contradiction, or a missing relation without false
/// resolution. It does not imply that the corpus lacks an answer and cannot
/// close itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OpenTension {
    /// Stable tension identity within the thread.
    pub tension_id: String,
    /// Containing problem-region identity.
    pub region_id: String,
    /// Declared tension category.
    pub tension_type: OpenTensionType,
    /// Source expression retained when useful for later interpretation.
    pub unresolved_expression: Option<String>,
    /// Candidate bindings or interpretations preserved without selection.
    pub candidate_bindings: Vec<String>,
    /// Source turn that introduced the tension.
    pub source_turn_id: String,
    /// Current lifecycle of the tension.
    pub lifecycle: TensionLifecycle,
}

/// Current attention bands over one relational problem-space state.
///
/// These vectors are views over shared region identities, not independent
/// topic stores. Activation changes neither identity, constraint applicability,
/// lifecycle, nor semantic strength. In a valid future folded state, every
/// operational region occupies exactly one band and this lens agrees with its
/// [`ProblemRegion::activation_band`]. An unresolved region may occupy any
/// band, and an active region may occupy background activation. The lens may
/// guide access but cannot score attention, persistence, confidence, decay,
/// coherence, or truth, and cannot admit evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttentionLens {
    /// Region identities currently in primary activation.
    pub primary_region_ids: Vec<String>,
    /// Region identities currently in secondary activation.
    pub secondary_region_ids: Vec<String>,
    /// Region identities currently in tertiary activation.
    pub tertiary_region_ids: Vec<String>,
    /// Region identities currently in background activation.
    pub background_region_ids: Vec<String>,
}

/// Append-only declaration of how one utterance perturbs prior problem state.
///
/// It carries typed operations and preservation or release declarations. This
/// record does not apply operations, infer semantic equivalence, enforce
/// bounds, or produce a new `ProblemSpaceState`.
///
/// The future deterministic fold has this normative phase order: (0) preflight
/// envelope and declared-identity uniqueness; (1) region operations; (2)
/// relation operations; (3) constraint operations; (4) tension operations; (5)
/// attention operations; (6) preservation/release declaration validation; (7)
/// rebuild derived incidence indexes and the attention lens; (8) validate final
/// referential and lifecycle closure; (9) enforce configured bounds; and (10)
/// atomically commit state, history, accepted-log entry, and version increment.
/// Operations within each category execute in declared vector order. The fold
/// does not sort, semantically consolidate, or reinterpret them. Later phases
/// may reference newly declared regions. Working-copy incompleteness never
/// permits partial commit; preservation and release declarations are audit
/// declarations, not a second mutation mechanism. Contradictory terminal
/// operations and configured-bound excess are rejected, not reconciled or
/// silently removed. This contract declares no executor in this PR.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoundaryContribution {
    /// Stable contribution identity.
    pub contribution_id: String,
    /// Source turn identity.
    pub source_turn_id: String,
    /// Source utterance identity.
    pub source_utterance_id: String,
    /// Declared region perturbations.
    pub region_operations: Vec<RegionOperation>,
    /// Declared relation perturbations.
    pub relation_operations: Vec<RelationOperation>,
    /// Declared constraint perturbations.
    pub constraint_operations: Vec<ConstraintOperation>,
    /// Declared tension perturbations.
    pub tension_operations: Vec<TensionOperation>,
    /// Declared attentional redirects over existing region identities.
    pub attention_operations: Vec<AttentionOperation>,
    /// Structure explicitly preserved by the contribution.
    pub preservation_declarations: Vec<PreservationDeclaration>,
    /// Structure explicitly released, superseded, or retired.
    pub release_declarations: Vec<ReleaseDeclaration>,
}

/// Ordered log of boundary contributions accepted for exactly one thread.
///
/// This is the authoritative replay input, separate from both the source
/// transcript and [`ProblemSpaceState::contribution_history`]. A fresh thread
/// begins at state version zero with an empty log. The first accepted entry has
/// sequence one; sequences are contiguous and unique, while vector order is
/// authoritative replay order. Contribution, source-turn, and source-utterance
/// identities cannot be accepted twice within the thread.
///
/// The log stores neither a transcript copy nor timestamps, storage paths,
/// provider metadata, or state snapshots. These are future runtime invariants,
/// not validations implemented by this representation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoundaryContributionLog {
    /// Identity of the one thread to which every entry belongs.
    pub thread_id: String,
    /// Accepted contributions in authoritative replay order.
    pub entries: Vec<AcceptedBoundaryContribution>,
}

/// One boundary contribution accepted for future deterministic replay.
///
/// `prior_state_version` is the version before application. Every successful
/// future fold appends exactly one entry and increments the state version once.
/// A failed contribution appends nothing, mutates no history, and does not
/// increment the version; no partial state may become observable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcceptedBoundaryContribution {
    /// Contiguous one-based position in the thread's accepted sequence.
    pub sequence: u64,
    /// State version observed immediately before successful application.
    pub prior_state_version: u64,
    /// Complete accepted declaration retained as authoritative replay input.
    pub contribution: BoundaryContribution,
}

/// Referent retained inside a problem region.
///
/// It helps preserve relational continuity but is not a canonical corpus
/// binding until semantic-access inference resolves one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProblemReferent {
    /// Thread-local referent identity.
    pub referent_id: String,
    /// Surface expression or compact label retained by boundary inference.
    pub expression: String,
    /// Source contribution that introduced or most recently revised it.
    pub source_contribution_id: String,
}

/// Persistence and lifecycle state of a problem region.
///
/// It records operational history, not truth, confidence, relevance, or an
/// automatic decay score.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RegionPersistenceState {
    /// Operationally active within the current problem representation.
    Active,
    /// Retained as background continuity.
    Background,
    /// Explicitly unresolved while remaining part of the state.
    Unresolved,
    /// Preserved in history but replaced by a declared framing.
    Superseded,
    /// No longer part of the current operational problem state.
    Retired,
}

/// Current activation band assigned to a problem region.
///
/// A band changes visibility and foregrounding only. It does not duplicate the
/// region or change its identity, applicability, lifecycle, or semantic
/// strength. It is categorical and never a numeric attention or relevance
/// score.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivationBand {
    /// Immediate operational focus.
    Primary,
    /// Live adjacent structure that materially informs primary focus.
    Secondary,
    /// Lower-priority connected structure available for continuation.
    Tertiary,
    /// Retained continuity outside the current foreground.
    Background,
}

/// Declared categories of problem-space relation.
///
/// Categories describe the inferred thread representation only. They do not
/// create corpus relations or authorize traversal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProblemRelationType {
    /// One region continues another.
    Continuation,
    /// One region depends on another.
    Dependency,
    /// Regions participate in a comparison.
    Comparison,
    /// One region corrects a prior framing.
    Correction,
    /// One region refines another.
    Refinement,
    /// The relation is itself a causal question.
    CausalQuestion,
    /// Regions are linked by a temporal question or ordering.
    Temporal,
    /// Regions share a retained referent.
    SharedReferent,
    /// Regions share an active constraint.
    SharedConstraint,
    /// A named relation declared by inference but not hard-coded as corpus data.
    Declared {
        /// Declared relation name.
        name: String,
    },
}

/// Lifecycle shared by relation and constraint records.
///
/// It records whether a declared record is operationally current or retained
/// historically. It grants no semantic decision authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordLifecycle {
    /// Currently active in the relational state.
    Active,
    /// Preserved in history after replacement.
    Superseded,
    /// Explicitly retired from the operational state.
    Retired,
}

/// Declared categories of unresolved problem-space tension.
///
/// A category preserves the kind of unresolved structure without choosing a
/// candidate interpretation or making a corpus-absence claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum OpenTensionType {
    /// A referent remains unresolved.
    UnresolvedReference,
    /// Two represented commitments or framings conflict.
    Contradiction,
    /// A requested comparison dimension remains unspecified.
    MissingComparisonDimension,
    /// Multiple framings remain live.
    CompetingFraming,
    /// A recurrent question remains unresolved across contributions.
    RecurrentUnresolvedQuestion,
    /// The current problem representation lacks a required connection.
    RequiredConnectionMissing,
    /// Named tension declared by boundary inference.
    Declared {
        /// Declared tension name.
        name: String,
    },
}

/// Lifecycle of an open-tension record.
///
/// It records thread history only. A deterministic component may not change
/// the lifecycle without an accepted declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TensionLifecycle {
    /// Still unresolved and operational.
    Open,
    /// Declared resolved by boundary inference.
    Resolved,
    /// Replaced by another framing or tension.
    Superseded,
    /// Explicitly abandoned.
    Abandoned,
}

/// Declared operation over problem-region records.
///
/// Variants describe perturbations only. They do not execute merges, splits,
/// lifecycle changes, or semantic deduplication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum RegionOperation {
    /// Introduce a new region record.
    Create {
        /// Fully declared new region.
        region: ProblemRegion,
    },
    /// Preserve an existing region unchanged through this contribution.
    Preserve {
        /// Region to preserve.
        region_id: String,
        /// Inference-issued semantic reason retained for audit.
        reason: String,
    },
    /// Reinforce the persistence history of an existing region.
    Reinforce {
        /// Region being reinforced.
        region_id: String,
        /// Inference-issued reason.
        reason: String,
    },
    /// Extend one region with a newly declared referent.
    Extend {
        /// Region being extended.
        region_id: String,
        /// Referent added by the declaration.
        referent: ProblemReferent,
    },
    /// Merge declared source regions into one declared resulting region.
    Merge {
        /// Source regions to be merged.
        source_region_ids: Vec<String>,
        /// Resulting region whose identity remains explicit.
        resulting_region: ProblemRegion,
        /// Inference-issued semantic reason.
        reason: String,
    },
    /// Split one source region into declared resulting regions.
    Split {
        /// Source region to split.
        source_region_id: String,
        /// Resulting regions.
        resulting_regions: Vec<ProblemRegion>,
        /// Inference-issued semantic reason.
        reason: String,
    },
    /// Mark one region as superseded by another declared region.
    Supersede {
        /// Region being superseded.
        region_id: String,
        /// Region that replaces it operationally.
        superseded_by_region_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
    /// Retire a region from the current operational state.
    Retire {
        /// Region being retired.
        region_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
}

/// Declared operation over problem-space relations.
///
/// It records connection or disconnection requests but cannot validate or
/// mutate the relation graph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum RelationOperation {
    /// Add a declared relation.
    Connect {
        /// Fully declared relation record.
        relation: ProblemRelation,
    },
    /// Retire an existing relation.
    Disconnect {
        /// Relation to disconnect.
        relation_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
}

/// Declared operation over problem-space constraints.
///
/// It cannot interpret, enforce, or optimize a constraint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConstraintOperation {
    /// Add a declared constraint.
    Add {
        /// New constraint record.
        constraint: ProblemConstraint,
    },
    /// Replace one constraint with another declared constraint.
    Replace {
        /// Constraint being superseded.
        prior_constraint_id: String,
        /// Replacement constraint.
        replacement: ProblemConstraint,
        /// Inference-issued semantic reason.
        reason: String,
    },
    /// Retire a constraint without replacement.
    Retire {
        /// Constraint being retired.
        constraint_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
}

/// Declared operation over open tensions.
///
/// It preserves explicit changes to tension lifecycle but cannot resolve
/// ambiguity or contradiction independently.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum TensionOperation {
    /// Open a new tension.
    Open {
        /// New tension record.
        tension: OpenTension,
    },
    /// Declare a tension resolved.
    Resolve {
        /// Tension being resolved.
        tension_id: String,
        /// Inference-issued resolution statement.
        resolution: String,
    },
    /// Declare a tension superseded by another tension.
    Supersede {
        /// Tension being superseded.
        tension_id: String,
        /// Replacement tension identity.
        superseded_by_tension_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
    /// Explicitly abandon an unresolved tension.
    Abandon {
        /// Tension being abandoned.
        tension_id: String,
        /// Inference-issued semantic reason.
        reason: String,
    },
}

/// Declared attentional assignment for one problem-region identity.
///
/// It redirects the shared lens only. It may not duplicate, merge, or delete a
/// region and cannot score semantic relevance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttentionOperation {
    /// Region whose activation band is declared.
    pub region_id: String,
    /// Declared destination band.
    pub band: ActivationBand,
}

/// Explicit preservation declaration retained with a boundary contribution.
///
/// It documents what inference intended to survive the perturbation. It does
/// not itself apply preservation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PreservationDeclaration {
    /// Problem-space subject to preserve.
    pub subject: ProblemSpaceSubject,
    /// Inference-issued semantic reason.
    pub reason: String,
}

/// Explicit release declaration retained with a boundary contribution.
///
/// It documents what inference intended to supersede, retire, or abandon. It
/// does not itself mutate state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReleaseDeclaration {
    /// Problem-space subject to release.
    pub subject: ProblemSpaceSubject,
    /// Declared release mode.
    pub mode: ReleaseMode,
    /// Inference-issued semantic reason.
    pub reason: String,
}

/// Addressable subject inside problem-space state.
///
/// This enum is thread-local provenance, not a semantic-space address and not
/// an executable operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProblemSpaceSubject {
    /// Problem-region identity.
    Region(String),
    /// Problem-relation identity.
    Relation(String),
    /// Problem-constraint identity.
    Constraint(String),
    /// Open-tension identity.
    OpenTension(String),
    /// Anchor-referent identity.
    Referent(String),
}

/// Declared mode by which a problem-space subject leaves current operation.
///
/// It records an inference-issued distinction and cannot enact it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseMode {
    /// Replaced by a newer framing or subject.
    Supersede,
    /// Removed from current operation while retained historically.
    Retire,
    /// Explicitly abandoned without resolution.
    Abandon,
}

/// Contribution and persistence history record retained by the state.
///
/// It supports reconstruction and recurrence tracking. It is not a confidence,
/// coherence, or relevance score.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ContributionHistoryRecord {
    /// Boundary contribution identity.
    pub contribution_id: String,
    /// Source turn identity.
    pub source_turn_id: String,
    /// Declared transformation categories represented by the contribution.
    pub transformations: Vec<BoundaryOperationKind>,
}

/// Coarse categories of boundary perturbation retained in history.
///
/// These categories summarize declared operations for audit only. They do not
/// authorize a fold or erase operation-specific provenance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryOperationKind {
    /// Region creation.
    Create,
    /// Explicit preservation.
    Preserve,
    /// Reinforcement or recurrence.
    Reinforce,
    /// Region extension.
    Extend,
    /// Region merge.
    Merge,
    /// Region split.
    Split,
    /// Relation connection or disconnection.
    Relate,
    /// Constraint change.
    Constrain,
    /// Tension lifecycle change.
    Tension,
    /// Attention redirection.
    RedirectAttention,
    /// Supersession.
    Supersede,
    /// Retirement or release.
    Retire,
}

/// Inclusive turn range represented in one problem-space state.
///
/// The range is bookkeeping for reconstruction, not a transcript substitute or
/// corpus-evidence boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SourceTurnRange {
    /// First source-turn identity included in the reconstructed state.
    pub first_turn_id: String,
    /// Most recent source-turn identity included in the reconstructed state.
    pub last_turn_id: String,
}
