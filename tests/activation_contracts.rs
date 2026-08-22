#[path = "../examples/schema_support/mod.rs"]
mod schema_support;

use std::{fs, path::PathBuf};

use semantic_traversal_core::{
    activation::*,
    model::{
        Direction, OccurrenceId, RecordProvenance, RetrievalSurfaceKind, SemanticAddress,
        SemanticObjectId, SemanticRegionAddress, SemanticUnitId, SourceSpan, TemporalAnchorId,
    },
    projection::{
        AuthoredBlockType, IdentifierValue, OccurrencePresentation, OccurrenceSource,
        ProjectionValidationStatus, SurfaceMatchMode, TemporalValue,
    },
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

fn obj() -> SemanticObjectId {
    SemanticObjectId::parse("11111111-1111-1111-1111-111111111111").unwrap()
}
fn unit() -> SemanticUnitId {
    SemanticUnitId::parse("unit:capital:1").unwrap()
}
fn occ() -> OccurrenceId {
    OccurrenceId::parse("occurrence:capital").unwrap()
}
fn anchor() -> TemporalAnchorId {
    TemporalAnchorId::parse("anchor:1867").unwrap()
}
fn region() -> SemanticRegionAddress {
    SemanticRegionAddress::parse(obj(), "Capital").unwrap()
}
fn addr() -> SemanticAddress {
    SemanticAddress::Object(obj())
}
fn prov() -> Vec<ActivationProvenance> {
    vec![
        ActivationProvenance::NewestUtterance {
            utterance_id: "utterance:1".into(),
        },
        ActivationProvenance::ProblemReferent {
            region_id: "region:chronology".into(),
            referent_id: "referent:capital".into(),
        },
        ActivationProvenance::OpenTensionCandidate {
            tension_id: "tension:dimension".into(),
            candidate_index: 1,
        },
    ]
}
fn record_prov() -> RecordProvenance {
    RecordProvenance::ObjectField {
        object_id: obj(),
        field_path: "frontmatter.title".into(),
    }
}
fn round<T>(v: &T) -> T
where
    T: Serialize + DeserializeOwned,
{
    serde_json::from_value(serde_json::to_value(v).unwrap()).unwrap()
}
fn band() -> ProjectionActivationBandConfig {
    ProjectionActivationBandConfig {
        maximum_textual_seeds: 1,
        maximum_structural_neighbors_per_record: 2,
        maximum_visible_units_per_region: 3,
        text_preview_character_limit: 40,
    }
}
fn config() -> ProjectionActivationConfig {
    ProjectionActivationConfig {
        configuration_snapshot_id: "configuration:1".into(),
        unbanded: band(),
        primary: band(),
        secondary: band(),
        tertiary: band(),
        background: band(),
        surface_limits: vec![ProjectionActivationSurfaceConfig {
            surface_id: "surface:exact".into(),
            unbanded_candidate_limit: 1,
            primary_candidate_limit: 2,
            secondary_candidate_limit: 3,
            tertiary_candidate_limit: 4,
            background_candidate_limit: 5,
        }],
        maximum_expansion_budget: 9,
        hub_degree_threshold: 10,
        maximum_initial_relation_depth: 1,
        continuation_page_limit: 10,
        maximum_activated_objects: 1,
        maximum_activated_regions: 1,
        maximum_activated_units: 1,
        maximum_activated_identifier_assignments: 1,
        maximum_activated_occurrences: 1,
        maximum_activated_temporal_anchors: 1,
        maximum_activated_edges: 1,
        maximum_telemetry_records: 1,
        maximum_continuation_handles: 1,
    }
}
fn assignment() -> ActivatedIdentifierAssignmentRecord {
    ActivatedIdentifierAssignmentRecord {
        assignment_id: "assignment:title".into(),
        identifier_name: "title".into(),
        subject: addr(),
        value: IdentifierValue::String("Capital".into()),
        record_provenance: record_prov(),
        available_surface_ids: vec!["surface:exact".into()],
        activation_provenance: prov(),
    }
}
fn occurrence() -> ActivatedOccurrenceRecord {
    ActivatedOccurrenceRecord {
        occurrence_id: occ(),
        source: OccurrenceSource::SemanticUnit { unit_id: unit() },
        authored_target_text: "Capital".into(),
        display_alias: Some("Capital".into()),
        resolved_target: addr(),
        presentation_mode: OccurrencePresentation::Link,
        direction: Direction::Outgoing,
        source_span: Some(SourceSpan {
            source: "capital.md".into(),
            start_byte: Some(1),
            end_byte: Some(8),
        }),
        available_surface_ids: vec!["surface:exact".into()],
        activation_provenance: prov(),
    }
}
fn temporal() -> ActivatedTemporalAnchorRecord {
    ActivatedTemporalAnchorRecord {
        anchor_id: anchor(),
        subject: addr(),
        value: TemporalValue::ExactYear(1867),
        record_provenance: record_prov(),
        available_surface_ids: vec!["surface:temporal".into()],
        activation_provenance: prov(),
    }
}
fn handle(origin: ContinuationOrigin) -> ContinuationHandle {
    handle_with_access(
        origin,
        ContinuationAccess::RetrievalSurface {
            surface_id: "surface:exact".into(),
            surface_kind: RetrievalSurfaceKind::Exact,
        },
    )
}
fn handle_with_access(
    origin: ContinuationOrigin,
    access: ContinuationAccess,
) -> ContinuationHandle {
    ContinuationHandle {
        handle_id: "handle:1".into(),
        projection_snapshot_id: "projection:1".into(),
        configuration_snapshot_id: "configuration:1".into(),
        problem_space_thread_id: "thread:1".into(),
        problem_space_version: 7,
        newest_utterance_id: "utterance:1".into(),
        origin,
        access,
        filters: vec![
            ContinuationFilter::ObjectClass {
                object_class: "concept".into(),
            },
            ContinuationFilter::Identifier {
                identifier_name: "title".into(),
                represented_value: Some(IdentifierValue::String("Capital".into())),
            },
        ],
        ordering: ContinuationOrdering::SurfaceDeclared {
            ordering_key: "surface-order".into(),
        },
        next_offset: 1,
        remaining_count: Some(2),
        next_page_limit: 10,
        activation_provenance: prov(),
    }
}
fn activated_projection() -> ActivatedProjection {
    ActivatedProjection {
        projection_snapshot_id: "projection:1".into(),
        configuration_snapshot_id: "configuration:1".into(),
        problem_space_thread_id: "thread:1".into(),
        problem_space_version: 7,
        newest_utterance_id: "utterance:1".into(),
        activated_objects: vec![ActivatedObjectRecord {
            object_id: obj(),
            title: "Capital".into(),
            aliases: vec!["Das Kapital".into()],
            object_class: "concept".into(),
            visible_region_addresses: vec![region()],
            visible_unit_ids: vec![unit()],
            visible_identifier_assignment_ids: vec!["assignment:title".into()],
            contained_region_count: 1,
            contained_unit_count: 1,
            incoming_occurrence_count: 0,
            outgoing_occurrence_count: 1,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: prov(),
        }],
        activated_regions: vec![ActivatedRegionRecord {
            address: region(),
            heading_path: vec!["Capital".into()],
            heading_identity: "heading:capital".into(),
            visible_identifier_assignment_ids: vec!["assignment:title".into()],
            visible_unit_ids: vec![unit()],
            contained_unit_count: 1,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: prov(),
        }],
        activated_units: vec![ActivatedUnitRecord {
            unit_id: unit(),
            parent_object_id: obj(),
            parent_region_address: region(),
            authored_block_type: AuthoredBlockType::Paragraph,
            heading_path: vec!["Capital".into()],
            visible_inherited_identifier_assignment_ids: vec!["assignment:title".into()],
            visible_unit_local_identifier_assignment_ids: vec![],
            text_preview: ActivatedTextPreview::Inline {
                text: "Capital preview".into(),
                truncated: true,
            },
            incoming_occurrence_count: 0,
            outgoing_occurrence_count: 1,
            temporal_anchor_count: 1,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: prov(),
        }],
        activated_identifier_assignments: vec![assignment()],
        activated_occurrences: vec![occurrence()],
        activated_temporal_anchors: vec![temporal()],
        edges: vec![ActivatedEdge {
            edge_id: "edge:1".into(),
            source: addr(),
            transition_id: "transition:contains".into(),
            direction: Direction::Outgoing,
            target: SemanticAddress::Unit(unit()),
            activation_provenance: prov(),
        }],
        telemetry: vec![ProjectionTelemetry {
            telemetry_id: "activation-telemetry:0".into(),
            probe_id: "activation-probe:0".into(),
            match_mode: SurfaceMatchMode::Literal,
            surface_kind: RetrievalSurfaceKind::Exact,
            surface_id: "surface:exact".into(),
            candidate_count: CandidateCount::Exact(1),
            current_depth: 0,
            maximum_depth: 1,
            returned_count: 1,
            remaining_expansion_budget: 9,
            truncation_state: TruncationState::Bounded,
            identifier_type_distribution: vec![CountByLabel {
                label: "title".into(),
                count: 1,
            }],
            temporal_anchor_count: 1,
            unresolved_target_count: 0,
            continuation_available: true,
            activation_provenance: prov(),
        }],
        continuation_handles: vec![handle(ContinuationOrigin::TextProbe {
            query_text: "Capital".into(),
            match_mode: SurfaceMatchMode::Literal,
        })],
    }
}

#[test]
fn activation_utterance_round_trip() {
    let u = ActivationUtterance {
        utterance_id: "utterance:1".into(),
        text: "Find capital".into(),
    };
    assert_eq!(round(&u), u);
    assert!(serde_json::from_value::<ActivationUtterance>(json!({"text":"x"})).is_err());
    assert!(
        serde_json::from_value::<ActivationUtterance>(
            json!({"utterance_id":"u","text":"x","extra":1})
        )
        .is_err()
    );
}
#[test]
fn projection_activation_config_round_trip() {
    let c = config();
    assert_eq!(round(&c), c);
}
#[test]
fn projection_activation_config_preserves_expansion_budget() {
    let c = config();
    assert_eq!(c.maximum_expansion_budget, 9);
    assert_eq!(round(&c).maximum_expansion_budget, 9);
}
#[test]
fn activation_config_requires_configuration_snapshot_identity() {
    let mut value = serde_json::to_value(config()).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .remove("configuration_snapshot_id");
    assert!(serde_json::from_value::<ProjectionActivationConfig>(value).is_err());
}
#[test]
fn activated_text_preview_inline_round_trip() {
    let preview = ActivatedTextPreview::Inline {
        text: "Capital preview".into(),
        truncated: true,
    };
    assert_eq!(round(&preview), preview);
    assert_eq!(
        serde_json::to_value(&preview).unwrap(),
        json!({"kind":"inline","text":"Capital preview","truncated":true})
    );
}
#[test]
fn activated_text_preview_unavailable_without_hydration_round_trip() {
    let preview = ActivatedTextPreview::UnavailableWithoutHydration;
    assert_eq!(round(&preview), preview);
    assert_eq!(
        serde_json::to_value(&preview).unwrap(),
        json!({"kind":"unavailable_without_hydration"})
    );
}
#[test]
fn activated_text_preview_distinguishes_empty_from_unavailable() {
    assert_ne!(
        ActivatedTextPreview::Inline {
            text: String::new(),
            truncated: false,
        },
        ActivatedTextPreview::UnavailableWithoutHydration
    );
}
#[test]
fn activated_projection_preserves_input_snapshot_identity() {
    let p = activated_projection();
    assert_eq!(p.projection_snapshot_id, "projection:1");
    assert_eq!(p.configuration_snapshot_id, "configuration:1");
    assert_eq!(p.problem_space_thread_id, "thread:1");
    assert_eq!(p.problem_space_version, 7);
    assert_eq!(p.newest_utterance_id, "utterance:1");
}
#[test]
fn activated_identifier_assignment_preserves_dual_provenance() {
    let v = serde_json::to_value(assignment()).unwrap();
    assert!(v.get("record_provenance").is_some());
    assert!(v.get("activation_provenance").is_some());
    assert_eq!(round(&assignment()), assignment());
}
#[test]
fn activated_occurrence_preserves_authored_and_canonical_structure() {
    let o = occurrence();
    assert_eq!(round(&o), o);
    assert_eq!(o.authored_target_text, "Capital");
    assert_eq!(o.resolved_target, addr());
}
#[test]
fn activated_temporal_anchor_preserves_material_provenance() {
    let t = temporal();
    assert_eq!(round(&t), t);
    assert!(matches!(
        t.record_provenance,
        RecordProvenance::ObjectField { .. }
    ));
}
#[test]
fn referent_exposure_provenance_is_not_a_binding_record() {
    let v = serde_json::to_value(ActivationProvenance::ProblemReferent {
        region_id: "region:chronology".into(),
        referent_id: "referent:capital".into(),
    })
    .unwrap();
    assert_eq!(
        v,
        json!({"kind":"problem_referent","region_id":"region:chronology","referent_id":"referent:capital"})
    );
    let p = serde_json::to_value(activated_projection()).unwrap();
    assert!(p.get("referent_binding").is_none());
    assert!(p.get("canonical_binding").is_none());
    assert!(p.get("problem_region_binding").is_none());
}
#[test]
fn open_tension_candidate_provenance_preserves_candidate_index() {
    let v = serde_json::to_value(ActivationProvenance::OpenTensionCandidate {
        tension_id: "tension:dimension".into(),
        candidate_index: 1,
    })
    .unwrap();
    assert_eq!(
        v,
        json!({"kind":"open_tension_candidate","tension_id":"tension:dimension","candidate_index":1})
    );
}
#[test]
fn activated_object_exposes_bounded_discovery_surfaces() {
    let o = &activated_projection().activated_objects[0];
    assert_eq!(o.aliases, ["Das Kapital"]);
    assert_eq!(o.visible_region_addresses.len(), 1);
    assert_eq!(o.visible_unit_ids.len(), 1);
}
#[test]
fn activated_region_exposes_heading_and_identifier_structure() {
    let r = &activated_projection().activated_regions[0];
    assert_eq!(r.heading_path, ["Capital"]);
    assert_eq!(r.heading_identity, "heading:capital");
    assert_eq!(r.visible_identifier_assignment_ids, ["assignment:title"]);
}
#[test]
fn projection_telemetry_preserves_probe_identity_and_match_mode() {
    let telemetry = &activated_projection().telemetry[0];
    assert_eq!(telemetry.telemetry_id, "activation-telemetry:0");
    assert_eq!(telemetry.probe_id, "activation-probe:0");
    assert_eq!(telemetry.match_mode, SurfaceMatchMode::Literal);
    assert_eq!(
        telemetry.remaining_expansion_budget,
        config().maximum_expansion_budget
    );
}
#[test]
fn activated_unit_marks_truncated_preview() {
    let u = &activated_projection().activated_units[0];
    assert_eq!(
        u.text_preview,
        ActivatedTextPreview::Inline {
            text: "Capital preview".into(),
            truncated: true,
        }
    );
    assert_eq!(u.authored_block_type, AuthoredBlockType::Paragraph);
}
#[test]
fn continuation_text_probe_round_trip() {
    let h = handle(ContinuationOrigin::TextProbe {
        query_text: "Capital".into(),
        match_mode: SurfaceMatchMode::Literal,
    });
    assert_eq!(round(&h), h);
}
#[test]
fn continuation_structural_neighbourhood_round_trip() {
    let h = handle(ContinuationOrigin::StructuralNeighbourhood {
        subject: addr(),
        transition_id: Some("t".into()),
        direction: Some(Direction::Incoming),
    });
    assert_eq!(round(&h), h);
}
#[test]
fn continuation_temporal_probe_round_trip() {
    let h = handle(ContinuationOrigin::TemporalProbe {
        start: Some(TemporalValue::ExactYear(1800)),
        end: Some(TemporalValue::ExactYear(1900)),
    });
    assert_eq!(round(&h), h);
}
#[test]
fn continuation_filters_preserve_declared_order() {
    let h = handle(ContinuationOrigin::TextProbe {
        query_text: "Capital".into(),
        match_mode: SurfaceMatchMode::Literal,
    });
    assert!(matches!(
        h.filters[0],
        ContinuationFilter::ObjectClass { .. }
    ));
    assert!(matches!(
        h.filters[1],
        ContinuationFilter::Identifier { .. }
    ));
}
#[test]
fn continuation_identifier_filter_preserves_typed_value() {
    let cases = [
        IdentifierValue::Integer(1867),
        IdentifierValue::Boolean(true),
        IdentifierValue::SemanticAddress(addr()),
        IdentifierValue::Strings(vec!["Capital".into(), "Volume I".into()]),
    ];

    for represented_value in cases {
        let filter = ContinuationFilter::Identifier {
            identifier_name: "typed".into(),
            represented_value: Some(represented_value.clone()),
        };
        assert_eq!(round(&filter), filter);

        let serialized = serde_json::to_value(&filter).unwrap();
        let typed_value = serialized
            .get("represented_value")
            .expect("represented value must serialize");
        if matches!(represented_value, IdentifierValue::Integer(_)) {
            assert_eq!(typed_value, &json!({"kind":"integer","value":1867}));
        }
        if matches!(represented_value, IdentifierValue::Boolean(_)) {
            assert_eq!(typed_value, &json!({"kind":"boolean","value":true}));
        }
    }
}
#[test]
fn continuation_projection_structure_access_round_trip() {
    let access = ContinuationAccess::ProjectionStructure;
    assert_eq!(round(&access), access);
    assert_eq!(
        serde_json::to_value(access).unwrap(),
        json!({"kind":"projection_structure"})
    );
}
#[test]
fn continuation_retrieval_surface_access_round_trip() {
    let access = ContinuationAccess::RetrievalSurface {
        surface_id: "surface:graph".into(),
        surface_kind: RetrievalSurfaceKind::Graph,
    };
    assert_eq!(round(&access), access);
    assert_eq!(
        serde_json::to_value(access).unwrap(),
        json!({"kind":"retrieval_surface","surface_id":"surface:graph","surface_kind":"graph"})
    );
}
#[test]
fn continuation_preserves_activation_input_context() {
    let handle = handle(ContinuationOrigin::TextProbe {
        query_text: "Capital".into(),
        match_mode: SurfaceMatchMode::Literal,
    });
    assert_eq!(handle.projection_snapshot_id, "projection:1");
    assert_eq!(handle.configuration_snapshot_id, "configuration:1");
    assert_eq!(handle.problem_space_thread_id, "thread:1");
    assert_eq!(handle.problem_space_version, 7);
    assert_eq!(handle.newest_utterance_id, "utterance:1");
}
#[test]
fn continuation_structural_neighbourhood_projection_structure_round_trip() {
    let handle = handle_with_access(
        ContinuationOrigin::StructuralNeighbourhood {
            subject: addr(),
            transition_id: Some("transition:contains".into()),
            direction: Some(Direction::Outgoing),
        },
        ContinuationAccess::ProjectionStructure,
    );
    assert_eq!(round(&handle), handle);
    assert_eq!(
        serde_json::to_value(&handle.access).unwrap(),
        json!({"kind":"projection_structure"})
    );
}
#[test]
fn continuation_structural_neighbourhood_retrieval_surface_round_trip() {
    let handle = handle_with_access(
        ContinuationOrigin::StructuralNeighbourhood {
            subject: addr(),
            transition_id: Some("transition:contains".into()),
            direction: Some(Direction::Outgoing),
        },
        ContinuationAccess::RetrievalSurface {
            surface_id: "surface:graph".into(),
            surface_kind: RetrievalSurfaceKind::Graph,
        },
    );
    assert_eq!(round(&handle), handle);
}
#[test]
fn activated_record_requires_record_provenance() {
    let mut value = serde_json::to_value(assignment()).unwrap();
    value.as_object_mut().unwrap().remove("record_provenance");
    assert!(serde_json::from_value::<ActivatedIdentifierAssignmentRecord>(value).is_err());
}
#[test]
fn activated_record_requires_activation_provenance() {
    let mut value = serde_json::to_value(occurrence()).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .remove("activation_provenance");
    assert!(serde_json::from_value::<ActivatedOccurrenceRecord>(value).is_err());
}
#[test]
fn activation_violation_categories_round_trip() {
    let values = vec![
        ProjectionActivationViolation::EmptyRequiredIdentity {
            field: "projection_snapshot_id".into(),
        },
        ProjectionActivationViolation::ProjectionNotValidated {
            status: ProjectionValidationStatus::Unvalidated,
        },
        ProjectionActivationViolation::RuntimeConfigurationSnapshotMismatch {
            expected_configuration_snapshot_id: "a".into(),
            actual_configuration_snapshot_id: "b".into(),
        },
        ProjectionActivationViolation::MissingAvailableSurfaceConfiguration {
            surface_id: "s".into(),
        },
        ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
            surface_id: "s".into(),
        },
        ProjectionActivationViolation::DuplicateSurfaceConfiguration {
            surface_id: "s".into(),
        },
        ProjectionActivationViolation::InvalidConfigurationValue { field: "f".into() },
        ProjectionActivationViolation::SurfaceCandidateLimitExceedsHardLimit {
            surface_id: "s".into(),
            requested: 2,
            hard_maximum: 1,
        },
        ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: "s".into(),
            probe_id: "activation-probe:0".into(),
            context: "scripted failure".into(),
        },
        ProjectionActivationViolation::DuplicateActivatedIdentity {
            kind: ActivatedRecordKind::Object,
            identity: "o".into(),
        },
        ProjectionActivationViolation::InvalidActivatedReference {
            context: "c".into(),
        },
        ProjectionActivationViolation::InvalidActivationProvenance {
            context: "c".into(),
        },
        ProjectionActivationViolation::InvalidContinuationHandle {
            handle_id: "h".into(),
            context: "c".into(),
        },
        ProjectionActivationViolation::InvalidTelemetry {
            surface_id: "s".into(),
            context: "c".into(),
        },
        ProjectionActivationViolation::ActivatedViewBoundExceeded {
            kind: ActivatedRecordKind::Unit,
            actual: 2,
            maximum: 1,
        },
        ProjectionActivationViolation::CountOverflow,
    ];
    for v in values {
        assert_eq!(round(&v), v);
    }
    assert!(
        serde_json::from_value::<ProjectionActivationViolation>(json!({"violation":"unknown"}))
            .is_err()
    );
}
#[test]
fn surface_access_failure_violation_round_trip() {
    let violation = ProjectionActivationViolation::SurfaceAccessFailed {
        surface_id: "surface:exact".into(),
        probe_id: "activation-probe:0".into(),
        context: "declared mode unavailable".into(),
    };
    assert_eq!(round(&violation), violation);
}
fn schema(name: &str) -> String {
    schema_support::generated_schemas()
        .into_iter()
        .find(|(n, _)| *n == name)
        .unwrap()
        .1
}
#[test]
fn activated_text_preview_schema_is_current() {
    assert_eq!(
        fs::read_to_string(PathBuf::from("schemas/activated-text-preview.schema.json")).unwrap(),
        schema("activated-text-preview.schema.json")
    );
}
#[test]
fn activated_projection_schema_is_current() {
    assert_eq!(
        fs::read_to_string(PathBuf::from("schemas/activated-projection.schema.json")).unwrap(),
        schema("activated-projection.schema.json")
    );
}
#[test]
fn new_activation_schemas_are_current() {
    for name in [
        "activation-utterance.schema.json",
        "activated-text-preview.schema.json",
        "projection-activation-config.schema.json",
        "projection-activation-violation.schema.json",
        "activated-identifier-assignment-record.schema.json",
        "activated-occurrence-record.schema.json",
        "activated-temporal-anchor-record.schema.json",
        "continuation-handle.schema.json",
    ] {
        assert_eq!(
            fs::read_to_string(PathBuf::from("schemas").join(name)).unwrap(),
            schema(name)
        );
    }
}
#[test]
fn invalid_activation_exchange_shapes_are_rejected() {
    assert!(
        serde_json::from_value::<ActivatedIdentifierAssignmentRecord>(json!({"assignment_id":"a"}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ActivationProvenance>(
            json!({"kind":"problem_referent","region_id":"r"})
        )
        .is_err()
    );
    assert!(serde_json::from_value::<ActivatedTextPreview>(json!({"kind":"bogus"})).is_err());
    assert!(
        serde_json::from_value::<ActivatedTextPreview>(json!({"kind":"inline","truncated":true}))
            .is_err()
    );
    assert!(
        serde_json::from_value::<ActivatedTextPreview>(json!({"kind":"inline","text":"x"}))
            .is_err()
    );
    assert!(serde_json::from_value::<ContinuationOrigin>(json!({"kind":"bogus"})).is_err());
    assert!(serde_json::from_value::<ContinuationFilter>(json!({"kind":"bogus"})).is_err());
    assert!(serde_json::from_value::<ContinuationAccess>(json!({"kind":"bogus"})).is_err());
    assert!(
        serde_json::from_value::<ContinuationAccess>(
            json!({"kind":"retrieval_surface","surface_kind":"graph"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<ContinuationAccess>(
            json!({"kind":"retrieval_surface","surface_id":"surface:graph"})
        )
        .is_err()
    );
    assert!(serde_json::from_value::<ContinuationOrdering>(json!({"kind":"bogus"})).is_err());

    for required_field in ["telemetry_id", "probe_id", "match_mode"] {
        let mut value = serde_json::to_value(&activated_projection().telemetry[0]).unwrap();
        value.as_object_mut().unwrap().remove(required_field);
        assert!(serde_json::from_value::<ProjectionTelemetry>(value).is_err());
    }

    let mut missing_budget = serde_json::to_value(config()).unwrap();
    missing_budget
        .as_object_mut()
        .unwrap()
        .remove("maximum_expansion_budget");
    assert!(serde_json::from_value::<ProjectionActivationConfig>(missing_budget).is_err());

    for required_field in ["surface_id", "probe_id"] {
        let mut value = serde_json::to_value(ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: "surface:exact".into(),
            probe_id: "activation-probe:0".into(),
            context: "failure".into(),
        })
        .unwrap();
        value.as_object_mut().unwrap().remove(required_field);
        assert!(serde_json::from_value::<ProjectionActivationViolation>(value).is_err());
    }

    let valid_handle = serde_json::to_value(handle(ContinuationOrigin::TextProbe {
        query_text: "Capital".into(),
        match_mode: SurfaceMatchMode::Literal,
    }))
    .unwrap();

    for required_field in [
        "problem_space_thread_id",
        "problem_space_version",
        "newest_utterance_id",
    ] {
        let mut value = valid_handle.clone();
        value.as_object_mut().unwrap().remove(required_field);
        assert!(serde_json::from_value::<ContinuationHandle>(value).is_err());
    }

    let mut old_shape = valid_handle;
    let object = old_shape.as_object_mut().unwrap();
    object.remove("access");
    object.insert("surface_id".into(), json!("surface:exact"));
    object.insert("surface_kind".into(), json!("exact"));
    assert!(serde_json::from_value::<ContinuationHandle>(old_shape).is_err());
}
