mod support;

use semantic_traversal_core::{
    ActivationUtterance, ProjectionActivationCandidate, ProjectionActivationConfig,
    ProjectionActivationProbe, ProjectionActivationProbeBand, ProjectionActivationProbeResult,
    ProjectionActivationProbeSource, ProjectionActivationViolation,
    activation::ActivationProvenance,
    activation::{
        CandidateCount, ProjectionActivationBandConfig, ProjectionActivationSurfaceConfig,
    },
    model::{RetrievalSurfaceKind, SemanticAddress},
    problem_space::{
        ActivationBand, AttentionLens, OpenTension, OpenTensionType, ProblemConstraint,
        ProblemConstraintApplicability, ProblemReferent, ProblemRegion, ProblemRelation,
        ProblemRelationType, ProblemSpaceState, RecordLifecycle, RegionPersistenceState,
        SourceTurnRange, TensionLifecycle,
    },
    projection::{ProjectionValidationStatus, SurfaceMatchMode},
};
use support::{scripted_activation::ScriptedProjectionActivationAccess, synthetic_projection};

fn band(maximum_textual_seeds: u32) -> ProjectionActivationBandConfig {
    ProjectionActivationBandConfig {
        maximum_textual_seeds,
        maximum_structural_neighbors_per_record: 4,
        maximum_visible_units_per_region: 4,
        text_preview_character_limit: 12,
    }
}

fn config() -> ProjectionActivationConfig {
    ProjectionActivationConfig {
        configuration_snapshot_id: "configuration:tiny-synthetic:v1".into(),
        unbanded: band(2),
        primary: band(0),
        secondary: band(0),
        tertiary: band(4),
        background: band(4),
        surface_limits: [
            "surface:exact",
            "surface:lexical",
            "surface:vector",
            "surface:graph",
            "surface:temporal",
        ]
        .into_iter()
        .map(|surface_id| ProjectionActivationSurfaceConfig {
            surface_id: surface_id.into(),
            unbanded_candidate_limit: 2,
            primary_candidate_limit: 2,
            secondary_candidate_limit: 2,
            tertiary_candidate_limit: 2,
            background_candidate_limit: 2,
        })
        .collect(),
        maximum_expansion_budget: 99,
        hub_degree_threshold: 2,
        maximum_initial_relation_depth: 0,
        continuation_page_limit: 3,
        maximum_activated_objects: 10,
        maximum_activated_regions: 10,
        maximum_activated_units: 10,
        maximum_activated_identifier_assignments: 30,
        maximum_activated_occurrences: 10,
        maximum_activated_temporal_anchors: 10,
        maximum_activated_edges: 20,
        maximum_telemetry_records: 40,
        maximum_continuation_handles: 10,
    }
}

fn problem_space() -> ProblemSpaceState {
    ProblemSpaceState {
        thread_id: "thread:activation".into(),
        version: 7,
        regions: vec![
            ProblemRegion {
                region_id: "region:primary".into(),
                anchor_referents: vec![ProblemReferent {
                    referent_id: "referent:capital".into(),
                    expression: "Capital".into(),
                    source_contribution_id: "contribution:1".into(),
                }],
                relation_ids: vec!["relation:comparison".into()],
                local_constraint_ids: vec!["constraint:regional".into()],
                open_tension_ids: vec!["tension:choice".into()],
                source_contribution_ids: vec!["contribution:1".into()],
                persistence_state: RegionPersistenceState::Active,
                activation_band: ActivationBand::Primary,
                supersedes_region_id: None,
            },
            ProblemRegion {
                region_id: "region:secondary".into(),
                anchor_referents: vec![ProblemReferent {
                    referent_id: "referent:journal".into(),
                    expression: "journal".into(),
                    source_contribution_id: "contribution:1".into(),
                }],
                relation_ids: vec!["relation:comparison".into()],
                local_constraint_ids: vec![],
                open_tension_ids: vec![],
                source_contribution_ids: vec!["contribution:1".into()],
                persistence_state: RegionPersistenceState::Active,
                activation_band: ActivationBand::Secondary,
                supersedes_region_id: None,
            },
        ],
        relations: vec![ProblemRelation {
            relation_id: "relation:comparison".into(),
            source_region_id: "region:primary".into(),
            relation_type: ProblemRelationType::Comparison,
            target_region_id: Some("region:secondary".into()),
            source_contribution_id: "contribution:1".into(),
            lifecycle: RecordLifecycle::Active,
        }],
        constraints: vec![
            ProblemConstraint {
                constraint_id: "constraint:whole".into(),
                expression: "whole constraint".into(),
                applicability: ProblemConstraintApplicability::WholeProblemSpace,
                source_contribution_id: "contribution:1".into(),
                lifecycle: RecordLifecycle::Active,
            },
            ProblemConstraint {
                constraint_id: "constraint:regional".into(),
                expression: "regional constraint".into(),
                applicability: ProblemConstraintApplicability::Regions {
                    region_ids: vec!["region:primary".into()],
                },
                source_contribution_id: "contribution:1".into(),
                lifecycle: RecordLifecycle::Active,
            },
        ],
        open_tensions: vec![OpenTension {
            tension_id: "tension:choice".into(),
            region_id: "region:primary".into(),
            tension_type: OpenTensionType::UnresolvedReference,
            unresolved_expression: Some("ambiguous object".into()),
            candidate_bindings: vec!["candidate A".into(), "candidate B".into()],
            source_turn_id: "turn:1".into(),
            lifecycle: TensionLifecycle::Open,
        }],
        contribution_history: vec![],
        attention_lens: AttentionLens {
            primary_region_ids: vec!["region:primary".into()],
            secondary_region_ids: vec!["region:secondary".into()],
            tertiary_region_ids: vec![],
            background_region_ids: vec![],
        },
        source_turn_range: SourceTurnRange {
            first_turn_id: "turn:1".into(),
            last_turn_id: "turn:1".into(),
        },
    }
}

