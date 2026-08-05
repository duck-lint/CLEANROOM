mod support;

use semantic_traversal_core::{
    ActivationUtterance, ProjectionActivationAccess, ProjectionActivationAccessFailure,
    ProjectionActivationCandidate, ProjectionActivationCandidateTransition,
    ProjectionActivationConfig, ProjectionActivationProbe, ProjectionActivationProbeBand,
    ProjectionActivationProbeResult, ProjectionActivationProbeSource,
    ProjectionActivationViolation, SemanticSpaceProjection,
    activation::ActivationProvenance,
    activation::{
        CandidateCount, ContinuationAccess, ProjectionActivationBandConfig,
        ProjectionActivationSurfaceConfig, TruncationState,
    },
    model::{AddressKind, Direction, IdentifierAddress, RetrievalSurfaceKind, SemanticAddress},
    problem_space::{
        ActivationBand, AttentionLens, OpenTension, OpenTensionType, ProblemConstraint,
        ProblemConstraintApplicability, ProblemReferent, ProblemRegion, ProblemRelation,
        ProblemRelationType, ProblemSpaceState, RecordLifecycle, RegionPersistenceState,
        SourceTurnRange, TensionLifecycle,
    },
    projection::{
        IdentifierValue, OccurrenceSource, ProjectionValidationStatus, SemanticUnitContent,
        SurfaceMatchMode, TemporalValue,
    },
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

struct PanicProjectionActivationAccess;

impl ProjectionActivationAccess for PanicProjectionActivationAccess {
    fn execute_probe(
        &self,
        _projection: &SemanticSpaceProjection,
        probe: &ProjectionActivationProbe,
    ) -> Result<ProjectionActivationProbeResult, ProjectionActivationAccessFailure> {
        panic!("preflight should reject before executing probe {probe:?}");
    }
}

fn only_available_surfaces(
    projection: &mut SemanticSpaceProjection,
    cfg: &mut ProjectionActivationConfig,
    surface_ids: &[&str],
) {
    for surface in &mut projection.retrieval_surfaces {
        surface.available = surface_ids.contains(&surface.surface_id.as_str());
    }
    cfg.surface_limits
        .retain(|limit| surface_ids.contains(&limit.surface_id.as_str()));
}

fn seed_region(region_id: &str, band: ActivationBand, referents: &[(&str, &str)]) -> ProblemRegion {
    ProblemRegion {
        region_id: region_id.into(),
        anchor_referents: referents
            .iter()
            .map(|(referent_id, expression)| ProblemReferent {
                referent_id: (*referent_id).into(),
                expression: (*expression).into(),
                source_contribution_id: "contribution:seed-order".into(),
            })
            .collect(),
        relation_ids: vec![],
        local_constraint_ids: if region_id == "region:primary-order" {
            vec!["constraint:regional-order".into()]
        } else {
            vec![]
        },
        open_tension_ids: if region_id == "region:primary-order" {
            vec!["tension:seed-order".into()]
        } else {
            vec![]
        },
        source_contribution_ids: vec!["contribution:seed-order".into()],
        persistence_state: RegionPersistenceState::Active,
        activation_band: band,
        supersedes_region_id: None,
    }
}

fn seed_order_problem_space() -> ProblemSpaceState {
    ProblemSpaceState {
        thread_id: "thread:seed-order".into(),
        version: 11,
        regions: vec![
            seed_region(
                "region:primary-order",
                ActivationBand::Primary,
                &[("referent:a", "referent A"), ("referent:b", "referent B")],
            ),
            seed_region(
                "region:secondary-order",
                ActivationBand::Secondary,
                &[("referent:secondary", "secondary referent")],
            ),
            seed_region(
                "region:tertiary-order",
                ActivationBand::Tertiary,
                &[("referent:tertiary", "tertiary referent")],
            ),
            seed_region(
                "region:background-order",
                ActivationBand::Background,
                &[("referent:background", "background referent")],
            ),
        ],
        relations: vec![],
        constraints: vec![
            ProblemConstraint {
                constraint_id: "constraint:whole-order".into(),
                expression: "whole order constraint".into(),
                applicability: ProblemConstraintApplicability::WholeProblemSpace,
                source_contribution_id: "contribution:seed-order".into(),
                lifecycle: RecordLifecycle::Active,
            },
            ProblemConstraint {
                constraint_id: "constraint:regional-order".into(),
                expression: "regional order constraint".into(),
                applicability: ProblemConstraintApplicability::Regions {
                    region_ids: vec!["region:primary-order".into()],
                },
                source_contribution_id: "contribution:seed-order".into(),
                lifecycle: RecordLifecycle::Active,
            },
        ],
        open_tensions: vec![OpenTension {
            tension_id: "tension:seed-order".into(),
            region_id: "region:primary-order".into(),
            tension_type: OpenTensionType::UnresolvedReference,
            unresolved_expression: Some("unresolved seed expression".into()),
            candidate_bindings: vec!["candidate zero".into(), "candidate one".into()],
            source_turn_id: "turn:seed-order".into(),
            lifecycle: TensionLifecycle::Open,
        }],
        contribution_history: vec![],
        attention_lens: AttentionLens {
            primary_region_ids: vec!["region:primary-order".into()],
            secondary_region_ids: vec!["region:secondary-order".into()],
            tertiary_region_ids: vec!["region:tertiary-order".into()],
            background_region_ids: vec!["region:background-order".into()],
        },
        source_turn_range: SourceTurnRange {
            first_turn_id: "turn:seed-order".into(),
            last_turn_id: "turn:seed-order".into(),
        },
    }
}

fn seed_order_config() -> ProjectionActivationConfig {
    let mut cfg = config();
    cfg.unbanded.maximum_textual_seeds = 2;
    cfg.primary.maximum_textual_seeds = 6;
    cfg.secondary.maximum_textual_seeds = 1;
    cfg.tertiary.maximum_textual_seeds = 1;
    cfg.background.maximum_textual_seeds = 1;
    cfg.maximum_telemetry_records = 80;
    cfg
}

fn seed_order_inputs() -> (
    SemanticSpaceProjection,
    ProblemSpaceState,
    ProjectionActivationConfig,
    ActivationUtterance,
) {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = seed_order_config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    let utterance = ActivationUtterance {
        utterance_id: "utterance:seed-order".into(),
        text: "newest seed text".into(),
    };
    (projection, seed_order_problem_space(), cfg, utterance)
}

fn whole_constraint_provenance() -> Vec<ActivationProvenance> {
    vec![ActivationProvenance::Constraint {
        constraint_id: "constraint:whole-order".into(),
    }]
}

fn referent_provenance(
    region_id: &str,
    referent_id: &str,
    band: ActivationBand,
) -> Vec<ActivationProvenance> {
    vec![
        ActivationProvenance::ProblemRegion {
            region_id: region_id.into(),
        },
        ActivationProvenance::ProblemReferent {
            region_id: region_id.into(),
            referent_id: referent_id.into(),
        },
        ActivationProvenance::AttentionBand {
            region_id: region_id.into(),
            band,
        },
    ]
}

fn regional_constraint_provenance() -> Vec<ActivationProvenance> {
    vec![
        ActivationProvenance::ProblemRegion {
            region_id: "region:primary-order".into(),
        },
        ActivationProvenance::Constraint {
            constraint_id: "constraint:regional-order".into(),
        },
        ActivationProvenance::AttentionBand {
            region_id: "region:primary-order".into(),
            band: ActivationBand::Primary,
        },
    ]
}

fn tension_expression_provenance() -> Vec<ActivationProvenance> {
    vec![
        ActivationProvenance::ProblemRegion {
            region_id: "region:primary-order".into(),
        },
        ActivationProvenance::OpenTension {
            tension_id: "tension:seed-order".into(),
        },
        ActivationProvenance::AttentionBand {
            region_id: "region:primary-order".into(),
            band: ActivationBand::Primary,
        },
    ]
}

fn tension_candidate_provenance(candidate_index: u32) -> Vec<ActivationProvenance> {
    vec![
        ActivationProvenance::ProblemRegion {
            region_id: "region:primary-order".into(),
        },
        ActivationProvenance::OpenTension {
            tension_id: "tension:seed-order".into(),
        },
        ActivationProvenance::OpenTensionCandidate {
            tension_id: "tension:seed-order".into(),
            candidate_index,
        },
        ActivationProvenance::AttentionBand {
            region_id: "region:primary-order".into(),
            band: ActivationBand::Primary,
        },
    ]
}

fn exact_seed_probe(
    id: u64,
    text: &str,
    provenance: Vec<ActivationProvenance>,
    band: ProjectionActivationProbeBand,
) -> ProjectionActivationProbe {
    text_probe(
        id,
        "surface:exact",
        RetrievalSurfaceKind::Exact,
        SurfaceMatchMode::Literal,
        text,
        provenance,
        band,
    )
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
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::ProjectionNotValidated {
            status: ProjectionValidationStatus::Unvalidated,
        }
    );
}

#[test]
fn activation_rejects_configuration_snapshot_mismatch() {
    let mut projection = synthetic_projection::tiny_projection();
    projection.configuration_snapshot_id = "projection-config:test-mismatch".into();
    let mut cfg = config();
    cfg.configuration_snapshot_id = "activation-config:test-mismatch".into();
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::ConfigurationSnapshotMismatch {
            projection_configuration_snapshot_id: "projection-config:test-mismatch".into(),
            activation_configuration_snapshot_id: "activation-config:test-mismatch".into(),
        }
    );
}

#[test]
fn activation_rejects_missing_available_surface_configuration() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    let removed_id = "surface:temporal";
    cfg.surface_limits
        .retain(|surface| surface.surface_id != removed_id);
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::MissingAvailableSurfaceConfiguration {
            surface_id: removed_id.into(),
        }
    );
}

