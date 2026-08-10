mod support;

use std::collections::HashSet;

use semantic_traversal_core::{
    SemanticObjectId,
    construction::join_region_by_exact_span,
    model::{
        AddressKind, Direction, RecordProvenance, RetrievalSurfaceKind, SemanticAddress, SourceSpan,
    },
    projection::{
        CoverageSemantics, IdentifierAssignmentMode, IdentifierRole, OccurrenceSource,
        SemanticUnitContent, StructuralTransitionOperation, SurfaceMatchMode, TemporalAffordance,
        TemporalValue,
    },
};

use support::synthetic_projection::{
    CLEO_OBJECT, HEADING_ONLY_OBJECT, JOURNAL_ONE_OBJECT, MARX_OBJECT, MCCARTHY_OBJECT, object,
    occurrence, region, tiny_projection,
};

#[test]
fn fixed_projection_identity_and_exact_fixture_inventory_are_stable() {
    let projection = tiny_projection();
    assert_eq!(
        projection.projection_snapshot_id,
        "projection:tiny-synthetic:v2"
    );
    assert_eq!(projection.ingest_identity, "ingest:tiny-synthetic:v2");
    assert_eq!(projection.schema_version, "v0.1.0");
    assert_eq!(
        projection.logical_hash,
        "sha256:tiny-synthetic-projection-v2"
    );
    assert_eq!(
        projection.corpus_snapshot_identity,
        "corpus:tiny-synthetic:v2"
    );
    assert_eq!(
        projection.configuration_snapshot_id,
        "configuration:tiny-synthetic:v1"
    );
    assert_eq!(projection.objects.len(), 6);
    assert_eq!(projection.regions.len(), 5);
    assert_eq!(projection.units.len(), 5);
    assert_eq!(projection.identifier_descriptors.len(), 9);
    assert_eq!(projection.identifier_assignments.len(), 16);
    assert_eq!(projection.occurrences.len(), 5);
    assert_eq!(projection.temporal_anchors.len(), 2);
    assert_eq!(projection.retrieval_surfaces.len(), 5);
    assert_eq!(projection.valid_transitions.len(), 21);
    assert_eq!(
        projection
            .units
            .iter()
            .flat_map(|unit| unit.transport_segments.iter())
            .count(),
        1
    );

    let marx = projection
        .objects
        .iter()
        .find(|object| object.title == "Capital")
        .unwrap();
    assert_eq!(
        marx.object_id,
        SemanticObjectId::parse(MARX_OBJECT).unwrap()
    );
    assert_ne!(marx.title, marx.object_id.to_string());
    assert_ne!(marx.filename, marx.object_id.to_string());
    assert_ne!(marx.canonical_path, marx.object_id.to_string());
    assert!(marx.aliases.iter().any(|alias| alias == "Das Kapital"));
    assert_ne!(
        marx.object_id,
        SemanticObjectId::parse(MCCARTHY_OBJECT).unwrap()
    );
    assert_eq!(marx.object_class, "source_material");
    for assignment_name in ["title", "creator", "note_type"] {
        let assignment = projection
            .identifier_assignments
            .iter()
            .find(|assignment| {
                assignment.subject == SemanticAddress::Object(marx.object_id.clone())
                    && assignment.identifier_name == assignment_name
            })
            .unwrap();
        assert_ne!(assignment.assignment_id, marx.object_id.to_string());
        let descriptor = projection
            .identifier_descriptors
            .iter()
            .find(|descriptor| descriptor.identifier_name == assignment_name)
            .unwrap();
        assert!(matches!(
            &descriptor.semantic_role,
            IdentifierRole::CanonicalNaming | IdentifierRole::ObjectClass
        ));
    }
}

