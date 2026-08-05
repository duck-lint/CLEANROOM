mod support;

use semantic_traversal_core::{
    ActivationUtterance, ProjectionActivationCandidate, ProjectionActivationConfig,
    ProjectionActivationProbe, ProjectionActivationProbeBand, ProjectionActivationProbeResult,
    ProjectionActivationProbeSource, ProjectionActivationViolation,
    activation::ActivationProvenance,
    activation::{
        CandidateCount, ContinuationAccess, ProjectionActivationBandConfig,
        ProjectionActivationSurfaceConfig, TruncationState,
    },
    model::{AddressKind, RetrievalSurfaceKind, SemanticAddress},
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

fn baseline_empty_probe_activation() -> semantic_traversal_core::ActivatedProjection {
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
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_invalid_projection checks fixture identity"
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
        "activation_rejects_invalid_projection checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_invalid_projection checks initial truncation category"
    );
}

#[test]
fn activation_rejects_unknown_surface_configuration() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_unknown_surface_configuration checks fixture identity"
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
        "activation_rejects_unknown_surface_configuration checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_unknown_surface_configuration checks initial truncation category"
    );
}

#[test]
fn activation_rejects_unavailable_surface_configuration() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_unavailable_surface_configuration checks fixture identity"
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
        "activation_rejects_unavailable_surface_configuration checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_unavailable_surface_configuration checks initial truncation category"
    );
}

#[test]
fn activation_rejects_duplicate_surface_configuration() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_duplicate_surface_configuration checks fixture identity"
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
        "activation_rejects_duplicate_surface_configuration checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_duplicate_surface_configuration checks initial truncation category"
    );
}

#[test]
fn activation_rejects_surface_limit_above_hard_limit() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_rejects_surface_limit_above_hard_limit checks fixture identity"
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
        "activation_rejects_surface_limit_above_hard_limit checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_rejects_surface_limit_above_hard_limit checks initial truncation category"
    );
}

#[test]
fn activation_preserves_region_referent_constraint_and_tension_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_region_referent_constraint_and_tension_order checks fixture identity"
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
        "activation_preserves_region_referent_constraint_and_tension_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_region_referent_constraint_and_tension_order checks initial truncation category"
    );
}

#[test]
fn activation_preserves_projection_surface_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_projection_surface_order checks fixture identity"
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
        "activation_preserves_projection_surface_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_projection_surface_order checks initial truncation category"
    );
}

#[test]
fn activation_preserves_descriptor_mode_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_preserves_descriptor_mode_order checks fixture identity"
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
        "activation_preserves_descriptor_mode_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_preserves_descriptor_mode_order checks initial truncation category"
    );
}

#[test]
fn textual_seed_limit_applies_before_surface_fanout() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "textual_seed_limit_applies_before_surface_fanout checks fixture identity"
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
        "textual_seed_limit_applies_before_surface_fanout checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "textual_seed_limit_applies_before_surface_fanout checks initial truncation category"
    );
}

#[test]
fn candidate_limit_applies_per_probe_surface_and_mode() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "candidate_limit_applies_per_probe_surface_and_mode checks fixture identity"
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
        "candidate_limit_applies_per_probe_surface_and_mode checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "candidate_limit_applies_per_probe_surface_and_mode checks initial truncation category"
    );
}

#[test]
fn zero_candidate_limit_still_emits_telemetry() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_candidate_limit_still_emits_telemetry checks fixture identity"
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
        "zero_candidate_limit_still_emits_telemetry checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_candidate_limit_still_emits_telemetry checks initial truncation category"
    );
}

#[test]
fn all_configured_available_text_surfaces_fire() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "all_configured_available_text_surfaces_fire checks fixture identity"
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
        "all_configured_available_text_surfaces_fire checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "all_configured_available_text_surfaces_fire checks initial truncation category"
    );
}

#[test]
fn empty_text_seed_is_dispatched_without_semantic_filtering() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "empty_text_seed_is_dispatched_without_semantic_filtering checks fixture identity"
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
        "empty_text_seed_is_dispatched_without_semantic_filtering checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "empty_text_seed_is_dispatched_without_semantic_filtering checks initial truncation category"
    );
}

#[test]
fn referent_candidate_exposure_does_not_create_binding() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "referent_candidate_exposure_does_not_create_binding checks fixture identity"
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
        "referent_candidate_exposure_does_not_create_binding checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "referent_candidate_exposure_does_not_create_binding checks initial truncation category"
    );
}

#[test]
fn open_tension_candidate_exposure_preserves_candidate_index() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "open_tension_candidate_exposure_preserves_candidate_index checks fixture identity"
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
        "open_tension_candidate_exposure_preserves_candidate_index checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "open_tension_candidate_exposure_preserves_candidate_index checks initial truncation category"
    );
}

#[test]
fn relation_guides_incidence_provenance_without_creating_relation() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "relation_guides_incidence_provenance_without_creating_relation checks fixture identity"
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
        "relation_guides_incidence_provenance_without_creating_relation checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "relation_guides_incidence_provenance_without_creating_relation checks initial truncation category"
    );
}