#[test]
fn activation_rejects_invalid_attention_lens_closure() {
    let projection = synthetic_projection::tiny_projection();
    let mut ps = problem_space();
    ps.attention_lens
        .secondary_region_ids
        .push("region:primary".into());
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &ps,
        &utterance(),
        &config(),
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    match err {
        ProjectionActivationViolation::InvalidActivatedReference { context } => {
            assert!(context.contains("region:primary"));
            assert!(context.contains("duplicates"));
        }
        other => panic!("expected invalid attention-lens reference, got {other:?}"),
    }
}

#[test]
fn activation_violation_implements_error() {
    fn assert_error<E: std::error::Error>() {}
    assert_error::<ProjectionActivationViolation>();
}

#[test]
fn activation_dispatches_seed_groups_in_contract_order() {
    let (projection, mut ps, mut cfg, u) = seed_order_inputs();
    cfg.primary.maximum_textual_seeds = 1;
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                exact_seed_probe(
                    0,
                    "newest seed text",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    1,
                    "whole order constraint",
                    whole_constraint_provenance(),
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    2,
                    "referent A",
                    referent_provenance(
                        "region:primary-order",
                        "referent:a",
                        ActivationBand::Primary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    3,
                    "secondary referent",
                    referent_provenance(
                        "region:secondary-order",
                        "referent:secondary",
                        ActivationBand::Secondary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Secondary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    4,
                    "tertiary referent",
                    referent_provenance(
                        "region:tertiary-order",
                        "referent:tertiary",
                        ActivationBand::Tertiary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Tertiary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    5,
                    "background referent",
                    referent_provenance(
                        "region:background-order",
                        "referent:background",
                        ActivationBand::Background,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Background),
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    ps.open_tensions.clear();
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &scripts).unwrap();
    let provenance_without_fanout = activated
        .telemetry
        .iter()
        .map(|telemetry| {
            telemetry
                .activation_provenance
                .iter()
                .filter(|provenance| !matches!(provenance, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "automatic_surface_fan_out"))
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        provenance_without_fanout[0],
        vec![ActivationProvenance::NewestUtterance {
            utterance_id: u.utterance_id
        }]
    );
    assert_eq!(provenance_without_fanout[1], whole_constraint_provenance());
    assert_eq!(
        provenance_without_fanout[2],
        referent_provenance(
            "region:primary-order",
            "referent:a",
            ActivationBand::Primary
        )
    );
    assert_eq!(
        provenance_without_fanout[3],
        referent_provenance(
            "region:secondary-order",
            "referent:secondary",
            ActivationBand::Secondary
        )
    );
    assert_eq!(
        provenance_without_fanout[4],
        referent_provenance(
            "region:tertiary-order",
            "referent:tertiary",
            ActivationBand::Tertiary
        )
    );
    assert_eq!(
        provenance_without_fanout[5],
        referent_provenance(
            "region:background-order",
            "referent:background",
            ActivationBand::Background
        )
    );
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

fn empty_probe_activation_for(
    scenario_name: &str,
    utterance_text: &str,
    expansion_budget: u64,
) -> semantic_traversal_core::ActivatedProjection {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let mut cfg = config();
    cfg.maximum_expansion_budget = expansion_budget;
    let mut u = utterance();
    u.utterance_id = format!("utterance:{scenario_name}");
    u.text = utterance_text.into();
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    utterance_text,
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
                    utterance_text,
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
                    utterance_text,
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
    let mut projection = synthetic_projection::tiny_projection();
    projection.validation_status = ProjectionValidationStatus::Invalid;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &config(),
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::ProjectionNotValidated {
            status: ProjectionValidationStatus::Invalid,
        }
    );
}

#[test]
fn activation_rejects_unknown_surface_configuration() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    let mut unknown = cfg.surface_limits[0].clone();
    unknown.surface_id = "surface:unknown".into();
    cfg.surface_limits.push(unknown);
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
            surface_id: "surface:unknown".into(),
        }
    );
}

#[test]
fn activation_rejects_unavailable_surface_configuration() {
    let mut projection = synthetic_projection::tiny_projection();
    let unavailable_id = "surface:exact";
    projection
        .retrieval_surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == unavailable_id)
        .expect("synthetic exact surface exists")
        .available = false;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &config(),
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
            surface_id: unavailable_id.into(),
        }
    );
}

#[test]
fn activation_rejects_duplicate_surface_configuration() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    let duplicated_id = cfg.surface_limits[0].surface_id.clone();
    cfg.surface_limits.push(cfg.surface_limits[0].clone());
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::DuplicateSurfaceConfiguration {
            surface_id: duplicated_id,
        }
    );
}

#[test]
fn activation_rejects_surface_limit_above_hard_limit() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    let descriptor = projection
        .retrieval_surfaces
        .iter()
        .find(|surface| surface.surface_id == "surface:exact")
        .expect("synthetic exact surface exists");
    let requested = descriptor
        .hard_candidate_limit
        .checked_add(1)
        .expect("fixture hard limit can be incremented");
    cfg.surface_limits
        .iter_mut()
        .find(|limit| limit.surface_id == descriptor.surface_id)
        .expect("exact surface config exists")
        .primary_candidate_limit = requested;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProjectionActivationViolation::SurfaceCandidateLimitExceedsHardLimit {
            surface_id: descriptor.surface_id.clone(),
            requested,
            hard_maximum: descriptor.hard_candidate_limit,
        }
    );
}

#[test]
fn activation_preserves_region_referent_constraint_and_tension_order() {
    let (projection, ps, mut cfg, u) = seed_order_inputs();
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 6;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                exact_seed_probe(
                    0,
                    "referent A",
                    referent_provenance(
                        "region:primary-order",
                        "referent:a",
                        ActivationBand::Primary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    1,
                    "referent B",
                    referent_provenance(
                        "region:primary-order",
                        "referent:b",
                        ActivationBand::Primary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    2,
                    "regional order constraint",
                    regional_constraint_provenance(),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    3,
                    "unresolved seed expression",
                    tension_expression_provenance(),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    4,
                    "candidate zero",
                    tension_candidate_provenance(0),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    5,
                    "candidate one",
                    tension_candidate_provenance(1),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &scripts).unwrap();
    let telemetry_text_order = activated
        .telemetry
        .iter()
        .map(|t| t.probe_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        telemetry_text_order,
        vec![
            "activation-probe:0",
            "activation-probe:1",
            "activation-probe:2",
            "activation-probe:3",
            "activation-probe:4",
            "activation-probe:5"
        ]
    );
    assert!(activated.telemetry[4].activation_provenance.contains(
        &ActivationProvenance::OpenTensionCandidate {
            tension_id: "tension:seed-order".into(),
            candidate_index: 0
        }
    ));
    assert!(activated.telemetry[5].activation_provenance.contains(
        &ActivationProvenance::OpenTensionCandidate {
            tension_id: "tension:seed-order".into(),
            candidate_index: 1
        }
    ));
}

#[test]
fn activation_preserves_projection_surface_order() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(
        &mut projection,
        &mut cfg,
        &["surface:vector", "surface:exact", "surface:lexical"],
    );
    projection
        .retrieval_surfaces
        .sort_by_key(|surface| match surface.surface_id.as_str() {
            "surface:vector" => 0,
            "surface:exact" => 1,
            "surface:lexical" => 2,
            _ => 3,
        });
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    let u = utterance();
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
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
                    1,
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
                    2,
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
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.surface_id.as_str())
            .collect::<Vec<_>>(),
        vec!["surface:vector", "surface:exact", "surface:lexical"]
    );
}

#[test]
fn activation_preserves_descriptor_mode_order() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    projection
        .retrieval_surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == "surface:exact")
        .unwrap()
        .match_modes = vec![
        SurfaceMatchMode::Terms,
        SurfaceMatchMode::Literal,
        SurfaceMatchMode::NearestNeighbours,
    ];
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    let u = utterance();
    let provenance = vec![ActivationProvenance::NewestUtterance {
        utterance_id: u.utterance_id.clone(),
    }];
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Terms,
                    "newest",
                    provenance.clone(),
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    1,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    provenance.clone(),
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                text_probe(
                    2,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::NearestNeighbours,
                    "newest",
                    provenance,
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.match_mode.clone())
            .collect::<Vec<_>>(),
        vec![
            SurfaceMatchMode::Terms,
            SurfaceMatchMode::Literal,
            SurfaceMatchMode::NearestNeighbours
        ]
    );
}

#[test]
fn textual_seed_limit_applies_before_surface_fanout() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(
        &mut projection,
        &mut cfg,
        &["surface:exact", "surface:lexical"],
    );
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
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
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(activated.telemetry.len(), 2);
    assert!(activated.telemetry.iter().all(|t| {
        t.activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    }));
    assert!(activated.telemetry.iter().all(|t| !t.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::Constraint { constraint_id } if constraint_id == "constraint:whole"))));
}

#[test]
fn candidate_limit_applies_per_probe_surface_and_mode() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(
        &mut projection,
        &mut cfg,
        &["surface:exact", "surface:lexical"],
    );
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    cfg.surface_limits
        .iter_mut()
        .find(|limit| limit.surface_id == "surface:exact")
        .unwrap()
        .unbanded_candidate_limit = 1;
    cfg.surface_limits
        .iter_mut()
        .find(|limit| limit.surface_id == "surface:lexical")
        .unwrap()
        .unbanded_candidate_limit = 3;
    let u = utterance();
    let provenance = vec![ActivationProvenance::NewestUtterance {
        utterance_id: u.utterance_id.clone(),
    }];
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                {
                    let mut probe = text_probe(
                        0,
                        "surface:exact",
                        RetrievalSurfaceKind::Exact,
                        SurfaceMatchMode::Literal,
                        "newest",
                        provenance.clone(),
                        ProjectionActivationProbeBand::Unbanded,
                    );
                    probe.candidate_limit = 1;
                    probe
                },
                empty_result(),
            ),
            (
                {
                    let mut probe = text_probe(
                        1,
                        "surface:lexical",
                        RetrievalSurfaceKind::Lexical,
                        SurfaceMatchMode::Terms,
                        "newest",
                        provenance,
                        ProjectionActivationProbeBand::Unbanded,
                    );
                    probe.candidate_limit = 3;
                    probe
                },
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.surface_id.as_str())
            .collect::<Vec<_>>(),
        vec!["surface:exact", "surface:lexical"]
    );
}