#[test]
fn capital_unit_has_one_subordinate_transport_segment() {
    let projection = tiny_projection();
    let capital = projection
        .units
        .iter()
        .find(|unit| unit.unit_id.as_str() == "unit:capital:chapter-2:2")
        .unwrap();
    let [segment] = capital.transport_segments.as_slice() else {
        panic!("Capital fixture must contain exactly one transport segment")
    };
    assert_eq!(segment.segment_ordinal, 0);
    assert_eq!(segment.total_segments, 1);
    assert_eq!(segment.parent_unit_id, capital.unit_id);
    assert_eq!(
        segment.reconstruction_group,
        "reconstruction:capital:chapter-2:2"
    );
    assert_eq!(segment.segment_id.as_str(), "segment:capital:chapter-2:2:0");
    assert_ne!(segment.segment_id.as_str(), capital.unit_id.as_str());
    assert_eq!(
        projection
            .units
            .iter()
            .filter(|unit| unit.unit_id == segment.parent_unit_id)
            .count(),
        1
    );
    assert_eq!(
        projection
            .units
            .iter()
            .filter(|unit| unit.unit_id.as_str() == segment.segment_id.as_str())
            .count(),
        0
    );
}

#[test]
fn parent_and_region_inventories_agree_with_units_and_inherited_assignments() {
    let projection = tiny_projection();
    for unit in &projection.units {
        let parent = projection
            .objects
            .iter()
            .find(|object| object.object_id == unit.parent_object_id)
            .unwrap();
        let region = projection
            .regions
            .iter()
            .find(|region| region.address == unit.parent_region_address)
            .unwrap();
        assert!(parent.unit_ids.contains(&unit.unit_id));
        assert!(parent.region_addresses.contains(&region.address));
        assert!(region.contained_unit_ids.contains(&unit.unit_id));
        assert_eq!(
            unit.inherited_identifier_assignment_ids,
            region.inherited_identifier_assignment_ids
        );
        for assignment_id in &unit.inherited_identifier_assignment_ids {
            assert!(parent.identifier_assignment_ids.contains(assignment_id));
            let assignment = projection
                .identifier_assignments
                .iter()
                .find(|assignment| &assignment.assignment_id == assignment_id)
                .unwrap();
            assert_eq!(
                assignment.subject,
                SemanticAddress::Object(parent.object_id.clone())
            );
            assert!(
                matches!(&assignment.provenance, RecordProvenance::ObjectField { object_id, .. } if object_id == &parent.object_id)
            );
        }
    }
    for object in &projection.objects {
        for region_address in &object.region_addresses {
            assert!(
                projection
                    .regions
                    .iter()
                    .any(|region| &region.address == region_address)
            );
        }
        for unit_id in &object.unit_ids {
            assert!(projection.units.iter().any(|unit| &unit.unit_id == unit_id));
        }
    }

    let journal = projection
        .objects
        .iter()
        .find(|object| object.object_id.to_string() == JOURNAL_ONE_OBJECT)
        .unwrap();
    let assignment = projection
        .identifier_assignments
        .iter()
        .find(|assignment| assignment.assignment_id == "assignment:journal-one:book")
        .unwrap();
    assert!(
        journal
            .identifier_assignment_ids
            .contains(&assignment.assignment_id)
    );
    assert!(
        matches!(assignment.provenance, RecordProvenance::ObjectField { ref object_id, ref field_path } if object_id == &journal.object_id && field_path == "book_read_today")
    );
    assert!(
        projection
            .regions
            .iter()
            .find(|region| region.address.object_id == journal.object_id)
            .unwrap()
            .inherited_identifier_assignment_ids
            .contains(&assignment.assignment_id)
    );
    assert!(
        projection
            .units
            .iter()
            .find(|unit| unit.parent_object_id == journal.object_id)
            .unwrap()
            .inherited_identifier_assignment_ids
            .contains(&assignment.assignment_id)
    );
}