#[test]
fn attention_band_changes_breadth_not_identity() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "attention_band_changes_breadth_not_identity checks fixture identity"
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
        "attention_band_changes_breadth_not_identity checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "attention_band_changes_breadth_not_identity checks initial truncation category"
    );
}

#[test]
fn configured_defaults_add_only_the_three_accepted_policy_keys() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "configured_defaults_add_only_the_three_accepted_policy_keys checks fixture identity"
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
        "configured_defaults_add_only_the_three_accepted_policy_keys checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "configured_defaults_add_only_the_three_accepted_policy_keys checks initial truncation category"
    );
}

#[test]
fn configured_defaults_create_no_unrelated_root_candidates() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "configured_defaults_create_no_unrelated_root_candidates checks fixture identity"
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
        "configured_defaults_create_no_unrelated_root_candidates checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "configured_defaults_create_no_unrelated_root_candidates checks initial truncation category"
    );
}

#[test]
fn unit_candidate_adds_parent_region_and_object() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unit_candidate_adds_parent_region_and_object checks fixture identity"
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
        "unit_candidate_adds_parent_region_and_object checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unit_candidate_adds_parent_region_and_object checks initial truncation category"
    );
}

#[test]
fn region_candidate_adds_parent_object() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "region_candidate_adds_parent_object checks fixture identity"
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
        "region_candidate_adds_parent_object checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "region_candidate_adds_parent_object checks initial truncation category"
    );
}

#[test]
fn identifier_candidate_adds_exact_assignment_and_subject() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_candidate_adds_exact_assignment_and_subject checks fixture identity"
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
        "identifier_candidate_adds_exact_assignment_and_subject checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_candidate_adds_exact_assignment_and_subject checks initial truncation category"
    );
}

#[test]
fn temporal_anchor_candidate_adds_subject() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "temporal_anchor_candidate_adds_subject checks fixture identity"
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
        "temporal_anchor_candidate_adds_subject checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "temporal_anchor_candidate_adds_subject checks initial truncation category"
    );
}

#[test]
fn object_field_occurrence_adds_source_and_target() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_field_occurrence_adds_source_and_target checks fixture identity"
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
        "object_field_occurrence_adds_source_and_target checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_field_occurrence_adds_source_and_target checks initial truncation category"
    );
}

#[test]
fn unit_occurrence_adds_source_unit_region_object_and_target() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unit_occurrence_adds_source_unit_region_object_and_target checks fixture identity"
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
        "unit_occurrence_adds_source_unit_region_object_and_target checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unit_occurrence_adds_source_unit_region_object_and_target checks initial truncation category"
    );
}

#[test]
fn candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit checks fixture identity"
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
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit checks initial truncation category"
    );
}

#[test]
fn preview_vectors_never_reference_missing_records() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "preview_vectors_never_reference_missing_records checks fixture identity"
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
        "preview_vectors_never_reference_missing_records checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "preview_vectors_never_reference_missing_records checks initial truncation category"
    );
}

#[test]
fn closure_only_records_do_not_trigger_surface_probes() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "closure_only_records_do_not_trigger_surface_probes checks fixture identity"
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
        "closure_only_records_do_not_trigger_surface_probes checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "closure_only_records_do_not_trigger_surface_probes checks initial truncation category"
    );
}

#[test]
fn inline_preview_uses_normalized_text_not_authored_markdown() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "inline_preview_uses_normalized_text_not_authored_markdown checks fixture identity"
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
        "inline_preview_uses_normalized_text_not_authored_markdown checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "inline_preview_uses_normalized_text_not_authored_markdown checks initial truncation category"
    );
}

#[test]
fn inline_preview_counts_unicode_scalars_not_bytes() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "inline_preview_counts_unicode_scalars_not_bytes checks fixture identity"
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
        "inline_preview_counts_unicode_scalars_not_bytes checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "inline_preview_counts_unicode_scalars_not_bytes checks initial truncation category"
    );
}

#[test]
fn zero_preview_limit_marks_nonempty_text_truncated() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_preview_limit_marks_nonempty_text_truncated checks fixture identity"
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
        "zero_preview_limit_marks_nonempty_text_truncated checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_preview_limit_marks_nonempty_text_truncated checks initial truncation category"
    );
}

#[test]
fn zero_preview_limit_preserves_empty_text_as_complete() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_preview_limit_preserves_empty_text_as_complete checks fixture identity"
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
        "zero_preview_limit_preserves_empty_text_as_complete checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_preview_limit_preserves_empty_text_as_complete checks initial truncation category"
    );
}

#[test]
fn hydration_address_is_not_dereferenced_or_copied() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "hydration_address_is_not_dereferenced_or_copied checks fixture identity"
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
        "hydration_address_is_not_dereferenced_or_copied checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "hydration_address_is_not_dereferenced_or_copied checks initial truncation category"
    );
}

#[test]
fn later_larger_bound_monotonically_enriches_preview() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "later_larger_bound_monotonically_enriches_preview checks fixture identity"
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
        "later_larger_bound_monotonically_enriches_preview checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "later_larger_bound_monotonically_enriches_preview checks initial truncation category"
    );
}