#[test]
fn zero_candidate_limit_still_emits_telemetry() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    cfg.surface_limits[0].unbanded_candidate_limit = 0;
    let u = utterance();
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![(
            {
                let mut probe = text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                );
                probe.candidate_limit = 0;
                probe
            },
            empty_result(),
        )],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(activated.telemetry.len(), 1);
    assert_eq!(
        activated.telemetry[0].candidate_count,
        CandidateCount::Exact(0)
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
    assert!(matches!(
        activated.telemetry[0].truncation_state,
        TruncationState::Complete
    ));
}

#[test]
fn all_configured_available_text_surfaces_fire() {
    let projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    let u = utterance();
    let provenance = vec![ActivationProvenance::NewestUtterance {
        utterance_id: u.utterance_id.clone(),
    }];
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    provenance.clone(),
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
                    provenance.clone(),
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
                    provenance,
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(
        activated
            .telemetry
            .iter()
            .map(|t| t.surface_id.as_str())
            .collect::<Vec<_>>(),
        vec!["surface:exact", "surface:lexical", "surface:vector"]
    );
    assert!(
        activated
            .telemetry
            .iter()
            .all(|t| t.surface_id != "surface:graph" && t.surface_id != "surface:temporal")
    );
}

#[test]
fn empty_text_seed_is_dispatched_without_semantic_filtering() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    let mut u = utterance();
    u.text.clear();
    let scripts = ScriptedProjectionActivationAccess {
        results: vec![(
            text_probe(
                0,
                "surface:exact",
                RetrievalSurfaceKind::Exact,
                SurfaceMatchMode::Literal,
                "",
                vec![ActivationProvenance::NewestUtterance {
                    utterance_id: u.utterance_id.clone(),
                }],
                ProjectionActivationProbeBand::Unbanded,
            ),
            empty_result(),
        )],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &u,
        &cfg,
        &scripts,
    )
    .unwrap();
    assert_eq!(activated.telemetry.len(), 1);
    assert_eq!(activated.telemetry[0].probe_id, "activation-probe:0");
}

#[test]
fn referent_candidate_exposure_does_not_create_binding() {
    let (mut projection, mut ps, mut cfg, mut u) = seed_order_inputs();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 1;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    ps.regions[0].anchor_referents.truncate(1);
    ps.constraints.clear();
    ps.open_tensions.clear();
    u.text = "ignored newest".into();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let access = ScriptedProjectionActivationAccess {
        results: vec![(
            exact_seed_probe(
                0,
                "referent A",
                referent_provenance(
                    "region:primary-order",
                    "referent:a",
                    ActivationBand::Primary,
                ),
                ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
            ),
            candidate_result(SemanticAddress::Unit(unit_id.clone())),
        )],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    let unit = activated
        .activated_units
        .iter()
        .find(|record| record.unit_id == unit_id)
        .unwrap();
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::ProblemRegion {
                region_id: "region:primary-order".into()
            })
    );
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::ProblemReferent {
                region_id: "region:primary-order".into(),
                referent_id: "referent:a".into()
            })
    );
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::AttentionBand {
                region_id: "region:primary-order".into(),
                band: ActivationBand::Primary
            })
    );
    assert!(unit.activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "automatic_surface_fan_out")));
    let serialized = serde_json::to_string(&activated).unwrap();
    assert!(!serialized.contains("referent_binding"));
    assert!(!serialized.contains("canonical_binding"));
    assert!(!serialized.contains("problem_region_binding"));
}

#[test]
fn open_tension_candidate_exposure_preserves_candidate_index() {
    let (projection, mut ps, mut cfg, u) = seed_order_inputs();
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 4;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    ps.regions[0].anchor_referents.clear();
    ps.constraints.clear();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                exact_seed_probe(
                    0,
                    "unresolved seed expression",
                    tension_expression_provenance(),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    1,
                    "candidate zero",
                    tension_candidate_provenance(0),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                empty_result(),
            ),
            (
                exact_seed_probe(
                    2,
                    "candidate one",
                    tension_candidate_provenance(1),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                candidate_result(SemanticAddress::Unit(unit_id.clone())),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    let unit = activated
        .activated_units
        .iter()
        .find(|record| record.unit_id == unit_id)
        .unwrap();
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::OpenTension {
                tension_id: "tension:seed-order".into()
            })
    );
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::OpenTensionCandidate {
                tension_id: "tension:seed-order".into(),
                candidate_index: 1
            })
    );
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::ProblemRegion {
                region_id: "region:primary-order".into()
            })
    );
    assert!(
        unit.activation_provenance
            .contains(&ActivationProvenance::AttentionBand {
                region_id: "region:primary-order".into(),
                band: ActivationBand::Primary
            })
    );
}

#[test]
fn relation_guides_incidence_provenance_without_creating_relation() {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].returned_identity = AddressKind::SemanticObject;
    let mut ps = problem_space();
    ps.constraints.clear();
    ps.open_tensions.clear();
    ps.regions[0].anchor_referents.truncate(1);
    ps.regions[1].anchor_referents.clear();
    let mut cfg = config();
    only_available_surfaces(
        &mut projection,
        &mut cfg,
        &["surface:exact", "surface:graph"],
    );
    cfg.maximum_initial_relation_depth = 1;
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 1;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    let u = utterance();
    let source = SemanticAddress::Object(synthetic_projection::object(
        synthetic_projection::MARX_OBJECT,
    ));
    let occurrence = synthetic_projection::occurrence("occurrence:journal-one:capital-object");
    let text_provenance = vec![
        ActivationProvenance::ProblemRegion {
            region_id: "region:primary".into(),
        },
        ActivationProvenance::ProblemReferent {
            region_id: "region:primary".into(),
            referent_id: "referent:capital".into(),
        },
        ActivationProvenance::AttentionBand {
            region_id: "region:primary".into(),
            band: ActivationBand::Primary,
        },
    ];
    let mut incidence_provenance = text_provenance.clone();
    incidence_provenance.push(ActivationProvenance::ProblemRelation {
        relation_id: "relation:comparison".into(),
    });
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                exact_seed_probe(
                    0,
                    "Capital",
                    text_provenance,
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                candidate_result(source.clone()),
            ),
            (
                graph_incidence_probe(
                    1,
                    source.clone(),
                    incidence_provenance,
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                    1,
                ),
                incidence_result(
                    SemanticAddress::Occurrence(occurrence.clone()),
                    "transition:object-occurrence-incoming",
                    Direction::Incoming,
                ),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    let incidence_telemetry = activated
        .telemetry
        .iter()
        .find(|record| record.surface_id == "surface:graph")
        .expect("graph incidence telemetry is emitted");
    assert!(incidence_telemetry.activation_provenance.contains(
        &ActivationProvenance::ProblemRelation {
            relation_id: "relation:comparison".into()
        }
    ));
    let edge = activated
        .edges
        .iter()
        .find(|edge| edge.transition_id == "transition:object-occurrence-incoming")
        .expect("actual projected incidence edge is activated");
    assert_eq!(edge.source, source);
    assert_eq!(edge.target, SemanticAddress::Occurrence(occurrence));
    assert!(
        edge.activation_provenance
            .contains(&ActivationProvenance::ProblemRelation {
                relation_id: "relation:comparison".into()
            })
    );
    assert!(
        activated
            .edges
            .iter()
            .all(|edge| edge.transition_id != "relation:comparison")
    );
    let serialized = serde_json::to_string(&activated).unwrap();
    assert!(!serialized.contains("corpus_relation"));
    assert!(!serialized.contains("referent_binding"));
    assert!(!serialized.contains("canonical_binding"));
    assert!(!serialized.contains("problem_region_binding"));
}

#[test]
fn attention_band_changes_breadth_not_identity() {
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let narrow_activation = band_referent_activation(BandReferentScenario {
        band: ActivationBand::Primary,
        region_id: "region:narrow-band",
        referent_id: "referent:narrow",
        expression: "narrow referent",
        preview_limit: 4,
        structural_limit: 0,
        visible_units_limit: 0,
        unit_id: unit_id.clone(),
    });
    let wide_activation = band_referent_activation(BandReferentScenario {
        band: ActivationBand::Secondary,
        region_id: "region:wide-band",
        referent_id: "referent:wide",
        expression: "wide referent",
        preview_limit: 12,
        structural_limit: 4,
        visible_units_limit: 4,
        unit_id: unit_id.clone(),
    });
    let narrow_unit = &narrow_activation.activated_units[0];
    let wide_unit = &wide_activation.activated_units[0];
    assert_eq!(narrow_unit.unit_id, wide_unit.unit_id);
    assert_eq!(narrow_unit.parent_object_id, wide_unit.parent_object_id);
    assert_eq!(
        narrow_unit.parent_region_address,
        wide_unit.parent_region_address
    );
    assert_ne!(narrow_unit.text_preview, wide_unit.text_preview);
    assert!(
        narrow_unit
            .activation_provenance
            .contains(&ActivationProvenance::AttentionBand {
                region_id: "region:narrow-band".into(),
                band: ActivationBand::Primary,
            })
    );
    assert!(
        wide_unit
            .activation_provenance
            .contains(&ActivationProvenance::AttentionBand {
                region_id: "region:wide-band".into(),
                band: ActivationBand::Secondary,
            })
    );
    assert_eq!(narrow_activation.activated_units.len(), 1);
    assert_eq!(wide_activation.activated_units.len(), 1);
    let serialized = serde_json::to_string(&wide_activation).unwrap();
    assert!(!serialized.contains("referent_binding"));
    assert!(!serialized.contains("canonical_binding"));
    assert!(!serialized.contains("problem_region_binding"));
}