#[test]
fn every_fixture_reference_resolves() {
    let projection = tiny_projection();
    let assignment_ids: HashSet<_> = projection
        .identifier_assignments
        .iter()
        .map(|record| &record.assignment_id)
        .collect();
    let occurrence_ids: HashSet<_> = projection
        .occurrences
        .iter()
        .map(|record| &record.occurrence_id)
        .collect();
    let anchor_ids: HashSet<_> = projection
        .temporal_anchors
        .iter()
        .map(|record| &record.anchor_id)
        .collect();
    let surface_ids: HashSet<_> = projection
        .retrieval_surfaces
        .iter()
        .map(|record| &record.surface_id)
        .collect();
    let transition_ids: HashSet<_> = projection
        .valid_transitions
        .iter()
        .map(|record| &record.transition_id)
        .collect();

    for object in &projection.objects {
        assert!(
            object
                .identifier_assignment_ids
                .iter()
                .all(|id| assignment_ids.contains(id))
        );
        assert!(
            object
                .object_field_occurrence_ids
                .iter()
                .chain(&object.body_occurrence_ids)
                .chain(&object.incoming_occurrence_ids)
                .all(|id| occurrence_ids.contains(id))
        );
        assert!(
            object
                .temporal_anchor_ids
                .iter()
                .all(|id| anchor_ids.contains(id))
        );
        assert!(
            object
                .retrieval_surface_ids
                .iter()
                .all(|id| surface_ids.contains(id))
        );
    }
    for region in &projection.regions {
        assert!(
            region
                .inherited_identifier_assignment_ids
                .iter()
                .all(|id| assignment_ids.contains(id))
        );
        assert!(
            region
                .incoming_occurrence_ids
                .iter()
                .chain(&region.outgoing_occurrence_ids)
                .all(|id| occurrence_ids.contains(id))
        );
        assert!(
            region
                .retrieval_surface_ids
                .iter()
                .all(|id| surface_ids.contains(id))
        );
        assert!(region.child_region_addresses.iter().all(|address| {
            projection
                .regions
                .iter()
                .any(|candidate| &candidate.address == address)
        }));
        assert!(region.block_target_mappings.iter().all(|mapping| {
            projection
                .units
                .iter()
                .any(|unit| unit.unit_id == mapping.target_unit_id)
        }));
    }
    for unit in &projection.units {
        assert!(
            unit.inherited_identifier_assignment_ids
                .iter()
                .chain(&unit.unit_local_identifier_assignment_ids)
                .all(|id| assignment_ids.contains(id))
        );
        assert!(
            unit.outgoing_occurrence_ids
                .iter()
                .chain(&unit.incoming_occurrence_ids)
                .all(|id| occurrence_ids.contains(id))
        );
        assert!(
            unit.temporal_anchor_ids
                .iter()
                .all(|id| anchor_ids.contains(id))
        );
        assert!(
            unit.retrieval_surface_ids
                .iter()
                .all(|id| surface_ids.contains(id))
        );
    }
    for descriptor in &projection.identifier_descriptors {
        assert!(
            descriptor
                .retrieval_surface_ids
                .iter()
                .all(|id| surface_ids.contains(id))
        );
        assert!(
            descriptor
                .enabled_transition_ids
                .iter()
                .all(|id| transition_ids.contains(id))
        );
    }
    for transition in &projection.valid_transitions {
        assert!(
            transition
                .retrieval_surface_id
                .as_ref()
                .is_none_or(|id| surface_ids.contains(id))
        );
    }
}