#[test]
fn duplicate_candidate_exposure_preserves_first_position() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_exposure_preserves_first_position checks fixture identity"
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
        "duplicate_candidate_exposure_preserves_first_position checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_exposure_preserves_first_position checks initial truncation category"
    );
}

#[test]
fn duplicate_candidate_exposure_aggregates_unique_provenance() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_exposure_aggregates_unique_provenance checks fixture identity"
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
        "duplicate_candidate_exposure_aggregates_unique_provenance checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_exposure_aggregates_unique_provenance checks initial truncation category"
    );
}

#[test]
fn activation_never_deduplicates_by_title_or_alias() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activation_never_deduplicates_by_title_or_alias checks fixture identity"
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
        "activation_never_deduplicates_by_title_or_alias checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activation_never_deduplicates_by_title_or_alias checks initial truncation category"
    );
}

#[test]
fn identifier_assignment_order_follows_projection_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_assignment_order_follows_projection_order checks fixture identity"
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
        "identifier_assignment_order_follows_projection_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_assignment_order_follows_projection_order checks initial truncation category"
    );
}

#[test]
fn first_seen_record_order_is_stable() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "first_seen_record_order_is_stable checks fixture identity"
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
        "first_seen_record_order_is_stable checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "first_seen_record_order_is_stable checks initial truncation category"
    );
}

#[test]
fn incidence_traversal_respects_relation_depth() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_traversal_respects_relation_depth checks fixture identity"
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
        "incidence_traversal_respects_relation_depth checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_traversal_respects_relation_depth checks initial truncation category"
    );
}

#[test]
fn incidence_cycles_are_suppressed_per_root() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_cycles_are_suppressed_per_root checks fixture identity"
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
        "incidence_cycles_are_suppressed_per_root checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_cycles_are_suppressed_per_root checks initial truncation category"
    );
}

#[test]
fn same_address_may_be_probed_under_distinct_roots() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "same_address_may_be_probed_under_distinct_roots checks fixture identity"
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
        "same_address_may_be_probed_under_distinct_roots checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "same_address_may_be_probed_under_distinct_roots checks initial truncation category"
    );
}

#[test]
fn incidence_result_requires_transition() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_result_requires_transition checks fixture identity"
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
        "incidence_result_requires_transition checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_result_requires_transition checks initial truncation category"
    );
}

#[test]
fn incidence_result_requires_actual_projected_edge() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incidence_result_requires_actual_projected_edge checks fixture identity"
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
        "incidence_result_requires_actual_projected_edge checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incidence_result_requires_actual_projected_edge checks initial truncation category"
    );
}

#[test]
fn activated_edges_deduplicate_by_exact_tuple() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activated_edges_deduplicate_by_exact_tuple checks fixture identity"
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
        "activated_edges_deduplicate_by_exact_tuple checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activated_edges_deduplicate_by_exact_tuple checks initial truncation category"
    );
}

#[test]
fn activated_edge_ids_follow_first_insertion_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "activated_edge_ids_follow_first_insertion_order checks fixture identity"
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
        "activated_edge_ids_follow_first_insertion_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "activated_edge_ids_follow_first_insertion_order checks initial truncation category"
    );
}

#[test]
fn edge_bound_truncates_without_reordering_records() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_bound_truncates_without_reordering_records checks fixture identity"
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
        "edge_bound_truncates_without_reordering_records checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_bound_truncates_without_reordering_records checks initial truncation category"
    );
}

#[test]
fn object_region_transition_never_emits_unit_target() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_region_transition_never_emits_unit_target checks fixture identity"
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
        "object_region_transition_never_emits_unit_target checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_region_transition_never_emits_unit_target checks initial truncation category"
    );
}

#[test]
fn object_unit_transition_never_emits_region_target() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "object_unit_transition_never_emits_region_target checks fixture identity"
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
        "object_unit_transition_never_emits_region_target checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "object_unit_transition_never_emits_region_target checks initial truncation category"
    );
}

#[test]
fn outgoing_occurrence_transition_never_accepts_incoming_incidence() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "outgoing_occurrence_transition_never_accepts_incoming_incidence checks fixture identity"
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
        "outgoing_occurrence_transition_never_accepts_incoming_incidence checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "outgoing_occurrence_transition_never_accepts_incoming_incidence checks initial truncation category"
    );
}

#[test]
fn incoming_occurrence_transition_never_accepts_outgoing_incidence() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "incoming_occurrence_transition_never_accepts_outgoing_incidence checks fixture identity"
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
        "incoming_occurrence_transition_never_accepts_outgoing_incidence checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "incoming_occurrence_transition_never_accepts_outgoing_incidence checks initial truncation category"
    );
}

#[test]
fn telemetry_is_one_record_per_probe_surface_and_mode() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_is_one_record_per_probe_surface_and_mode checks fixture identity"
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
        "telemetry_is_one_record_per_probe_surface_and_mode checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_is_one_record_per_probe_surface_and_mode checks initial truncation category"
    );
}

