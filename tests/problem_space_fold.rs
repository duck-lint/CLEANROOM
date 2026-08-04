mod support;
use semantic_traversal_core::problem_space::*;
use semantic_traversal_core::problem_space_fold::*;
use support::scripted_boundary::ScriptedBoundaryCase;

fn limits() -> ProblemSpaceFoldLimits {
    ProblemSpaceFoldLimits {
        max_total_declarations_per_contribution: 100,
        max_operational_regions: 20,
        max_active_relations: 20,
        max_open_tensions: 20,
        max_background_regions: 20,
    }
}
fn log(thread: &str) -> BoundaryContributionLog {
    BoundaryContributionLog {
        thread_id: thread.into(),
        entries: vec![],
    }
}
fn region(id: &str) -> ProblemRegion {
    ProblemRegion {
        region_id: id.into(),
        anchor_referents: vec![],
        relation_ids: vec!["stale".into()],
        local_constraint_ids: vec![],
        open_tension_ids: vec![],
        source_contribution_ids: vec![],
        persistence_state: RegionPersistenceState::Active,
        activation_band: ActivationBand::Primary,
        supersedes_region_id: None,
    }
}
fn contribution(id: &str, turn: &str, ops: Vec<RegionOperation>) -> BoundaryContribution {
    BoundaryContribution {
        contribution_id: id.into(),
        source_turn_id: turn.into(),
        source_utterance_id: format!("u-{turn}"),
        region_operations: ops,
        relation_operations: vec![],
        constraint_operations: vec![],
        tension_operations: vec![],
        attention_operations: vec![],
        preservation_declarations: vec![],
        release_declarations: vec![],
    }
}
fn first() -> ProblemSpaceFoldOutput {
    fold_boundary_contribution(
        None,
        &log("thread"),
        &contribution(
            "c1",
            "t1",
            vec![RegionOperation::Create {
                region: region("r1"),
            }],
        ),
        &limits(),
    )
    .unwrap()
}
fn continue_with(
    prior: &ProblemSpaceFoldOutput,
    c: BoundaryContribution,
) -> Result<ProblemSpaceFoldOutput, ProblemSpaceFoldViolation> {
    fold_boundary_contribution(Some(&prior.state), &prior.accepted_log, &c, &limits())
}

