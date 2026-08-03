//! One small, deterministic projection used to exercise the frozen contract.
//!
//! This is deliberately a hand-authored test fixture. It is not an ingest
//! path, builder, validator, or runtime repository.

use semantic_traversal_core::{
    OccurrenceId, SemanticObjectId, SemanticRegionAddress, SemanticUnitId, TemporalAnchorId,
    TransportSegmentId,
    model::{
        AddressKind, Direction, RecordProvenance, RetrievalSurfaceKind, SemanticAddress,
        SourceSpan,
    },
    projection::{
        AuthoredBlockType, BlockTargetMapping, CoverageSemantics, IdentifierAssignment,
        IdentifierAssignmentMode, IdentifierCardinality, IdentifierDescriptor, IdentifierRole,
        IdentifierValue, IdentifierValueShape, OccurrencePresentation, OccurrenceRecord,
        OccurrenceSource, ProjectionValidationStatus, RetrievalSurfaceDescriptor,
        SemanticObjectClassDescriptor, SemanticObjectRecord, SemanticRegionRecord,
        SemanticUnitContent, SemanticUnitRecord, SourceKind, StructuralTransition,
        StructuralTransitionOperation, SurfaceMatchMode, TemporalAffordance, TemporalAnchorRecord,
        TemporalValue, TransportSegmentRecord,
    },
    projection::SemanticSpaceProjection,
};

pub const MARX_OBJECT: &str = "019fc58d-42aa-7919-95f8-a69b609aadff";
pub const MCCARTHY_OBJECT: &str = "019fc58d-7a15-7e1f-8cf0-4b36e9d54c21";
pub const JOURNAL_ONE_OBJECT: &str = "019fc58d-8b62-7f0a-9d24-5e7b1c3a4401";
pub const JOURNAL_TWO_OBJECT: &str = "019fc58d-9c73-7a1b-8e35-6f8c2d4b5502";
pub const CLEO_OBJECT: &str = "019fc58d-ad84-7b2c-9f46-7a9d3e5c6603";

pub fn object(value: &str) -> SemanticObjectId {
    SemanticObjectId::parse(value).expect("fixture UUID is valid")
}

pub fn unit(value: &str) -> SemanticUnitId {
    SemanticUnitId::parse(value).expect("fixture unit identity is non-empty")
}

pub fn occurrence(value: &str) -> OccurrenceId {
    OccurrenceId::parse(value).expect("fixture occurrence identity is non-empty")
}

pub fn anchor(value: &str) -> TemporalAnchorId {
    TemporalAnchorId::parse(value).expect("fixture anchor identity is non-empty")
}

pub fn segment(value: &str) -> TransportSegmentId {
    TransportSegmentId::parse(value).expect("fixture segment identity is non-empty")
}

pub fn region(object_id: &SemanticObjectId, address: &str) -> SemanticRegionAddress {
    SemanticRegionAddress::parse(object_id.clone(), address)
        .expect("fixture region address is non-empty")
}

fn span(source: &str, start: u64, end: u64) -> SourceSpan {
    SourceSpan {
        source: source.into(),
        start_byte: Some(start),
        end_byte: Some(end),
    }
}

fn provenance_unit(unit_id: &SemanticUnitId, source: &str) -> RecordProvenance {
    RecordProvenance::SemanticUnit {
        unit_id: unit_id.clone(),
        source_span: Some(span(source, 10, 42)),
    }
}

fn provenance_field(object_id: &SemanticObjectId, field_path: &str) -> RecordProvenance {
    RecordProvenance::ObjectField {
        object_id: object_id.clone(),
        field_path: field_path.into(),
    }
}

fn classes() -> Vec<SemanticObjectClassDescriptor> {
    vec![
        SemanticObjectClassDescriptor {
            class_name: "source_material".into(),
            applicable_identifier_names: vec![
                "note_type".into(),
                "title".into(),
                "creator".into(),
                "format".into(),
                "original_year_published".into(),
            ],
            permitted_source_kinds: vec![SourceKind::Markdown],
        },
        SemanticObjectClassDescriptor {
            class_name: "journal_entry".into(),
            applicable_identifier_names: vec!["note_type".into(), "journal_entry_date".into()],
            permitted_source_kinds: vec![SourceKind::Markdown],
        },
        SemanticObjectClassDescriptor {
            class_name: "entity".into(),
            applicable_identifier_names: vec![
                "note_type".into(),
                "entity_type".into(),
                "canonical_name".into(),
            ],
            permitted_source_kinds: vec![SourceKind::Markdown],
        },
    ]
}