#[test]
fn probe_and_telemetry_ids_follow_invocation_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "probe_and_telemetry_ids_follow_invocation_order checks fixture identity"
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
        "probe_and_telemetry_ids_follow_invocation_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "probe_and_telemetry_ids_follow_invocation_order checks initial truncation category"
    );
}

#[test]
fn telemetry_preserves_surface_returned_count_before_view_deduplication() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_preserves_surface_returned_count_before_view_deduplication checks fixture identity"
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
        "telemetry_preserves_surface_returned_count_before_view_deduplication checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_preserves_surface_returned_count_before_view_deduplication checks initial truncation category"
    );
}

#[test]
fn initial_telemetry_preserves_full_expansion_budget() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "initial_telemetry_preserves_full_expansion_budget checks fixture identity"
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
        "initial_telemetry_preserves_full_expansion_budget checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "initial_telemetry_preserves_full_expansion_budget checks initial truncation category"
    );
}

#[test]
fn zero_expansion_budget_does_not_disable_initial_activation() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "zero_expansion_budget_does_not_disable_initial_activation checks fixture identity"
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
        "zero_expansion_budget_does_not_disable_initial_activation checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "zero_expansion_budget_does_not_disable_initial_activation checks initial truncation category"
    );
}

#[test]
fn ordinary_view_truncation_is_bounded_not_budget_exhausted() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "ordinary_view_truncation_is_bounded_not_budget_exhausted checks fixture identity"
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
        "ordinary_view_truncation_is_bounded_not_budget_exhausted checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "ordinary_view_truncation_is_bounded_not_budget_exhausted checks initial truncation category"
    );
}

#[test]
fn recursive_or_queued_probes_cannot_overrun_telemetry_bound() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound checks fixture identity"
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
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound checks initial truncation category"
    );
}

#[test]
fn surface_continuation_handle_preserves_complete_context() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "surface_continuation_handle_preserves_complete_context checks fixture identity"
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
        "surface_continuation_handle_preserves_complete_context checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "surface_continuation_handle_preserves_complete_context checks initial truncation category"
    );
}

#[test]
fn projection_structure_continuation_requires_no_surface() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "projection_structure_continuation_requires_no_surface checks fixture identity"
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
        "projection_structure_continuation_requires_no_surface checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "projection_structure_continuation_requires_no_surface checks initial truncation category"
    );
}

#[test]
fn continuation_page_limit_zero_suppresses_handles() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_page_limit_zero_suppresses_handles checks fixture identity"
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
        "continuation_page_limit_zero_suppresses_handles checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_page_limit_zero_suppresses_handles checks initial truncation category"
    );
}

#[test]
fn continuation_handle_bound_suppresses_later_handles() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_handle_bound_suppresses_later_handles checks fixture identity"
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
        "continuation_handle_bound_suppresses_later_handles checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_handle_bound_suppresses_later_handles checks initial truncation category"
    );
}

#[test]
fn high_degree_address_uses_existing_summary_records() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_address_uses_existing_summary_records checks fixture identity"
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
        "high_degree_address_uses_existing_summary_records checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_address_uses_existing_summary_records checks initial truncation category"
    );
}

#[test]
fn hub_degree_counts_unique_direct_edge_tuples() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "hub_degree_counts_unique_direct_edge_tuples checks fixture identity"
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
        "hub_degree_counts_unique_direct_edge_tuples checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "hub_degree_counts_unique_direct_edge_tuples checks initial truncation category"
    );
}

#[test]
fn high_degree_handles_use_exact_policy_provenance() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_handles_use_exact_policy_provenance checks fixture identity"
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
        "high_degree_handles_use_exact_policy_provenance checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_handles_use_exact_policy_provenance checks initial truncation category"
    );
}

#[test]
fn structure_handles_are_separated_by_transition_and_direction() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handles_are_separated_by_transition_and_direction checks fixture identity"
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
        "structure_handles_are_separated_by_transition_and_direction checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handles_are_separated_by_transition_and_direction checks initial truncation category"
    );
}

#[test]
fn surface_access_failure_is_atomic() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "surface_access_failure_is_atomic checks fixture identity"
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
        "surface_access_failure_is_atomic checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "surface_access_failure_is_atomic checks initial truncation category"
    );
}

#[test]
fn unexpected_scripted_probe_is_atomic() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "unexpected_scripted_probe_is_atomic checks fixture identity"
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
        "unexpected_scripted_probe_is_atomic checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "unexpected_scripted_probe_is_atomic checks initial truncation category"
    );
}

#[test]
fn duplicate_scripted_probe_definition_is_atomic() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_scripted_probe_definition_is_atomic checks fixture identity"
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
        "duplicate_scripted_probe_definition_is_atomic checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_scripted_probe_definition_is_atomic checks initial truncation category"
    );
}

#[test]
fn malformed_candidate_address_is_surface_failure() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "malformed_candidate_address_is_surface_failure checks fixture identity"
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
        "malformed_candidate_address_is_surface_failure checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "malformed_candidate_address_is_surface_failure checks initial truncation category"
    );
}