#[test]
fn authored_occurrences_are_listed_by_source_and_target_and_body_markdown() {
    let projection = tiny_projection();
    for occurrence in &projection.occurrences {
        let authored_link = match &occurrence.display_alias {
            Some(alias) => format!("[[{}|{alias}]]", occurrence.authored_target_text),
            None => format!("[[{}]]", occurrence.authored_target_text),
        };
        match &occurrence.source {
            OccurrenceSource::ObjectField {
                object_id,
                field_path,
            } => {
                let object = projection
                    .objects
                    .iter()
                    .find(|object| &object.object_id == object_id)
                    .unwrap();
                assert!(
                    object
                        .object_field_occurrence_ids
                        .contains(&occurrence.occurrence_id)
                );
                assert_eq!(field_path, "book_read_today");
            }
            OccurrenceSource::SemanticUnit { unit_id } => {
                let unit = projection
                    .units
                    .iter()
                    .find(|unit| &unit.unit_id == unit_id)
                    .unwrap();
                assert!(
                    unit.outgoing_occurrence_ids
                        .contains(&occurrence.occurrence_id)
                );
                assert!(
                    projection
                        .objects
                        .iter()
                        .find(|object| object.object_id == unit.parent_object_id)
                        .unwrap()
                        .body_occurrence_ids
                        .contains(&occurrence.occurrence_id)
                );
                let SemanticUnitContent::Inline {
                    authored_markdown, ..
                } = &unit.content
                else {
                    panic!("fixture units must be inline")
                };
                assert!(authored_markdown.contains(&authored_link));
            }
            OccurrenceSource::SemanticRegion { region_address } => {
                let region = projection
                    .regions
                    .iter()
                    .find(|region| &region.address == region_address)
                    .unwrap();
                assert!(
                    region
                        .outgoing_occurrence_ids
                        .contains(&occurrence.occurrence_id)
                );
                let object = projection
                    .objects
                    .iter()
                    .find(|object| object.object_id == region_address.object_id)
                    .unwrap();
                assert!(
                    object
                        .body_occurrence_ids
                        .contains(&occurrence.occurrence_id)
                );
                let region_span = region
                    .source_span
                    .as_ref()
                    .expect("region-sourced occurrence requires a region span");
                let occurrence_span = occurrence
                    .source_span
                    .as_ref()
                    .expect("region-sourced occurrence requires an exact span");
                assert_eq!(region_span.source, occurrence_span.source);
                let region_start = region_span
                    .start_byte
                    .expect("region-sourced occurrence requires a region start");
                let region_end = region_span
                    .end_byte
                    .expect("region-sourced occurrence requires a region end");
                let occurrence_start = occurrence_span
                    .start_byte
                    .expect("region-sourced occurrence requires an occurrence start");
                let occurrence_end = occurrence_span
                    .end_byte
                    .expect("region-sourced occurrence requires an occurrence end");
                assert!(region_start <= occurrence_start);
                assert!(occurrence_end <= region_end);
            }
        }
        match occurrence
            .resolved_target
            .as_ref()
            .expect("synthetic occurrence is resolved")
        {
            SemanticAddress::Object(id) => assert!(
                projection
                    .objects
                    .iter()
                    .find(|object| &object.object_id == id)
                    .unwrap()
                    .incoming_occurrence_ids
                    .contains(&occurrence.occurrence_id)
            ),
            SemanticAddress::Region(address) => assert!(
                projection
                    .regions
                    .iter()
                    .find(|region| &region.address == address)
                    .unwrap()
                    .incoming_occurrence_ids
                    .contains(&occurrence.occurrence_id)
            ),
            SemanticAddress::Unit(id) => assert!(
                projection
                    .units
                    .iter()
                    .find(|unit| &unit.unit_id == id)
                    .unwrap()
                    .incoming_occurrence_ids
                    .contains(&occurrence.occurrence_id)
            ),
            _ => panic!("fixture occurrence target must be object, region, or unit"),
        }
    }
    assert!(!projection.occurrences.iter().any(|record| {
        record
            .occurrence_id
            .as_str()
            .starts_with("occurrence:capital:")
    }));
}

