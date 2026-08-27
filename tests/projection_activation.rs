//! Synthetic mechanical coverage for the current Phase 7 access boundary.
//! These tests are not real-corpus Phase 8 acceptance.

#[path = "support/mod.rs"]
mod support;

use semantic_traversal_core::{
    ProjectionActivationAccess,
    access::{
        AccessFailure, EmbeddingProvider, VectorProviderContract, VectorProviderIdentity,
        build_projection_access_artifacts,
    },
    activate_projection,
    activation::{
        ActivationProvenance, ActivationUtterance, ProjectionActivationBandConfig,
        ProjectionActivationConfig, ProjectionActivationSurfaceConfig,
        ProjectionActivationViolation, TruncationState,
    },
    model::RetrievalSurfaceKind,
    problem_space::{
        ActivationBand, AttentionLens, ProblemRegion, ProblemSpaceState, RegionPersistenceState,
        SourceTurnRange,
    },
    projection::SemanticSpaceProjection,
};
use sha2::{Digest, Sha256};
use support::synthetic_projection::{occurrence, tiny_projection, unit};

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

#[derive(Clone)]
struct DeterministicEmbeddingProvider {
    endpoint: String,
    digest: String,
}

impl DeterministicEmbeddingProvider {
    fn new(tag: &str) -> Self {
        Self {
            endpoint: format!("test://activation/{tag}"),
            digest: format!("sha256:activation-{tag}"),
        }
    }
}

impl EmbeddingProvider for DeterministicEmbeddingProvider {
    fn identity(&self) -> Result<VectorProviderIdentity, AccessFailure> {
        Ok(VectorProviderIdentity {
            contract: VectorProviderContract {
                provider: "test".into(),
                requested_model: "activation-test".into(),
                dimension: 2,
                dtype: "float32".into(),
                normalization: "L2".into(),
                similarity: "cosine".into(),
                truncation: "disabled".into(),
            },
            endpoint: self.endpoint.clone(),
            resolved_model: "activation-test@fixed".into(),
            model_digest: self.digest.clone(),
            max_input_chars: None,
        })
    }

    fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, AccessFailure> {
        Ok(inputs
            .iter()
            .map(|input| vec![input.len().max(1) as f32, 1.0])
            .collect())
    }
}

fn band(maximum_textual_seeds: u32) -> ProjectionActivationBandConfig {
    ProjectionActivationBandConfig {
        maximum_textual_seeds,
        maximum_structural_neighbors_per_record: 8,
        maximum_visible_units_per_region: 8,
        text_preview_character_limit: 80,
    }
}

fn config(
    exact: u32,
    lexical: u32,
    vector: u32,
    graph: u32,
    temporal: u32,
) -> ProjectionActivationConfig {
    let surface_limit = |surface_id: &str, limit| ProjectionActivationSurfaceConfig {
        surface_id: surface_id.into(),
        unbanded_candidate_limit: limit,
        primary_candidate_limit: limit,
        secondary_candidate_limit: limit,
        tertiary_candidate_limit: limit,
        background_candidate_limit: limit,
    };
    ProjectionActivationConfig {
        configuration_snapshot_id: "configuration:activation-test".into(),
        unbanded: band(1),
        primary: band(0),
        secondary: band(0),
        tertiary: band(0),
        background: band(0),
        surface_limits: vec![
            surface_limit("surface:exact", exact),
            surface_limit("surface:lexical", lexical),
            surface_limit("surface:vector", vector),
            surface_limit("surface:graph", graph),
            surface_limit("surface:temporal", temporal),
        ],
        maximum_expansion_budget: 64,
        hub_degree_threshold: 32,
        maximum_initial_relation_depth: 2,
        continuation_page_limit: 4,
        maximum_activated_objects: 16,
        maximum_activated_regions: 16,
        maximum_activated_units: 16,
        maximum_activated_identifier_assignments: 32,
        maximum_activated_occurrences: 16,
        maximum_activated_temporal_anchors: 16,
        maximum_activated_edges: 64,
        maximum_telemetry_records: 128,
        maximum_continuation_handles: 16,
    }
}