#[test]
fn fresh_thread_fold_creates_versioned_state_and_log() {
    let x = first();
    assert_eq!(x.state.version, 1);
    assert_eq!(x.accepted_log.entries.len(), 1);
    assert_eq!(x.state.source_turn_range.first_turn_id, "t1");
    assert_eq!(x.state.regions[0].relation_ids, Vec::<String>::new())
}
#[test]
fn scripted_boundary_inference_receives_exact_turn_inputs() {
    let c = contribution("c1", "t1", vec![]);
    let f = ScriptedBoundaryCase {
        expected_prior_version: None,
        expected_newest_utterance: "new".into(),
        expected_previous_turn: None,
        contribution: c.clone(),
    };
    assert_eq!(f.infer(None, "new", None), c)
}
#[test]
fn continuation_reinforces_one_region_without_duplicate_accumulation() {
    let a = first();
    let b = continue_with(
        &a,
        contribution(
            "c2",
            "t2",
            vec![
                RegionOperation::Reinforce {
                    region_id: "r1".into(),
                    reason: "declared".into(),
                },
                RegionOperation::Reinforce {
                    region_id: "r1".into(),
                    reason: "declared".into(),
                },
            ],
        ),
    )
    .unwrap();
    assert_eq!(b.state.regions.len(), 1);
    assert_eq!(b.state.regions[0].source_contribution_ids, vec!["c1", "c2"])
}
#[test]
fn region_operations_respect_declared_vector_order() {
    let c = contribution(
        "c1",
        "t1",
        vec![
            RegionOperation::Create {
                region: region("z"),
            },
            RegionOperation::Create {
                region: region("a"),
            },
        ],
    );
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert_eq!(
        x.state
            .regions
            .iter()
            .map(|r| r.region_id.as_str())
            .collect::<Vec<_>>(),
        vec!["z", "a"]
    )
}
#[test]
fn later_phases_can_reference_a_region_created_in_phase_one() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create {
            region: region("r"),
        }],
    );
    c.relation_operations.push(RelationOperation::Connect {
        relation: ProblemRelation {
            relation_id: "e".into(),
            source_region_id: "r".into(),
            relation_type: ProblemRelationType::Continuation,
            target_region_id: None,
            source_contribution_id: "c1".into(),
            lifecycle: RecordLifecycle::Active,
        },
    });
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert_eq!(x.state.regions[0].relation_ids, vec!["e"])
}
#[test]
fn late_phase_failure_leaves_prior_state_and_log_unchanged() {
    let a = first();
    let before = a.clone();
    let mut c = contribution(
        "c2",
        "t2",
        vec![RegionOperation::Create {
            region: region("r2"),
        }],
    );
    c.attention_operations.push(AttentionOperation {
        region_id: "missing".into(),
        band: ActivationBand::Primary,
    });
    assert!(continue_with(&a, c).is_err());
    assert_eq!(a, before)
}
#[test]
fn merge_marks_sources_superseded_without_automatic_transfer() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![
            RegionOperation::Create {
                region: region("a"),
            },
            RegionOperation::Create {
                region: region("b"),
            },
            RegionOperation::Merge {
                source_region_ids: vec!["a".into(), "b".into()],
                resulting_region: region("m"),
                reason: "declared".into(),
            },
        ],
    );
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert_eq!(
        x.state.regions[0].persistence_state,
        RegionPersistenceState::Superseded
    );
    assert!(x.state.regions[2].anchor_referents.is_empty());
    c.region_operations.clear()
}
#[test]
fn split_requires_two_declared_results_and_preserves_source_history() {
    let a = first();
    let mut one = region("x");
    one.supersedes_region_id = Some("r1".into());
    let e = continue_with(
        &a,
        contribution(
            "c2",
            "t2",
            vec![RegionOperation::Split {
                source_region_id: "r1".into(),
                resulting_regions: vec![one],
                reason: "x".into(),
            }],
        ),
    );
    assert!(matches!(
        e,
        Err(ProblemSpaceFoldViolation::InvalidSplitShape)
    ));

    let mut first_result = region("result-b");
    first_result.supersedes_region_id = Some("r1".into());
    let mut second_result = region("result-a");
    second_result.supersedes_region_id = Some("r1".into());
    let split = continue_with(
        &a,
        contribution(
            "c2",
            "t2",
            vec![RegionOperation::Split {
                source_region_id: "r1".into(),
                resulting_regions: vec![first_result, second_result],
                reason: "fully declared mechanical split".into(),
            }],
        ),
    )
    .unwrap();
    assert_eq!(
        split.state.regions[0].persistence_state,
        RegionPersistenceState::Superseded
    );
    assert_eq!(
        split.state.regions[0].source_contribution_ids,
        vec!["c1", "c2"]
    );
    assert_eq!(
        split
            .state
            .regions
            .iter()
            .map(|region| region.region_id.as_str())
            .collect::<Vec<_>>(),
        vec!["r1", "result-b", "result-a"]
    );
    for result in &split.state.regions[1..] {
        assert_eq!(result.source_contribution_ids, vec!["c2"]);
        assert!(result.anchor_referents.is_empty());
        assert!(result.relation_ids.is_empty());
        assert!(result.local_constraint_ids.is_empty());
        assert!(result.open_tension_ids.is_empty());
    }
}
#[test]
fn supersession_requires_an_explicit_replacement_region() {
    let a = first();
    let e = continue_with(
        &a,
        contribution(
            "c2",
            "t2",
            vec![RegionOperation::Supersede {
                region_id: "r1".into(),
                superseded_by_region_id: "absent".into(),
                reason: "x".into(),
            }],
        ),
    );
    assert!(e.is_err())
}
#[test]
fn retirement_requires_explicit_cleanup_of_active_references() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create {
            region: region("r"),
        }],
    );
    c.relation_operations.push(RelationOperation::Connect {
        relation: ProblemRelation {
            relation_id: "e".into(),
            source_region_id: "r".into(),
            relation_type: ProblemRelationType::Continuation,
            target_region_id: None,
            source_contribution_id: "c1".into(),
            lifecycle: RecordLifecycle::Active,
        },
    });
    let a = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert!(
        continue_with(
            &a,
            contribution(
                "c2",
                "t2",
                vec![RegionOperation::Retire {
                    region_id: "r".into(),
                    reason: "x".into()
                }]
            )
        )
        .is_err()
    )
}
fn relation_contribution(disconnect: bool) -> BoundaryContribution {
    let mut c = contribution(
        if disconnect { "c2" } else { "c1" },
        if disconnect { "t2" } else { "t1" },
        if disconnect {
            vec![]
        } else {
            vec![RegionOperation::Create {
                region: region("r1"),
            }]
        },
    );
    c.relation_operations = if disconnect {
        vec![RelationOperation::Disconnect {
            relation_id: "rel".into(),
            reason: "declared".into(),
        }]
    } else {
        vec![RelationOperation::Connect {
            relation: ProblemRelation {
                relation_id: "rel".into(),
                source_region_id: "r1".into(),
                relation_type: ProblemRelationType::Temporal,
                target_region_id: None,
                source_contribution_id: "c1".into(),
                lifecycle: RecordLifecycle::Active,
            },
        }]
    };
    c
}
#[test]
fn relation_connect_and_disconnect_rebuild_active_incidence() {
    let a = fold_boundary_contribution(None, &log("x"), &relation_contribution(false), &limits())
        .unwrap();
    assert_eq!(a.state.regions[0].relation_ids, vec!["rel"]);
    let b = continue_with(&a, relation_contribution(true)).unwrap();
    assert!(b.state.regions[0].relation_ids.is_empty())
}
fn constraint(id: &str, cid: &str, ids: Vec<&str>) -> ProblemConstraint {
    ProblemConstraint {
        constraint_id: id.into(),
        expression: "declared".into(),
        applicability: ProblemConstraintApplicability::Regions {
            region_ids: ids.into_iter().map(Into::into).collect(),
        },
        source_contribution_id: cid.into(),
        lifecycle: RecordLifecycle::Active,
    }
}
#[test]
fn constraint_applicability_rebuilds_shared_regional_incidence() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![
            RegionOperation::Create {
                region: region("a"),
            },
            RegionOperation::Create {
                region: region("b"),
            },
        ],
    );
    c.constraint_operations.push(ConstraintOperation::Add {
        constraint: constraint("q", "c1", vec!["a", "b"]),
    });
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert!(
        x.state
            .regions
            .iter()
            .all(|r| r.local_constraint_ids == vec!["q"])
    )
}
#[test]
fn whole_problem_space_constraints_never_enter_local_indexes() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create {
            region: region("a"),
        }],
    );
    let mut q = constraint("q", "c1", vec!["a"]);
    q.applicability = ProblemConstraintApplicability::WholeProblemSpace;
    c.constraint_operations
        .push(ConstraintOperation::Add { constraint: q });
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert!(x.state.regions[0].local_constraint_ids.is_empty())
}
#[test]
fn constraint_replacement_does_not_inherit_applicability() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![
            RegionOperation::Create {
                region: region("a"),
            },
            RegionOperation::Create {
                region: region("b"),
            },
        ],
    );
    c.constraint_operations.push(ConstraintOperation::Add {
        constraint: constraint("old", "c1", vec!["a"]),
    });
    let a = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    let mut d = contribution("c2", "t2", vec![]);
    d.constraint_operations.push(ConstraintOperation::Replace {
        prior_constraint_id: "old".into(),
        replacement: constraint("new", "c2", vec!["b"]),
        reason: "chronology correction".into(),
    });
    let b = continue_with(&a, d).unwrap();
    assert!(b.state.regions[0].local_constraint_ids.is_empty());
    assert_eq!(b.state.regions[1].local_constraint_ids, vec!["new"])
}
#[test]
fn open_tension_remains_explicit_without_runtime_resolution() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create {
            region: region("r1"),
        }],
    );
    c.tension_operations.push(TensionOperation::Open {
        tension: OpenTension {
            tension_id: "before".into(),
            region_id: "r1".into(),
            tension_type: OpenTensionType::CompetingFraming,
            unresolved_expression: Some("before".into()),
            candidate_bindings: vec!["reading".into(), "publication".into()],
            source_turn_id: "t1".into(),
            lifecycle: TensionLifecycle::Open,
        },
    });
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert_eq!(x.state.open_tensions[0].lifecycle, TensionLifecycle::Open);
    assert_eq!(
        x.state.open_tensions[0].candidate_bindings,
        vec!["reading", "publication"]
    );
    let replayed = replay_boundary_contribution_log(&x.accepted_log, &limits())
        .unwrap()
        .unwrap();
    assert_eq!(replayed.open_tensions[0], x.state.open_tensions[0]);
}
#[test]
fn tension_lifecycle_operations_are_declared_and_terminal() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create {
            region: region("r1"),
        }],
    );
    c.tension_operations.push(TensionOperation::Open {
        tension: OpenTension {
            tension_id: "q".into(),
            region_id: "r1".into(),
            tension_type: OpenTensionType::Contradiction,
            unresolved_expression: None,
            candidate_bindings: vec![],
            source_turn_id: "t1".into(),
            lifecycle: TensionLifecycle::Open,
        },
    });
    let a = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    let mut d = contribution("c2", "t2", vec![]);
    d.tension_operations = vec![
        TensionOperation::Resolve {
            tension_id: "q".into(),
            resolution: "declared".into(),
        },
        TensionOperation::Abandon {
            tension_id: "q".into(),
            reason: "again".into(),
        },
    ];
    assert!(continue_with(&a, d).is_err())
}
#[test]
fn attention_rebuilds_one_lens_without_changing_lifecycle() {
    let a = first();
    let mut c = contribution("c2", "t2", vec![]);
    c.attention_operations.push(AttentionOperation {
        region_id: "r1".into(),
        band: ActivationBand::Background,
    });
    let b = continue_with(&a, c).unwrap();
    assert_eq!(
        b.state.regions[0].persistence_state,
        RegionPersistenceState::Active
    );
    assert_eq!(b.state.attention_lens.background_region_ids, vec!["r1"])
}
#[test]
fn unresolved_primary_and_active_background_remain_valid() {
    let mut a = region("a");
    a.persistence_state = RegionPersistenceState::Unresolved;
    let mut b = region("b");
    b.activation_band = ActivationBand::Background;
    let x = fold_boundary_contribution(
        None,
        &log("x"),
        &contribution(
            "c1",
            "t1",
            vec![
                RegionOperation::Create { region: a },
                RegionOperation::Create { region: b },
            ],
        ),
        &limits(),
    )
    .unwrap();
    assert_eq!(x.state.attention_lens.primary_region_ids, vec!["a"]);
    assert_eq!(x.state.attention_lens.background_region_ids, vec!["b"])
}
#[test]
fn preservation_declarations_validate_without_mutating() {
    let a = first();
    let mut c = contribution("c2", "t2", vec![]);
    c.preservation_declarations.push(PreservationDeclaration {
        subject: ProblemSpaceSubject::Region("r1".into()),
        reason: "audit".into(),
    });
    let b = continue_with(&a, c).unwrap();
    assert_eq!(b.state.regions[0].source_contribution_ids, vec!["c1"])
}
#[test]
fn release_declarations_require_matching_typed_operations() {
    let a = first();
    let mut c = contribution("c2", "t2", vec![]);
    c.release_declarations.push(ReleaseDeclaration {
        subject: ProblemSpaceSubject::Region("r1".into()),
        mode: ReleaseMode::Retire,
        reason: "claim only".into(),
    });
    assert!(matches!(
        continue_with(&a, c),
        Err(ProblemSpaceFoldViolation::InvalidReleaseDeclaration)
    ))
}
#[test]
fn duplicate_accepted_identities_are_rejected() {
    let a = first();
    let c = contribution("c1", "t2", vec![]);
    assert!(matches!(
        continue_with(&a, c),
        Err(ProblemSpaceFoldViolation::DuplicateAcceptedIdentity { .. })
    ))
}
#[test]
fn state_log_version_and_history_mismatches_are_rejected() {
    let a = first();
    let mut state = a.state.clone();
    state.version = 2;
    assert!(matches!(
        fold_boundary_contribution(
            Some(&state),
            &a.accepted_log,
            &contribution("c2", "t2", vec![]),
            &limits()
        ),
        Err(ProblemSpaceFoldViolation::StateVersionLogLengthMismatch)
    ))
}
#[test]
fn configured_bounds_reject_without_silent_removal() {
    let mut l = limits();
    l.max_operational_regions = 0;
    assert!(matches!(
        fold_boundary_contribution(
            None,
            &log("x"),
            &contribution(
                "c1",
                "t1",
                vec![RegionOperation::Create {
                    region: region("r")
                }]
            ),
            &l
        ),
        Err(ProblemSpaceFoldViolation::ConfiguredFinalStateBoundExcess { .. })
    ))
}
#[test]
fn accepted_log_replay_reconstructs_exact_final_state() {
    let a = first();
    let b = continue_with(
        &a,
        contribution(
            "c2",
            "t2",
            vec![RegionOperation::Reinforce {
                region_id: "r1".into(),
                reason: "calf continuation".into(),
            }],
        ),
    )
    .unwrap();
    assert_eq!(
        replay_boundary_contribution_log(&b.accepted_log, &limits()).unwrap(),
        Some(b.state)
    )
}
#[test]
fn serialized_log_restart_reconstructs_exact_final_state() {
    let a = first();
    let json = serde_json::to_string(&a.accepted_log).unwrap();
    let restored: BoundaryContributionLog = serde_json::from_str(&json).unwrap();
    assert_eq!(
        replay_boundary_contribution_log(&restored, &limits()).unwrap(),
        Some(a.state)
    )
}
#[test]
fn separate_threads_do_not_share_state() {
    let a = fold_boundary_contribution(
        None,
        &log("a"),
        &contribution(
            "ca",
            "ta",
            vec![RegionOperation::Create {
                region: region("ra"),
            }],
        ),
        &limits(),
    )
    .unwrap();
    let b = fold_boundary_contribution(
        None,
        &log("b"),
        &contribution(
            "cb",
            "tb",
            vec![RegionOperation::Create {
                region: region("rb"),
            }],
        ),
        &limits(),
    )
    .unwrap();
    assert_ne!(a.state.thread_id, b.state.thread_id);
    assert_ne!(a.state.regions, b.state.regions)
}
#[test]
fn cross_thread_state_and_log_pairing_is_rejected() {
    let a = first();
    assert!(matches!(
        fold_boundary_contribution(
            Some(&a.state),
            &log("other"),
            &contribution("c2", "t2", vec![]),
            &limits()
        ),
        Err(ProblemSpaceFoldViolation::InvalidFreshStateLogCombination
            | ProblemSpaceFoldViolation::ThreadMismatch)
    ))
}
#[test]
fn contribution_history_categories_are_canonical_and_deduplicated() {
    let mut c = contribution(
        "c1",
        "t1",
        vec![
            RegionOperation::Create {
                region: region("r1"),
            },
            RegionOperation::Reinforce {
                region_id: "r1".into(),
                reason: "x".into(),
            },
            RegionOperation::Reinforce {
                region_id: "r1".into(),
                reason: "y".into(),
            },
        ],
    );
    c.attention_operations.push(AttentionOperation {
        region_id: "r1".into(),
        band: ActivationBand::Primary,
    });
    let x = fold_boundary_contribution(None, &log("x"), &c, &limits()).unwrap();
    assert_eq!(
        x.state.contribution_history[0].transformations,
        vec![
            BoundaryOperationKind::Create,
            BoundaryOperationKind::Reinforce,
            BoundaryOperationKind::RedirectAttention
        ]
    )
}
#[test]
fn source_turn_range_advances_once_per_accepted_contribution() {
    let a = first();
    let b = continue_with(&a, contribution("c2", "t2", vec![])).unwrap();
    assert_eq!(
        b.state.source_turn_range,
        SourceTurnRange {
            first_turn_id: "t1".into(),
            last_turn_id: "t2".into()
        }
    );
    assert_eq!(b.state.version, 2)
}
#[test]
fn failed_contribution_never_enters_history_or_accepted_log() {
    let a = first();
    let c = contribution(
        "c2",
        "t2",
        vec![RegionOperation::Retire {
            region_id: "missing".into(),
            reason: "x".into(),
        }],
    );
    assert!(continue_with(&a, c).is_err());
    assert_eq!(a.state.contribution_history.len(), 1);
    assert_eq!(a.accepted_log.entries.len(), 1)
}

