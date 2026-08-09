//! Strongly typed, serializable contracts for the CLEANROOM kernel.
//!
//! This crate defines typed contracts, a bounded deterministic problem-space
//! fold, and deterministic initial projection activation. It does not perform
//! boundary inference, projection construction, semantic-access inference,
//! expansion, conformance evaluation, retrieval, packet assembly, or synthesis.
#![forbid(unsafe_code)]

pub mod activation;
pub mod conformance;
pub mod execution;
pub mod model;
pub mod packet;
pub mod problem_space;
pub mod problem_space_fold;
pub mod projection;
pub mod projection_activation;
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
pub use projection_activation::{
    ProjectionActivationAccess, ProjectionActivationAccessFailure, ProjectionActivationCandidate,
    ProjectionActivationCandidateTransition, ProjectionActivationProbe,
    ProjectionActivationProbeBand, ProjectionActivationProbeContinuation,
    ProjectionActivationProbeResult, ProjectionActivationProbeSource,
    ProjectionActivationProbeSourceKind, activate_projection,
};
pub use semantic_access::SemanticAccessPlan;
pub use synthesis::SynthesisInput;
