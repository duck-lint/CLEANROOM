use std::collections::HashSet;

use semantic_traversal_core::{
    AuthoredRegionHeading, RegionIdentityError, SemanticObjectId, canonical_region_identities,
    model::{Direction, RecordProvenance, SemanticAddress, SourceSpan},
    projection::{
        AuthoredBlockType, OccurrencePresentation, OccurrenceRecord, OccurrenceSource,
        SemanticRegionRecord, SemanticUnitContent, SemanticUnitRecord,
    },
};

const OBJECT: &str = "00000000-0000-0000-0000-000000000001";

fn object() -> SemanticObjectId {
    SemanticObjectId::parse(OBJECT).unwrap()
}

fn heading(level: u8, address: &str, start: u64) -> AuthoredRegionHeading {
    AuthoredRegionHeading {
        level,
        authored_structural_address: address.into(),
        source_span: Some(SourceSpan {
            source: "synthetic.md".into(),
            start_byte: Some(start),
            end_byte: Some(start + 4),
        }),
    }
}

#[test]
fn invalid_heading_input_is_rejected_before_topology_construction() {
    let zero_level = canonical_region_identities(object(), &[heading(0, "Foo", 0)]);
    assert_eq!(zero_level, Err(RegionIdentityError::ZeroHeadingLevel));

    let empty_address = canonical_region_identities(object(), &[heading(1, "", 0)]);
    assert_eq!(
        empty_address,
        Err(RegionIdentityError::EmptyStructuralAddress)
    );

    let whitespace_address = canonical_region_identities(object(), &[heading(1, "  \t", 0)]);
    assert_eq!(
        whitespace_address,
        Err(RegionIdentityError::EmptyStructuralAddress)
    );

    let invalid_after_valid = canonical_region_identities(
        object(),
        &[heading(1, "Valid", 0), heading(0, "Invalid", 10)],
    );
    assert_eq!(
        invalid_after_valid,
        Err(RegionIdentityError::ZeroHeadingLevel)
    );
}

#[test]
fn equivalent_sibling_ordinals_are_one_based() {
    let identities =
        canonical_region_identities(object(), &[heading(1, "Foo", 0), heading(1, "Foo", 10)])
            .unwrap();

    assert_eq!(
        identities[0].address.authored_structural_address,
        "region-v1:3:Foo:1;"
    );
    assert_eq!(
        identities[1].address.authored_structural_address,
        "region-v1:3:Foo:2;"
    );
}

#[test]
fn equivalent_siblings_are_disambiguated_locally() {
    let identities = canonical_region_identities(
        object(),
        &[
            heading(1, "Foo", 0),
            heading(1, "Bar", 10),
            heading(1, "Foo", 20),
        ],
    )
    .unwrap();

    assert_ne!(identities[0].address, identities[2].address);
    assert_ne!(identities[0].address, identities[1].address);

    let without_bar =
        canonical_region_identities(object(), &[heading(1, "Foo", 0), heading(1, "Foo", 20)])
            .unwrap();
    assert_eq!(identities[0].address, without_bar[0].address);
    assert_eq!(identities[2].address, without_bar[1].address);
}

#[test]
fn unrelated_edits_do_not_change_unchanged_region_identity() {
    let baseline = canonical_region_identities(
        object(),
        &[heading(1, "Parent", 0), heading(2, "Target", 10)],
    )
    .unwrap();
    let with_unrelated_heading = canonical_region_identities(
        object(),
        &[
            heading(1, "Different", 0),
            heading(1, "Parent", 10),
            heading(2, "Target", 20),
        ],
    )
    .unwrap();
    let with_changed_content = canonical_region_identities(
        object(),
        &[heading(1, "Parent", 999), heading(2, "Target", 1234)],
    )
    .unwrap();

    assert_eq!(baseline[0].address, with_unrelated_heading[1].address);
    assert_eq!(baseline[1].address, with_unrelated_heading[2].address);
    assert_eq!(baseline[1].address, with_changed_content[1].address);
}