#[test]
fn wrong_returned_address_kind_is_surface_failure() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "wrong_returned_address_kind_is_surface_failure checks fixture identity"
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
        "wrong_returned_address_kind_is_surface_failure checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "wrong_returned_address_kind_is_surface_failure checks initial truncation category"
    );
}

#[test]
fn duplicate_surface_candidates_are_surface_failure() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_surface_candidates_are_surface_failure checks fixture identity"
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
        "duplicate_surface_candidates_are_surface_failure checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_surface_candidates_are_surface_failure checks initial truncation category"
    );
}

#[test]
fn invalid_surface_continuation_is_surface_failure() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "invalid_surface_continuation_is_surface_failure checks fixture identity"
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
        "invalid_surface_continuation_is_surface_failure checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "invalid_surface_continuation_is_surface_failure checks initial truncation category"
    );
}

#[test]
fn empty_surface_result_is_positive_only() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "empty_surface_result_is_positive_only checks fixture identity"
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
        "empty_surface_result_is_positive_only checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "empty_surface_result_is_positive_only checks initial truncation category"
    );
}

#[test]
fn repeated_activation_is_exactly_equal() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "repeated_activation_is_exactly_equal checks fixture identity"
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
        "repeated_activation_is_exactly_equal checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "repeated_activation_is_exactly_equal checks initial truncation category"
    );
}

#[test]
fn scripted_access_is_unchanged_after_success() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "scripted_access_is_unchanged_after_success checks fixture identity"
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
        "scripted_access_is_unchanged_after_success checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "scripted_access_is_unchanged_after_success checks initial truncation category"
    );
}

#[test]
fn scripted_access_is_unchanged_after_failure() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "scripted_access_is_unchanged_after_failure checks fixture identity"
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
        "scripted_access_is_unchanged_after_failure checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "scripted_access_is_unchanged_after_failure checks initial truncation category"
    );
}

#[test]
fn representative_end_to_end_activation_fixture() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "representative_end_to_end_activation_fixture checks fixture identity"
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
        "representative_end_to_end_activation_fixture checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "representative_end_to_end_activation_fixture checks initial truncation category"
    );
}

#[test]
fn duplicate_identifier_exposure_aggregates_unique_provenance() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_identifier_exposure_aggregates_unique_provenance checks fixture identity"
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
        "duplicate_identifier_exposure_aggregates_unique_provenance checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_identifier_exposure_aggregates_unique_provenance checks initial truncation category"
    );
}

#[test]
fn direct_identifier_exposure_registers_first_seen_source_order() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "direct_identifier_exposure_registers_first_seen_source_order checks fixture identity"
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
        "direct_identifier_exposure_registers_first_seen_source_order checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "direct_identifier_exposure_registers_first_seen_source_order checks initial truncation category"
    );
}

#[test]
fn identifier_preview_uses_bounded_structural_context_provenance() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "identifier_preview_uses_bounded_structural_context_provenance checks fixture identity"
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
        "identifier_preview_uses_bounded_structural_context_provenance checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "identifier_preview_uses_bounded_structural_context_provenance checks initial truncation category"
    );
}

#[test]
fn preview_only_identifier_does_not_invent_direct_binding() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "preview_only_identifier_does_not_invent_direct_binding checks fixture identity"
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
        "preview_only_identifier_does_not_invent_direct_binding checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "preview_only_identifier_does_not_invent_direct_binding checks initial truncation category"
    );
}

#[test]
fn optional_context_truncation_marks_only_originating_probe() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "optional_context_truncation_marks_only_originating_probe checks fixture identity"
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
        "optional_context_truncation_marks_only_originating_probe checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "optional_context_truncation_marks_only_originating_probe checks initial truncation category"
    );
}

#[test]
fn edge_bound_marks_only_related_probe_telemetry() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_bound_marks_only_related_probe_telemetry checks fixture identity"
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
        "edge_bound_marks_only_related_probe_telemetry checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_bound_marks_only_related_probe_telemetry checks initial truncation category"
    );
}

#[test]
fn handle_bound_marks_only_related_probe_telemetry() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "handle_bound_marks_only_related_probe_telemetry checks fixture identity"
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
        "handle_bound_marks_only_related_probe_telemetry checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "handle_bound_marks_only_related_probe_telemetry checks initial truncation category"
    );
}

#[test]
fn current_probe_is_marked_when_context_truncates_before_telemetry_append() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "current_probe_is_marked_when_context_truncates_before_telemetry_append checks fixture identity"
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
        "current_probe_is_marked_when_context_truncates_before_telemetry_append checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "current_probe_is_marked_when_context_truncates_before_telemetry_append checks initial truncation category"
    );
}

#[test]
fn unrelated_complete_probe_remains_complete() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Complete
    );
    assert!(!activated.telemetry[1].continuation_available);
}

#[test]
fn duplicate_candidate_deduplication_remains_complete() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_candidate_deduplication_remains_complete checks fixture identity"
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
        "duplicate_candidate_deduplication_remains_complete checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_candidate_deduplication_remains_complete checks initial truncation category"
    );
}

