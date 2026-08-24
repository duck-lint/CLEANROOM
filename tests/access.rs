#[path = "support/mod.rs"]
mod support;

use std::{env, fs};

use semantic_traversal_core::{
    access::{
        AccessOperand, EmbeddingProvider, TemporalPrecision, TemporalQuery, VectorProviderContract,
        VectorProviderIdentity, build_projection_access_artifacts,
    },
    model::{Direction, SemanticAddress},
    projection::{ProjectionValidationStatus, SemanticSpaceProjection, TemporalValue},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use support::synthetic_projection::{JOURNAL_ONE_OBJECT, object, tiny_projection};

fn bind_logical_hash(mut projection: SemanticSpaceProjection) -> SemanticSpaceProjection {
    projection.logical_hash.clear();
    let bytes = serde_json::to_vec(&projection).expect("projection serializes");
    let digest = Sha256::digest(bytes);
    projection.logical_hash = format!(
        "sha256:{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    projection
}

fn probe(
    projection: &SemanticSpaceProjection,
    surface_id: &str,
    surface_kind: semantic_traversal_core::model::RetrievalSurfaceKind,
    match_mode: semantic_traversal_core::projection::SurfaceMatchMode,
    operand: AccessOperand,
    page_size: usize,
) -> semantic_traversal_core::access::ProjectionAccessProbe {
    semantic_traversal_core::access::ProjectionAccessProbe {
        probe_id: format!("probe:{surface_id}"),
        projection_snapshot_id: projection.projection_snapshot_id.clone(),
        surface_id: surface_id.into(),
        surface_kind,
        match_mode,
        operand,
        page_size,
        cursor: None,
    }
}

fn address_exists(projection: &SemanticSpaceProjection, address: &SemanticAddress) -> bool {
    match address {
        SemanticAddress::Object(id) => projection
            .objects
            .iter()
            .any(|record| &record.object_id == id),
        SemanticAddress::Region(address) => projection
            .regions
            .iter()
            .any(|record| &record.address == address),
        SemanticAddress::Unit(id) => projection.units.iter().any(|record| &record.unit_id == id),
        SemanticAddress::Occurrence(id) => projection
            .occurrences
            .iter()
            .any(|record| &record.occurrence_id == id),
        SemanticAddress::TemporalAnchor(id) => projection
            .temporal_anchors
            .iter()
            .any(|record| &record.anchor_id == id),
        SemanticAddress::Identifier(_) | SemanticAddress::RetrievalSurface(_) => false,
    }
}

#[test]
fn synthetic_access_builds_and_executes_all_five_surfaces() {
    let projection = bind_logical_hash(tiny_projection());
    assert_eq!(
        projection.validation_status,
        ProjectionValidationStatus::Validated
    );
    let artifacts =
        build_projection_access_artifacts(&projection, None, None).expect("access builds");
    artifacts
        .validate_against(&projection)
        .expect("access remains projection-bound");
    let repeated =
        build_projection_access_artifacts(&projection, None, None).expect("repeated access builds");
    assert_eq!(
        serde_json::to_vec(&artifacts).expect("artifact serializes"),
        serde_json::to_vec(&repeated).expect("repeated artifact serializes")
    );

    let exact = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:exact",
                semantic_traversal_core::model::RetrievalSurfaceKind::Exact,
                semantic_traversal_core::projection::SurfaceMatchMode::Literal,
                AccessOperand::ExactLiteral(projection.units[0].unit_id.to_string()),
                10,
            ),
        )
        .expect("exact probe executes");
    assert_eq!(exact.returned_count, 1);

    let lexical_probe = probe(
        &projection,
        "surface:lexical",
        semantic_traversal_core::model::RetrievalSurfaceKind::Lexical,
        semantic_traversal_core::projection::SurfaceMatchMode::Terms,
        AccessOperand::LexicalTerms(vec!["capital".into()]),
        2,
    );
    let lexical = artifacts
        .probe(&projection, &lexical_probe)
        .expect("lexical probe executes");
    assert!(!lexical.candidates.is_empty());
    assert!(
        lexical
            .candidates
            .iter()
            .all(|candidate| address_exists(&projection, &candidate.identity))
    );

    let graph = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:graph",
                semantic_traversal_core::model::RetrievalSurfaceKind::Graph,
                semantic_traversal_core::projection::SurfaceMatchMode::Incidence,
                AccessOperand::Graph {
                    seed: SemanticAddress::Object(object(JOURNAL_ONE_OBJECT)),
                    direction: Direction::Outgoing,
                    transition_ids: vec![],
                },
                100,
            ),
        )
        .expect("graph probe executes");
    assert!(!graph.candidates.is_empty());
    assert!(
        graph
            .candidates
            .iter()
            .all(|candidate| address_exists(&projection, &candidate.identity))
    );

    let temporal = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:temporal",
                semantic_traversal_core::model::RetrievalSurfaceKind::Temporal,
                semantic_traversal_core::projection::SurfaceMatchMode::Temporal,
                AccessOperand::Temporal(TemporalQuery::Exact {
                    precision: TemporalPrecision::FullDate,
                    value: "2026-07-02".into(),
                }),
                10,
            ),
        )
        .expect("temporal probe executes");
    assert_eq!(temporal.returned_count, 1);
    assert!(
        temporal
            .candidates
            .iter()
            .all(|candidate| address_exists(&projection, &candidate.identity))
    );

    let vector = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:vector",
                semantic_traversal_core::model::RetrievalSurfaceKind::Vector,
                semantic_traversal_core::projection::SurfaceMatchMode::NearestNeighbours,
                AccessOperand::Vector(vec![0.0; 1024]),
                10,
            ),
        )
        .expect("unavailable vector probe is represented as a result");
    assert_eq!(vector.returned_count, 0);
    assert_eq!(
        vector.failure.as_ref().map(|failure| failure.code.as_str()),
        Some("provider_not_configured")
    );
}