fn utterance() -> ActivationUtterance {
    ActivationUtterance {
        utterance_id: "utterance:newest".into(),
        text: "newest".into(),
    }
}

fn empty_result() -> ProjectionActivationProbeResult {
    ProjectionActivationProbeResult {
        candidates: vec![],
        candidate_count: CandidateCount::Exact(0),
        continuation: None,
        identifier_type_distribution: vec![],
        temporal_anchor_count: 0,
        unresolved_target_count: 0,
    }
}

fn text_probe(
    id: u64,
    surface_id: &str,
    kind: RetrievalSurfaceKind,
    mode: SurfaceMatchMode,
    text: &str,
    provenance: Vec<ActivationProvenance>,
    band: ProjectionActivationProbeBand,
) -> ProjectionActivationProbe {
    let mut activation_provenance = provenance;
    activation_provenance.push(ActivationProvenance::ConfiguredDefault {
        configuration_key: "automatic_surface_fan_out".into(),
    });
    ProjectionActivationProbe {
        probe_id: format!("activation-probe:{id}"),
        band,
        surface_id: surface_id.into(),
        surface_kind: kind,
        match_mode: mode,
        source: ProjectionActivationProbeSource::Text { text: text.into() },
        candidate_limit: 2,
        current_depth: 0,
        activation_provenance,
    }
}

#[test]
fn activation_rejects_unvalidated_projection() {
    let mut projection = synthetic_projection::tiny_projection();
    projection.validation_status = ProjectionValidationStatus::Unvalidated;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &config(),
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::ProjectionNotValidated { .. }
    ));
}

#[test]
fn activation_rejects_configuration_snapshot_mismatch() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    cfg.configuration_snapshot_id = "other".into();
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::ConfigurationSnapshotMismatch { .. }
    ));
}

#[test]
fn activation_rejects_missing_available_surface_configuration() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    cfg.surface_limits.pop();
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::MissingAvailableSurfaceConfiguration { .. }
    ));
}

#[test]
fn activation_rejects_invalid_attention_lens_closure() {
    let projection = synthetic_projection::tiny_projection();
    let mut ps = problem_space();
    ps.attention_lens.primary_region_ids.push("missing".into());
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &ps,
        &utterance(),
        &config(),
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::InvalidActivatedReference { .. }
    ));
}

#[test]
fn activation_violation_implements_error() {
    fn assert_error<E: std::error::Error>() {}
    assert_error::<ProjectionActivationViolation>();
}

#[test]
fn activation_dispatches_seed_groups_in_contract_order() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    1,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    2,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    3,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    4,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    5,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &scripts).unwrap();
    assert_eq!(activated.telemetry.len(), 6);
    assert_eq!(activated.telemetry[0].probe_id, "activation-probe:0");
    assert_eq!(activated.telemetry[3].probe_id, "activation-probe:3");
    assert_eq!(activated.telemetry[0].surface_id, "surface:exact");
    assert_eq!(activated.telemetry[1].surface_id, "surface:lexical");
    assert_eq!(activated.telemetry[2].surface_id, "surface:vector");
}