#[test]
fn prior_state_must_equal_replay_of_accepted_log() {
    let accepted = first();
    let original_state = accepted.state.clone();
    let original_log = accepted.accepted_log.clone();
    let mut forged = accepted.state.clone();
    let mut extra = region("structurally-plausible-extra");
    extra.source_contribution_ids = vec!["c1".into()];
    forged.regions.push(extra);
    forged
        .attention_lens
        .primary_region_ids
        .push("structurally-plausible-extra".into());

    assert!(matches!(
        fold_boundary_contribution(
            Some(&forged),
            &accepted.accepted_log,
            &contribution("c2", "t2", vec![]),
            &limits(),
        ),
        Err(ProblemSpaceFoldViolation::PriorStateReplayMismatch)
    ));
    assert_eq!(accepted.state, original_state);
    assert_eq!(accepted.accepted_log, original_log);
}

#[test]
fn duplicate_region_contribution_provenance_is_rejected() {
    let mut declared = region("r");
    declared.source_contribution_ids = vec!["c1".into(), "c1".into()];
    assert!(matches!(
        fold_boundary_contribution(
            None,
            &log("thread"),
            &contribution("c1", "t1", vec![RegionOperation::Create { region: declared }]),
            &limits(),
        ),
        Err(ProblemSpaceFoldViolation::DuplicateRegionContributionProvenance {
            region_id,
            contribution_id
        }) if region_id == "r" && contribution_id == "c1"
    ));
}