fn problem_space() -> ProblemSpaceState {
    ProblemSpaceState {
        thread_id: "thread:activation-test".into(),
        version: 1,
        regions: vec![ProblemRegion {
            region_id: "problem-region:capital".into(),
            anchor_referents: vec![],
            relation_ids: vec![],
            local_constraint_ids: vec![],
            open_tension_ids: vec![],
            source_contribution_ids: vec!["contribution:activation-test".into()],
            persistence_state: RegionPersistenceState::Active,
            activation_band: ActivationBand::Primary,
            supersedes_region_id: None,
        }],
        relations: vec![],
        constraints: vec![],
        open_tensions: vec![],
        contribution_history: vec![],
        attention_lens: AttentionLens {
            primary_region_ids: vec!["problem-region:capital".into()],
            secondary_region_ids: vec![],
            tertiary_region_ids: vec![],
            background_region_ids: vec![],
        },
        source_turn_range: SourceTurnRange {
            first_turn_id: "turn:1".into(),
            last_turn_id: "turn:1".into(),
        },
    }
}

fn utterance(text: &str) -> ActivationUtterance {
    ActivationUtterance {
        utterance_id: "utterance:activation-test".into(),
        text: text.into(),
    }
}

#[test]
fn synthetic_mechanical_activation_consumes_current_phase7_access() {
    let projection = bind_logical_hash(tiny_projection());
    let provider = DeterministicEmbeddingProvider::new("same");
    let artifacts = build_projection_access_artifacts(&projection, None, Some(&provider))
        .expect("access builds");
    let access = ProjectionActivationAccess::with_query_embedding_provider(&artifacts, &provider);
    let activated = activate_projection(
        &projection,
        &problem_space(),
        &utterance("Capital"),
        &config(4, 4, 4, 16, 4),
        &access,
    )
    .expect("activation succeeds over current access artifacts");

    assert_eq!(
        activated.projection_snapshot_id,
        projection.projection_snapshot_id
    );
    for kind in [
        RetrievalSurfaceKind::Exact,
        RetrievalSurfaceKind::Lexical,
        RetrievalSurfaceKind::Vector,
        RetrievalSurfaceKind::Graph,
        RetrievalSurfaceKind::Temporal,
    ] {
        assert!(
            activated
                .telemetry
                .iter()
                .any(|telemetry| telemetry.surface_kind == kind),
            "missing telemetry for {kind:?}"
        );
    }
    assert!(
        activated
            .activated_units
            .iter()
            .any(|record| record.unit_id == unit("unit:capital:chapter-2:1"))
    );
    assert!(
        activated
            .activated_occurrences
            .iter()
            .any(|record| record.occurrence_id == occurrence("occurrence:heading-only:capital"))
    );
    assert!(activated.telemetry.iter().any(|telemetry| {
        telemetry
            .activation_provenance
            .contains(&ActivationProvenance::NewestUtterance {
                utterance_id: "utterance:activation-test".into(),
            })
            && telemetry
                .activation_provenance
                .contains(&ActivationProvenance::ConfiguredDefault {
                    configuration_key: "automatic_surface_fan_out".into(),
                })
    }));
    assert!(activated.telemetry.iter().any(|telemetry| {
        telemetry.surface_kind == RetrievalSurfaceKind::Temporal
            && telemetry.temporal_anchor_count > 0
    }));
    assert!(activated.telemetry.iter().any(|telemetry| {
        telemetry.surface_kind == RetrievalSurfaceKind::Lexical
            && telemetry
                .identifier_type_distribution
                .iter()
                .any(|entry| entry.label == "semantic_unit" && entry.count > 0)
    }));
    assert!(
        !serde_json::to_string(&activated)
            .expect("activation serializes")
            .contains("configuration_snapshot_id\":\"projection")
    );
}