#[test]
fn structural_changes_can_change_identity() {
    let duplicate =
        canonical_region_identities(object(), &[heading(1, "Foo", 0), heading(1, "Foo", 10)])
            .unwrap();
    let added_duplicate = canonical_region_identities(
        object(),
        &[
            heading(1, "Foo", 0),
            heading(1, "Foo", 10),
            heading(1, "Foo", 20),
        ],
    )
    .unwrap();
    let renamed = canonical_region_identities(object(), &[heading(1, "Renamed", 0)]).unwrap();
    let moved =
        canonical_region_identities(object(), &[heading(1, "Other", 0), heading(2, "Foo", 10)])
            .unwrap();

    assert_ne!(duplicate[1].address, added_duplicate[2].address);
    assert_ne!(duplicate[0].address, renamed[0].address);
    assert_ne!(duplicate[0].address, moved[1].address);
}

#[test]
fn equal_leaf_headings_under_different_parents_remain_distinct() {
    let identities = canonical_region_identities(
        object(),
        &[
            heading(1, "Parent A", 0),
            heading(2, "Foo", 10),
            heading(1, "Parent B", 20),
            heading(2, "Foo", 30),
        ],
    )
    .unwrap();

    assert_ne!(identities[1].address, identities[3].address);
    assert_eq!(identities[1].heading_path, ["Parent A", "Foo"]);
    assert_eq!(identities[3].heading_path, ["Parent B", "Foo"]);
}

#[test]
fn nested_duplicate_parents_are_resolved_before_duplicate_children() {
    let identities = canonical_region_identities(
        object(),
        &[
            heading(1, "Parent", 0),
            heading(2, "Child", 10),
            heading(1, "Parent", 20),
            heading(2, "Child", 30),
        ],
    )
    .unwrap();

    assert_ne!(identities[0].address, identities[2].address);
    assert_ne!(identities[1].address, identities[3].address);
    assert_eq!(
        identities
            .iter()
            .map(|i| &i.address)
            .collect::<HashSet<_>>()
            .len(),
        4
    );
}

#[test]
fn source_spans_are_provenance_not_canonical_identity() {
    let first = canonical_region_identities(object(), &[heading(1, "Foo", 10)]).unwrap();
    let moved = canonical_region_identities(object(), &[heading(1, "Foo", 999)]).unwrap();
    assert_eq!(first[0].address, moved[0].address);
    assert_ne!(first[0].source_span, moved[0].source_span);

    let duplicate =
        canonical_region_identities(object(), &[heading(1, "Foo", 10), heading(1, "Foo", 20)])
            .unwrap();
    assert_eq!(
        duplicate[0].source_span.as_ref().unwrap().start_byte,
        Some(10)
    );
    assert_eq!(
        duplicate[1].source_span.as_ref().unwrap().start_byte,
        Some(20)
    );
}