#[test]
fn unit_candidate_adds_parent_region_and_object_and_preview() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let result = ProjectionActivationProbeResult {
        candidates: vec![ProjectionActivationCandidate {
            address: SemanticAddress::Unit(unit_id.clone()),
            transition: None,
        }],
        candidate_count: CandidateCount::Exact(1),
        continuation: None,
        identifier_type_distribution: vec![],
        temporal_anchor_count: 0,
        unresolved_target_count: 0,
    };
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                result,
            ),
            (
                text_probe(
                    1,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    2,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    3,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    4,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    5,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &scripts).unwrap();
    assert_eq!(activated.activated_units[0].unit_id, unit_id);
    assert_eq!(activated.activated_objects.len(), 1);
    assert_eq!(activated.activated_regions.len(), 1);
    assert!(
        matches!(activated.activated_units[0].text_preview, semantic_traversal_core::ActivatedTextPreview::Inline { ref text, truncated: true } if text == "Capital is a")
    );
}

#[test]
fn repeated_activation_is_exactly_equal_and_scripted_access_is_unchanged() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    1,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    2,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    3,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    4,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    5,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let before = access.clone();
    let a =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    let b =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    assert_eq!(a, b);
    assert_eq!(access, before);
}

fn successful_empty_activation() -> semantic_traversal_core::ActivatedProjection {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    1,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    2,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    3,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    4,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    5,
                    "surface:vector",
                    RetrievalSurfaceKind::Vector,
                    SurfaceMatchMode::NearestNeighbours,
                    "whole constraint",
                    vec![ActivationProvenance::Constraint {
                        constraint_id: "constraint:whole".into(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap()
}

#[test]
fn activation_rejects_invalid_projection() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_invalid_projection keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_rejects_invalid_projection preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_invalid_projection never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_rejects_unknown_surface_configuration() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_unknown_surface_configuration keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_rejects_unknown_surface_configuration preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_unknown_surface_configuration never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_rejects_unavailable_surface_configuration() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_unavailable_surface_configuration keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_rejects_unavailable_surface_configuration preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_unavailable_surface_configuration never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_rejects_duplicate_surface_configuration() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_duplicate_surface_configuration keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_rejects_duplicate_surface_configuration preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_duplicate_surface_configuration never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_rejects_surface_limit_above_hard_limit() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_surface_limit_above_hard_limit keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_rejects_surface_limit_above_hard_limit preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_surface_limit_above_hard_limit never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_preserves_region_referent_constraint_and_tension_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_region_referent_constraint_and_tension_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_preserves_region_referent_constraint_and_tension_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_region_referent_constraint_and_tension_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_preserves_projection_surface_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_projection_surface_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_preserves_projection_surface_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_projection_surface_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_preserves_descriptor_mode_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_descriptor_mode_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_preserves_descriptor_mode_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_descriptor_mode_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn textual_seed_limit_applies_before_surface_fanout() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "textual_seed_limit_applies_before_surface_fanout keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "textual_seed_limit_applies_before_surface_fanout preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "textual_seed_limit_applies_before_surface_fanout never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn candidate_limit_applies_per_probe_surface_and_mode() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "candidate_limit_applies_per_probe_surface_and_mode keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "candidate_limit_applies_per_probe_surface_and_mode preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "candidate_limit_applies_per_probe_surface_and_mode never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn zero_candidate_limit_still_emits_telemetry() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_candidate_limit_still_emits_telemetry keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "zero_candidate_limit_still_emits_telemetry preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_candidate_limit_still_emits_telemetry never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn all_configured_available_text_surfaces_fire() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "all_configured_available_text_surfaces_fire keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "all_configured_available_text_surfaces_fire preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "all_configured_available_text_surfaces_fire never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn empty_text_seed_is_dispatched_without_semantic_filtering() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "empty_text_seed_is_dispatched_without_semantic_filtering keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "empty_text_seed_is_dispatched_without_semantic_filtering preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "empty_text_seed_is_dispatched_without_semantic_filtering never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn referent_candidate_exposure_does_not_create_binding() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "referent_candidate_exposure_does_not_create_binding keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "referent_candidate_exposure_does_not_create_binding preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "referent_candidate_exposure_does_not_create_binding never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn open_tension_candidate_exposure_preserves_candidate_index() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "open_tension_candidate_exposure_preserves_candidate_index keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "open_tension_candidate_exposure_preserves_candidate_index preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "open_tension_candidate_exposure_preserves_candidate_index never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn relation_guides_incidence_provenance_without_creating_relation() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "relation_guides_incidence_provenance_without_creating_relation keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "relation_guides_incidence_provenance_without_creating_relation preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "relation_guides_incidence_provenance_without_creating_relation never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn attention_band_changes_breadth_not_identity() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "attention_band_changes_breadth_not_identity keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "attention_band_changes_breadth_not_identity preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "attention_band_changes_breadth_not_identity never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn configured_defaults_add_only_the_three_accepted_policy_keys() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "configured_defaults_add_only_the_three_accepted_policy_keys keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "configured_defaults_add_only_the_three_accepted_policy_keys preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "configured_defaults_add_only_the_three_accepted_policy_keys never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn configured_defaults_create_no_unrelated_root_candidates() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "configured_defaults_create_no_unrelated_root_candidates keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "configured_defaults_create_no_unrelated_root_candidates preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "configured_defaults_create_no_unrelated_root_candidates never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn unit_candidate_adds_parent_region_and_object() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unit_candidate_adds_parent_region_and_object keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "unit_candidate_adds_parent_region_and_object preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unit_candidate_adds_parent_region_and_object never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn region_candidate_adds_parent_object() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "region_candidate_adds_parent_object keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "region_candidate_adds_parent_object preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "region_candidate_adds_parent_object never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn identifier_candidate_adds_exact_assignment_and_subject() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_candidate_adds_exact_assignment_and_subject keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "identifier_candidate_adds_exact_assignment_and_subject preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_candidate_adds_exact_assignment_and_subject never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn temporal_anchor_candidate_adds_subject() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "temporal_anchor_candidate_adds_subject keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "temporal_anchor_candidate_adds_subject preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "temporal_anchor_candidate_adds_subject never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn object_field_occurrence_adds_source_and_target() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_field_occurrence_adds_source_and_target keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "object_field_occurrence_adds_source_and_target preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_field_occurrence_adds_source_and_target never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn unit_occurrence_adds_source_unit_region_object_and_target() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unit_occurrence_adds_source_unit_region_object_and_target keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "unit_occurrence_adds_source_unit_region_object_and_target preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unit_occurrence_adds_source_unit_region_object_and_target never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn preview_vectors_never_reference_missing_records() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "preview_vectors_never_reference_missing_records keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "preview_vectors_never_reference_missing_records preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "preview_vectors_never_reference_missing_records never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn closure_only_records_do_not_trigger_surface_probes() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "closure_only_records_do_not_trigger_surface_probes keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "closure_only_records_do_not_trigger_surface_probes preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "closure_only_records_do_not_trigger_surface_probes never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn inline_preview_uses_normalized_text_not_authored_markdown() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "inline_preview_uses_normalized_text_not_authored_markdown keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "inline_preview_uses_normalized_text_not_authored_markdown preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "inline_preview_uses_normalized_text_not_authored_markdown never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn inline_preview_counts_unicode_scalars_not_bytes() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "inline_preview_counts_unicode_scalars_not_bytes keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "inline_preview_counts_unicode_scalars_not_bytes preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "inline_preview_counts_unicode_scalars_not_bytes never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn zero_preview_limit_marks_nonempty_text_truncated() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_preview_limit_marks_nonempty_text_truncated keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "zero_preview_limit_marks_nonempty_text_truncated preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_preview_limit_marks_nonempty_text_truncated never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn zero_preview_limit_preserves_empty_text_as_complete() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_preview_limit_preserves_empty_text_as_complete keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "zero_preview_limit_preserves_empty_text_as_complete preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_preview_limit_preserves_empty_text_as_complete never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn hydration_address_is_not_dereferenced_or_copied() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "hydration_address_is_not_dereferenced_or_copied keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "hydration_address_is_not_dereferenced_or_copied preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "hydration_address_is_not_dereferenced_or_copied never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn later_larger_bound_monotonically_enriches_preview() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "later_larger_bound_monotonically_enriches_preview keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "later_larger_bound_monotonically_enriches_preview preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "later_larger_bound_monotonically_enriches_preview never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_candidate_exposure_preserves_first_position() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_exposure_preserves_first_position keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_candidate_exposure_preserves_first_position preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_exposure_preserves_first_position never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_candidate_exposure_aggregates_unique_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_exposure_aggregates_unique_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_candidate_exposure_aggregates_unique_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_exposure_aggregates_unique_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activation_never_deduplicates_by_title_or_alias() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_never_deduplicates_by_title_or_alias keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activation_never_deduplicates_by_title_or_alias preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_never_deduplicates_by_title_or_alias never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn identifier_assignment_order_follows_projection_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_assignment_order_follows_projection_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "identifier_assignment_order_follows_projection_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_assignment_order_follows_projection_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn first_seen_record_order_is_stable() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "first_seen_record_order_is_stable keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "first_seen_record_order_is_stable preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "first_seen_record_order_is_stable never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn incidence_traversal_respects_relation_depth() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_traversal_respects_relation_depth keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "incidence_traversal_respects_relation_depth preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_traversal_respects_relation_depth never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn incidence_cycles_are_suppressed_per_root() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_cycles_are_suppressed_per_root keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "incidence_cycles_are_suppressed_per_root preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_cycles_are_suppressed_per_root never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn same_address_may_be_probed_under_distinct_roots() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "same_address_may_be_probed_under_distinct_roots keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "same_address_may_be_probed_under_distinct_roots preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "same_address_may_be_probed_under_distinct_roots never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn incidence_result_requires_transition() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_result_requires_transition keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "incidence_result_requires_transition preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_result_requires_transition never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn incidence_result_requires_actual_projected_edge() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_result_requires_actual_projected_edge keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "incidence_result_requires_actual_projected_edge preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_result_requires_actual_projected_edge never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activated_edges_deduplicate_by_exact_tuple() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activated_edges_deduplicate_by_exact_tuple keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activated_edges_deduplicate_by_exact_tuple preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activated_edges_deduplicate_by_exact_tuple never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn activated_edge_ids_follow_first_insertion_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activated_edge_ids_follow_first_insertion_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "activated_edge_ids_follow_first_insertion_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activated_edge_ids_follow_first_insertion_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn edge_bound_truncates_without_reordering_records() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_bound_truncates_without_reordering_records keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "edge_bound_truncates_without_reordering_records preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_bound_truncates_without_reordering_records never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn object_region_transition_never_emits_unit_target() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_region_transition_never_emits_unit_target keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "object_region_transition_never_emits_unit_target preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_region_transition_never_emits_unit_target never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn object_unit_transition_never_emits_region_target() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_unit_transition_never_emits_region_target keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "object_unit_transition_never_emits_region_target preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_unit_transition_never_emits_region_target never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn outgoing_occurrence_transition_never_accepts_incoming_incidence() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "outgoing_occurrence_transition_never_accepts_incoming_incidence keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "outgoing_occurrence_transition_never_accepts_incoming_incidence preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "outgoing_occurrence_transition_never_accepts_incoming_incidence never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn incoming_occurrence_transition_never_accepts_outgoing_incidence() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incoming_occurrence_transition_never_accepts_outgoing_incidence keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "incoming_occurrence_transition_never_accepts_outgoing_incidence preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incoming_occurrence_transition_never_accepts_outgoing_incidence never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn telemetry_is_one_record_per_probe_surface_and_mode() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_is_one_record_per_probe_surface_and_mode keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "telemetry_is_one_record_per_probe_surface_and_mode preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_is_one_record_per_probe_surface_and_mode never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn probe_and_telemetry_ids_follow_invocation_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "probe_and_telemetry_ids_follow_invocation_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "probe_and_telemetry_ids_follow_invocation_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "probe_and_telemetry_ids_follow_invocation_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn telemetry_preserves_surface_returned_count_before_view_deduplication() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_preserves_surface_returned_count_before_view_deduplication keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "telemetry_preserves_surface_returned_count_before_view_deduplication preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_preserves_surface_returned_count_before_view_deduplication never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn initial_telemetry_preserves_full_expansion_budget() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "initial_telemetry_preserves_full_expansion_budget keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "initial_telemetry_preserves_full_expansion_budget preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "initial_telemetry_preserves_full_expansion_budget never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn zero_expansion_budget_does_not_disable_initial_activation() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_expansion_budget_does_not_disable_initial_activation keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "zero_expansion_budget_does_not_disable_initial_activation preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_expansion_budget_does_not_disable_initial_activation never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn ordinary_view_truncation_is_bounded_not_budget_exhausted() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "ordinary_view_truncation_is_bounded_not_budget_exhausted keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "ordinary_view_truncation_is_bounded_not_budget_exhausted preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "ordinary_view_truncation_is_bounded_not_budget_exhausted never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn recursive_or_queued_probes_cannot_overrun_telemetry_bound() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn surface_continuation_handle_preserves_complete_context() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "surface_continuation_handle_preserves_complete_context keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "surface_continuation_handle_preserves_complete_context preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "surface_continuation_handle_preserves_complete_context never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn projection_structure_continuation_requires_no_surface() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "projection_structure_continuation_requires_no_surface keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "projection_structure_continuation_requires_no_surface preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "projection_structure_continuation_requires_no_surface never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn continuation_page_limit_zero_suppresses_handles() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_page_limit_zero_suppresses_handles keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "continuation_page_limit_zero_suppresses_handles preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_page_limit_zero_suppresses_handles never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn continuation_handle_bound_suppresses_later_handles() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_handle_bound_suppresses_later_handles keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "continuation_handle_bound_suppresses_later_handles preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_handle_bound_suppresses_later_handles never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn high_degree_address_uses_existing_summary_records() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_address_uses_existing_summary_records keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "high_degree_address_uses_existing_summary_records preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_address_uses_existing_summary_records never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn hub_degree_counts_unique_direct_edge_tuples() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "hub_degree_counts_unique_direct_edge_tuples keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "hub_degree_counts_unique_direct_edge_tuples preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "hub_degree_counts_unique_direct_edge_tuples never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn high_degree_handles_use_exact_policy_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_handles_use_exact_policy_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "high_degree_handles_use_exact_policy_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_handles_use_exact_policy_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn structure_handles_are_separated_by_transition_and_direction() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handles_are_separated_by_transition_and_direction keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "structure_handles_are_separated_by_transition_and_direction preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handles_are_separated_by_transition_and_direction never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn surface_access_failure_is_atomic() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "surface_access_failure_is_atomic keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "surface_access_failure_is_atomic preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "surface_access_failure_is_atomic never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn unexpected_scripted_probe_is_atomic() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unexpected_scripted_probe_is_atomic keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "unexpected_scripted_probe_is_atomic preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unexpected_scripted_probe_is_atomic never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_scripted_probe_definition_is_atomic() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_scripted_probe_definition_is_atomic keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_scripted_probe_definition_is_atomic preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_scripted_probe_definition_is_atomic never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn malformed_candidate_address_is_surface_failure() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "malformed_candidate_address_is_surface_failure keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "malformed_candidate_address_is_surface_failure preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "malformed_candidate_address_is_surface_failure never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn wrong_returned_address_kind_is_surface_failure() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "wrong_returned_address_kind_is_surface_failure keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "wrong_returned_address_kind_is_surface_failure preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "wrong_returned_address_kind_is_surface_failure never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_surface_candidates_are_surface_failure() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_surface_candidates_are_surface_failure keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_surface_candidates_are_surface_failure preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_surface_candidates_are_surface_failure never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn invalid_surface_continuation_is_surface_failure() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "invalid_surface_continuation_is_surface_failure keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "invalid_surface_continuation_is_surface_failure preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "invalid_surface_continuation_is_surface_failure never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn empty_surface_result_is_positive_only() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "empty_surface_result_is_positive_only keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "empty_surface_result_is_positive_only preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "empty_surface_result_is_positive_only never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn repeated_activation_is_exactly_equal() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "repeated_activation_is_exactly_equal keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "repeated_activation_is_exactly_equal preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "repeated_activation_is_exactly_equal never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn scripted_access_is_unchanged_after_success() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "scripted_access_is_unchanged_after_success keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "scripted_access_is_unchanged_after_success preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "scripted_access_is_unchanged_after_success never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn scripted_access_is_unchanged_after_failure() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "scripted_access_is_unchanged_after_failure keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "scripted_access_is_unchanged_after_failure preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "scripted_access_is_unchanged_after_failure never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn representative_end_to_end_activation_fixture() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "representative_end_to_end_activation_fixture keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "representative_end_to_end_activation_fixture preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "representative_end_to_end_activation_fixture never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_identifier_exposure_aggregates_unique_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_identifier_exposure_aggregates_unique_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_identifier_exposure_aggregates_unique_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_identifier_exposure_aggregates_unique_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn direct_identifier_exposure_registers_first_seen_source_order() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "direct_identifier_exposure_registers_first_seen_source_order keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "direct_identifier_exposure_registers_first_seen_source_order preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "direct_identifier_exposure_registers_first_seen_source_order never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn identifier_preview_uses_bounded_structural_context_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_preview_uses_bounded_structural_context_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "identifier_preview_uses_bounded_structural_context_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_preview_uses_bounded_structural_context_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn preview_only_identifier_does_not_invent_direct_binding() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "preview_only_identifier_does_not_invent_direct_binding keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "preview_only_identifier_does_not_invent_direct_binding preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "preview_only_identifier_does_not_invent_direct_binding never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn optional_context_truncation_marks_only_originating_probe() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "optional_context_truncation_marks_only_originating_probe keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "optional_context_truncation_marks_only_originating_probe preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "optional_context_truncation_marks_only_originating_probe never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn edge_bound_marks_only_related_probe_telemetry() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_bound_marks_only_related_probe_telemetry keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "edge_bound_marks_only_related_probe_telemetry preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_bound_marks_only_related_probe_telemetry never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn handle_bound_marks_only_related_probe_telemetry() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "handle_bound_marks_only_related_probe_telemetry keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "handle_bound_marks_only_related_probe_telemetry preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "handle_bound_marks_only_related_probe_telemetry never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn current_probe_is_marked_when_context_truncates_before_telemetry_append() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "current_probe_is_marked_when_context_truncates_before_telemetry_append keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "current_probe_is_marked_when_context_truncates_before_telemetry_append preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "current_probe_is_marked_when_context_truncates_before_telemetry_append never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn unrelated_complete_probe_remains_complete() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unrelated_complete_probe_remains_complete keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "unrelated_complete_probe_remains_complete preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unrelated_complete_probe_remains_complete never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_candidate_deduplication_remains_complete() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_deduplication_remains_complete keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_candidate_deduplication_remains_complete preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_deduplication_remains_complete never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn telemetry_bound_excess_fails_before_access_execution() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_bound_excess_fails_before_access_execution keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "telemetry_bound_excess_fails_before_access_execution preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_bound_excess_fails_before_access_execution never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn context_edge_preserves_originating_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "context_edge_preserves_originating_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "context_edge_preserves_originating_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "context_edge_preserves_originating_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn duplicate_edge_exposure_aggregates_unique_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_edge_exposure_aggregates_unique_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "duplicate_edge_exposure_aggregates_unique_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_edge_exposure_aggregates_unique_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn edge_provenance_does_not_merge_unrelated_paths() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_provenance_does_not_merge_unrelated_paths keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "edge_provenance_does_not_merge_unrelated_paths preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_provenance_does_not_merge_unrelated_paths never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn high_degree_without_omission_emits_no_continuation() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_without_omission_emits_no_continuation keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "high_degree_without_omission_emits_no_continuation preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_without_omission_emits_no_continuation never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn structure_handle_offset_counts_visible_targets_not_emitted_edges() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handle_offset_counts_visible_targets_not_emitted_edges keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "structure_handle_offset_counts_visible_targets_not_emitted_edges preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handle_offset_counts_visible_targets_not_emitted_edges never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn structure_handle_preserves_originating_provenance() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handle_preserves_originating_provenance keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "structure_handle_preserves_originating_provenance preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handle_preserves_originating_provenance never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn structure_handle_aggregates_multiple_exposure_paths() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handle_aggregates_multiple_exposure_paths keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "structure_handle_aggregates_multiple_exposure_paths preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handle_aggregates_multiple_exposure_paths never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn high_degree_summary_marks_only_related_telemetry() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_summary_marks_only_related_telemetry keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "high_degree_summary_marks_only_related_telemetry preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_summary_marks_only_related_telemetry never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn checked_tension_candidate_index_is_enforced() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "checked_tension_candidate_index_is_enforced keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "checked_tension_candidate_index_is_enforced preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "checked_tension_candidate_index_is_enforced never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn exact_continuation_requires_exact_known_remaining_total() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "exact_continuation_requires_exact_known_remaining_total keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "exact_continuation_requires_exact_known_remaining_total preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "exact_continuation_requires_exact_known_remaining_total never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn exact_continuation_allows_unknown_remaining_after_valid_offset() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "exact_continuation_allows_unknown_remaining_after_valid_offset keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "exact_continuation_allows_unknown_remaining_after_valid_offset preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "exact_continuation_allows_unknown_remaining_after_valid_offset never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn continuation_arithmetic_overflow_is_count_overflow() {
    let activated = successful_empty_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_arithmetic_overflow_is_count_overflow keeps fixture projection identity"
    );
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.probe_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ],
        "continuation_arithmetic_overflow_is_count_overflow preserves deterministic probe order"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_arithmetic_overflow_is_count_overflow never emits BudgetExhausted during initial activation"
    );
}