#[test]
fn actual_occurrence_topology_has_complete_typed_transition_possibilities() {
    let projection = tiny_projection();
    let has = |from, direction, to| {
        projection.valid_transitions.iter().any(|transition| {
            transition.from == from
                && transition.operation == StructuralTransitionOperation::Occurrence
                && transition.direction == direction
                && transition.to == to
        })
    };
    for occurrence in &projection.occurrences {
        let source_kind = match occurrence.source {
            OccurrenceSource::ObjectField { .. } => AddressKind::SemanticObject,
            OccurrenceSource::SemanticRegion { .. } => AddressKind::SemanticRegion,
            OccurrenceSource::SemanticUnit { .. } => AddressKind::SemanticUnit,
        };
        let target_kind = occurrence
            .resolved_target
            .as_ref()
            .expect("synthetic occurrence is resolved")
            .kind();
        assert!(has(
            source_kind.clone(),
            Direction::Outgoing,
            AddressKind::Occurrence
        ));
        assert!(has(
            AddressKind::Occurrence,
            Direction::Outgoing,
            target_kind.clone()
        ));
        assert!(has(
            target_kind,
            Direction::Incoming,
            AddressKind::Occurrence
        ));
        assert!(has(
            AddressKind::Occurrence,
            Direction::Incoming,
            source_kind
        ));
    }
    for expected in [
        AddressKind::SemanticObject,
        AddressKind::SemanticRegion,
        AddressKind::SemanticUnit,
    ] {
        assert!(projection.occurrences.iter().any(|record| {
            record
                .resolved_target
                .as_ref()
                .is_some_and(|target| target.kind() == expected)
        }));
    }
}

#[test]
fn heading_region_can_source_occurrence_without_manufacturing_a_unit() {
    let projection = tiny_projection();
    let source_object_id = SemanticObjectId::parse(HEADING_ONLY_OBJECT).unwrap();
    let source_object = projection
        .objects
        .iter()
        .find(|object| object.object_id == source_object_id)
        .unwrap();
    let region = projection
        .regions
        .iter()
        .find(|region| region.address.object_id == source_object_id)
        .unwrap();
    let occurrence_id = occurrence("occurrence:heading-only:capital");
    let occurrence = projection
        .occurrences
        .iter()
        .find(|record| record.occurrence_id == occurrence_id)
        .unwrap();

    assert!(region.contained_unit_ids.is_empty());
    assert!(region.block_target_mappings.is_empty());
    assert!(source_object.unit_ids.is_empty());
    assert!(!projection.units.iter().any(|unit| {
        unit.parent_region_address == region.address || unit.parent_object_id == source_object_id
    }));
    assert!(region.outgoing_occurrence_ids.contains(&occurrence_id));
    assert!(source_object.body_occurrence_ids.contains(&occurrence_id));
    assert_eq!(
        occurrence.source,
        OccurrenceSource::SemanticRegion {
            region_address: region.address.clone()
        }
    );
    let Some(SemanticAddress::Object(target_id)) = &occurrence.resolved_target else {
        panic!("dedicated region occurrence must target an object")
    };
    assert!(
        projection
            .objects
            .iter()
            .find(|object| &object.object_id == target_id)
            .unwrap()
            .incoming_occurrence_ids
            .contains(&occurrence_id)
    );

    let region_span = region
        .source_span
        .as_ref()
        .expect("region span represented");
    let occurrence_span = occurrence
        .source_span
        .as_ref()
        .expect("occurrence span represented");
    assert_eq!(region_span.source, occurrence_span.source);
    let region_start = region_span.start_byte.expect("region start represented");
    let region_end = region_span.end_byte.expect("region end represented");
    let occurrence_start = occurrence_span
        .start_byte
        .expect("occurrence start represented");
    let occurrence_end = occurrence_span
        .end_byte
        .expect("occurrence end represented");
    assert!(region_start <= occurrence_start);
    assert!(occurrence_end <= region_end);

    assert!(projection.valid_transitions.iter().any(|transition| {
        transition.transition_id == "transition:region-occurrence-outgoing"
            && transition.from == AddressKind::SemanticRegion
            && transition.operation == StructuralTransitionOperation::Occurrence
            && transition.direction == Direction::Outgoing
            && transition.to == AddressKind::Occurrence
    }));
    assert!(projection.valid_transitions.iter().any(|transition| {
        transition.transition_id == "transition:occurrence-region-source"
            && transition.from == AddressKind::Occurrence
            && transition.operation == StructuralTransitionOperation::Occurrence
            && transition.direction == Direction::Incoming
            && transition.to == AddressKind::SemanticRegion
    }));
}

