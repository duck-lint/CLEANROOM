mod support;

use semantic_traversal_core::{
    SemanticObjectId,
    model::{AddressKind, Direction, RecordProvenance, SemanticAddress, RetrievalSurfaceKind},
    projection::{CoverageSemantics, StructuralTransitionOperation, TemporalValue},
};

use support::synthetic_projection::{
    CLEO_OBJECT, JOURNAL_ONE_OBJECT, MARX_OBJECT, MCCARTHY_OBJECT, tiny_projection,
};

#[test]
fn fixed_projection_identity_and_canonical_object_identity_are_stable() {
    let projection = tiny_projection();
    assert_eq!(projection.projection_snapshot_id, "projection:tiny-synthetic:v1");
    assert_eq!(projection.ingest_identity, "ingest:tiny-synthetic:v1");
    assert_eq!(projection.schema_version, "v0.1.0");
    assert_eq!(projection.logical_hash, "sha256:tiny-synthetic-projection-v1");
    assert_eq!(projection.corpus_snapshot_identity, "corpus:tiny-synthetic:v1");
    assert_eq!(projection.configuration_snapshot_id, "configuration:tiny-synthetic:v1");
    let marx = projection.objects.iter().find(|object| object.title == "Capital").unwrap();
    assert_eq!(marx.object_id, SemanticObjectId::parse(MARX_OBJECT).unwrap());
    assert_ne!(marx.title, marx.object_id.to_string());
    assert_ne!(marx.filename, marx.object_id.to_string());
    assert_ne!(marx.canonical_path, marx.object_id.to_string());
    assert!(marx.aliases.iter().any(|alias| alias == "Das Kapital"));
    assert_ne!(marx.object_id, SemanticObjectId::parse(MCCARTHY_OBJECT).unwrap());
}

#[test]
fn top_down_and_lateral_structure_preserve_canonical_identity_and_provenance() {
    let projection = tiny_projection();
    assert_eq!(projection.objects.len(), 5);
    assert_eq!(projection.regions.len(), 4);
    assert_eq!(projection.units.len(), 5);
    for unit in &projection.units {
        let parent = projection.objects.iter().find(|object| object.object_id == unit.parent_object_id).unwrap();
        assert!(parent.unit_ids.contains(&unit.unit_id));
        assert!(parent.region_addresses.contains(&unit.parent_region_address));
        assert!(projection.regions.iter().any(|region| region.address == unit.parent_region_address && region.contained_unit_ids.contains(&unit.unit_id)));
        assert!(!unit.inherited_identifier_assignment_ids.is_empty());
        for assignment_id in &unit.inherited_identifier_assignment_ids {
            let assignment = projection.identifier_assignments.iter().find(|assignment| &assignment.assignment_id == assignment_id).unwrap();
            assert!(matches!(&assignment.provenance, RecordProvenance::ObjectField { .. }));
        }
    }
    let segment = &projection.units[0].transport_segments[0];
    assert_ne!(segment.parent_unit_id.as_str(), segment.segment_id.as_str());
    assert_eq!(segment.parent_unit_id, projection.units[0].unit_id);
}

#[test]
fn occurrences_preserve_outgoing_incoming_identity_and_contextual_target_typing() {
    let projection = tiny_projection();
    let object_occurrence = projection.occurrences.iter().find(|occurrence| occurrence.occurrence_id.as_str() == "occurrence:journal-one:capital-object").unwrap();
    assert!(matches!(object_occurrence.resolved_target, SemanticAddress::Object(_)));
    let marx = projection.objects.iter().find(|object| object.object_id.to_string() == MARX_OBJECT).unwrap();
    assert!(marx.incoming_occurrence_ids.contains(&object_occurrence.occurrence_id));
    let journal = projection.objects.iter().find(|object| object.object_id.to_string() == JOURNAL_ONE_OBJECT).unwrap();
    assert!(journal.object_field_occurrence_ids.contains(&object_occurrence.occurrence_id));
    assert_eq!(object_occurrence.direction, Direction::Outgoing);
    assert_eq!(object_occurrence.source, semantic_traversal_core::projection::OccurrenceSource::ObjectField { object_id: journal.object_id.clone(), field_path: "book_read_today".into() });
    assert!(projection.objects.iter().find(|object| object.object_id.to_string() == CLEO_OBJECT).unwrap().temporal_anchor_ids.is_empty());
}