#[test]
fn required_identities_reject_before_lookup() {
    let empty_referent = ProblemReferent {
        referent_id: "referent".into(),
        expression: "declared".into(),
        source_contribution_id: String::new(),
    };
    let empty_source_region_relation = ProblemRelation {
        relation_id: "relation".into(),
        source_region_id: String::new(),
        relation_type: ProblemRelationType::Continuation,
        target_region_id: None,
        source_contribution_id: "c1".into(),
        lifecycle: RecordLifecycle::Active,
    };
    let empty_source_contribution_relation = ProblemRelation {
        source_region_id: "r".into(),
        source_contribution_id: String::new(),
        ..empty_source_region_relation.clone()
    };
    let empty_tension = |empty_region: bool| OpenTension {
        tension_id: "tension".into(),
        region_id: if empty_region {
            String::new()
        } else {
            "r".into()
        },
        tension_type: OpenTensionType::CompetingFraming,
        unresolved_expression: None,
        candidate_bindings: vec![],
        source_turn_id: if empty_region {
            "t1".into()
        } else {
            String::new()
        },
        lifecycle: TensionLifecycle::Open,
    };
    let mut cases = Vec::new();
    for relation in [
        empty_source_region_relation,
        empty_source_contribution_relation,
    ] {
        let mut c = contribution("c1", "t1", vec![]);
        c.relation_operations
            .push(RelationOperation::Connect { relation });
        cases.push(c);
    }
    let mut regional = contribution("c1", "t1", vec![]);
    regional
        .constraint_operations
        .push(ConstraintOperation::Add {
            constraint: constraint("q", "c1", vec![""]),
        });
    cases.push(regional);
    for tension in [empty_tension(true), empty_tension(false)] {
        let mut c = contribution("c1", "t1", vec![]);
        c.tension_operations
            .push(TensionOperation::Open { tension });
        cases.push(c);
    }
    cases.push(contribution(
        "c1",
        "t1",
        vec![RegionOperation::Supersede {
            region_id: "r".into(),
            superseded_by_region_id: String::new(),
            reason: "declared".into(),
        }],
    ));
    let mut bad_region = region("r");
    bad_region.source_contribution_ids.push(String::new());
    cases.push(contribution(
        "c1",
        "t1",
        vec![RegionOperation::Create { region: bad_region }],
    ));
    cases.push(contribution(
        "c1",
        "t1",
        vec![RegionOperation::Extend {
            region_id: "r".into(),
            referent: empty_referent,
        }],
    ));
    let mut preservation = contribution("c1", "t1", vec![]);
    preservation
        .preservation_declarations
        .push(PreservationDeclaration {
            subject: ProblemSpaceSubject::Region(String::new()),
            reason: "declared".into(),
        });
    cases.push(preservation);
    let mut release = contribution("c1", "t1", vec![]);
    release.release_declarations.push(ReleaseDeclaration {
        subject: ProblemSpaceSubject::Relation(String::new()),
        mode: ReleaseMode::Retire,
        reason: "declared".into(),
    });
    cases.push(release);

    for case in cases {
        assert!(matches!(
            fold_boundary_contribution(None, &log("thread"), &case, &limits()),
            Err(ProblemSpaceFoldViolation::EmptyRequiredIdentity { .. })
        ));
    }
}