#[test]
fn heading_target_join_uses_exact_span_and_fails_closed() {
    let object_id = object(MARX_OBJECT);
    let base = tiny_projection()
        .regions
        .into_iter()
        .find(|region| region.address.object_id == object_id)
        .expect("synthetic object has a region");
    let mut first = base.clone();
    first.address = region(&object_id, "duplicate-heading-first");
    first.heading_identity = "same-rendered-heading".into();
    first.heading_path = vec!["same-rendered-heading".into()];
    first.source_span = Some(SourceSpan {
        source: "synthetic.md".into(),
        start_byte: Some(10),
        end_byte: Some(20),
    });
    let mut second = base;
    second.address = region(&object_id, "duplicate-heading-second");
    second.heading_identity = "same-rendered-heading".into();
    second.heading_path = vec!["same-rendered-heading".into()];
    second.source_span = Some(SourceSpan {
        source: "synthetic.md".into(),
        start_byte: Some(30),
        end_byte: Some(40),
    });
    let regions = vec![first.clone(), second.clone()];
    let matched_span = second.source_span.clone().unwrap();

    let selected = join_region_by_exact_span(&regions, &object_id, &matched_span).unwrap();
    assert_eq!(selected, second.address);
    assert_ne!(selected, first.address);
    assert_eq!(first.heading_identity, second.heading_identity);
    assert_eq!(first.heading_path, second.heading_path);

    let zero = join_region_by_exact_span(&[], &object_id, &matched_span).unwrap_err();
    assert!(zero.to_string().contains("zero"));

    let multiple = join_region_by_exact_span(
        &[first.clone(), {
            let mut duplicate = second.clone();
            duplicate.source_span = first.source_span.clone();
            duplicate
        }],
        &object_id,
        first.source_span.as_ref().unwrap(),
    )
    .unwrap_err();
    assert!(multiple.to_string().contains("multiple"));

    let occurrence_id = occurrence("occurrence:synthetic:exact-span");
    let mut selected_region = regions
        .into_iter()
        .find(|region| region.address == selected)
        .unwrap();
    selected_region
        .incoming_occurrence_ids
        .push(occurrence_id.clone());
    assert!(
        selected_region
            .incoming_occurrence_ids
            .contains(&occurrence_id)
    );
}