fn descriptors() -> Vec<IdentifierDescriptor> {
    let none = TemporalAffordance::None;
    let field = |name: &str,
                 role: IdentifierRole,
                 shape: IdentifierValueShape,
                 mode: IdentifierAssignmentMode,
                 temporal: TemporalAffordance,
                 links: bool| IdentifierDescriptor {
        identifier_name: name.into(),
        semantic_role: role,
        value_shape: shape,
        cardinality: IdentifierCardinality::Scalar,
        applicable_address_kinds: vec![AddressKind::SemanticObject, AddressKind::SemanticUnit],
        assignment_mode: mode,
        source_surface: format!("frontmatter:{name}"),
        may_contain_canonical_links: links,
        temporal_affordance: temporal,
        retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into()],
        enabled_transition_ids: vec!["transition:identifier".into()],
    };

    vec![
        field(
            "note_type",
            IdentifierRole::ObjectClass,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        field(
            "title",
            IdentifierRole::CanonicalNaming,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        field(
            "creator",
            IdentifierRole::CanonicalNaming,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        field(
            "format",
            IdentifierRole::Grouping,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        field(
            "original_year_published",
            IdentifierRole::TemporalAnchoring,
            IdentifierValueShape::Integer,
            IdentifierAssignmentMode::Intrinsic,
            TemporalAffordance::CreatesAnchor,
            false,
        ),
        field(
            "journal_entry_date",
            IdentifierRole::TemporalAnchoring,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            TemporalAffordance::CreatesAnchor,
            false,
        ),
        field(
            "entity_type",
            IdentifierRole::ObjectClass,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        field(
            "canonical_name",
            IdentifierRole::CanonicalNaming,
            IdentifierValueShape::String,
            IdentifierAssignmentMode::Intrinsic,
            none.clone(),
            false,
        ),
        IdentifierDescriptor {
            identifier_name: "book_read_today".into(),
            semantic_role: IdentifierRole::ContextualRelation,
            value_shape: IdentifierValueShape::SemanticAddress,
            cardinality: IdentifierCardinality::Collection,
            applicable_address_kinds: vec![AddressKind::SemanticObject, AddressKind::SemanticUnit],
            assignment_mode: IdentifierAssignmentMode::Relational,
            source_surface: "frontmatter:book_read_today".into(),
            may_contain_canonical_links: true,
            temporal_affordance: TemporalAffordance::ReferencesAnchor,
            retrieval_surface_ids: vec!["surface:exact".into(), "surface:graph".into(), "surface:temporal".into()],
            enabled_transition_ids: vec!["transition:identifier".into(), "transition:occurrence".into()],
        },
    ]
}

fn assignment(id: &str, name: &str, subject: SemanticAddress, value: IdentifierValue, provenance: RecordProvenance) -> IdentifierAssignment {
    IdentifierAssignment {
        assignment_id: id.into(),
        identifier_name: name.into(),
        subject,
        value,
        provenance,
    }
}

fn unit_record(
    id: &str,
    parent: &SemanticObjectId,
    parent_region: &SemanticRegionAddress,
    heading: &[&str],
    ordinal: u32,
    block_id: Option<&str>,
    content: &str,
    inherited: &[&str],
    outgoing: &[&str],
    anchors: &[&str],
    source: &str,
    segments: Vec<TransportSegmentRecord>,
) -> SemanticUnitRecord {
    let unit_id = unit(id);
    SemanticUnitRecord {
        unit_id,
        parent_object_id: parent.clone(),
        parent_region_address: parent_region.clone(),
        authored_block_type: AuthoredBlockType::Paragraph,
        heading_path: heading.iter().map(|value| (*value).into()).collect(),
        block_ordinal: ordinal,
        explicit_block_id: block_id.map(str::to_owned),
        content: SemanticUnitContent::Inline {
            authored_markdown: content.into(),
            normalized_text: content.into(),
        },
        inherited_identifier_assignment_ids: inherited.iter().map(|value| (*value).into()).collect(),
        unit_local_identifier_assignment_ids: vec![],
        outgoing_occurrence_ids: outgoing.iter().map(|value| occurrence(value)).collect(),
        incoming_occurrence_ids: vec![],
        temporal_anchor_ids: anchors.iter().map(|value| anchor(value)).collect(),
        retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()],
        source_provenance: provenance_unit(&unit(id), source),
        transport_segments: segments,
    }
}

pub fn tiny_projection() -> SemanticSpaceProjection {
    let marx = object(MARX_OBJECT);
    let mccarthy = object(MCCARTHY_OBJECT);
    let journal_one = object(JOURNAL_ONE_OBJECT);
    let journal_two = object(JOURNAL_TWO_OBJECT);
    let cleo = object(CLEO_OBJECT);
    let marx_region = region(&marx, "heading:Chapter 2");
    let blood_region = region(&mccarthy, "heading:Chapter 1");
    let journal_one_region = region(&journal_one, "root");
    let journal_two_region = region(&journal_two, "root");
    let capital_one = unit("unit:capital:chapter-2:1");
    let capital_two = unit("unit:capital:chapter-2:2");
    let blood_one = unit("unit:blood-meridian:chapter-1:1");
    let journal_one_unit = unit("unit:journal:2026-07-02:1");
    let journal_two_unit = unit("unit:journal:2026-07-15:1");

    let assignments = vec![
        assignment("assignment:marx:note_type", "note_type", SemanticAddress::Object(marx.clone()), IdentifierValue::String("source_material".into()), provenance_field(&marx, "note_type")),
        assignment("assignment:marx:title", "title", SemanticAddress::Object(marx.clone()), IdentifierValue::String("Capital".into()), provenance_field(&marx, "title")),
        assignment("assignment:marx:creator", "creator", SemanticAddress::Object(marx.clone()), IdentifierValue::String("Karl Marx".into()), provenance_field(&marx, "creator")),
        assignment("assignment:marx:format", "format", SemanticAddress::Object(marx.clone()), IdentifierValue::String("book".into()), provenance_field(&marx, "format")),
        assignment("assignment:mccarthy:note_type", "note_type", SemanticAddress::Object(mccarthy.clone()), IdentifierValue::String("source_material".into()), provenance_field(&mccarthy, "note_type")),
        assignment("assignment:mccarthy:title", "title", SemanticAddress::Object(mccarthy.clone()), IdentifierValue::String("Blood Meridian".into()), provenance_field(&mccarthy, "title")),
        assignment("assignment:mccarthy:creator", "creator", SemanticAddress::Object(mccarthy.clone()), IdentifierValue::String("Cormac McCarthy".into()), provenance_field(&mccarthy, "creator")),
        assignment("assignment:mccarthy:format", "format", SemanticAddress::Object(mccarthy.clone()), IdentifierValue::String("book".into()), provenance_field(&mccarthy, "format")),
        assignment("assignment:journal-one:note_type", "note_type", SemanticAddress::Object(journal_one.clone()), IdentifierValue::String("journal_entry".into()), provenance_field(&journal_one, "note_type")),
        assignment("assignment:journal-one:date", "journal_entry_date", SemanticAddress::Object(journal_one.clone()), IdentifierValue::String("2026-07-02".into()), provenance_field(&journal_one, "journal_entry_date")),
        assignment("assignment:journal-one:book", "book_read_today", SemanticAddress::Object(journal_one.clone()), IdentifierValue::SemanticAddresses(vec![SemanticAddress::Object(marx.clone())]), provenance_field(&journal_one, "book_read_today")),
        assignment("assignment:journal-two:note_type", "note_type", SemanticAddress::Object(journal_two.clone()), IdentifierValue::String("journal_entry".into()), provenance_field(&journal_two, "note_type")),
        assignment("assignment:journal-two:date", "journal_entry_date", SemanticAddress::Object(journal_two.clone()), IdentifierValue::String("2026-07-15".into()), provenance_field(&journal_two, "journal_entry_date")),
        assignment("assignment:cleo:note_type", "note_type", SemanticAddress::Object(cleo.clone()), IdentifierValue::String("entity".into()), provenance_field(&cleo, "note_type")),
        assignment("assignment:cleo:entity_type", "entity_type", SemanticAddress::Object(cleo.clone()), IdentifierValue::String("cat".into()), provenance_field(&cleo, "entity_type")),
        assignment("assignment:cleo:name", "canonical_name", SemanticAddress::Object(cleo.clone()), IdentifierValue::String("Cleo".into()), provenance_field(&cleo, "canonical_name")),
    ];

    let occurrences = vec![
        OccurrenceRecord { occurrence_id: occurrence("occurrence:journal-one:capital-object"), source: OccurrenceSource::ObjectField { object_id: journal_one.clone(), field_path: "book_read_today".into() }, authored_target_text: "[[Marx, Karl — Capital]]".into(), display_alias: None, resolved_target: SemanticAddress::Object(marx.clone()), presentation_mode: OccurrencePresentation::Link, direction: Direction::Outgoing, source_span: Some(span("2026-07-02.md", 10, 38)) },
        OccurrenceRecord { occurrence_id: occurrence("occurrence:journal-one:capital-heading"), source: OccurrenceSource::SemanticUnit { unit_id: journal_one_unit.clone() }, authored_target_text: "[[Marx, Karl — Capital#Chapter 2]]".into(), display_alias: None, resolved_target: SemanticAddress::Region(marx_region.clone()), presentation_mode: OccurrencePresentation::Link, direction: Direction::Outgoing, source_span: Some(span("2026-07-02.md", 43, 80)) },
        OccurrenceRecord { occurrence_id: occurrence("occurrence:journal-two:capital-block"), source: OccurrenceSource::SemanticUnit { unit_id: journal_two_unit.clone() }, authored_target_text: "[[Marx, Karl — Capital#^capital-block-2]]".into(), display_alias: Some("the second Capital passage".into()), resolved_target: SemanticAddress::Unit(capital_two.clone()), presentation_mode: OccurrencePresentation::Link, direction: Direction::Outgoing, source_span: Some(span("2026-07-15.md", 10, 55)) },
        OccurrenceRecord { occurrence_id: occurrence("occurrence:journal-one:cleo"), source: OccurrenceSource::SemanticUnit { unit_id: journal_one_unit.clone() }, authored_target_text: "[[Cleo]]".into(), display_alias: None, resolved_target: SemanticAddress::Object(cleo.clone()), presentation_mode: OccurrencePresentation::Link, direction: Direction::Outgoing, source_span: Some(span("2026-07-02.md", 82, 90)) },
        OccurrenceRecord { occurrence_id: occurrence("occurrence:capital:outgoing-region"), source: OccurrenceSource::SemanticUnit { unit_id: capital_one.clone() }, authored_target_text: "[[Marx, Karl — Capital#Chapter 2]]".into(), display_alias: None, resolved_target: SemanticAddress::Region(marx_region.clone()), presentation_mode: OccurrencePresentation::Link, direction: Direction::Outgoing, source_span: Some(span("Capital.md", 100, 135)) },
    ];

    let surfaces = [
        ("surface:exact", RetrievalSurfaceKind::Exact, vec![AddressKind::SemanticObject, AddressKind::SemanticRegion, AddressKind::SemanticUnit], vec![SurfaceMatchMode::Literal], AddressKind::SemanticUnit, CoverageSemantics::Exhaustive),
        ("surface:lexical", RetrievalSurfaceKind::Lexical, vec![AddressKind::SemanticObject, AddressKind::SemanticRegion, AddressKind::SemanticUnit], vec![SurfaceMatchMode::Terms], AddressKind::SemanticUnit, CoverageSemantics::Bounded),
        ("surface:vector", RetrievalSurfaceKind::Vector, vec![AddressKind::SemanticObject, AddressKind::SemanticRegion, AddressKind::SemanticUnit], vec![SurfaceMatchMode::NearestNeighbours], AddressKind::SemanticUnit, CoverageSemantics::Bounded),
        ("surface:graph", RetrievalSurfaceKind::Graph, vec![AddressKind::SemanticObject, AddressKind::SemanticRegion, AddressKind::SemanticUnit, AddressKind::Occurrence], vec![SurfaceMatchMode::Incidence], AddressKind::Occurrence, CoverageSemantics::Bounded),
        ("surface:temporal", RetrievalSurfaceKind::Temporal, vec![AddressKind::SemanticObject, AddressKind::SemanticUnit, AddressKind::TemporalAnchor], vec![SurfaceMatchMode::Temporal], AddressKind::SemanticUnit, CoverageSemantics::Bounded),
    ];
    let retrieval_surfaces = surfaces.into_iter().map(|(id, kind, visible, modes, returned, coverage)| RetrievalSurfaceDescriptor {
        surface_id: id.into(), kind, available: true, visible_address_kinds: visible, match_modes: modes,
        default_candidate_limit: 8, hard_candidate_limit: 32, returned_identity: returned,
        hydrates_to_semantic_units: true, coverage_semantics: coverage, exhaustive_total_count_supported: id == "surface:exact",
        continuation_supported: true, technical_limitations: vec!["synthetic fixture only".into()],
    }).collect();

    let mut objects = vec![
        SemanticObjectRecord { object_id: marx.clone(), source_identity: "synthetic:Marx, Karl — Capital".into(), source_kind: SourceKind::Markdown, canonical_path: "LAYER-2/READING & RESEARCH/SOURCE MATERIAL/Marx, Karl — Capital.md".into(), filename: "Marx, Karl — Capital.md".into(), title: "Capital".into(), aliases: vec!["Das Kapital".into()], object_class: "source_material".into(), region_addresses: vec![marx_region.clone()], unit_ids: vec![capital_one.clone(), capital_two.clone()], identifier_assignment_ids: vec!["assignment:marx:note_type".into(), "assignment:marx:title".into(), "assignment:marx:creator".into(), "assignment:marx:format".into()], object_field_occurrence_ids: vec![], body_occurrence_ids: vec![], incoming_occurrence_ids: vec![occurrence("occurrence:journal-one:capital-object")], temporal_anchor_ids: vec![], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticObjectRecord { object_id: mccarthy.clone(), source_identity: "synthetic:McCarthy, Cormac — Blood Meridian".into(), source_kind: SourceKind::Markdown, canonical_path: "LAYER-2/READING & RESEARCH/SOURCE MATERIAL/McCarthy, Cormac — Blood Meridian.md".into(), filename: "McCarthy, Cormac — Blood Meridian.md".into(), title: "Blood Meridian".into(), aliases: vec!["Blood Meridian".into()], object_class: "source_material".into(), region_addresses: vec![blood_region.clone()], unit_ids: vec![blood_one.clone()], identifier_assignment_ids: vec!["assignment:mccarthy:note_type".into(), "assignment:mccarthy:title".into(), "assignment:mccarthy:creator".into(), "assignment:mccarthy:format".into()], object_field_occurrence_ids: vec![], body_occurrence_ids: vec![], incoming_occurrence_ids: vec![], temporal_anchor_ids: vec![], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticObjectRecord { object_id: journal_one.clone(), source_identity: "synthetic:2026-07-02".into(), source_kind: SourceKind::Markdown, canonical_path: "LAYER-1/JOURNAL/2026/2026-07-02.md".into(), filename: "2026-07-02.md".into(), title: "2026-07-02".into(), aliases: vec![], object_class: "journal_entry".into(), region_addresses: vec![journal_one_region.clone()], unit_ids: vec![journal_one_unit.clone()], identifier_assignment_ids: vec!["assignment:journal-one:note_type".into(), "assignment:journal-one:date".into(), "assignment:journal-one:book".into()], object_field_occurrence_ids: vec![occurrence("occurrence:journal-one:capital-object")], body_occurrence_ids: vec![occurrence("occurrence:journal-one:capital-heading"), occurrence("occurrence:journal-one:cleo")], incoming_occurrence_ids: vec![], temporal_anchor_ids: vec![anchor("anchor:journal-one:2026-07-02")], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticObjectRecord { object_id: journal_two.clone(), source_identity: "synthetic:2026-07-15".into(), source_kind: SourceKind::Markdown, canonical_path: "LAYER-1/JOURNAL/2026/2026-07-15.md".into(), filename: "2026-07-15.md".into(), title: "2026-07-15".into(), aliases: vec![], object_class: "journal_entry".into(), region_addresses: vec![journal_two_region.clone()], unit_ids: vec![journal_two_unit.clone()], identifier_assignment_ids: vec!["assignment:journal-two:note_type".into(), "assignment:journal-two:date".into()], object_field_occurrence_ids: vec![], body_occurrence_ids: vec![occurrence("occurrence:journal-two:capital-block")], incoming_occurrence_ids: vec![], temporal_anchor_ids: vec![anchor("anchor:journal-two:2026-07-15")], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticObjectRecord { object_id: cleo.clone(), source_identity: "synthetic:Cleo".into(), source_kind: SourceKind::Markdown, canonical_path: "LAYER-1/ENTITY INDEX/Cleo.md".into(), filename: "Cleo.md".into(), title: "Cleo".into(), aliases: vec!["Cleo".into()], object_class: "entity".into(), region_addresses: vec![], unit_ids: vec![], identifier_assignment_ids: vec!["assignment:cleo:note_type".into(), "assignment:cleo:entity_type".into(), "assignment:cleo:name".into()], object_field_occurrence_ids: vec![], body_occurrence_ids: vec![], incoming_occurrence_ids: vec![occurrence("occurrence:journal-one:cleo")], temporal_anchor_ids: vec![], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
    ];

    let regions = vec![
        SemanticRegionRecord { address: marx_region.clone(), heading_path: vec!["Chapter 2".into()], heading_identity: "heading:capital:chapter-2".into(), source_span: Some(span("Marx, Karl — Capital.md", 0, 18)), child_region_addresses: vec![], contained_unit_ids: vec![capital_one.clone(), capital_two.clone()], block_target_mappings: vec![BlockTargetMapping { authored_block_id: "^capital-block-2".into(), target_unit_id: capital_two.clone() }], incoming_occurrence_ids: vec![occurrence("occurrence:journal-one:capital-heading"), occurrence("occurrence:capital:outgoing-region")], inherited_identifier_assignment_ids: vec!["assignment:marx:note_type".into(), "assignment:marx:title".into(), "assignment:marx:creator".into(), "assignment:marx:format".into()], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticRegionRecord { address: blood_region.clone(), heading_path: vec!["Chapter 1".into()], heading_identity: "heading:blood:chapter-1".into(), source_span: Some(span("McCarthy, Cormac — Blood Meridian.md", 0, 18)), child_region_addresses: vec![], contained_unit_ids: vec![blood_one.clone()], block_target_mappings: vec![], incoming_occurrence_ids: vec![], inherited_identifier_assignment_ids: vec!["assignment:mccarthy:note_type".into(), "assignment:mccarthy:title".into(), "assignment:mccarthy:creator".into(), "assignment:mccarthy:format".into()], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticRegionRecord { address: journal_one_region.clone(), heading_path: vec![], heading_identity: "region:journal-one:root".into(), source_span: None, child_region_addresses: vec![], contained_unit_ids: vec![journal_one_unit.clone()], block_target_mappings: vec![], incoming_occurrence_ids: vec![], inherited_identifier_assignment_ids: vec!["assignment:journal-one:note_type".into(), "assignment:journal-one:date".into()], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
        SemanticRegionRecord { address: journal_two_region.clone(), heading_path: vec![], heading_identity: "region:journal-two:root".into(), source_span: None, child_region_addresses: vec![], contained_unit_ids: vec![journal_two_unit.clone()], block_target_mappings: vec![], incoming_occurrence_ids: vec![], inherited_identifier_assignment_ids: vec!["assignment:journal-two:note_type".into(), "assignment:journal-two:date".into()], retrieval_surface_ids: vec!["surface:exact".into(), "surface:lexical".into(), "surface:vector".into(), "surface:graph".into(), "surface:temporal".into()] },
    ];

    let units = vec![
        unit_record("unit:capital:chapter-2:1", &marx, &marx_region, &["Chapter 2"], 1, Some("capital-block-1"), "Capital is a source-material object.", &["assignment:marx:note_type", "assignment:marx:title", "assignment:marx:creator", "assignment:marx:format"], &["occurrence:capital:outgoing-region"], &[], "Marx, Karl — Capital.md", vec![TransportSegmentRecord { segment_id: segment("segment:capital:chapter-2:1:1"), parent_unit_id: capital_one.clone(), segment_ordinal: 0, source_span: span("Marx, Karl — Capital.md", 20, 60), total_segments: 1, reconstruction_group: "reconstruct:capital:chapter-2:1".into() }]),
        unit_record("unit:capital:chapter-2:2", &marx, &marx_region, &["Chapter 2"], 2, Some("capital-block-2"), "The second Capital passage is block-addressable.", &["assignment:marx:note_type", "assignment:marx:title", "assignment:marx:creator", "assignment:marx:format"], &[], &[], "Marx, Karl — Capital.md", vec![]),
        unit_record("unit:blood-meridian:chapter-1:1", &mccarthy, &blood_region, &["Chapter 1"], 1, None, "Blood Meridian is a source-material object.", &["assignment:mccarthy:note_type", "assignment:mccarthy:title", "assignment:mccarthy:creator", "assignment:mccarthy:format"], &[], &[], "McCarthy, Cormac — Blood Meridian.md", vec![]),
        unit_record("unit:journal:2026-07-02:1", &journal_one, &journal_one_region, &[], 1, None, "I read Capital and mentioned Cleo.", &["assignment:journal-one:note_type", "assignment:journal-one:date"], &["occurrence:journal-one:capital-heading", "occurrence:journal-one:cleo"], &["anchor:journal-one:2026-07-02"], "2026-07-02.md", vec![]),
        unit_record("unit:journal:2026-07-15:1", &journal_two, &journal_two_region, &[], 1, None, "I linked the Capital block.", &["assignment:journal-two:note_type", "assignment:journal-two:date"], &["occurrence:journal-two:capital-block"], &["anchor:journal-two:2026-07-15"], "2026-07-15.md", vec![]),
    ];

    let mut units = units;
    units.iter_mut().find(|record| record.unit_id == capital_two).unwrap().incoming_occurrence_ids.push(occurrence("occurrence:journal-two:capital-block"));
    let transitions = vec![
        ("object-region", AddressKind::SemanticObject, StructuralTransitionOperation::Containment, Direction::Outgoing, AddressKind::SemanticRegion, None),
        ("object-unit", AddressKind::SemanticObject, StructuralTransitionOperation::Containment, Direction::Outgoing, AddressKind::SemanticUnit, None),
        ("region-unit", AddressKind::SemanticRegion, StructuralTransitionOperation::Containment, Direction::Outgoing, AddressKind::SemanticUnit, None),
        ("unit-object", AddressKind::SemanticUnit, StructuralTransitionOperation::Parent, Direction::Outgoing, AddressKind::SemanticObject, None),
        ("unit-region", AddressKind::SemanticUnit, StructuralTransitionOperation::Parent, Direction::Outgoing, AddressKind::SemanticRegion, None),
        ("occurrence-outgoing", AddressKind::SemanticUnit, StructuralTransitionOperation::Occurrence, Direction::Outgoing, AddressKind::SemanticObject, Some("surface:graph")),
        ("occurrence-incoming", AddressKind::SemanticObject, StructuralTransitionOperation::Occurrence, Direction::Incoming, AddressKind::Occurrence, Some("surface:graph")),
        ("identifier", AddressKind::SemanticObject, StructuralTransitionOperation::Identifier, Direction::Outgoing, AddressKind::Identifier, Some("surface:exact")),
        ("temporal-anchor", AddressKind::SemanticObject, StructuralTransitionOperation::TemporalAnchor, Direction::Outgoing, AddressKind::TemporalAnchor, Some("surface:temporal")),
        ("hydration", AddressKind::SemanticUnit, StructuralTransitionOperation::Hydration, Direction::Outgoing, AddressKind::SemanticUnit, Some("surface:vector")),
    ].into_iter().map(|(id, from, operation, direction, to, surface)| StructuralTransition { transition_id: format!("transition:{id}"), from, operation, direction, to, retrieval_surface_id: surface.map(str::to_owned) }).collect();

    objects.sort_by_key(|record| record.object_id.to_string());
    SemanticSpaceProjection {
        projection_snapshot_id: "projection:tiny-synthetic:v1".into(),
        ingest_identity: "ingest:tiny-synthetic:v1".into(),
        schema_version: "v0.1.0".into(),
        logical_hash: "sha256:tiny-synthetic-projection-v1".into(),
        corpus_snapshot_identity: "corpus:tiny-synthetic:v1".into(),
        configuration_snapshot_id: "configuration:tiny-synthetic:v1".into(),
        validation_status: ProjectionValidationStatus::Validated,
        object_classes: classes(), objects, regions, units,
        identifier_descriptors: descriptors(), identifier_assignments: assignments,
        occurrences,
        temporal_anchors: vec![
            TemporalAnchorRecord { anchor_id: anchor("anchor:journal-one:2026-07-02"), subject: SemanticAddress::Object(journal_one), value: TemporalValue::Date("2026-07-02".into()), provenance: provenance_field(&object(JOURNAL_ONE_OBJECT), "journal_entry_date") },
            TemporalAnchorRecord { anchor_id: anchor("anchor:journal-two:2026-07-15"), subject: SemanticAddress::Object(journal_two), value: TemporalValue::Date("2026-07-15".into()), provenance: provenance_field(&object(JOURNAL_TWO_OBJECT), "journal_entry_date") },
        ],
        retrieval_surfaces, valid_transitions: transitions,
    }
}