#[test]
fn relation_supersession_release_is_unsupported() {
    let connected = fold_boundary_contribution(
        None,
        &log("thread"),
        &relation_contribution(false),
        &limits(),
    )
    .unwrap();
    let mut retired = relation_contribution(true);
    retired.release_declarations.push(ReleaseDeclaration {
        subject: ProblemSpaceSubject::Relation("rel".into()),
        mode: ReleaseMode::Retire,
        reason: "declared disconnect".into(),
    });
    let retired = continue_with(&connected, retired).unwrap();
    assert_eq!(
        retired.state.relations[0].lifecycle,
        RecordLifecycle::Retired
    );

    let mut unsupported = contribution("c2", "t2", vec![]);
    unsupported.release_declarations.push(ReleaseDeclaration {
        subject: ProblemSpaceSubject::Relation("rel".into()),
        mode: ReleaseMode::Supersede,
        reason: "no typed operation exists".into(),
    });
    assert!(matches!(
        continue_with(&connected, unsupported),
        Err(ProblemSpaceFoldViolation::UnsupportedSubjectReleaseModeCombination)
    ));
}

#[test]
fn chronology_correction_is_explicit_and_retains_superseded_history() {
    let mut initial = contribution(
        "chronology-c1",
        "chronology-t1",
        vec![RegionOperation::Create {
            region: region("reading-chronology"),
        }],
    );
    initial
        .constraint_operations
        .push(ConstraintOperation::Add {
            constraint: constraint("reading-order", "chronology-c1", vec!["reading-chronology"]),
        });
    let initial =
        fold_boundary_contribution(None, &log("chronology-thread"), &initial, &limits()).unwrap();

    let mut correction = contribution(
        "chronology-c2",
        "chronology-t2",
        vec![
            RegionOperation::Create {
                region: region("publication-chronology"),
            },
            RegionOperation::Supersede {
                region_id: "reading-chronology".into(),
                superseded_by_region_id: "publication-chronology".into(),
                reason: "publication rather than reading chronology".into(),
            },
        ],
    );
    correction
        .constraint_operations
        .push(ConstraintOperation::Replace {
            prior_constraint_id: "reading-order".into(),
            replacement: constraint(
                "publication-order",
                "chronology-c2",
                vec!["publication-chronology"],
            ),
            reason: "explicit corrected applicability".into(),
        });
    correction.release_declarations = vec![
        ReleaseDeclaration {
            subject: ProblemSpaceSubject::Region("reading-chronology".into()),
            mode: ReleaseMode::Supersede,
            reason: "declared framing replacement".into(),
        },
        ReleaseDeclaration {
            subject: ProblemSpaceSubject::Constraint("reading-order".into()),
            mode: ReleaseMode::Supersede,
            reason: "declared constraint replacement".into(),
        },
    ];
    let corrected = continue_with(&initial, correction).unwrap();
    assert_eq!(
        corrected.state.regions[0].persistence_state,
        RegionPersistenceState::Superseded
    );
    assert_eq!(
        corrected.state.constraints[0].lifecycle,
        RecordLifecycle::Superseded
    );
    assert_eq!(
        corrected.state.regions[1].region_id,
        "publication-chronology"
    );
    assert_eq!(
        corrected.state.constraints[1].applicability,
        ProblemConstraintApplicability::Regions {
            region_ids: vec!["publication-chronology".into()]
        }
    );
    assert!(corrected.state.regions[0].local_constraint_ids.is_empty());
    assert_eq!(
        corrected.state.regions[1].local_constraint_ids,
        vec!["publication-order"]
    );
}

