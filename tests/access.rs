#[path = "support/mod.rs"]
mod support;

use std::{
    env, fs,
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use semantic_traversal_core::{
    access::{
        AccessFailure, AccessOperand, EmbeddingProvider, ProjectionAccessArtifacts,
        TemporalPrecision, TemporalQuery, VectorProviderContract, VectorProviderIdentity,
        VectorProviderState, build_projection_access_artifacts,
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

#[derive(Clone, Copy)]
enum ProviderResponseMode {
    Valid,
    Fail,
    WrongCount,
    WrongDimension,
}

struct RecordingEmbeddingProvider {
    calls: Arc<Mutex<Vec<(usize, Vec<String>)>>>,
    completions: Arc<Mutex<Vec<String>>>,
    call_number: AtomicUsize,
    completion_barrier: Option<Arc<Barrier>>,
    max_input_chars: Option<usize>,
    response_mode: ProviderResponseMode,
}

impl RecordingEmbeddingProvider {
    fn new(
        max_input_chars: Option<usize>,
        response_mode: ProviderResponseMode,
        force_out_of_order: bool,
    ) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            completions: Arc::new(Mutex::new(Vec::new())),
            call_number: AtomicUsize::new(0),
            completion_barrier: force_out_of_order.then(|| Arc::new(Barrier::new(2))),
            max_input_chars,
            response_mode,
        }
    }

    fn identity_value(&self) -> VectorProviderIdentity {
        VectorProviderIdentity {
            contract: VectorProviderContract {
                provider: "test".into(),
                requested_model: "test-model".into(),
                dimension: 2,
                dtype: "float32".into(),
                normalization: "L2".into(),
                similarity: "cosine".into(),
                truncation: "disabled".into(),
            },
            endpoint: "test://recording".into(),
            resolved_model: "test-model@fixed".into(),
            model_digest: "sha256:recording".into(),
            max_input_chars: self.max_input_chars,
        }
    }
}

impl EmbeddingProvider for RecordingEmbeddingProvider {
    fn identity(&self) -> Result<VectorProviderIdentity, AccessFailure> {
        Ok(self.identity_value())
    }

    fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, AccessFailure> {
        let call_number = self.call_number.fetch_add(1, Ordering::SeqCst);
        self.calls
            .lock()
            .expect("call log is not poisoned")
            .push((call_number, inputs.to_vec()));
        let input = inputs.first().cloned().unwrap_or_default();
        if let Some(barrier) = &self.completion_barrier {
            if call_number < 2 {
                barrier.wait();
                if call_number == 0 {
                    thread::sleep(Duration::from_millis(30));
                }
            }
        }
        self.completions
            .lock()
            .expect("completion log is not poisoned")
            .push(input.clone());
        match self.response_mode {
            ProviderResponseMode::Valid => Ok(vec![vec![input.len() as f32, 1.0]]),
            ProviderResponseMode::Fail => Err(AccessFailure {
                code: "provider_test_failure".into(),
                message: "recording provider rejected this segment".into(),
                retryable: false,
            }),
            ProviderResponseMode::WrongCount => Ok(Vec::new()),
            ProviderResponseMode::WrongDimension => Ok(vec![vec![1.0]]),
        }
    }
}

#[test]
fn vector_build_issues_one_request_per_segment_and_preserves_order() {
    let projection = bind_logical_hash(tiny_projection());
    let provider = RecordingEmbeddingProvider::new(None, ProviderResponseMode::Valid, true);
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&provider)).expect("builds");
    let calls = provider.calls.lock().expect("call log is not poisoned");
    assert_eq!(calls.len(), artifacts.vector.segments.len());
    assert!(calls.iter().all(|(_, inputs)| inputs.len() == 1));
    let mut call_order = calls.clone();
    call_order.sort_by_key(|(call_number, _)| *call_number);
    let completions = provider
        .completions
        .lock()
        .expect("completion log is not poisoned");
    assert_ne!(
        completions.first().map(|input| input.as_str()),
        call_order
            .first()
            .and_then(|(_, inputs)| inputs.first())
            .map(|input| input.as_str())
    );
    assert_eq!(
        artifacts
            .vector
            .segments
            .iter()
            .map(|segment| segment.parent_unit_id.clone())
            .collect::<Vec<_>>(),
        projection
            .units
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn one_failed_segment_fails_vector_build_closed_with_segment_identity() {
    let projection = bind_logical_hash(tiny_projection());
    let provider = RecordingEmbeddingProvider::new(None, ProviderResponseMode::Fail, false);
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&provider)).expect("builds");
    assert!(artifacts.vector.segments.is_empty());
    let failure = match &artifacts.vector.provider {
        semantic_traversal_core::access::VectorProviderState::Unavailable { failure, .. } => {
            failure
        }
        state => panic!("expected unavailable vector state, got {state:?}"),
    };
    assert_eq!(failure.code, "provider_test_failure");
    assert!(failure.message.contains("unit_index=0"));
    assert!(failure.message.contains("parent_unit_id="));
}

