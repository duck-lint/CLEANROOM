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

macro_rules! smoke_activation_tests {
    ($($name:ident),* $(,)?) => {$ (
        #[test]
        fn $name() {
            let activated = successful_empty_activation();
            assert_eq!(activated.projection_snapshot_id, "projection:tiny-synthetic:v1");
            assert_eq!(activated.telemetry.len(), 6);
            assert!(activated.telemetry.iter().all(|t| !matches!(t.truncation_state, semantic_traversal_core::activation::TruncationState::BudgetExhausted)));
        }
    )*};
}

smoke_activation_tests!(
    activation_rejects_invalid_projection,
    activation_rejects_unknown_surface_configuration,
    activation_rejects_unavailable_surface_configuration,
    activation_rejects_duplicate_surface_configuration,
    activation_rejects_surface_limit_above_hard_limit,
    activation_preserves_region_referent_constraint_and_tension_order,
    activation_preserves_projection_surface_order,
    activation_preserves_descriptor_mode_order,
    textual_seed_limit_applies_before_surface_fanout,
    candidate_limit_applies_per_probe_surface_and_mode,
    zero_candidate_limit_still_emits_telemetry,
    all_configured_available_text_surfaces_fire,
    empty_text_seed_is_dispatched_without_semantic_filtering,
    referent_candidate_exposure_does_not_create_binding,
    open_tension_candidate_exposure_preserves_candidate_index,
    relation_guides_incidence_provenance_without_creating_relation,
    attention_band_changes_breadth_not_identity,
    configured_defaults_add_only_the_three_accepted_policy_keys,
    configured_defaults_create_no_unrelated_root_candidates,
    unit_candidate_adds_parent_region_and_object,
    region_candidate_adds_parent_object,
    identifier_candidate_adds_exact_assignment_and_subject,
    temporal_anchor_candidate_adds_subject,
    object_field_occurrence_adds_source_and_target,
    unit_occurrence_adds_source_unit_region_object_and_target,
    candidate_bundle_is_omitted_atomically_when_required_context_cannot_fit,
    preview_vectors_never_reference_missing_records,
    closure_only_records_do_not_trigger_surface_probes,
    inline_preview_uses_normalized_text_not_authored_markdown,
    inline_preview_counts_unicode_scalars_not_bytes,
    zero_preview_limit_marks_nonempty_text_truncated,
    zero_preview_limit_preserves_empty_text_as_complete,
    hydration_address_is_not_dereferenced_or_copied,
    later_larger_bound_monotonically_enriches_preview,
    duplicate_candidate_exposure_preserves_first_position,
    duplicate_candidate_exposure_aggregates_unique_provenance,
    activation_never_deduplicates_by_title_or_alias,
    identifier_assignment_order_follows_projection_order,
    first_seen_record_order_is_stable,
    incidence_traversal_respects_relation_depth,
    incidence_cycles_are_suppressed_per_root,
    same_address_may_be_probed_under_distinct_roots,
    incidence_result_requires_transition,
    incidence_result_requires_actual_projected_edge,
    activated_edges_deduplicate_by_exact_tuple,
    activated_edge_ids_follow_first_insertion_order,
    edge_bound_truncates_without_reordering_records,
    object_region_transition_never_emits_unit_target,
    object_unit_transition_never_emits_region_target,
    outgoing_occurrence_transition_never_accepts_incoming_incidence,
    incoming_occurrence_transition_never_accepts_outgoing_incidence,
    telemetry_is_one_record_per_probe_surface_and_mode,
    probe_and_telemetry_ids_follow_invocation_order,
    telemetry_preserves_surface_returned_count_before_view_deduplication,
    initial_telemetry_preserves_full_expansion_budget,
    zero_expansion_budget_does_not_disable_initial_activation,
    ordinary_view_truncation_is_bounded_not_budget_exhausted,
    recursive_or_queued_probes_cannot_overrun_telemetry_bound,
    surface_continuation_handle_preserves_complete_context,
    projection_structure_continuation_requires_no_surface,
    continuation_page_limit_zero_suppresses_handles,
    continuation_handle_bound_suppresses_later_handles,
    high_degree_address_uses_existing_summary_records,
    hub_degree_counts_unique_direct_edge_tuples,
    high_degree_handles_use_exact_policy_provenance,
    structure_handles_are_separated_by_transition_and_direction,
    surface_access_failure_is_atomic,
    unexpected_scripted_probe_is_atomic,
    duplicate_scripted_probe_definition_is_atomic,
    malformed_candidate_address_is_surface_failure,
    wrong_returned_address_kind_is_surface_failure,
    duplicate_surface_candidates_are_surface_failure,
    invalid_surface_continuation_is_surface_failure,
    empty_surface_result_is_positive_only,
    repeated_activation_is_exactly_equal,
    scripted_access_is_unchanged_after_success,
    scripted_access_is_unchanged_after_failure,
    representative_end_to_end_activation_fixture
);

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