#[test]
fn lexical_paging_is_deterministic_and_zero_results_are_valid() {
    let projection = bind_logical_hash(tiny_projection());
    let artifacts =
        build_projection_access_artifacts(&projection, None, None).expect("access builds");
    let first_probe = probe(
        &projection,
        "surface:lexical",
        semantic_traversal_core::model::RetrievalSurfaceKind::Lexical,
        semantic_traversal_core::projection::SurfaceMatchMode::Terms,
        AccessOperand::LexicalTerms(vec!["the".into()]),
        1,
    );
    let first = artifacts
        .probe(&projection, &first_probe)
        .expect("first page executes");
    assert!(first.truncated);
    let mut second_probe = first_probe.clone();
    second_probe.cursor = first.continuation.map(|continuation| continuation.cursor);
    let second = artifacts
        .probe(&projection, &second_probe)
        .expect("continuation executes");
    assert_ne!(first.candidates[0].identity, second.candidates[0].identity);

    let zero = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:exact",
                semantic_traversal_core::model::RetrievalSurfaceKind::Exact,
                semantic_traversal_core::projection::SurfaceMatchMode::Literal,
                AccessOperand::ExactLiteral("no-such-literal".into()),
                10,
            ),
        )
        .expect("zero result executes");
    assert_eq!(zero.returned_count, 0);
    assert_eq!(zero.total_candidate_count, Some(0));
}

struct FakeEmbeddingProvider;

impl EmbeddingProvider for FakeEmbeddingProvider {
    fn identity(
        &self,
    ) -> Result<VectorProviderIdentity, semantic_traversal_core::access::AccessFailure> {
        Ok(VectorProviderIdentity {
            contract: VectorProviderContract {
                provider: "test".into(),
                requested_model: "test-model".into(),
                dimension: 2,
                dtype: "float32".into(),
                normalization: "L2".into(),
                similarity: "cosine".into(),
                truncation: "disabled".into(),
            },
            endpoint: "test://deterministic".into(),
            resolved_model: "test-model@fixed".into(),
            model_digest: "sha256:test".into(),
            max_input_chars: Some(8),
        })
    }

    fn embed(
        &self,
        inputs: &[String],
    ) -> Result<Vec<Vec<f32>>, semantic_traversal_core::access::AccessFailure> {
        Ok(inputs
            .iter()
            .map(|input| vec![input.len() as f32, 1.0])
            .collect())
    }
}

#[test]
fn vector_segments_remain_subordinate_to_canonical_units() {
    let projection = bind_logical_hash(tiny_projection());
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&FakeEmbeddingProvider))
            .expect("vector access builds");
    assert!(!artifacts.vector.segments.is_empty());
    assert!(artifacts.vector.segments.iter().all(|segment| {
        projection
            .units
            .iter()
            .any(|unit| unit.unit_id == segment.parent_unit_id)
            && !projection
                .units
                .iter()
                .any(|unit| unit.unit_id.to_string() == segment.segment_id)
    }));
    let result = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:vector",
                semantic_traversal_core::model::RetrievalSurfaceKind::Vector,
                semantic_traversal_core::projection::SurfaceMatchMode::NearestNeighbours,
                AccessOperand::Vector(vec![1.0, 0.0]),
                5,
            ),
        )
        .expect("vector probe executes");
    assert!(
        result
            .candidates
            .iter()
            .all(|candidate| { matches!(candidate.identity, SemanticAddress::Unit(_)) })
    );
}

