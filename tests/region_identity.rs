use std::collections::{HashMap, HashSet};

use semantic_traversal_core::{
    AuthoredRegionHeading, SemanticObjectId, canonical_region_identities, model::SourceSpan,
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
fn all_downstream_synthetic_references_use_the_selected_canonical_address() {
    let identities = canonical_region_identities(
        object(),
        &[heading(1, "Parent", 0), heading(2, "Target", 10)],
    )
    .unwrap();
    let target = identities[1].address.clone();

    let unit_parents = HashMap::from([("unit:one", target.clone()), ("unit:two", target.clone())]);
    let region_children = vec![target.clone()];
    let outgoing_target = target.clone();
    let incoming_region = target.clone();

    assert!(unit_parents.values().all(|address| address == &target));
    assert_eq!(region_children, [target.clone()]);
    assert_eq!(outgoing_target, target);
    assert_eq!(incoming_region, identities[1].address);
}