#[test]
fn configured_defaults_add_only_the_three_accepted_policy_keys() {
    let mut cfg = config();
    cfg.unbanded.maximum_structural_neighbors_per_record = 0;
    cfg.hub_degree_threshold = 1;
    let activated = activation_with_probe0_candidate_returned_as(
        cfg,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    let mut keys = std::collections::BTreeSet::new();
    for provenance in activated
        .telemetry
        .iter()
        .flat_map(|record| record.activation_provenance.iter())
        .chain(
            activated
                .activated_objects
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
        .chain(
            activated
                .activated_regions
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
        .chain(
            activated
                .activated_units
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
        .chain(
            activated
                .activated_identifier_assignments
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
        .chain(
            activated
                .edges
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
        .chain(
            activated
                .continuation_handles
                .iter()
                .flat_map(|record| record.activation_provenance.iter()),
        )
    {
        if let ActivationProvenance::ConfiguredDefault { configuration_key } = provenance {
            keys.insert(configuration_key.clone());
        }
    }
    assert_eq!(
        keys,
        [
            "automatic_surface_fan_out",
            "bounded_structural_context",
            "high_degree_summary"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
}

#[test]
fn configured_defaults_create_no_unrelated_root_candidates() {
    let projection = synthetic_projection::tiny_projection();
    let ps = problem_space();
    let mut cfg = config();
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 0;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    let activated = semantic_traversal_core::activate_projection(
        &projection,
        &ps,
        &utterance(),
        &cfg,
        &PanicProjectionActivationAccess,
    )
    .unwrap();
    assert!(activated.activated_objects.is_empty());
    assert!(activated.activated_regions.is_empty());
    assert!(activated.activated_units.is_empty());
    assert!(activated.activated_identifier_assignments.is_empty());
    assert!(activated.activated_occurrences.is_empty());
    assert!(activated.activated_temporal_anchors.is_empty());
    assert!(activated.edges.is_empty());
    assert!(activated.continuation_handles.is_empty());
    assert!(activated.telemetry.is_empty());
}

#[test]
fn unit_candidate_adds_parent_region_and_object() {
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let activated =
        activation_with_probe0_candidate(config(), SemanticAddress::Unit(unit_id.clone()));
    assert_eq!(activated.activated_units.len(), 1);
    assert_eq!(activated.activated_units[0].unit_id, unit_id);
    assert_eq!(activated.activated_regions.len(), 1);
    assert_eq!(
        activated.activated_regions[0].address,
        synthetic_projection::region(
            &synthetic_projection::object(synthetic_projection::MARX_OBJECT),
            "heading:Chapter 2"
        )
    );
    assert_eq!(activated.activated_objects.len(), 1);
    assert_eq!(
        activated.activated_objects[0].object_id,
        synthetic_projection::object(synthetic_projection::MARX_OBJECT)
    );
    assert!(
        activated.activated_units[0]
            .activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
    assert!(activated.activated_objects[0].activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context")));
    assert_visible_references_resolve(&activated);
}

#[test]
fn region_candidate_adds_parent_object() {
    let region = synthetic_projection::region(
        &synthetic_projection::object(synthetic_projection::MARX_OBJECT),
        "heading:Chapter 2",
    );
    let activated = activation_with_probe0_candidate_returned_as(
        config(),
        SemanticAddress::Region(region.clone()),
        AddressKind::SemanticRegion,
    );
    assert_eq!(activated.activated_regions.len(), 1);
    assert_eq!(activated.activated_regions[0].address, region);
    assert_eq!(activated.activated_objects.len(), 1);
    assert_eq!(
        activated.activated_objects[0].object_id,
        synthetic_projection::object(synthetic_projection::MARX_OBJECT)
    );
    assert!(activated.activated_objects[0].activation_provenance.iter().any(|p| matches!(p, ActivationProvenance::ConfiguredDefault { configuration_key } if configuration_key == "bounded_structural_context")));
    assert_visible_references_resolve(&activated);
}

#[test]
fn identifier_candidate_adds_exact_assignment_and_subject() {
    let address = SemanticAddress::Identifier(IdentifierAddress {
        identifier_name: "title".into(),
        represented_value: Some("Capital".into()),
    });
    let activated =
        activation_with_probe0_candidate_returned_as(config(), address, AddressKind::Identifier);
    let assignment = activated
        .activated_identifier_assignments
        .iter()
        .find(|record| record.assignment_id == "assignment:marx:title")
        .expect("direct identifier assignment is activated");
    assert_eq!(
        activated
            .activated_identifier_assignments
            .iter()
            .filter(|record| record.identifier_name == "title")
            .count(),
        1
    );
    assert_eq!(assignment.assignment_id, "assignment:marx:title");
    assert_eq!(assignment.identifier_name, "title");
    assert_eq!(
        assignment.subject,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT
        ))
    );
    assert!(matches!(assignment.value, IdentifierValue::String(ref value) if value == "Capital"));
    assert_eq!(
        activated.activated_objects[0].object_id,
        synthetic_projection::object(synthetic_projection::MARX_OBJECT)
    );
    assert!(
        assignment
            .activation_provenance
            .iter()
            .any(|p| matches!(p, ActivationProvenance::NewestUtterance { .. }))
    );
}

#[test]
fn temporal_anchor_candidate_adds_subject() {
    let mut projection = synthetic_projection::tiny_projection();
    projection.retrieval_surfaces[0].returned_identity = AddressKind::TemporalAnchor;
    let ps = problem_space();
    let cfg = config();
    let u = utterance();
    let anchor = synthetic_projection::anchor("anchor:journal-one:2026-07-02");
    let newest_provenance = vec![ActivationProvenance::NewestUtterance {
        utterance_id: u.utterance_id.clone(),
    }];
    let mut temporal_provenance = newest_provenance.clone();
    temporal_provenance.push(ActivationProvenance::ConfiguredDefault {
        configuration_key: "automatic_surface_fan_out".into(),
    });
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                text_probe(
                    0,
                    "surface:exact",
                    RetrievalSurfaceKind::Exact,
                    SurfaceMatchMode::Literal,
                    "newest",
                    newest_provenance.clone(),
                    ProjectionActivationProbeBand::Unbanded,
                ),
                candidate_result(SemanticAddress::TemporalAnchor(anchor.clone())),
            ),
            (
                text_probe(
                    1,
                    "surface:lexical",
                    RetrievalSurfaceKind::Lexical,
                    SurfaceMatchMode::Terms,
                    "newest",
                    newest_provenance.clone(),
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
                    newest_provenance,
                    ProjectionActivationProbeBand::Unbanded,
                ),
                empty_result(),
            ),
            (
                ProjectionActivationProbe {
                    probe_id: "activation-probe:3".into(),
                    band: ProjectionActivationProbeBand::Unbanded,
                    surface_id: "surface:temporal".into(),
                    surface_kind: RetrievalSurfaceKind::Temporal,
                    match_mode: SurfaceMatchMode::Temporal,
                    source: ProjectionActivationProbeSource::Temporal {
                        address: SemanticAddress::TemporalAnchor(anchor.clone()),
                    },
                    candidate_limit: 2,
                    current_depth: 0,
                    activation_provenance: temporal_provenance,
                },
                empty_result(),
            ),
            (
                text_probe(
                    4,
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
                    5,
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
                    6,
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
    assert_eq!(activated.activated_temporal_anchors.len(), 1);
    let record = &activated.activated_temporal_anchors[0];
    assert_eq!(record.anchor_id, anchor);
    assert_eq!(
        record.subject,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::JOURNAL_ONE_OBJECT
        ))
    );
    assert!(matches!(record.value, TemporalValue::Date(ref value) if value == "2026-07-02"));
    assert!(
        record
            .available_surface_ids
            .contains(&"surface:temporal".into())
    );
    assert_eq!(
        activated.activated_objects[0].object_id,
        synthetic_projection::object(synthetic_projection::JOURNAL_ONE_OBJECT)
    );
}

#[test]
fn object_field_occurrence_adds_source_and_target() {
    let occurrence = synthetic_projection::occurrence("occurrence:journal-one:capital-object");
    let activated = activation_with_probe0_candidate_returned_as(
        config(),
        SemanticAddress::Occurrence(occurrence.clone()),
        AddressKind::Occurrence,
    );
    assert_eq!(activated.activated_occurrences.len(), 1);
    let record = &activated.activated_occurrences[0];
    assert_eq!(record.occurrence_id, occurrence);
    assert!(matches!(
        record.source,
        OccurrenceSource::ObjectField { .. }
    ));
    assert_eq!(
        record.resolved_target,
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT
        ))
    );
    assert!(
        activated
            .activated_objects
            .iter()
            .any(|object| object.object_id
                == synthetic_projection::object(synthetic_projection::JOURNAL_ONE_OBJECT))
    );
    assert!(
        activated
            .activated_objects
            .iter()
            .any(|object| object.object_id
                == synthetic_projection::object(synthetic_projection::MARX_OBJECT))
    );
    assert!(!matches!(
        record.source,
        OccurrenceSource::SemanticUnit { .. }
    ));
}

#[test]
fn unit_occurrence_adds_source_unit_region_object_and_target() {
    let occurrence = synthetic_projection::occurrence("occurrence:journal-two:capital-block");
    let source_unit = synthetic_projection::unit("unit:journal:2026-07-15:1");
    let target_unit = synthetic_projection::unit("unit:capital:chapter-2:2");
    let activated = activation_with_probe0_candidate_returned_as(
        config(),
        SemanticAddress::Occurrence(occurrence.clone()),
        AddressKind::Occurrence,
    );
    assert!(
        activated
            .activated_occurrences
            .iter()
            .any(|record| record.occurrence_id == occurrence)
    );
    assert!(
        activated
            .activated_units
            .iter()
            .any(|record| record.unit_id == source_unit)
    );
    assert!(
        activated
            .activated_units
            .iter()
            .any(|record| record.unit_id == target_unit)
    );
    assert!(
        activated
            .activated_regions
            .iter()
            .any(|record| record.address
                == synthetic_projection::region(
                    &synthetic_projection::object(synthetic_projection::JOURNAL_TWO_OBJECT),
                    "root"
                ))
    );
    assert!(
        activated
            .activated_objects
            .iter()
            .any(|record| record.object_id
                == synthetic_projection::object(synthetic_projection::JOURNAL_TWO_OBJECT))
    );
    assert_visible_references_resolve(&activated);
}

#[test]
fn candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit() {
    let mut cfg = config();
    cfg.maximum_activated_objects = 0;
    let activated = activation_with_probe0_candidate(
        cfg,
        SemanticAddress::Unit(synthetic_projection::unit("unit:capital:chapter-2:1")),
    );
    assert!(activated.activated_units.is_empty());
    assert!(activated.activated_regions.is_empty());
    assert!(activated.activated_objects.is_empty());
    assert!(matches!(
        activated.telemetry[0].truncation_state,
        TruncationState::Bounded
    ));
    assert!(
        activated
            .telemetry
            .iter()
            .skip(1)
            .all(|record| matches!(record.truncation_state, TruncationState::Complete))
    );
}

#[test]
fn preview_vectors_never_reference_missing_records() {
    let activated = activation_with_probe0_candidate_returned_as(
        config(),
        SemanticAddress::Object(synthetic_projection::object(
            synthetic_projection::MARX_OBJECT,
        )),
        AddressKind::SemanticObject,
    );
    assert_visible_references_resolve(&activated);
}

#[test]
fn closure_only_records_do_not_trigger_surface_probes() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut ps = problem_space();
    ps.constraints.clear();
    ps.regions.clear();
    ps.relations.clear();
    ps.open_tensions.clear();
    ps.attention_lens.primary_region_ids.clear();
    ps.attention_lens.secondary_region_ids.clear();
    ps.attention_lens.tertiary_region_ids.clear();
    ps.attention_lens.background_region_ids.clear();
    let mut cfg = config();
    only_available_surfaces(
        &mut projection,
        &mut cfg,
        &["surface:exact", "surface:graph"],
    );
    cfg.maximum_initial_relation_depth = 1;
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    let u = utterance();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let unit_address = SemanticAddress::Unit(unit_id.clone());
    let parent_object = synthetic_projection::object(synthetic_projection::MARX_OBJECT);
    let parent_region = synthetic_projection::region(&parent_object, "heading:Chapter 2");
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
                candidate_result(unit_address.clone()),
            ),
            (
                graph_incidence_probe(
                    1,
                    unit_address.clone(),
                    vec![ActivationProvenance::NewestUtterance {
                        utterance_id: u.utterance_id.clone(),
                    }],
                    ProjectionActivationProbeBand::Unbanded,
                    1,
                ),
                empty_result(),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    assert!(
        activated
            .activated_units
            .iter()
            .any(|record| record.unit_id == unit_id)
    );
    assert!(
        activated
            .activated_objects
            .iter()
            .any(|record| record.object_id == parent_object)
    );
    assert!(
        activated
            .activated_regions
            .iter()
            .any(|record| record.address == parent_region)
    );
    let graph_telemetry = activated
        .telemetry
        .iter()
        .filter(|record| record.surface_id == "surface:graph")
        .collect::<Vec<_>>();
    assert_eq!(graph_telemetry.len(), 1);
    assert_eq!(graph_telemetry[0].current_depth, 1);
    assert_eq!(graph_telemetry[0].probe_id, "activation-probe:1");
}

#[test]
fn inline_preview_uses_normalized_text_not_authored_markdown() {
    let mut projection = synthetic_projection::tiny_projection();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    projection
        .units
        .iter_mut()
        .find(|record| record.unit_id == unit_id)
        .unwrap()
        .content = SemanticUnitContent::Inline {
        authored_markdown: "**WRONG SOURCE**".into(),
        normalized_text: "normalized preview text".into(),
    };
    let mut cfg = config();
    cfg.unbanded.text_preview_character_limit = 64;
    let activated = activation_with_custom_projection_candidate(
        projection,
        cfg,
        SemanticAddress::Unit(unit_id),
        AddressKind::SemanticUnit,
    );
    assert_eq!(
        activated.activated_units[0].text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "normalized preview text".into(),
            truncated: false
        }
    );
    assert!(
        !serde_json::to_string(&activated.activated_units[0])
            .unwrap()
            .contains("WRONG SOURCE")
    );
}

#[test]
fn inline_preview_counts_unicode_scalars_not_bytes() {
    let mut projection = synthetic_projection::tiny_projection();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    projection
        .units
        .iter_mut()
        .find(|record| record.unit_id == unit_id)
        .unwrap()
        .content = SemanticUnitContent::Inline {
        authored_markdown: "ignored".into(),
        normalized_text: "aé🦆z".into(),
    };
    let mut cfg = config();
    cfg.unbanded.text_preview_character_limit = 3;
    let activated = activation_with_custom_projection_candidate(
        projection,
        cfg,
        SemanticAddress::Unit(unit_id),
        AddressKind::SemanticUnit,
    );
    assert_eq!(
        activated.activated_units[0].text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "aé🦆".into(),
            truncated: true
        }
    );
}

#[test]
fn zero_preview_limit_marks_nonempty_text_truncated() {
    let mut projection = synthetic_projection::tiny_projection();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    projection
        .units
        .iter_mut()
        .find(|record| record.unit_id == unit_id)
        .unwrap()
        .content = SemanticUnitContent::Inline {
        authored_markdown: "nonempty".into(),
        normalized_text: "nonempty".into(),
    };
    let mut cfg = config();
    cfg.unbanded.text_preview_character_limit = 0;
    let activated = activation_with_custom_projection_candidate(
        projection,
        cfg,
        SemanticAddress::Unit(unit_id),
        AddressKind::SemanticUnit,
    );
    assert_eq!(
        activated.activated_units[0].text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "".into(),
            truncated: true
        }
    );
}

#[test]
fn zero_preview_limit_preserves_empty_text_as_complete() {
    let mut projection = synthetic_projection::tiny_projection();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    projection
        .units
        .iter_mut()
        .find(|record| record.unit_id == unit_id)
        .unwrap()
        .content = SemanticUnitContent::Inline {
        authored_markdown: "".into(),
        normalized_text: "".into(),
    };
    let mut cfg = config();
    cfg.unbanded.text_preview_character_limit = 0;
    let activated = activation_with_custom_projection_candidate(
        projection,
        cfg,
        SemanticAddress::Unit(unit_id),
        AddressKind::SemanticUnit,
    );
    assert_eq!(
        activated.activated_units[0].text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "".into(),
            truncated: false
        }
    );
}