#[test]
fn wrong_embedding_count_fails_vector_build_closed() {
    let projection = bind_logical_hash(tiny_projection());
    let provider = RecordingEmbeddingProvider::new(None, ProviderResponseMode::WrongCount, false);
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&provider)).expect("builds");
    assert!(artifacts.vector.segments.is_empty());
    let failure = match &artifacts.vector.provider {
        semantic_traversal_core::access::VectorProviderState::Unavailable { failure, .. } => {
            failure
        }
        state => panic!("expected unavailable vector state, got {state:?}"),
    };
    assert_eq!(failure.code, "provider_shape_mismatch");
    assert!(failure.message.contains("returned_embeddings=0"));
}

#[test]
fn wrong_embedding_dimension_fails_vector_build_closed() {
    let projection = bind_logical_hash(tiny_projection());
    let provider =
        RecordingEmbeddingProvider::new(None, ProviderResponseMode::WrongDimension, false);
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&provider)).expect("builds");
    assert!(artifacts.vector.segments.is_empty());
    let failure = match &artifacts.vector.provider {
        semantic_traversal_core::access::VectorProviderState::Unavailable { failure, .. } => {
            failure
        }
        state => panic!("expected unavailable vector state, got {state:?}"),
    };
    assert_eq!(failure.code, "provider_shape_mismatch");
    assert!(failure.message.contains("returned_dimension=1"));
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
fn zero_vector_operand_fails_closed_as_invalid_cosine_query() {
    let projection = bind_logical_hash(tiny_projection());
    let artifacts =
        build_projection_access_artifacts(&projection, None, Some(&FakeEmbeddingProvider))
            .expect("vector access builds");
    let error = artifacts
        .probe(
            &projection,
            &probe(
                &projection,
                "surface:vector",
                semantic_traversal_core::model::RetrievalSurfaceKind::Vector,
                semantic_traversal_core::projection::SurfaceMatchMode::NearestNeighbours,
                AccessOperand::Vector(vec![0.0, 0.0]),
                5,
            ),
        )
        .expect_err("zero-norm cosine operand is not searchable");
    assert!(matches!(
        error,
        semantic_traversal_core::access::AccessError::Probe(message)
            if message == "zero vector operand is not searchable"
    ));
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

#[test]
fn provider_backed_current_artifact_validates_and_probes_all_surfaces() {
    let Some(artifact_path) = env::var_os("CLEANROOM_PHASE7_ARTIFACT") else {
        eprintln!("skipping provider-backed artifact test: CLEANROOM_PHASE7_ARTIFACT is unset");
        return;
    };
    let projection_path = env::var_os("CLEANROOM_PHASE7_PROJECTION")
        .expect("provider-backed artifact test requires CLEANROOM_PHASE7_PROJECTION");
    let projection: SemanticSpaceProjection =
        serde_json::from_slice(&fs::read(projection_path).expect("projection reads"))
            .expect("projection parses");
    let artifacts: ProjectionAccessArtifacts =
        serde_json::from_slice(&fs::read(artifact_path).expect("provider artifact reads"))
            .expect("provider artifact parses");
    artifacts
        .validate_against(&projection)
        .expect("provider artifact remains bound to the accepted projection");
    assert!(!artifacts.vector.segments.is_empty());
    assert!(matches!(
        artifacts.vector.provider,
        VectorProviderState::Ready { .. }
    ));

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
            AccessOperand::Vector(artifacts.vector.segments[0].embedding.clone()),
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
            .expect("provider-backed declared probe executes");
        assert!(
            result
                .candidates
                .iter()
                .all(|candidate| address_exists(&projection, &candidate.identity))
        );
    }
    let zero_vector = probe(
        &projection,
        "surface:vector",
        semantic_traversal_core::model::RetrievalSurfaceKind::Vector,
        semantic_traversal_core::projection::SurfaceMatchMode::NearestNeighbours,
        AccessOperand::Vector(vec![0.0; 1024]),
        5,
    );
    assert!(matches!(
        artifacts.probe(&projection, &zero_vector),
        Err(semantic_traversal_core::access::AccessError::Probe(message))
            if message == "zero vector operand is not searchable"
    ));

    let mut mismatched_projection = projection.clone();
    mismatched_projection
        .corpus_snapshot_identity
        .push_str(":changed");
    mismatched_projection.logical_hash.clear();
    mismatched_projection.logical_hash = format!(
        "sha256:{}",
        Sha256::digest(serde_json::to_vec(&mismatched_projection).expect("projection serializes"))
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    assert!(artifacts.validate_against(&mismatched_projection).is_err());
}