#[test]
fn current_corpus_access_executes_when_private_inputs_are_supplied() {
    let Some(projection_path) = env::var_os("CLEANROOM_PHASE7_PROJECTION") else {
        eprintln!("skipping current-corpus access test: CLEANROOM_PHASE7_PROJECTION is unset");
        return;
    };
    let Some(observation_path) = env::var_os("CLEANROOM_PHASE7_OBSERVATION") else {
        eprintln!("skipping current-corpus access test: CLEANROOM_PHASE7_OBSERVATION is unset");
        return;
    };
    let projection: SemanticSpaceProjection =
        serde_json::from_slice(&fs::read(projection_path).expect("projection reads"))
            .expect("projection parses");
    let observation: Value =
        serde_json::from_slice(&fs::read(observation_path).expect("observation reads"))
            .expect("observation parses");
    let projection_bytes_before = fs::read(
        env::var_os("CLEANROOM_PHASE7_PROJECTION").expect("projection path remains available"),
    )
    .expect("projection rereads");
    let observation_bytes_before = fs::read(
        env::var_os("CLEANROOM_PHASE7_OBSERVATION").expect("observation path remains available"),
    )
    .expect("observation rereads");
    let artifacts = build_projection_access_artifacts(&projection, Some(&observation), None)
        .expect("current access builds");
    artifacts
        .validate_against(&projection)
        .expect("current access binds exactly");

    let first_unit = projection
        .units
        .first()
        .expect("current projection has units");
    let first_object = projection
        .objects
        .first()
        .expect("current projection has objects");
    let first_anchor = projection
        .temporal_anchors
        .first()
        .expect("current projection has anchors");
    let (precision, value) = match &first_anchor.value {
        TemporalValue::FullDate(value) => (TemporalPrecision::FullDate, value.clone()),
        TemporalValue::DateTime(value) => (TemporalPrecision::DateTime, value.clone()),
        TemporalValue::ExactYear(value) => (TemporalPrecision::ExactYear, value.to_string()),
        TemporalValue::MonthDay(value) => (TemporalPrecision::MonthDay, value.clone()),
        TemporalValue::ApproximateYear(value) => {
            (TemporalPrecision::ApproximateYear, value.clone())
        }
    };
    let probes = vec![
        probe(
            &projection,
            "surface:exact",
            semantic_traversal_core::model::RetrievalSurfaceKind::Exact,
            semantic_traversal_core::projection::SurfaceMatchMode::Literal,
            AccessOperand::ExactLiteral(first_unit.unit_id.to_string()),
            5,
        ),
        probe(
            &projection,
            "surface:lexical",
            semantic_traversal_core::model::RetrievalSurfaceKind::Lexical,
            semantic_traversal_core::projection::SurfaceMatchMode::Terms,
            AccessOperand::LexicalTerms(vec!["the".into()]),
            5,
        ),
        probe(
            &projection,
            "surface:vector",
            semantic_traversal_core::model::RetrievalSurfaceKind::Vector,
            semantic_traversal_core::projection::SurfaceMatchMode::NearestNeighbours,
            AccessOperand::Vector(vec![0.0; 1024]),
            5,
        ),
        probe(
            &projection,
            "surface:graph",
            semantic_traversal_core::model::RetrievalSurfaceKind::Graph,
            semantic_traversal_core::projection::SurfaceMatchMode::Incidence,
            AccessOperand::Graph {
                seed: SemanticAddress::Object(first_object.object_id.clone()),
                direction: Direction::Outgoing,
                transition_ids: vec![],
            },
            5,
        ),
        probe(
            &projection,
            "surface:temporal",
            semantic_traversal_core::model::RetrievalSurfaceKind::Temporal,
            semantic_traversal_core::projection::SurfaceMatchMode::Temporal,
            AccessOperand::Temporal(TemporalQuery::Exact { precision, value }),
            5,
        ),
    ];
    for probe in probes {
        let result = artifacts
            .probe(&projection, &probe)
            .expect("declared current probe executes");
        assert!(
            result
                .candidates
                .iter()
                .all(|candidate| address_exists(&projection, &candidate.identity))
        );
    }
    assert_eq!(
        projection_bytes_before,
        fs::read(
            env::var_os("CLEANROOM_PHASE7_PROJECTION").expect("projection path remains available")
        )
        .expect("projection rereads after access")
    );
    assert_eq!(
        observation_bytes_before,
        fs::read(
            env::var_os("CLEANROOM_PHASE7_OBSERVATION")
                .expect("observation path remains available")
        )
        .expect("observation rereads after access")
    );
}