#[test]
fn temporal_context_is_sourced_on_journals_and_not_directly_on_cleo() {
    let projection = tiny_projection();
    let temporal = "surface:temporal".to_owned();
    let cleo = SemanticObjectId::parse(CLEO_OBJECT).unwrap();
    let cleo_record = projection
        .objects
        .iter()
        .find(|object| object.object_id == cleo)
        .unwrap();
    assert!(cleo_record.temporal_anchor_ids.is_empty());
    assert!(!cleo_record.retrieval_surface_ids.contains(&temporal));
    assert!(
        !projection
            .identifier_assignments
            .iter()
            .any(
                |assignment| assignment.subject == SemanticAddress::Object(cleo.clone())
                    && assignment.identifier_name == "journal_entry_date"
            )
    );
    assert!(
        !projection
            .temporal_anchors
            .iter()
            .any(|anchor| anchor.subject == SemanticAddress::Object(cleo.clone()))
    );

    let relation = projection
        .identifier_descriptors
        .iter()
        .find(|descriptor| descriptor.identifier_name == "book_read_today")
        .unwrap();
    assert_eq!(
        relation.assignment_mode,
        IdentifierAssignmentMode::Relational
    );
    assert_eq!(relation.temporal_affordance, TemporalAffordance::None);
    assert!(!relation.retrieval_surface_ids.contains(&temporal));
    let date = projection
        .identifier_descriptors
        .iter()
        .find(|descriptor| descriptor.identifier_name == "journal_entry_date")
        .unwrap();
    assert_eq!(date.temporal_affordance, TemporalAffordance::CreatesAnchor);
    assert!(date.retrieval_surface_ids.contains(&temporal));
    assert!(
        date.enabled_transition_ids
            .contains(&"transition:temporal-anchor".to_owned())
    );

    for anchor in &projection.temporal_anchors {
        assert!(
            matches!(&anchor.value, TemporalValue::FullDate(value) if value.starts_with("2026-07-"))
        );
        assert!(
            matches!(&anchor.provenance, RecordProvenance::ObjectField { field_path, .. } if field_path == "journal_entry_date")
        );
        let SemanticAddress::Object(object_id) = &anchor.subject else {
            panic!("fixture anchors belong to dated journal objects")
        };
        let object = projection
            .objects
            .iter()
            .find(|object| &object.object_id == object_id)
            .unwrap();
        assert!(object.retrieval_surface_ids.contains(&temporal));
        assert!(
            projection
                .regions
                .iter()
                .find(|region| region.address.object_id == *object_id)
                .unwrap()
                .retrieval_surface_ids
                .contains(&temporal)
        );
        assert!(
            projection
                .units
                .iter()
                .find(|unit| unit.parent_object_id == *object_id)
                .unwrap()
                .retrieval_surface_ids
                .contains(&temporal)
        );
    }
    for object in &projection.objects {
        assert_eq!(
            object.retrieval_surface_ids.contains(&temporal),
            !object.temporal_anchor_ids.is_empty()
        );
    }
}

#[test]
fn descriptor_surfaces_and_transitions_are_consistent_with_capabilities() {
    let projection = tiny_projection();
    for descriptor in &projection.identifier_descriptors {
        for surface_id in &descriptor.retrieval_surface_ids {
            let surface = projection
                .retrieval_surfaces
                .iter()
                .find(|surface| &surface.surface_id == surface_id)
                .unwrap();
            assert!(
                surface
                    .visible_address_kinds
                    .contains(&AddressKind::Identifier)
            );
        }
        for transition_id in &descriptor.enabled_transition_ids {
            assert_eq!(
                projection
                    .valid_transitions
                    .iter()
                    .filter(|transition| &transition.transition_id == transition_id)
                    .count(),
                1
            );
        }
    }
    for surface in &projection.retrieval_surfaces {
        assert_eq!(
            projection
                .retrieval_surfaces
                .iter()
                .filter(|candidate| candidate.surface_id == surface.surface_id)
                .count(),
            1
        );
    }
    for transition in &projection.valid_transitions {
        assert_eq!(
            projection
                .valid_transitions
                .iter()
                .filter(|candidate| candidate.transition_id == transition.transition_id)
                .count(),
            1
        );
    }
}

#[test]
fn retrieval_coverage_is_explicitly_exhaustive_or_bounded() {
    let projection = tiny_projection();
    for kind in [
        RetrievalSurfaceKind::Exact,
        RetrievalSurfaceKind::Lexical,
        RetrievalSurfaceKind::Vector,
        RetrievalSurfaceKind::Graph,
        RetrievalSurfaceKind::Temporal,
    ] {
        let surface = projection
            .retrieval_surfaces
            .iter()
            .find(|surface| surface.kind == kind)
            .unwrap();
        assert!(surface.available);
        assert_eq!(surface.default_candidate_limit, 8);
        assert_eq!(surface.hard_candidate_limit, 32);
        assert!(surface.continuation_supported);
        assert!(surface.hydrates_to_semantic_units);
        assert_eq!(
            surface.technical_limitations,
            vec!["synthetic fixture only".to_owned()]
        );
        if kind == RetrievalSurfaceKind::Exact {
            assert_eq!(surface.coverage_semantics, CoverageSemantics::Exhaustive);
            assert!(surface.exhaustive_total_count_supported);
        } else {
            assert_eq!(surface.coverage_semantics, CoverageSemantics::Bounded);
            assert!(!surface.exhaustive_total_count_supported);
        }
    }
}