#[test]
fn calf_continuation_uses_only_scripted_relation_and_attention_changes() {
    let mut first_declaration = contribution(
        "calf-c1",
        "calf-t1",
        vec![RegionOperation::Create {
            region: region("calf-diet"),
        }],
    );
    first_declaration
        .relation_operations
        .push(RelationOperation::Connect {
            relation: ProblemRelation {
                relation_id: "calf-food-relation".into(),
                source_region_id: "calf-diet".into(),
                relation_type: ProblemRelationType::Declared {
                    name: "food".into(),
                },
                target_region_id: None,
                source_contribution_id: "calf-c1".into(),
                lifecycle: RecordLifecycle::Active,
            },
        });
    let first =
        fold_boundary_contribution(None, &log("calf-thread"), &first_declaration, &limits())
            .unwrap();
    let mut second_declaration = contribution(
        "calf-c2",
        "calf-t2",
        vec![
            RegionOperation::Reinforce {
                region_id: "calf-diet".into(),
                reason: "script declares retained identity".into(),
            },
            RegionOperation::Reinforce {
                region_id: "calf-diet".into(),
                reason: "repeated touch must not duplicate provenance".into(),
            },
        ],
    );
    second_declaration.relation_operations = vec![
        RelationOperation::Disconnect {
            relation_id: "calf-food-relation".into(),
            reason: "explicitly replace active relation".into(),
        },
        RelationOperation::Connect {
            relation: ProblemRelation {
                relation_id: "calf-temporal-relation".into(),
                source_region_id: "calf-diet".into(),
                relation_type: ProblemRelationType::Temporal,
                target_region_id: None,
                source_contribution_id: "calf-c2".into(),
                lifecycle: RecordLifecycle::Active,
            },
        },
    ];
    second_declaration
        .attention_operations
        .push(AttentionOperation {
            region_id: "calf-diet".into(),
            band: ActivationBand::Secondary,
        });
    let scripted = ScriptedBoundaryCase {
        expected_prior_version: Some(1),
        expected_newest_utterance: "When did that change?".into(),
        expected_previous_turn: Some("What did the calf eat?".into()),
        contribution: second_declaration.clone(),
    };
    let inferred = scripted.infer(
        Some(&first.state),
        "When did that change?",
        Some("What did the calf eat?"),
    );
    assert_eq!(inferred, second_declaration);
    let second = continue_with(&first, inferred).unwrap();
    assert_eq!(second.state.regions.len(), 1);
    assert_eq!(second.state.regions[0].region_id, "calf-diet");
    assert_eq!(
        second.state.regions[0].source_contribution_ids,
        vec!["calf-c1", "calf-c2"]
    );
    assert_eq!(
        second.state.regions[0].relation_ids,
        vec!["calf-temporal-relation"]
    );
    assert_eq!(
        second.state.attention_lens.secondary_region_ids,
        vec!["calf-diet"]
    );
}