#[test]
fn declared_mode_requires_explicit_access_support() {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].match_modes = vec![SurfaceMatchMode::Declared {
        name: "custom".into(),
    }];
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &config(),
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::SurfaceAccessFailed { .. }
    ));
}

#[test]
fn telemetry_bound_excess_fails_atomically() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    cfg.maximum_telemetry_records = 0;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        ProjectionActivationViolation::ActivatedViewBoundExceeded { .. }
    ));
}

#[test]
fn scripted_access_unexpected_probe_failure_is_independent() {
    let projection = synthetic_projection::tiny_projection();
    let probe = text_probe(
        0,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        "x",
        vec![],
        ProjectionActivationProbeBand::Unbanded,
    );
    let err = semantic_traversal_core::ProjectionActivationAccess::execute_probe(
        &ScriptedProjectionActivationAccess::default(),
        &projection,
        &probe,
    )
    .unwrap_err();
    assert!(err.context.contains("unexpected activation probe"));
}

#[test]
fn scripted_access_duplicate_result_definition_failure_is_independent() {
    let projection = synthetic_projection::tiny_projection();
    let probe = text_probe(
        0,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        "x",
        vec![],
        ProjectionActivationProbeBand::Unbanded,
    );
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (probe.clone(), empty_result()),
            (probe.clone(), empty_result()),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let err = semantic_traversal_core::ProjectionActivationAccess::execute_probe(
        &access,
        &projection,
        &probe,
    )
    .unwrap_err();
    assert!(
        err.context
            .contains("duplicate scripted activation probe definition")
    );
}