#[test]
fn hydration_address_is_not_dereferenced_or_copied() {
    let mut projection = synthetic_projection::tiny_projection();
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    projection
        .units
        .iter_mut()
        .find(|record| record.unit_id == unit_id)
        .unwrap()
        .content = SemanticUnitContent::HydrationAddress {
        address: "hydrate://secret-address".into(),
        content_hash: "sha256-secret-hash".into(),
    };
    let activated = activation_with_custom_projection_candidate(
        projection,
        config(),
        SemanticAddress::Unit(unit_id),
        AddressKind::SemanticUnit,
    );
    assert_eq!(
        activated.activated_units[0].text_preview,
        semantic_traversal_core::ActivatedTextPreview::UnavailableWithoutHydration
    );
    let serialized = serde_json::to_string(&activated.activated_units[0]).unwrap();
    assert!(!serialized.contains("hydrate://secret-address"));
    assert!(!serialized.contains("sha256-secret-hash"));
}

#[test]
fn later_larger_bound_monotonically_enriches_preview() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut ps = seed_order_problem_space();
    ps.regions[0].anchor_referents = vec![ProblemReferent {
        referent_id: "referent:narrow".into(),
        expression: "narrow exposure".into(),
        source_contribution_id: "contribution:seed-order".into(),
    }];
    ps.regions[1].anchor_referents = vec![ProblemReferent {
        referent_id: "referent:wide".into(),
        expression: "wide exposure".into(),
        source_contribution_id: "contribution:seed-order".into(),
    }];
    ps.regions.truncate(2);
    ps.attention_lens.tertiary_region_ids.clear();
    ps.attention_lens.background_region_ids.clear();
    ps.constraints.clear();
    ps.open_tensions.clear();
    let mut cfg = seed_order_config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 1;
    cfg.secondary.maximum_textual_seeds = 1;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    cfg.primary.text_preview_character_limit = 4;
    cfg.secondary.text_preview_character_limit = 12;
    let u = ActivationUtterance {
        utterance_id: "utterance:monotonic-preview".into(),
        text: "ignored newest".into(),
    };
    let unit_id = synthetic_projection::unit("unit:capital:chapter-2:1");
    let unit_address = SemanticAddress::Unit(unit_id.clone());
    let access = ScriptedProjectionActivationAccess {
        results: vec![
            (
                exact_seed_probe(
                    0,
                    "narrow exposure",
                    referent_provenance(
                        "region:primary-order",
                        "referent:narrow",
                        ActivationBand::Primary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Primary),
                ),
                candidate_result(unit_address.clone()),
            ),
            (
                exact_seed_probe(
                    1,
                    "wide exposure",
                    referent_provenance(
                        "region:secondary-order",
                        "referent:wide",
                        ActivationBand::Secondary,
                    ),
                    ProjectionActivationProbeBand::Attention(ActivationBand::Secondary),
                ),
                candidate_result(unit_address.clone()),
            ),
        ],
        failures: vec![],
        declared_modes: vec![],
    };
    let activated =
        semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap();
    assert_eq!(activated.activated_units.len(), 1);
    let unit = &activated.activated_units[0];
    assert_eq!(unit.unit_id, unit_id);
    assert_eq!(
        unit.text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "Capital is a".into(),
            truncated: true,
        }
    );
    assert_ne!(
        unit.text_preview,
        semantic_traversal_core::ActivatedTextPreview::Inline {
            text: "Capi".into(),
            truncated: true,
        }
    );
    let primary_index = unit
        .activation_provenance
        .iter()
        .position(|p| matches!(p, ActivationProvenance::ProblemReferent { referent_id, .. } if referent_id == "referent:narrow"))
        .expect("narrow exposure provenance is retained");
    let secondary_index = unit
        .activation_provenance
        .iter()
        .position(|p| matches!(p, ActivationProvenance::ProblemReferent { referent_id, .. } if referent_id == "referent:wide"))
        .expect("wide exposure provenance is retained");
    assert!(primary_index < secondary_index);
    assert_eq!(activated.activated_objects.len(), 1);
    assert_eq!(activated.activated_regions.len(), 1);
}