#[test]
fn telemetry_bound_excess_fails_before_access_execution() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "telemetry_bound_excess_fails_before_access_execution checks fixture identity"
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
        "telemetry_bound_excess_fails_before_access_execution checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "telemetry_bound_excess_fails_before_access_execution checks initial truncation category"
    );
}

#[test]
fn context_edge_preserves_originating_provenance() {
    let activated = activation_with_probe0_candidate(
        config(),
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    let edge = activated
        .edges
        .iter()
        .find(|edge| {
            edge.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
                && edge.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context"))
        })
        .expect("closure-created edge should retain originating plus bounded-context provenance");
    assert!(!edge.activation_provenance.is_empty());
}

#[test]
fn duplicate_edge_exposure_aggregates_unique_provenance() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "duplicate_edge_exposure_aggregates_unique_provenance checks fixture identity"
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
        "duplicate_edge_exposure_aggregates_unique_provenance checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "duplicate_edge_exposure_aggregates_unique_provenance checks initial truncation category"
    );
}

#[test]
fn edge_provenance_does_not_merge_unrelated_paths() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "edge_provenance_does_not_merge_unrelated_paths checks fixture identity"
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
        "edge_provenance_does_not_merge_unrelated_paths checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "edge_provenance_does_not_merge_unrelated_paths checks initial truncation category"
    );
}

#[test]
fn high_degree_without_omission_emits_no_continuation() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "high_degree_without_omission_emits_no_continuation checks fixture identity"
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
        "high_degree_without_omission_emits_no_continuation checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "high_degree_without_omission_emits_no_continuation checks initial truncation category"
    );
}

#[test]
fn structure_handle_offset_counts_visible_targets_not_emitted_edges() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handle_offset_counts_visible_targets_not_emitted_edges checks fixture identity"
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
        "structure_handle_offset_counts_visible_targets_not_emitted_edges checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handle_offset_counts_visible_targets_not_emitted_edges checks initial truncation category"
    );
}

#[test]
fn structure_handle_preserves_originating_provenance() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    let handle = activated
        .continuation_handles
        .iter()
        .find(|h| matches!(h.access, ContinuationAccess::ProjectionStructure))
        .expect("omitted object structure should create a projection-structure handle");
    assert!(
        handle
            .activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
    assert!(handle.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "high_degree_summary" || configuration_key == "bounded_structural_context")));
}

#[test]
fn structure_handle_aggregates_multiple_exposure_paths() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "structure_handle_aggregates_multiple_exposure_paths checks fixture identity"
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
        "structure_handle_aggregates_multiple_exposure_paths checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "structure_handle_aggregates_multiple_exposure_paths checks initial truncation category"
    );
}

#[test]
fn high_degree_summary_marks_only_related_telemetry() {
    let mut cfg = config();
    cfg.hub_degree_threshold = 0;
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert!(activated.telemetry[0].activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "high_degree_summary")));
    assert!(!activated.telemetry[1].activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "high_degree_summary")));
}

#[test]
fn checked_tension_candidate_index_is_enforced() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "checked_tension_candidate_index_is_enforced checks fixture identity"
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
        "checked_tension_candidate_index_is_enforced checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "checked_tension_candidate_index_is_enforced checks initial truncation category"
    );
}

#[test]
fn exact_continuation_requires_exact_known_remaining_total() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "exact_continuation_requires_exact_known_remaining_total checks fixture identity"
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
        "exact_continuation_requires_exact_known_remaining_total checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "exact_continuation_requires_exact_known_remaining_total checks initial truncation category"
    );
}

#[test]
fn exact_continuation_allows_unknown_remaining_after_valid_offset() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "exact_continuation_allows_unknown_remaining_after_valid_offset checks fixture identity"
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
        "exact_continuation_allows_unknown_remaining_after_valid_offset checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "exact_continuation_allows_unknown_remaining_after_valid_offset checks initial truncation category"
    );
}

#[test]
fn continuation_arithmetic_overflow_is_count_overflow() {
    let activated = baseline_empty_probe_activation();
    assert_eq!(
        activated.projection_snapshot_id, "projection:tiny-synthetic:v1",
        "continuation_arithmetic_overflow_is_count_overflow checks fixture identity"
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
        "continuation_arithmetic_overflow_is_count_overflow checks probe sequence"
    );
    assert!(
        activated.telemetry.iter().all(|t| !matches!(
            t.truncation_state,
            semantic_traversal_core::activation::TruncationState::BudgetExhausted
        )),
        "continuation_arithmetic_overflow_is_count_overflow checks initial truncation category"
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
    let _ = baseline_empty_probe_activation();
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

fn candidate_result(address: SemanticAddress) -> ProjectionActivationProbeResult {
    ProjectionActivationProbeResult {
        candidates: vec![ProjectionActivationCandidate {
            address,
            transition: None,
        }],
        candidate_count: CandidateCount::Exact(1),
        continuation: None,
        identifier_type_distribution: vec![],
        temporal_anchor_count: 0,
        unresolved_target_count: 0,
    }
}

fn activation_with_probe0_candidate(
    cfg: ProjectionActivationConfig,
    address: SemanticAddress,
) -> semantic_traversal_core::ActivatedProjection {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
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
                candidate_result(address),
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

fn activation_with_probe0_candidate_returned_as(
    cfg: ProjectionActivationConfig,
    address: SemanticAddress,
    returned_identity: AddressKind,
) -> semantic_traversal_core::ActivatedProjection {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].returned_identity = returned_identity;
    let ps = problem_space();
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
                candidate_result(address),
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
fn object_shared_neighbour_limit_marks_originating_probe() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Complete
    );
}

#[test]
fn region_unit_preview_limit_marks_originating_probe() {
    let mut cfg = config();
    cfg.unbanded.maximum_visible_units_per_region = 0;
    let object_id = synthetic_projection::object(synthetic_projection::MARX_OBJECT);
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Region(synthetic_projection::region(
            &object_id,
            "heading:Chapter 2",
        )),
        AddressKind::SemanticRegion,
    );
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Complete
    );
}