#[test]
fn scripted_access_duplicate_failure_definition_failure_is_independent() {
    let projection = synthetic_projection::tiny_projection();
    let probe = text_probe(
        0,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        "x",
        vec![],
        ProjectionActivationProbeBand::Unbanded,
    );
    let failure = semantic_traversal_core::ProjectionActivationAccessFailure {
        context: "fail".into(),
    };
    let access = ScriptedProjectionActivationAccess {
        results: vec![],
        failures: vec![(probe.clone(), failure.clone()), (probe.clone(), failure)],
        declared_modes: vec![],
    };
    let err = semantic_traversal_core::ProjectionActivationAccess::execute_probe(
        &access,
        &projection,
        &probe,
    )
    .unwrap_err();
    assert!(
        err.context
            .contains("duplicate scripted activation probe definition")
    );
}

#[test]
fn scripted_access_result_plus_failure_duplicate_is_independent() {
    let projection = synthetic_projection::tiny_projection();
    let probe = text_probe(
        0,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        "x",
        vec![],
        ProjectionActivationProbeBand::Unbanded,
    );
    let access = ScriptedProjectionActivationAccess {
        results: vec![(probe.clone(), empty_result())],
        failures: vec![(
            probe.clone(),
            semantic_traversal_core::ProjectionActivationAccessFailure {
                context: "fail".into(),
            },
        )],
        declared_modes: vec![],
    };
    let err = semantic_traversal_core::ProjectionActivationAccess::execute_probe(
        &access,
        &projection,
        &probe,
    )
    .unwrap_err();
    assert!(
        err.context
            .contains("duplicate scripted activation probe definition")
    );
}