#[test]
fn synthetic_mechanical_activation_omits_atomic_bundle_when_bound_fails() {
    let projection = bind_logical_hash(tiny_projection());
    let provider = DeterministicEmbeddingProvider::new("same");
    let artifacts = build_projection_access_artifacts(&projection, None, Some(&provider))
        .expect("access builds");
    let access = ProjectionActivationAccess::with_query_embedding_provider(&artifacts, &provider);
    let mut config = config(1, 0, 0, 0, 0);
    config.maximum_activated_objects = 0;

    let activated = activate_projection(
        &projection,
        &problem_space(),
        &utterance("unit:journal:2026-07-02:1"),
        &config,
        &access,
    )
    .expect("bounded omission remains a positive activation result");

    assert!(activated.activated_objects.is_empty());
    assert!(activated.activated_regions.is_empty());
    assert!(activated.activated_units.is_empty());
    assert!(activated.activated_identifier_assignments.is_empty());
    assert!(
        activated
            .telemetry
            .iter()
            .any(|telemetry| telemetry.surface_id == "surface:exact"
                && telemetry.truncation_state == TruncationState::Bounded)
    );
}

#[test]
fn synthetic_mechanical_activation_rejects_mismatched_vector_query_provider() {
    let projection = bind_logical_hash(tiny_projection());
    let build_provider = DeterministicEmbeddingProvider::new("build");
    let query_provider = DeterministicEmbeddingProvider::new("query");
    let artifacts = build_projection_access_artifacts(&projection, None, Some(&build_provider))
        .expect("access builds");
    let access =
        ProjectionActivationAccess::with_query_embedding_provider(&artifacts, &query_provider);

    let error = activate_projection(
        &projection,
        &problem_space(),
        &utterance("Capital"),
        &config(0, 0, 1, 0, 0),
        &access,
    )
    .expect_err("mismatched vector provider fails closed");

    assert!(matches!(
        error,
        ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id,
            context,
            ..
        } if surface_id == "surface:vector"
            && context.contains("provider identity does not match")
    ));
}

#[test]
fn synthetic_mechanical_activation_uses_one_multi_term_lexical_probe() {
    let projection = bind_logical_hash(tiny_projection());
    let artifacts =
        build_projection_access_artifacts(&projection, None, None).expect("access builds");
    let access = ProjectionActivationAccess::new(&artifacts);

    let activated = activate_projection(
        &projection,
        &problem_space(),
        &utterance("Capital chapter"),
        &config(0, 4, 0, 0, 0),
        &access,
    )
    .expect("multi-term lexical activation succeeds");

    let lexical_probes = activated
        .telemetry
        .iter()
        .filter(|telemetry| telemetry.surface_kind == RetrievalSurfaceKind::Lexical)
        .count();
    assert_eq!(lexical_probes, 1);
}

#[test]
fn synthetic_mechanical_activation_applies_structural_preview_bounds() {
    let projection = bind_logical_hash(tiny_projection());
    let artifacts =
        build_projection_access_artifacts(&projection, None, None).expect("access builds");
    let access = ProjectionActivationAccess::new(&artifacts);
    let mut config = config(4, 4, 0, 16, 0);
    config.unbanded.maximum_structural_neighbors_per_record = 1;
    config.unbanded.maximum_visible_units_per_region = 1;

    let activated = activate_projection(
        &projection,
        &problem_space(),
        &utterance("Capital"),
        &config,
        &access,
    )
    .expect("bounded structural activation succeeds");

    assert!(activated.activated_objects.iter().all(|record| {
        record.visible_region_addresses.len() + record.visible_unit_ids.len() <= 1
    }));
    assert!(
        activated
            .activated_regions
            .iter()
            .all(|record| record.visible_unit_ids.len() <= 1)
    );
}
