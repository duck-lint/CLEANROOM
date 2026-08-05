use schemars::{JsonSchema, schema_for};
use semantic_traversal_core::{
    AcceptedBoundaryContribution, ActivatedIdentifierAssignmentRecord, ActivatedOccurrenceRecord,
    ActivatedProjection, ActivatedTemporalAnchorRecord, ActivatedTextPreview, ActivationUtterance,
    AttentionLens, BoundaryContribution, BoundaryContributionLog, ConformanceResult,
    ContinuationHandle, ExecutionLimits, OpenTension, ProblemRegion, ProblemRelation,
    ProblemSpaceState, ProjectionActivationConfig, ProjectionActivationViolation, RetrievalResult,
    SemanticAccessPlan, SemanticSpaceProjection, SynthesisInput,
};
use serde::Serialize;

fn render_schema<T: JsonSchema + Serialize>() -> String {
    let schema = schema_for!(T);
    let mut rendered = serde_json::to_string_pretty(&schema).expect("schema must serialize");
    rendered.push('\n');
    rendered
}

pub fn generated_schemas() -> Vec<(&'static str, String)> {
    vec![
        (
            "problem-space-state.schema.json",
            render_schema::<ProblemSpaceState>(),
        ),
        (
            "problem-region.schema.json",
            render_schema::<ProblemRegion>(),
        ),
        (
            "problem-relation.schema.json",
            render_schema::<ProblemRelation>(),
        ),
        ("open-tension.schema.json", render_schema::<OpenTension>()),
        (
            "attention-lens.schema.json",
            render_schema::<AttentionLens>(),
        ),
        (
            "boundary-contribution.schema.json",
            render_schema::<BoundaryContribution>(),
        ),
        (
            "boundary-contribution-log.schema.json",
            render_schema::<BoundaryContributionLog>(),
        ),
        (
            "accepted-boundary-contribution.schema.json",
            render_schema::<AcceptedBoundaryContribution>(),
        ),
        (
            "semantic-space-projection.schema.json",
            render_schema::<SemanticSpaceProjection>(),
        ),
        (
            "activated-projection.schema.json",
            render_schema::<ActivatedProjection>(),
        ),
        (
            "activated-text-preview.schema.json",
            render_schema::<ActivatedTextPreview>(),
        ),
        (
            "activation-utterance.schema.json",
            render_schema::<ActivationUtterance>(),
        ),
        (
            "projection-activation-config.schema.json",
            render_schema::<ProjectionActivationConfig>(),
        ),
        (
            "projection-activation-violation.schema.json",
            render_schema::<ProjectionActivationViolation>(),
        ),
        (
            "activated-identifier-assignment-record.schema.json",
            render_schema::<ActivatedIdentifierAssignmentRecord>(),
        ),
        (
            "activated-occurrence-record.schema.json",
            render_schema::<ActivatedOccurrenceRecord>(),
        ),
        (
            "activated-temporal-anchor-record.schema.json",
            render_schema::<ActivatedTemporalAnchorRecord>(),
        ),
        (
            "continuation-handle.schema.json",
            render_schema::<ContinuationHandle>(),
        ),
        (
            "semantic-access-plan.schema.json",
            render_schema::<SemanticAccessPlan>(),
        ),
        (
            "conformance-result.schema.json",
            render_schema::<ConformanceResult>(),
        ),
        (
            "retrieval-result.schema.json",
            render_schema::<RetrievalResult>(),
        ),
        (
            "execution-limits.schema.json",
            render_schema::<ExecutionLimits>(),
        ),
        (
            "synthesis-input.schema.json",
            render_schema::<SynthesisInput>(),
        ),
    ]
}
