use schemars::{JsonSchema, schema_for};
use semantic_traversal_core::{
    AcceptedBoundaryContribution, ActivatedProjection, AttentionLens, BoundaryContribution,
    BoundaryContributionLog, ConformanceResult, ExecutionLimits, OpenTension, ProblemRegion,
    ProblemRelation, ProblemSpaceState, RetrievalResult, SemanticAccessPlan,
    SemanticSpaceProjection, SynthesisInput,
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