#[test]
fn every_retrieval_surface_declares_its_intended_fixture_capability() {
    let projection = tiny_projection();
    let expected = [
        (
            RetrievalSurfaceKind::Exact,
            SurfaceMatchMode::Literal,
            AddressKind::SemanticUnit,
            CoverageSemantics::Exhaustive,
            vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::Identifier,
            ],
            true,
        ),
        (
            RetrievalSurfaceKind::Lexical,
            SurfaceMatchMode::Terms,
            AddressKind::SemanticUnit,
            CoverageSemantics::Bounded,
            vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::Identifier,
            ],
            false,
        ),
        (
            RetrievalSurfaceKind::Vector,
            SurfaceMatchMode::NearestNeighbours,
            AddressKind::SemanticUnit,
            CoverageSemantics::Bounded,
            vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
            ],
            false,
        ),
        (
            RetrievalSurfaceKind::Graph,
            SurfaceMatchMode::Incidence,
            AddressKind::Occurrence,
            CoverageSemantics::Bounded,
            vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::Occurrence,
                AddressKind::Identifier,
            ],
            false,
        ),
        (
            RetrievalSurfaceKind::Temporal,
            SurfaceMatchMode::Temporal,
            AddressKind::SemanticUnit,
            CoverageSemantics::Bounded,
            vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::TemporalAnchor,
                AddressKind::Identifier,
            ],
            false,
        ),
    ];
    for (kind, mode, returned, coverage, visible, total_count) in expected {
        let surface = projection
            .retrieval_surfaces
            .iter()
            .find(|surface| surface.kind == kind)
            .unwrap();
        assert!(surface.available);
        assert_eq!(surface.match_modes, vec![mode]);
        assert_eq!(surface.returned_identity, returned);
        assert_eq!(surface.coverage_semantics, coverage);
        assert_eq!(surface.exhaustive_total_count_supported, total_count);
        assert_eq!(surface.default_candidate_limit, 8);
        assert_eq!(surface.hard_candidate_limit, 32);
        assert!(surface.continuation_supported);
        assert!(surface.hydrates_to_semantic_units);
        assert_eq!(
            surface.technical_limitations,
            vec!["synthetic fixture only".to_owned()]
        );
        assert_eq!(surface.visible_address_kinds, visible);
    }
}

#[test]
fn every_record_surface_membership_matches_named_surface_visibility() {
    let projection = tiny_projection();
    let check = |surface_ids: &[String], address_kind: AddressKind| {
        for surface_id in surface_ids {
            let surface = projection
                .retrieval_surfaces
                .iter()
                .find(|surface| &surface.surface_id == surface_id)
                .unwrap();
            assert!(surface.available);
            assert!(surface.visible_address_kinds.contains(&address_kind));
        }
    };
    for object in &projection.objects {
        check(&object.retrieval_surface_ids, AddressKind::SemanticObject);
    }
    for region in &projection.regions {
        check(&region.retrieval_surface_ids, AddressKind::SemanticRegion);
    }
    for unit in &projection.units {
        check(&unit.retrieval_surface_ids, AddressKind::SemanticUnit);
    }
}

#[test]
fn graph_returns_occurrences_and_represents_hydration_to_canonical_units() {
    let projection = tiny_projection();
    let graph = projection
        .retrieval_surfaces
        .iter()
        .find(|surface| surface.kind == RetrievalSurfaceKind::Graph)
        .unwrap();
    assert_eq!(graph.returned_identity, AddressKind::Occurrence);
    assert!(graph.hydrates_to_semantic_units);
    assert!(
        projection
            .valid_transitions
            .iter()
            .any(|transition| transition.from == AddressKind::Occurrence
                && transition.operation == StructuralTransitionOperation::Hydration
                && transition.to == AddressKind::SemanticUnit
                && transition.retrieval_surface_id.as_ref() == Some(&graph.surface_id))
    );
}