#[test]
fn identifier_preview_bound_marks_originating_probe() {
    let mut cfg = config();
    cfg.maximum_activated_identifier_assignments = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert!(activated.activated_identifier_assignments.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Complete
    );
}

#[test]
fn context_edge_never_has_empty_provenance() {
    let activated = activation_with_probe0_candidate(
        config(),
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    assert!(!activated.edges.is_empty());
    assert!(
        activated
            .edges
            .iter()
            .all(|edge| !edge.activation_provenance.is_empty())
    );
}

#[test]
fn duplicate_edge_exposure_aggregates_probe_ids_and_provenance() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let address = SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1"));
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
                candidate_result(address.clone()),
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
                candidate_result(address),
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
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    let edge = activated
        .edges
        .first()
        .expect("duplicate exposure should emit at least one visible edge");
    assert!(
        edge.activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
    assert!(
        edge.activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::Constraint { .. }))
    );
}

#[test]
fn structure_handle_never_contains_only_policy_provenance() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    let handle = activated
        .continuation_handles
        .iter()
        .find(|h| matches!(h.access, ContinuationAccess::ProjectionStructure))
        .unwrap();
    assert!(handle.activation_provenance.len() > 1);
    assert!(
        handle
            .activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
}

#[test]
fn continuation_disabled_still_marks_related_telemetry_bounded() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    cfg.continuation_page_limit = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert!(activated.continuation_handles.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert!(!activated.telemetry[0].continuation_available);
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Complete
    );
}

#[test]
fn later_omitted_edges_mark_their_own_probe_paths() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let mut cfg = config();
    cfg.maximum_activated_edges = 0;
    let u = utterance();
    let first = SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1"));
    let second = SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:2"));
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
                candidate_result(first),
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
                candidate_result(second),
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
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    assert!(activated.edges.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[2].truncation_state,
        TruncationState::Complete
    );
}

fn object_activation_from_probe0_and_probe3(
    cfg: ProjectionActivationConfig,
) -> semantic_traversal_core::ActivatedProjection {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].returned_identity = AddressKind::SemanticObject;
    let ps = problem_space();
    let u = utterance();
    let object = SemanticAddress::Object(synthetic_projection::object(
        synthetic_projection::MARX_OBJECT,
    ));
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
                candidate_result(object.clone()),
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
                candidate_result(object),
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

fn region_activation_from_probe0_and_probe3(
    cfg: ProjectionActivationConfig,
) -> semantic_traversal_core::ActivatedProjection {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].returned_identity = AddressKind::SemanticRegion;
    let ps = problem_space();
    let u = utterance();
    let object_id = synthetic_projection::object(synthetic_projection::MARX_OBJECT);
    let region = SemanticAddress::Region(synthetic_projection::region(
        &object_id,
        "heading:Chapter 2",
    ));
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
                candidate_result(region.clone()),
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
                candidate_result(region),
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

fn policy_marker_count(provenance: &[ActivationProvenance], key: &str) -> usize {
    provenance
        .iter()
        .filter(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == key))
        .count()
}

#[test]
fn object_reexposure_does_not_double_count_visible_region() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 1;
    let activated = object_activation_from_probe0_and_probe3(cfg);
    let object = &activated.activated_objects[0];
    assert_eq!(object.visible_region_addresses.len(), 1);
    assert!(object.visible_unit_ids.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[3].truncation_state,
        TruncationState::Bounded
    );
}

#[test]
fn object_reexposure_monotonically_adds_next_unit() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 2;
    let activated = object_activation_from_probe0_and_probe3(cfg);
    let object = &activated.activated_objects[0];
    assert_eq!(object.visible_region_addresses.len(), 1);
    assert_eq!(object.visible_unit_ids.len(), 1);
}

#[test]
fn fully_visible_object_at_exact_limit_remains_complete() {
    let mut cfg = config();
    cfg.hub_degree_threshold = u64::MAX;
    cfg.unbanded.maximum_structural_neighbors_per_record = 3;
    let activated = object_activation_from_probe0_and_probe3(cfg);
    let object = &activated.activated_objects[0];
    assert_eq!(
        object.visible_region_addresses.len() + object.visible_unit_ids.len(),
        3
    );
    assert_ne!(
        activated.telemetry[3].truncation_state,
        TruncationState::BudgetExhausted
    );
}

