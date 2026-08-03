//! Strongly typed, serializable contracts for the CLEANROOM kernel.
//!
//! This crate defines identities and exchange shapes only. It does not perform
//! inference, folding, projection construction or activation, semantic access,
//! conformance evaluation, retrieval, packet assembly, or synthesis.
#![forbid(unsafe_code)]

pub mod activation;
pub mod conformance;
pub mod execution;
pub mod model;
pub mod packet;
pub mod problem_space;
pub mod projection;
pub mod semantic_access;
pub mod synthesis;

pub use activation::ActivatedProjection;
pub use conformance::ConformanceResult;
pub use execution::RetrievalResult;
pub use model::{
    OccurrenceId, SemanticObjectId, SemanticRegionAddress, SemanticUnitId, TemporalAnchorId,
    TransportSegmentId,
};
pub use packet::ExecutionLimits;
pub use problem_space::{
    AttentionLens, BoundaryContribution, OpenTension, ProblemRegion, ProblemRelation,
    ProblemSpaceState,
};
pub use projection::SemanticSpaceProjection;
pub use semantic_access::SemanticAccessPlan;
pub use synthesis::SynthesisInput;
