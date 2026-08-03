//! Structural conformance-result contracts.
//!
//! These records report whether a plan is structurally valid against one frozen
//! projection snapshot or enumerate exact structural violations. They do not
//! evaluate conformance, repair a plan, judge relevance, paraphrase, proposition,
//! confidence, coherence, or semantic adequacy.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::model::{AddressKind, Direction, Requirement, SemanticAddress};

/// Structural conformance outcome for one semantic-access plan.
///
/// A valid result records structural membership. An invalid result records at
/// least one exact structural violation. This type has no semantic authority and
/// contains no relevance, paraphrase, proposition, confidence, or coherence
/// judgment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConformanceResult {
    /// Plan is structurally valid against the named frozen projection snapshot.
    Valid {
        /// Plan identity checked.
        plan_id: String,
        /// Frozen projection snapshot used for the check.
        projection_snapshot_id: String,
    },
    /// Plan is structurally invalid with exact represented violations.
    Invalid {
        /// Plan identity checked.
        plan_id: String,
        /// Frozen projection snapshot used for the check.
        projection_snapshot_id: String,
        /// Non-empty exact violation collection.
        violations: StructuralViolations,
    },
}

/// Non-empty collection of exact structural violations.
///
/// The first violation is required by shape, preventing an invalid result from
/// deserializing without a stated structural reason. The collection cannot
/// carry semantic judgments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StructuralViolations {
    /// Required first structural violation.
    pub first: StructuralViolation,
    /// Additional exact structural violations.
    pub additional: Vec<StructuralViolation>,
}

/// One exact structural violation found during conformance.
///
/// It identifies the affected operation or address, a closed structural code,
/// and typed detail. It cannot express semantic relevance or repair meaning.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StructuralViolation {
    /// Plan operation associated with the violation when applicable.
    pub operation_id: Option<String>,
    /// Canonical or proposed address associated with the violation.
    pub address: Option<SemanticAddress>,
    /// Stable structural violation code.
    pub code: StructuralViolationCode,
    /// Exact typed structural detail.
    pub detail: StructuralViolationDetail,
    /// Requirement of the affected operation when applicable.
    pub requirement: Option<Requirement>,
}

/// Closed categories of structural conformance failure.
///
/// These categories describe projection membership, typing, direction, surface,
/// output, and configuration only. They cannot encode semantic disagreement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StructuralViolationCode {
    /// Referenced canonical or projected address is absent.
    MissingAddress,
    /// Referenced identifier descriptor or assignment is absent.
    MissingIdentifier,
    /// Identifier cannot apply to the proposed address kind.
    IdentifierNotApplicable,
    /// Referenced relation or transition is absent.
    MissingRelation,
    /// Requested incidence direction is not represented.
    DirectionUnavailable,
    /// Requested retrieval surface is absent or disabled.
    SurfaceUnavailable,
    /// Operation input kind does not match the represented transition.
    InputTypeMismatch,
    /// Requested output cannot be materialized from the operation graph.
    OutputUnavailable,
    /// Requested bound exceeds the configuration snapshot.
    ConfigurationBoundExceeded,
    /// Authored heading or block target remains unresolved.
    UnresolvedAuthoredTarget,
    /// Requested transition is unsupported.
    UnsupportedTransition,
}

/// Typed detail attached to one structural violation.
///
/// Variants retain the exact represented mismatch without introducing natural-
/// language semantic adjudication.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StructuralViolationDetail {
    /// A named projection member was not present.
    MissingMember {
        /// Structural member category.
        member_kind: AddressKind,
        /// Stable missing identifier or address rendering.
        member: String,
    },
    /// An identifier was proposed for an inapplicable address kind.
    IdentifierApplicability {
        /// Identifier name.
        identifier_name: String,
        /// Proposed address kind.
        proposed_address_kind: AddressKind,
        /// Address kinds admitted by the descriptor.
        applicable_address_kinds: Vec<AddressKind>,
    },
    /// A direction was requested but not represented.
    Direction {
        /// Requested direction.
        requested: Direction,
        /// Directions represented by the projection.
        available: Vec<Direction>,
    },
    /// A retrieval surface was unavailable for the requested address kind.
    Surface {
        /// Concrete surface identity.
        surface_id: String,
        /// Address kind supplied to the surface.
        address_kind: AddressKind,
    },
    /// Operation input or output type did not match projection grammar.
    TypeMismatch {
        /// Expected structural kind.
        expected: AddressKind,
        /// Actual structural kind.
        actual: AddressKind,
    },
    /// Requested numeric bound exceeded configuration.
    BoundExceeded {
        /// Stable bound name.
        bound_name: String,
        /// Requested value.
        requested: u64,
        /// Configured hard maximum.
        maximum: u64,
    },
    /// Authored target could not be resolved to the required kind.
    UnresolvedTarget {
        /// Authored target text retained by the projection.
        authored_target: String,
        /// Required resolved address kind.
        expected_target_kind: AddressKind,
    },
    /// A named transition could not consume or emit the requested structure.
    Transition {
        /// Referenced transition identity.
        transition_id: String,
        /// Exact structural explanation.
        explanation: String,
    },
}