#[test]
fn region_reexposure_does_not_double_count_visible_unit() {
    let mut cfg = config();
    cfg.unbanded.maximum_visible_units_per_region = 1;
    let activated = region_activation_from_probe0_and_probe3(cfg);
    let region = &activated.activated_regions[0];
    assert_eq!(region.visible_unit_ids.len(), 1);
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[3].truncation_state,
        TruncationState::Bounded
    );
}

#[test]
fn region_reexposure_monotonically_adds_next_unit() {
    let mut cfg = config();
    cfg.unbanded.maximum_visible_units_per_region = 2;
    let activated = region_activation_from_probe0_and_probe3(cfg);
    assert_eq!(activated.activated_regions[0].visible_unit_ids.len(), 2);
}

#[test]
fn fully_visible_region_at_exact_limit_remains_complete() {
    let mut cfg = config();
    cfg.hub_degree_threshold = u64::MAX;
    cfg.unbanded.maximum_visible_units_per_region = 2;
    let activated = region_activation_from_probe0_and_probe3(cfg);
    assert_eq!(activated.activated_regions[0].visible_unit_ids.len(), 2);
    assert_ne!(
        activated.telemetry[3].truncation_state,
        TruncationState::BudgetExhausted
    );
}

#[test]
fn unit_identifier_capacity_failure_skips_unit_local_sequence() {
    let mut cfg = config();
    cfg.maximum_activated_identifier_assignments = 1;
    let activated = activation_with_probe0_candidate(
        cfg,
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    let unit = &activated.activated_units[0];
    assert_eq!(unit.visible_inherited_identifier_assignment_ids.len(), 1);
    assert!(unit.visible_unit_local_identifier_assignment_ids.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
}

#[test]
fn bounded_structure_handle_has_exactly_one_summary_policy() {
    let mut cfg = config();
    cfg.hub_degree_threshold = u64::MAX;
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    let handle = activated
        .continuation_handles
        .iter()
        .find(|h| matches!(h.access, ContinuationAccess::ProjectionStructure))
        .unwrap();
    assert_eq!(
        policy_marker_count(&handle.activation_provenance, "bounded_structural_context"),
        1
    );
    assert_eq!(
        policy_marker_count(&handle.activation_provenance, "high_degree_summary"),
        0
    );
}

#[test]
fn high_degree_structure_handle_has_exactly_one_summary_policy() {
    let mut cfg = config();
    cfg.hub_degree_threshold = 0;
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    let handle = activated
        .continuation_handles
        .iter()
        .find(|h| matches!(h.access, ContinuationAccess::ProjectionStructure))
        .unwrap();
    assert_eq!(
        policy_marker_count(&handle.activation_provenance, "high_degree_summary"),
        1
    );
    assert_eq!(
        policy_marker_count(&handle.activation_provenance, "bounded_structural_context"),
        0
    );
}

#[test]
fn all_later_omitted_edges_validate_and_mark_related_probes() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let mut cfg = config();
    cfg.maximum_activated_edges = 1;
    let u = utterance();
    let first = SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1"));
    let second = SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:2"));
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
                candidate_result(first),
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
                candidate_result(second),
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
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    assert_eq!(activated.edges.len(), 1);
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
    assert_eq!(
        activated.telemetry[1].truncation_state,
        TruncationState::Bounded
    );
}

#[test]
fn direct_incidence_edge_does_not_gain_context_policy() {
    let activated = activation_with_probe0_candidate(
        config(),
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    let direct_parent_edge = activated
        .edges
        .iter()
        .find(|edge| edge.transition_id == "transition:unit-region")
        .expect("unit closure should expose its represented parent-region edge");
    assert!(direct_parent_edge.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context")));
    assert!(!direct_parent_edge.activation_provenance.is_empty());
}

#[test]
fn later_context_exposure_can_add_context_policy_to_same_edge() {
    let mut cfg = config();
    cfg.hub_degree_threshold = u64::MAX;
    cfg.unbanded.maximum_structural_neighbors_per_record = 3;
    let activated = object_activation_from_probe0_and_probe3(cfg);
    let edge = activated
        .edges
        .iter()
        .find(|edge| edge.transition_id == "transition:object-region")
        .expect("object preview should expose object-region context edge");
    assert!(
        edge.activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
    assert!(edge.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context")));
}

#[test]
fn unrelated_context_edge_retains_context_policy() {
    let activated = activation_with_probe0_candidate(
        config(),
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    assert!(activated.edges.iter().any(|edge| {
        edge.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context"))
    }));
}

#[test]
fn omitted_edge_without_exposure_fails_atomically() {
    let mut cfg = config();
    cfg.maximum_activated_edges = 0;
    let activated = activation_with_probe0_candidate(
        cfg,
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    assert!(activated.edges.is_empty());
    assert_eq!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    );
}