#[test]
fn heading_targets_resolve_to_regions_and_block_targets_to_exact_units() {
    let projection = tiny_projection();
    let heading = projection.occurrences.iter().find(|occurrence| occurrence.authored_target_text.contains("#Chapter 2")).unwrap();
    let region = match &heading.resolved_target { SemanticAddress::Region(address) => projection.regions.iter().find(|region| &region.address == address).unwrap(), _ => panic!("heading target must resolve to a region") };
    assert_eq!(region.contained_unit_ids.len(), 2);
    assert!(region.contained_unit_ids.len() > 1);
    assert!(!matches!(&heading.resolved_target, SemanticAddress::Object(_) | SemanticAddress::Unit(_)));
    let block = projection.occurrences.iter().find(|occurrence| occurrence.authored_target_text.contains("^capital-block-2")).unwrap();
    let target = match &block.resolved_target { SemanticAddress::Unit(unit_id) => unit_id, _ => panic!("block target must resolve to a unit") };
    assert_eq!(target.as_str(), "unit:capital:chapter-2:2");
    assert_eq!(projection.regions.iter().find(|region| region.address == *match &heading.resolved_target { SemanticAddress::Region(address) => address, _ => unreachable!() }).unwrap().block_target_mappings[0].target_unit_id, *target);
}

#[test]
fn temporal_anchors_preserve_authored_values_and_do_not_retype_cleo() {
    let projection = tiny_projection();
    assert_eq!(projection.temporal_anchors.len(), 2);
    for anchor in &projection.temporal_anchors {
        assert!(matches!(&anchor.value, TemporalValue::Date(value) if value.starts_with("2026-07-")));
        assert!(matches!(&anchor.provenance, RecordProvenance::ObjectField { field_path, .. } if field_path == "journal_entry_date"));
    }
    let cleo = SemanticObjectId::parse(CLEO_OBJECT).unwrap();
    assert!(!projection.identifier_assignments.iter().any(|assignment| assignment.subject == SemanticAddress::Object(cleo.clone()) && assignment.identifier_name == "journal_entry_date"));
    assert!(!projection.temporal_anchors.iter().any(|anchor| anchor.subject == SemanticAddress::Object(cleo.clone())));
    assert!(projection.identifier_assignments.iter().any(|assignment| assignment.subject == SemanticAddress::Object(cleo.clone()) && assignment.identifier_name == "entity_type"));
}

#[test]
fn retrieval_surfaces_directly_declare_capabilities_without_execution() {
    let projection = tiny_projection();
    assert_eq!(projection.retrieval_surfaces.len(), 5);
    for (kind, mode) in [(RetrievalSurfaceKind::Exact, "literal"), (RetrievalSurfaceKind::Lexical, "terms"), (RetrievalSurfaceKind::Vector, "nearest_neighbours"), (RetrievalSurfaceKind::Graph, "incidence"), (RetrievalSurfaceKind::Temporal, "temporal")] {
        let surface = projection.retrieval_surfaces.iter().find(|surface| surface.kind == kind).unwrap();
        assert!(surface.available);
        assert!(surface.visible_address_kinds.contains(&AddressKind::SemanticUnit));
        assert!(surface.match_modes.iter().any(|candidate| serde_json::to_string(candidate).unwrap().contains(mode)));
        assert_eq!(surface.default_candidate_limit, 8);
        assert_eq!(surface.hard_candidate_limit, 32);
        assert!(surface.hydrates_to_semantic_units);
        assert!(surface.continuation_supported);
        assert!(matches!(&surface.coverage_semantics, CoverageSemantics::Exhaustive | CoverageSemantics::Bounded));
    }
    assert_eq!(projection.retrieval_surfaces.iter().find(|surface| surface.kind == RetrievalSurfaceKind::Exact).unwrap().returned_identity, AddressKind::SemanticUnit);
    assert!(projection.retrieval_surfaces.iter().find(|surface| surface.kind == RetrievalSurfaceKind::Exact).unwrap().exhaustive_total_count_supported);
}

#[test]
fn transitions_cover_the_typed_routes_without_implementing_them() {
    let projection = tiny_projection();
    assert!(projection.valid_transitions.iter().any(|transition| transition.from == AddressKind::SemanticObject && transition.operation == StructuralTransitionOperation::Containment && transition.to == AddressKind::SemanticRegion));
    assert!(projection.valid_transitions.iter().any(|transition| transition.from == AddressKind::SemanticRegion && transition.to == AddressKind::SemanticUnit));
    assert!(projection.valid_transitions.iter().any(|transition| transition.direction == Direction::Incoming && transition.operation == StructuralTransitionOperation::Occurrence));
    assert!(projection.valid_transitions.iter().any(|transition| transition.operation == StructuralTransitionOperation::TemporalAnchor));
    assert!(projection.valid_transitions.iter().any(|transition| transition.operation == StructuralTransitionOperation::Hydration));
}