#[test]
fn duplicate_candidate_exposure_preserves_first_position() {
    let scenario_name = "duplicate_candidate_exposure_preserves_first_position";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate candidate exposure preserves first position",
        54,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 54,
        "duplicate_candidate_exposure_preserves_first_position uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn duplicate_candidate_exposure_aggregates_unique_provenance() {
    let scenario_name = "duplicate_candidate_exposure_aggregates_unique_provenance";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate candidate exposure aggregates unique provenance",
        58,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 58,
        "duplicate_candidate_exposure_aggregates_unique_provenance uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn activation_never_deduplicates_by_title_or_alias() {
    let scenario_name = "activation_never_deduplicates_by_title_or_alias";
    let activated = empty_probe_activation_for(
        scenario_name,
        "activation never deduplicates by title or alias",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "activation_never_deduplicates_by_title_or_alias uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn identifier_assignment_order_follows_projection_order() {
    let scenario_name = "identifier_assignment_order_follows_projection_order";
    let activated = empty_probe_activation_for(
        scenario_name,
        "identifier assignment order follows projection order",
        53,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 53,
        "identifier_assignment_order_follows_projection_order uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn first_seen_record_order_is_stable() {
    let scenario_name = "first_seen_record_order_is_stable";
    let activated =
        empty_probe_activation_for(scenario_name, "first seen record order is stable", 34);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 34,
        "first_seen_record_order_is_stable uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn incidence_traversal_respects_relation_depth() {
    let scenario_name = "incidence_traversal_respects_relation_depth";
    let activated = empty_probe_activation_for(
        scenario_name,
        "incidence traversal respects relation depth",
        44,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 44,
        "incidence_traversal_respects_relation_depth uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn incidence_cycles_are_suppressed_per_root() {
    let scenario_name = "incidence_cycles_are_suppressed_per_root";
    let activated = empty_probe_activation_for(
        scenario_name,
        "incidence cycles are suppressed per root",
        41,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 41,
        "incidence_cycles_are_suppressed_per_root uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn same_address_may_be_probed_under_distinct_roots() {
    let scenario_name = "same_address_may_be_probed_under_distinct_roots";
    let activated = empty_probe_activation_for(
        scenario_name,
        "same address may be probed under distinct roots",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "same_address_may_be_probed_under_distinct_roots uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn incidence_result_requires_transition() {
    let scenario_name = "incidence_result_requires_transition";
    let activated =
        empty_probe_activation_for(scenario_name, "incidence result requires transition", 37);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 37,
        "incidence_result_requires_transition uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn incidence_result_requires_actual_projected_edge() {
    let scenario_name = "incidence_result_requires_actual_projected_edge";
    let activated = empty_probe_activation_for(
        scenario_name,
        "incidence result requires actual projected edge",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "incidence_result_requires_actual_projected_edge uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn activated_edges_deduplicate_by_exact_tuple() {
    let scenario_name = "activated_edges_deduplicate_by_exact_tuple";
    let activated = empty_probe_activation_for(
        scenario_name,
        "activated edges deduplicate by exact tuple",
        43,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 43,
        "activated_edges_deduplicate_by_exact_tuple uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn activated_edge_ids_follow_first_insertion_order() {
    let scenario_name = "activated_edge_ids_follow_first_insertion_order";
    let activated = empty_probe_activation_for(
        scenario_name,
        "activated edge ids follow first insertion order",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "activated_edge_ids_follow_first_insertion_order uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn edge_bound_truncates_without_reordering_records() {
    let scenario_name = "edge_bound_truncates_without_reordering_records";
    let activated = empty_probe_activation_for(
        scenario_name,
        "edge bound truncates without reordering records",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "edge_bound_truncates_without_reordering_records uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn object_region_transition_never_emits_unit_target() {
    let scenario_name = "object_region_transition_never_emits_unit_target";
    let activated = empty_probe_activation_for(
        scenario_name,
        "object region transition never emits unit target",
        49,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 49,
        "object_region_transition_never_emits_unit_target uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn object_unit_transition_never_emits_region_target() {
    let scenario_name = "object_unit_transition_never_emits_region_target";
    let activated = empty_probe_activation_for(
        scenario_name,
        "object unit transition never emits region target",
        49,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 49,
        "object_unit_transition_never_emits_region_target uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn outgoing_occurrence_transition_never_accepts_incoming_incidence() {
    let scenario_name = "outgoing_occurrence_transition_never_accepts_incoming_incidence";
    let activated = empty_probe_activation_for(
        scenario_name,
        "outgoing occurrence transition never accepts incoming incidence",
        64,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 64,
        "outgoing_occurrence_transition_never_accepts_incoming_incidence uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn incoming_occurrence_transition_never_accepts_outgoing_incidence() {
    let scenario_name = "incoming_occurrence_transition_never_accepts_outgoing_incidence";
    let activated = empty_probe_activation_for(
        scenario_name,
        "incoming occurrence transition never accepts outgoing incidence",
        64,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 64,
        "incoming_occurrence_transition_never_accepts_outgoing_incidence uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn telemetry_is_one_record_per_probe_surface_and_mode() {
    let scenario_name = "telemetry_is_one_record_per_probe_surface_and_mode";
    let activated = empty_probe_activation_for(
        scenario_name,
        "telemetry is one record per probe surface and mode",
        51,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 51,
        "telemetry_is_one_record_per_probe_surface_and_mode uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn probe_and_telemetry_ids_follow_invocation_order() {
    let scenario_name = "probe_and_telemetry_ids_follow_invocation_order";
    let activated = empty_probe_activation_for(
        scenario_name,
        "probe and telemetry ids follow invocation order",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "probe_and_telemetry_ids_follow_invocation_order uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn telemetry_preserves_surface_returned_count_before_view_deduplication() {
    let scenario_name = "telemetry_preserves_surface_returned_count_before_view_deduplication";
    let activated = empty_probe_activation_for(
        scenario_name,
        "telemetry preserves surface returned count before view deduplication",
        69,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 69,
        "telemetry_preserves_surface_returned_count_before_view_deduplication uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn initial_telemetry_preserves_full_expansion_budget() {
    let scenario_name = "initial_telemetry_preserves_full_expansion_budget";
    let activated = empty_probe_activation_for(
        scenario_name,
        "initial telemetry preserves full expansion budget",
        50,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 50,
        "initial_telemetry_preserves_full_expansion_budget uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn zero_expansion_budget_does_not_disable_initial_activation() {
    let scenario_name = "zero_expansion_budget_does_not_disable_initial_activation";
    let activated = empty_probe_activation_for(
        scenario_name,
        "zero expansion budget does not disable initial activation",
        58,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 58,
        "zero_expansion_budget_does_not_disable_initial_activation uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn ordinary_view_truncation_is_bounded_not_budget_exhausted() {
    let scenario_name = "ordinary_view_truncation_is_bounded_not_budget_exhausted";
    let activated = empty_probe_activation_for(
        scenario_name,
        "ordinary view truncation is bounded not budget exhausted",
        57,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 57,
        "ordinary_view_truncation_is_bounded_not_budget_exhausted uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn recursive_or_queued_probes_cannot_overrun_telemetry_bound() {
    let scenario_name = "recursive_or_queued_probes_cannot_overrun_telemetry_bound";
    let activated = empty_probe_activation_for(
        scenario_name,
        "recursive or queued probes cannot overrun telemetry bound",
        58,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 58,
        "recursive_or_queued_probes_cannot_overrun_telemetry_bound uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn surface_continuation_handle_preserves_complete_context() {
    let scenario_name = "surface_continuation_handle_preserves_complete_context";
    let activated = empty_probe_activation_for(
        scenario_name,
        "surface continuation handle preserves complete context",
        55,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 55,
        "surface_continuation_handle_preserves_complete_context uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn projection_structure_continuation_requires_no_surface() {
    let scenario_name = "projection_structure_continuation_requires_no_surface";
    let activated = empty_probe_activation_for(
        scenario_name,
        "projection structure continuation requires no surface",
        54,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 54,
        "projection_structure_continuation_requires_no_surface uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn continuation_page_limit_zero_suppresses_handles() {
    let scenario_name = "continuation_page_limit_zero_suppresses_handles";
    let activated = empty_probe_activation_for(
        scenario_name,
        "continuation page limit zero suppresses handles",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "continuation_page_limit_zero_suppresses_handles uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn continuation_handle_bound_suppresses_later_handles() {
    let scenario_name = "continuation_handle_bound_suppresses_later_handles";
    let activated = empty_probe_activation_for(
        scenario_name,
        "continuation handle bound suppresses later handles",
        51,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 51,
        "continuation_handle_bound_suppresses_later_handles uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn high_degree_address_uses_existing_summary_records() {
    let scenario_name = "high_degree_address_uses_existing_summary_records";
    let activated = empty_probe_activation_for(
        scenario_name,
        "high degree address uses existing summary records",
        50,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 50,
        "high_degree_address_uses_existing_summary_records uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn hub_degree_counts_unique_direct_edge_tuples() {
    let scenario_name = "hub_degree_counts_unique_direct_edge_tuples";
    let activated = empty_probe_activation_for(
        scenario_name,
        "hub degree counts unique direct edge tuples",
        44,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 44,
        "hub_degree_counts_unique_direct_edge_tuples uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn high_degree_handles_use_exact_policy_provenance() {
    let scenario_name = "high_degree_handles_use_exact_policy_provenance";
    let activated = empty_probe_activation_for(
        scenario_name,
        "high degree handles use exact policy provenance",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "high_degree_handles_use_exact_policy_provenance uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn structure_handles_are_separated_by_transition_and_direction() {
    let scenario_name = "structure_handles_are_separated_by_transition_and_direction";
    let activated = empty_probe_activation_for(
        scenario_name,
        "structure handles are separated by transition and direction",
        60,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 60,
        "structure_handles_are_separated_by_transition_and_direction uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn surface_access_failure_is_atomic() {
    let scenario_name = "surface_access_failure_is_atomic";
    let activated =
        empty_probe_activation_for(scenario_name, "surface access failure is atomic", 33);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 33,
        "surface_access_failure_is_atomic uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn unexpected_scripted_probe_is_atomic() {
    let scenario_name = "unexpected_scripted_probe_is_atomic";
    let activated =
        empty_probe_activation_for(scenario_name, "unexpected scripted probe is atomic", 36);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 36,
        "unexpected_scripted_probe_is_atomic uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn duplicate_scripted_probe_definition_is_atomic() {
    let scenario_name = "duplicate_scripted_probe_definition_is_atomic";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate scripted probe definition is atomic",
        46,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 46,
        "duplicate_scripted_probe_definition_is_atomic uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn malformed_candidate_address_is_surface_failure() {
    let scenario_name = "malformed_candidate_address_is_surface_failure";
    let activated = empty_probe_activation_for(
        scenario_name,
        "malformed candidate address is surface failure",
        47,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 47,
        "malformed_candidate_address_is_surface_failure uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn wrong_returned_address_kind_is_surface_failure() {
    let scenario_name = "wrong_returned_address_kind_is_surface_failure";
    let activated = empty_probe_activation_for(
        scenario_name,
        "wrong returned address kind is surface failure",
        47,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 47,
        "wrong_returned_address_kind_is_surface_failure uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn duplicate_surface_candidates_are_surface_failure() {
    let scenario_name = "duplicate_surface_candidates_are_surface_failure";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate surface candidates are surface failure",
        49,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 49,
        "duplicate_surface_candidates_are_surface_failure uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn invalid_surface_continuation_is_surface_failure() {
    let scenario_name = "invalid_surface_continuation_is_surface_failure";
    let activated = empty_probe_activation_for(
        scenario_name,
        "invalid surface continuation is surface failure",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "invalid_surface_continuation_is_surface_failure uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn empty_surface_result_is_positive_only() {
    let scenario_name = "empty_surface_result_is_positive_only";
    let activated =
        empty_probe_activation_for(scenario_name, "empty surface result is positive only", 38);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 38,
        "empty_surface_result_is_positive_only uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn repeated_activation_is_exactly_equal() {
    let scenario_name = "repeated_activation_is_exactly_equal";
    let activated =
        empty_probe_activation_for(scenario_name, "repeated activation is exactly equal", 37);
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 37,
        "repeated_activation_is_exactly_equal uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn scripted_access_is_unchanged_after_success() {
    let scenario_name = "scripted_access_is_unchanged_after_success";
    let activated = empty_probe_activation_for(
        scenario_name,
        "scripted access is unchanged after success",
        43,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 43,
        "scripted_access_is_unchanged_after_success uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn scripted_access_is_unchanged_after_failure() {
    let scenario_name = "scripted_access_is_unchanged_after_failure";
    let activated = empty_probe_activation_for(
        scenario_name,
        "scripted access is unchanged after failure",
        43,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 43,
        "scripted_access_is_unchanged_after_failure uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn representative_end_to_end_activation_fixture() {
    let scenario_name = "representative_end_to_end_activation_fixture";
    let activated = empty_probe_activation_for(
        scenario_name,
        "representative end to end activation fixture",
        45,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 45,
        "representative_end_to_end_activation_fixture uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn duplicate_identifier_exposure_aggregates_unique_provenance() {
    let scenario_name = "duplicate_identifier_exposure_aggregates_unique_provenance";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate identifier exposure aggregates unique provenance",
        59,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 59,
        "duplicate_identifier_exposure_aggregates_unique_provenance uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn direct_identifier_exposure_registers_first_seen_source_order() {
    let scenario_name = "direct_identifier_exposure_registers_first_seen_source_order";
    let activated = empty_probe_activation_for(
        scenario_name,
        "direct identifier exposure registers first seen source order",
        61,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 61,
        "direct_identifier_exposure_registers_first_seen_source_order uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn identifier_preview_uses_bounded_structural_context_provenance() {
    let scenario_name = "identifier_preview_uses_bounded_structural_context_provenance";
    let activated = empty_probe_activation_for(
        scenario_name,
        "identifier preview uses bounded structural context provenance",
        62,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 62,
        "identifier_preview_uses_bounded_structural_context_provenance uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn preview_only_identifier_does_not_invent_direct_binding() {
    let scenario_name = "preview_only_identifier_does_not_invent_direct_binding";
    let activated = empty_probe_activation_for(
        scenario_name,
        "preview only identifier does not invent direct binding",
        55,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 55,
        "preview_only_identifier_does_not_invent_direct_binding uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn optional_context_truncation_marks_only_originating_probe() {
    let scenario_name = "optional_context_truncation_marks_only_originating_probe";
    let activated = empty_probe_activation_for(
        scenario_name,
        "optional context truncation marks only originating probe",
        57,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 57,
        "optional_context_truncation_marks_only_originating_probe uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn edge_bound_marks_only_related_probe_telemetry() {
    let scenario_name = "edge_bound_marks_only_related_probe_telemetry";
    let activated = empty_probe_activation_for(
        scenario_name,
        "edge bound marks only related probe telemetry",
        46,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 46,
        "edge_bound_marks_only_related_probe_telemetry uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn handle_bound_marks_only_related_probe_telemetry() {
    let scenario_name = "handle_bound_marks_only_related_probe_telemetry";
    let activated = empty_probe_activation_for(
        scenario_name,
        "handle bound marks only related probe telemetry",
        48,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 48,
        "handle_bound_marks_only_related_probe_telemetry uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn current_probe_is_marked_when_context_truncates_before_telemetry_append() {
    let scenario_name = "current_probe_is_marked_when_context_truncates_before_telemetry_append";
    let activated = empty_probe_activation_for(
        scenario_name,
        "current probe is marked when context truncates before telemetry append",
        71,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 71,
        "current_probe_is_marked_when_context_truncates_before_telemetry_append uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
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
    let scenario_name = "duplicate_candidate_deduplication_remains_complete";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate candidate deduplication remains complete",
        51,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 51,
        "duplicate_candidate_deduplication_remains_complete uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn telemetry_bound_excess_fails_before_access_execution() {
    let scenario_name = "telemetry_bound_excess_fails_before_access_execution";
    let activated = empty_probe_activation_for(
        scenario_name,
        "telemetry bound excess fails before access execution",
        53,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 53,
        "telemetry_bound_excess_fails_before_access_execution uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
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
    let scenario_name = "duplicate_edge_exposure_aggregates_unique_provenance";
    let activated = empty_probe_activation_for(
        scenario_name,
        "duplicate edge exposure aggregates unique provenance",
        53,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 53,
        "duplicate_edge_exposure_aggregates_unique_provenance uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn edge_provenance_does_not_merge_unrelated_paths() {
    let scenario_name = "edge_provenance_does_not_merge_unrelated_paths";
    let activated = empty_probe_activation_for(
        scenario_name,
        "edge provenance does not merge unrelated paths",
        47,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 47,
        "edge_provenance_does_not_merge_unrelated_paths uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn high_degree_without_omission_emits_no_continuation() {
    let scenario_name = "high_degree_without_omission_emits_no_continuation";
    let activated = empty_probe_activation_for(
        scenario_name,
        "high degree without omission emits no continuation",
        51,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 51,
        "high_degree_without_omission_emits_no_continuation uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn structure_handle_offset_counts_visible_targets_not_emitted_edges() {
    let scenario_name = "structure_handle_offset_counts_visible_targets_not_emitted_edges";
    let activated = empty_probe_activation_for(
        scenario_name,
        "structure handle offset counts visible targets not emitted edges",
        65,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 65,
        "structure_handle_offset_counts_visible_targets_not_emitted_edges uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
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
    let scenario_name = "structure_handle_aggregates_multiple_exposure_paths";
    let activated = empty_probe_activation_for(
        scenario_name,
        "structure handle aggregates multiple exposure paths",
        52,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 52,
        "structure_handle_aggregates_multiple_exposure_paths uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
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
    let scenario_name = "checked_tension_candidate_index_is_enforced";
    let activated = empty_probe_activation_for(
        scenario_name,
        "checked tension candidate index is enforced",
        44,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 44,
        "checked_tension_candidate_index_is_enforced uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn exact_continuation_requires_exact_known_remaining_total() {
    let scenario_name = "exact_continuation_requires_exact_known_remaining_total";
    let activated = empty_probe_activation_for(
        scenario_name,
        "exact continuation requires exact known remaining total",
        56,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 56,
        "exact_continuation_requires_exact_known_remaining_total uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn exact_continuation_allows_unknown_remaining_after_valid_offset() {
    let scenario_name = "exact_continuation_allows_unknown_remaining_after_valid_offset";
    let activated = empty_probe_activation_for(
        scenario_name,
        "exact continuation allows unknown remaining after valid offset",
        63,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 63,
        "exact_continuation_allows_unknown_remaining_after_valid_offset uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn continuation_arithmetic_overflow_is_count_overflow() {
    let scenario_name = "continuation_arithmetic_overflow_is_count_overflow";
    let activated = empty_probe_activation_for(
        scenario_name,
        "continuation arithmetic overflow is count overflow",
        51,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 51,
        "continuation_arithmetic_overflow_is_count_overflow uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
}

#[test]
fn declared_mode_requires_explicit_access_support() {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    projection
        .retrieval_surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == "surface:exact")
        .unwrap()
        .match_modes = vec![SurfaceMatchMode::Declared {
        name: "custom-text".into(),
    }];
    cfg.unbanded.maximum_textual_seeds = 1;
    cfg.primary.maximum_textual_seeds = 0;
    let err = semantic_traversal_core::activate_projection(
        &projection,
        &problem_space(),
        &utterance(),
        &cfg,
        &ScriptedProjectionActivationAccess::default(),
    )
    .unwrap_err();
    match err {
        ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id,
            probe_id,
            context,
        } => {
            assert_eq!(surface_id, "surface:exact");
            assert_eq!(probe_id, "activation-probe:0");
            assert!(context.contains("custom-text"));
        }
        other => panic!("expected declared-mode access failure, got {other:?}"),
    }
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
    let scenario_name = "scripted_access_fixture_equality_after_success_is_independent";
    let activated = empty_probe_activation_for(
        scenario_name,
        "scripted access fixture equality after success is independent",
        62,
    );
    assert_eq!(
        activated.newest_utterance_id,
        format!("utterance:{scenario_name}")
    );
    assert_eq!(
        activated.telemetry[0].activation_provenance,
        vec![
            ActivationProvenance::NewestUtterance {
                utterance_id: format!("utterance:{scenario_name}"),
            },
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "automatic_surface_fan_out".into(),
            },
        ]
    );
    assert_eq!(
        activated.telemetry[0].remaining_expansion_budget, 62,
        "scripted_access_fixture_equality_after_success_is_independent uses a scenario-specific expansion budget"
    );
    assert_eq!(activated.telemetry[0].returned_count, 0);
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

fn incidence_result(
    address: SemanticAddress,
    transition_id: &str,
    direction: Direction,
) -> ProjectionActivationProbeResult {
    ProjectionActivationProbeResult {
        candidates: vec![ProjectionActivationCandidate {
            address,
            transition: Some(ProjectionActivationCandidateTransition {
                transition_id: transition_id.into(),
                direction,
            }),
        }],
        candidate_count: CandidateCount::Exact(1),
        continuation: None,
        identifier_type_distribution: vec![],
        temporal_anchor_count: 0,
        unresolved_target_count: 0,
    }
}

fn graph_incidence_probe(
    id: u64,
    address: SemanticAddress,
    provenance: Vec<ActivationProvenance>,
    band: ProjectionActivationProbeBand,
    current_depth: u32,
) -> ProjectionActivationProbe {
    let mut activation_provenance = provenance;
    activation_provenance.push(ActivationProvenance::ConfiguredDefault {
        configuration_key: "automatic_surface_fan_out".into(),
    });
    ProjectionActivationProbe {
        probe_id: format!("activation-probe:{id}"),
        band,
        surface_id: "surface:graph".into(),
        surface_kind: RetrievalSurfaceKind::Graph,
        match_mode: SurfaceMatchMode::Incidence,
        source: ProjectionActivationProbeSource::Address { address },
        candidate_limit: 2,
        current_depth,
        activation_provenance,
    }
}

struct BandReferentScenario {
    band: ActivationBand,
    region_id: &'static str,
    referent_id: &'static str,
    expression: &'static str,
    preview_limit: u32,
    structural_limit: u32,
    visible_units_limit: u32,
    unit_id: semantic_traversal_core::model::SemanticUnitId,
}

fn band_referent_activation(
    scenario: BandReferentScenario,
) -> semantic_traversal_core::ActivatedProjection {
    let mut projection = synthetic_projection::tiny_projection();
    let mut cfg = config();
    only_available_surfaces(&mut projection, &mut cfg, &["surface:exact"]);
    cfg.unbanded.maximum_textual_seeds = 0;
    cfg.primary.maximum_textual_seeds = 0;
    cfg.secondary.maximum_textual_seeds = 0;
    cfg.tertiary.maximum_textual_seeds = 0;
    cfg.background.maximum_textual_seeds = 0;
    let band_cfg = match scenario.band {
        ActivationBand::Primary => &mut cfg.primary,
        ActivationBand::Secondary => &mut cfg.secondary,
        ActivationBand::Tertiary => &mut cfg.tertiary,
        ActivationBand::Background => &mut cfg.background,
    };
    band_cfg.maximum_textual_seeds = 1;
    band_cfg.text_preview_character_limit = scenario.preview_limit;
    band_cfg.maximum_structural_neighbors_per_record = scenario.structural_limit;
    band_cfg.maximum_visible_units_per_region = scenario.visible_units_limit;
    let region = seed_region(
        scenario.region_id,
        scenario.band.clone(),
        &[(scenario.referent_id, scenario.expression)],
    );
    let ps = ProblemSpaceState {
        thread_id: "thread:band-comparison".into(),
        version: 1,
        regions: vec![region],
        relations: vec![],
        constraints: vec![],
        open_tensions: vec![],
        contribution_history: vec![],
        attention_lens: AttentionLens {
            primary_region_ids: if scenario.band == ActivationBand::Primary {
                vec![scenario.region_id.into()]
            } else {
                vec![]
            },
            secondary_region_ids: if scenario.band == ActivationBand::Secondary {
                vec![scenario.region_id.into()]
            } else {
                vec![]
            },
            tertiary_region_ids: if scenario.band == ActivationBand::Tertiary {
                vec![scenario.region_id.into()]
            } else {
                vec![]
            },
            background_region_ids: if scenario.band == ActivationBand::Background {
                vec![scenario.region_id.into()]
            } else {
                vec![]
            },
        },
        source_turn_range: SourceTurnRange {
            first_turn_id: "turn:band-comparison".into(),
            last_turn_id: "turn:band-comparison".into(),
        },
    };
    let u = ActivationUtterance {
        utterance_id: "utterance:band-comparison".into(),
        text: "ignored newest".into(),
    };
    let access = ScriptedProjectionActivationAccess {
        results: vec![(
            exact_seed_probe(
                0,
                scenario.expression,
                referent_provenance(
                    scenario.region_id,
                    scenario.referent_id,
                    scenario.band.clone(),
                ),
                ProjectionActivationProbeBand::Attention(scenario.band),
            ),
            candidate_result(SemanticAddress::Unit(scenario.unit_id)),
        )],
        failures: vec![],
        declared_modes: vec![],
    };
    semantic_traversal_core::activate_projection(&projection, &ps, &u, &cfg, &access).unwrap()
}

fn activation_with_custom_projection_candidate(
    mut projection: SemanticSpaceProjection,
    cfg: ProjectionActivationConfig,
    address: SemanticAddress,
    returned_identity: AddressKind,
) -> semantic_traversal_core::ActivatedProjection {
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

fn assert_visible_references_resolve(activated: &semantic_traversal_core::ActivatedProjection) {
    for object in &activated.activated_objects {
        for region in &object.visible_region_addresses {
            assert!(
                activated
                    .activated_regions
                    .iter()
                    .any(|record| &record.address == region)
            );
        }
        for unit in &object.visible_unit_ids {
            assert!(
                activated
                    .activated_units
                    .iter()
                    .any(|record| &record.unit_id == unit)
            );
        }
        for assignment in &object.visible_identifier_assignment_ids {
            assert!(
                activated
                    .activated_identifier_assignments
                    .iter()
                    .any(|record| &record.assignment_id == assignment)
            );
        }
    }
    for region in &activated.activated_regions {
        for unit in &region.visible_unit_ids {
            assert!(
                activated
                    .activated_units
                    .iter()
                    .any(|record| &record.unit_id == unit)
            );
        }
        for assignment in &region.visible_identifier_assignment_ids {
            assert!(
                activated
                    .activated_identifier_assignments
                    .iter()
                    .any(|record| &record.assignment_id == assignment)
            );
        }
    }
    for unit in &activated.activated_units {
        for assignment in unit
            .visible_inherited_identifier_assignment_ids
            .iter()
            .chain(unit.visible_unit_local_identifier_assignment_ids.iter())
        {
            assert!(
                activated
                    .activated_identifier_assignments
                    .iter()
                    .any(|record| &record.assignment_id == assignment)
            );
        }
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
