//! Strongly typed, serializable contracts for the CLEANROOM kernel.
//!
//! This crate defines typed contracts and a bounded deterministic problem-space
//! fold. It does not perform inference, projection construction or activation, semantic access,
//! conformance evaluation, retrieval, packet assembly, or synthesis.
#![forbid(unsafe_code)]

pub mod activation;
pub mod conformance;
pub mod execution;
pub mod model;
pub mod packet;
pub mod problem_space;
pub mod problem_space_fold;
pub mod projection;
pub mod region_identity;
pub mod semantic_access;
pub mod synthesis;

pub use activation::{
    ActivatedIdentifierAssignmentRecord, ActivatedOccurrenceRecord, ActivatedProjection,
    ActivatedTemporalAnchorRecord, ActivatedTextPreview, ActivationUtterance, ContinuationHandle,
    ProjectionActivationConfig, ProjectionActivationViolation,
};
pub use conformance::ConformanceResult;
pub use execution::RetrievalResult;
pub use model::{
    OccurrenceId, SemanticObjectId, SemanticRegionAddress, SemanticUnitId, TemporalAnchorId,
    TransportSegmentId,
};
pub use packet::ExecutionLimits;
pub use problem_space::{
    AcceptedBoundaryContribution, AttentionLens, BoundaryContribution, BoundaryContributionLog,
    OpenTension, ProblemConstraintApplicability, ProblemRegion, ProblemRelation, ProblemSpaceState,
};
pub use projection::SemanticSpaceProjection;
pub use region_identity::{
    AuthoredRegionHeading, CanonicalRegionIdentity, canonical_region_identities,
};
pub use semantic_access::SemanticAccessPlan;
pub use synthesis::SynthesisInput;