#[test]
fn downstream_contract_records_discriminate_the_selected_region() {
    let identities = canonical_region_identities(
        object(),
        &[
            heading(1, "Parent A", 0),
            heading(2, "Foo", 10),
            heading(1, "Parent B", 20),
            heading(2, "Foo", 30),
        ],
    )
    .unwrap();
    let parent_a = &identities[0].address;
    let foo_a = &identities[1].address;
    let parent_b = &identities[2].address;
    let foo_b = &identities[3].address;
    assert_ne!(foo_a, foo_b);

    let unit_id = semantic_traversal_core::SemanticUnitId::parse("unit:foo-a").unwrap();
    let occurrence_id =
        semantic_traversal_core::OccurrenceId::parse("occurrence:foo-a-to-b").unwrap();
    let unit = SemanticUnitRecord {
        unit_id: unit_id.clone(),
        parent_object_id: object(),
        parent_region_address: foo_a.clone(),
        authored_block_type: AuthoredBlockType::Paragraph,
        heading_path: vec!["Parent A".into(), "Foo".into()],
        block_ordinal: 1,
        explicit_block_id: None,
        content: SemanticUnitContent::Inline {
            authored_markdown: "synthetic body".into(),
            normalized_text: "synthetic body".into(),
        },
        inherited_identifier_assignment_ids: vec![],
        unit_local_identifier_assignment_ids: vec![],
        outgoing_occurrence_ids: vec![],
        incoming_occurrence_ids: vec![],
        temporal_anchor_ids: vec![],
        retrieval_surface_ids: vec![],
        source_provenance: RecordProvenance::Materialization {
            rule: "synthetic-region-identity-test".into(),
            sources: vec![SemanticAddress::Region(foo_a.clone())],
        },
        transport_segments: vec![],
    };
    let region_a = SemanticRegionRecord {
        address: parent_a.clone(),
        heading_path: vec!["Parent A".into()],
        heading_identity: "synthetic-parent-a".into(),
        source_span: Some(heading(1, "Parent A", 0).source_span.unwrap()),
        child_region_addresses: vec![foo_a.clone()],
        contained_unit_ids: vec![],
        block_target_mappings: vec![],
        incoming_occurrence_ids: vec![],
        outgoing_occurrence_ids: vec![],
        inherited_identifier_assignment_ids: vec![],
        retrieval_surface_ids: vec![],
    };
    let foo_region_a = SemanticRegionRecord {
        address: foo_a.clone(),
        heading_path: vec!["Parent A".into(), "Foo".into()],
        heading_identity: "synthetic-foo-a".into(),
        source_span: Some(heading(2, "Foo", 10).source_span.unwrap()),
        child_region_addresses: vec![],
        contained_unit_ids: vec![unit_id.clone()],
        block_target_mappings: vec![],
        incoming_occurrence_ids: vec![],
        outgoing_occurrence_ids: vec![occurrence_id.clone()],
        inherited_identifier_assignment_ids: vec![],
        retrieval_surface_ids: vec![],
    };
    let region_b = SemanticRegionRecord {
        address: parent_b.clone(),
        heading_path: vec!["Parent B".into()],
        heading_identity: "synthetic-parent-b".into(),
        source_span: Some(heading(1, "Parent B", 20).source_span.unwrap()),
        child_region_addresses: vec![foo_b.clone()],
        contained_unit_ids: vec![],
        block_target_mappings: vec![],
        incoming_occurrence_ids: vec![],
        outgoing_occurrence_ids: vec![],
        inherited_identifier_assignment_ids: vec![],
        retrieval_surface_ids: vec![],
    };
    let foo_region_b = SemanticRegionRecord {
        address: foo_b.clone(),
        heading_path: vec!["Parent B".into(), "Foo".into()],
        heading_identity: "synthetic-foo-b".into(),
        source_span: Some(heading(2, "Foo", 30).source_span.unwrap()),
        child_region_addresses: vec![],
        contained_unit_ids: vec![],
        block_target_mappings: vec![],
        incoming_occurrence_ids: vec![occurrence_id.clone()],
        outgoing_occurrence_ids: vec![],
        inherited_identifier_assignment_ids: vec![],
        retrieval_surface_ids: vec![],
    };
    let occurrence = OccurrenceRecord {
        occurrence_id,
        source: OccurrenceSource::SemanticRegion {
            region_address: foo_a.clone(),
        },
        authored_target_text: "#Foo".into(),
        display_alias: None,
        resolved_target: SemanticAddress::Region(foo_b.clone()),
        presentation_mode: OccurrencePresentation::Link,
        direction: Direction::Outgoing,
        source_span: Some(heading(2, "Foo", 10).source_span.unwrap()),
    };

    assert_eq!(unit.parent_region_address, *foo_a);
    assert_ne!(unit.parent_region_address, *foo_b);
    assert!(unit.outgoing_occurrence_ids.is_empty());
    assert_eq!(foo_region_a.contained_unit_ids, [unit.unit_id.clone()]);
    assert_eq!(region_a.child_region_addresses, [foo_a.clone()]);
    assert_eq!(region_b.child_region_addresses, [foo_b.clone()]);
    assert_eq!(
        foo_region_a.outgoing_occurrence_ids,
        [occurrence.occurrence_id.clone()]
    );
    assert_eq!(
        foo_region_b.incoming_occurrence_ids,
        [occurrence.occurrence_id.clone()]
    );
    assert_eq!(
        occurrence.source,
        OccurrenceSource::SemanticRegion {
            region_address: foo_a.clone()
        }
    );
    assert_eq!(
        occurrence.resolved_target,
        SemanticAddress::Region(foo_b.clone())
    );
    assert_ne!(
        occurrence.source,
        OccurrenceSource::SemanticRegion {
            region_address: foo_b.clone()
        }
    );
    assert_ne!(
        occurrence.resolved_target,
        SemanticAddress::Region(foo_a.clone())
    );
}