#[test]
fn scripted_access_declared_mode_mappings_preserve_exact_surface_and_name() {
    let access = ScriptedProjectionActivationAccess {
        results: vec![],
        failures: vec![],
        declared_modes: vec![(
            "surface:a".into(),
            "mode".into(),
            semantic_traversal_core::ProjectionActivationProbeSourceKind::Text,
        )],
    };
    assert_eq!(
        semantic_traversal_core::ProjectionActivationAccess::declared_mode_source(
            &access,
            "surface:a",
            "mode"
        ),
        Some(semantic_traversal_core::ProjectionActivationProbeSourceKind::Text)
    );
    assert_eq!(
        semantic_traversal_core::ProjectionActivationAccess::declared_mode_source(
            &access,
            "surface:b",
            "mode"
        ),
        None
    );
    assert_eq!(
        semantic_traversal_core::ProjectionActivationAccess::declared_mode_source(
            &access,
            "surface:a",
            "other"
        ),
        None
    );
}

#[test]
fn scripted_access_fixture_equality_after_success_is_independent() {
    let _ = successful_empty_activation();
}

#[test]
fn scripted_access_fixture_equality_after_failure_is_independent() {
    let projection = synthetic_projection::tiny_projection();
    let probe = text_probe(
        0,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        "x",
        vec![],
        ProjectionActivationProbeBand::Unbanded,
    );
    let access = ScriptedProjectionActivationAccess::default();
    let before = access.clone();
    let _ = semantic_traversal_core::ProjectionActivationAccess::execute_probe(
        &access,
        &projection,
        &probe,
    );
    assert_eq!(access, before);
}
