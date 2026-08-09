//! Phase 5 adapter from the factual `vault-observation/v2` bundle.
//!
//! This module deliberately consumes observer JSON as facts, then applies the
//! CLEANROOM admission boundary. It does not import the observer's ontology or
//! build any retrieval index.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs,
    path::Path,
};

use serde_json::Value;

use crate::{
    model::*,
    projection::*,
    region_identity::{AuthoredRegionHeading, canonical_region_identities},
};

const EXCLUDED_FIELDS: [&str; 5] = ["address", "email", "phone", "likes", "dislikes"];

#[derive(Debug)]
pub enum ConstructionError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Contract(String),
}
impl From<std::io::Error> for ConstructionError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_json::Error> for ConstructionError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
impl std::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ConstructionError {}

fn text(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn array(v: &Value, key: &str) -> Vec<Value> {
    v.get(key)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}
fn span(v: &Value, source: &str) -> Option<SourceSpan> {
    let a = v.as_array()?;
    Some(SourceSpan {
        source: source.to_owned(),
        start_byte: a.first()?.as_u64(),
        end_byte: a.get(1)?.as_u64(),
    })
}
fn span_bounds(v: &Value) -> Option<(u64, u64)> {
    let span = span(v, "")?;
    Some((span.start_byte?, span.end_byte?))
}
fn span_contains(container: &SourceSpan, inner: &SourceSpan) -> bool {
    match (
        container.start_byte,
        container.end_byte,
        inner.start_byte,
        inner.end_byte,
    ) {
        (Some(container_start), Some(container_end), Some(inner_start), Some(inner_end)) => {
            container_start <= inner_start && inner_end <= container_end
        }
        _ => false,
    }
}
fn fnv(bytes: &[u8]) -> String {
    let mut h: u64 = 14695981039346656037;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    format!("{h:016x}")
}
fn id<T: std::str::FromStr>(s: String, what: &str) -> Result<T, ConstructionError>
where
    T::Err: std::fmt::Display,
{
    s.parse()
        .map_err(|e| ConstructionError::Contract(format!("{what}: {e}")))
}
fn typed_value(v: &Value) -> Result<IdentifierValue, ConstructionError> {
    match v {
        Value::Null => Ok(IdentifierValue::Null),
        Value::Bool(b) => Ok(IdentifierValue::Boolean(*b)),
        Value::Number(n) => Ok(IdentifierValue::Integer(n.as_i64().ok_or_else(|| {
            ConstructionError::Contract("non-integer numeric identifier value".into())
        })?)),
        Value::String(s) => Ok(IdentifierValue::String(s.clone())),
        Value::Array(values) => {
            let typed: Vec<_> = values.iter().map(typed_value).collect::<Result<_, _>>()?;
            if typed
                .iter()
                .all(|value| matches!(value, IdentifierValue::String(_)))
            {
                Ok(IdentifierValue::Strings(
                    typed
                        .into_iter()
                        .map(|value| match value {
                            IdentifierValue::String(value) => value,
                            _ => unreachable!("validated string collection"),
                        })
                        .collect(),
                ))
            } else if typed
                .iter()
                .all(|value| matches!(value, IdentifierValue::Integer(_)))
            {
                Ok(IdentifierValue::Integers(
                    typed
                        .into_iter()
                        .map(|value| match value {
                            IdentifierValue::Integer(value) => value,
                            _ => unreachable!("validated integer collection"),
                        })
                        .collect(),
                ))
            } else if typed
                .iter()
                .all(|value| matches!(value, IdentifierValue::Boolean(_)))
            {
                Ok(IdentifierValue::Booleans(
                    typed
                        .into_iter()
                        .map(|value| match value {
                            IdentifierValue::Boolean(value) => value,
                            _ => unreachable!("validated boolean collection"),
                        })
                        .collect(),
                ))
            } else {
                Ok(IdentifierValue::Values(typed))
            }
        }
        Value::Object(_) => Err(ConstructionError::Contract(
            "object-shaped identifier value has no accepted mechanical representation".into(),
        )),
    }
}

fn value_shapes(value: &IdentifierValue) -> Vec<IdentifierValueShape> {
    match value {
        IdentifierValue::Null => vec![],
        IdentifierValue::String(_) | IdentifierValue::Strings(_) => {
            vec![IdentifierValueShape::String]
        }
        IdentifierValue::Integer(_) | IdentifierValue::Integers(_) => {
            vec![IdentifierValueShape::Integer]
        }
        IdentifierValue::Boolean(_) | IdentifierValue::Booleans(_) => {
            vec![IdentifierValueShape::Boolean]
        }
        IdentifierValue::SemanticAddress(_) | IdentifierValue::SemanticAddresses(_) => {
            vec![IdentifierValueShape::SemanticAddress]
        }
        IdentifierValue::Values(values) => values.iter().flat_map(value_shapes).collect(),
    }
}

fn shape_rank(shape: &IdentifierValueShape) -> u8 {
    match shape {
        IdentifierValueShape::String => 0,
        IdentifierValueShape::Integer => 1,
        IdentifierValueShape::Boolean => 2,
        IdentifierValueShape::SemanticAddress => 3,
        IdentifierValueShape::Mixed(_) => 4,
    }
}

fn descriptor_shape(assignments: &[IdentifierAssignment], name: &str) -> IdentifierValueShape {
    let mut shapes = Vec::new();
    for assignment in assignments
        .iter()
        .filter(|assignment| assignment.identifier_name == name)
    {
        for shape in value_shapes(&assignment.value) {
            if !shapes.contains(&shape) {
                shapes.push(shape);
            }
        }
    }
    shapes.sort_by_key(shape_rank);
    match shapes.as_slice() {
        [shape] => shape.clone(),
        _ => IdentifierValueShape::Mixed(shapes),
    }
}

fn descriptor_cardinality(
    assignments: &[IdentifierAssignment],
    name: &str,
) -> IdentifierCardinality {
    let mut scalar = false;
    let mut collection = false;
    for assignment in assignments
        .iter()
        .filter(|assignment| assignment.identifier_name == name)
    {
        match assignment.value {
            IdentifierValue::Null => {}
            IdentifierValue::String(_)
            | IdentifierValue::Integer(_)
            | IdentifierValue::Boolean(_)
            | IdentifierValue::SemanticAddress(_) => scalar = true,
            IdentifierValue::Strings(_)
            | IdentifierValue::Integers(_)
            | IdentifierValue::Booleans(_)
            | IdentifierValue::SemanticAddresses(_)
            | IdentifierValue::Values(_) => collection = true,
        }
    }
    match (scalar, collection) {
        (true, true) => IdentifierCardinality::Mixed,
        (false, true) => IdentifierCardinality::Collection,
        _ => IdentifierCardinality::Scalar,
    }
}

fn is_bridge_field(name: &str) -> bool {
    matches!(
        name,
        "bridge_applicability_scope"
            | "bridge_applied"
            | "bridge_broken"
            | "bridge_conditions"
            | "bridge_isomorphism"
            | "bridge_justification"
            | "bridge_methods"
            | "bridge_preservation"
            | "bridge_required"
            | "cash_out"
            | "from_mode"
            | "from_register"
            | "interface"
            | "iso_broken"
            | "iso_justification"
            | "iso_structure"
            | "quarantine_reasons"
            | "revision_triggers"
            | "speculation_quarantine"
            | "stop_rule"
            | "to_mode"
            | "to_register"
    )
}

fn descriptor_role(name: &str) -> IdentifierRole {
    match name {
        "uuid" => IdentifierRole::Individuation,
        "note_type" | "entity_type" | "format" => IdentifierRole::ObjectClass,
        "layer" | "pillar" | "unity_level" => IdentifierRole::FrameworkPosition,
        "register" | "register_mode" => IdentifierRole::RegisterTyping,
        "vector_direction" => IdentifierRole::AnalysisOrientation,
        "title" | "canonical_name" | "creator" | "aliases" => IdentifierRole::CanonicalNaming,
        "journal_entry_date" | "birthday" | "first_met" | "original_year_published" => {
            IdentifierRole::TemporalAnchoring
        }
        "book_read_today" | "dream_motif" => IdentifierRole::ContextualRelation,
        "tags" => IdentifierRole::Grouping,
        "dream_location"
        | "dream_lucidity"
        | "dream_motif_valence"
        | "hypnagogic_resonance"
        | "reactivity"
        | "recall_ability"
        | "temporal_pace" => IdentifierRole::IndexicalTelemetry,
        "architect_or_operator" => IdentifierRole::JournalStateClassification,
        "occupation" => IdentifierRole::EntityMetadata,
        "origin" | "publish_studio" => IdentifierRole::SourceMetadata,
        "relationship" => IdentifierRole::ProfileRelation,
        name if is_bridge_field(name) => IdentifierRole::BridgeConstitutive,
        _ => IdentifierRole::Declared {
            name: "accepted_admitted_field".into(),
        },
    }
}

fn descriptor_assignment_mode(role: &IdentifierRole) -> IdentifierAssignmentMode {
    match role {
        IdentifierRole::ContextualRelation | IdentifierRole::ProfileRelation => {
            IdentifierAssignmentMode::Relational
        }
        _ => IdentifierAssignmentMode::Intrinsic,
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ClosureMetrics {
    semantic_unit_source_attribution_failures: usize,
    semantic_unit_outgoing_incidence_failures: usize,
    unresolved_block_target_degradations: usize,
    identifier_descriptor_assignment_failures: usize,
    inherited_assignment_reference_failures: usize,
    unit_identity_duplicates: usize,
    region_source_incidence_failures: usize,
    target_incidence_failures: usize,
    unit_parent_failures: usize,
    region_containment_failures: usize,
    region_inherited_assignment_failures: usize,
    excluded_region_inheritance: usize,
    block_mapping_missing: usize,
    block_mapping_duplicates: usize,
    block_mapping_wrong_region: usize,
    block_mapping_missing_target: usize,
    assignment_mode_failures: usize,
    retrieval_affordance_failures: usize,
    class_applicability_failures: usize,
    present_null_temporal_anchor_failures: usize,
}

fn occurrence_count<T: PartialEq>(ids: &[T], occurrence_id: &T) -> usize {
    ids.iter().filter(|id| *id == occurrence_id).count()
}

fn identifier_value_matches(
    value: &IdentifierValue,
    shape: &IdentifierValueShape,
    cardinality: &IdentifierCardinality,
) -> bool {
    if matches!(value, IdentifierValue::Null) {
        return true;
    }
    let is_collection = matches!(
        value,
        IdentifierValue::Strings(_)
            | IdentifierValue::Integers(_)
            | IdentifierValue::Booleans(_)
            | IdentifierValue::SemanticAddresses(_)
            | IdentifierValue::Values(_)
    );
    let cardinality_ok = match cardinality {
        IdentifierCardinality::Scalar => !is_collection,
        IdentifierCardinality::Collection | IdentifierCardinality::Mixed => true,
    };
    let observed_shapes = value_shapes(value);
    let shape_ok = match shape {
        IdentifierValueShape::Mixed(allowed) => observed_shapes
            .iter()
            .all(|observed| allowed.contains(observed)),
        expected => observed_shapes.iter().all(|observed| observed == expected),
    };
    cardinality_ok && shape_ok
}

fn validate_projection(
    projection: &SemanticSpaceProjection,
) -> Result<ClosureMetrics, ConstructionError> {
    let mut metrics = ClosureMetrics::default();
    let objects: HashSet<_> = projection
        .objects
        .iter()
        .map(|record| record.object_id.clone())
        .collect();
    let regions: HashMap<_, _> = projection
        .regions
        .iter()
        .map(|record| (record.address.clone(), record))
        .collect();
    let units: HashMap<_, _> = projection
        .units
        .iter()
        .map(|record| (record.unit_id.clone(), record))
        .collect();
    let descriptors: BTreeMap<_, _> = projection
        .identifier_descriptors
        .iter()
        .map(|descriptor| (descriptor.identifier_name.as_str(), descriptor))
        .collect();
    let assignments: BTreeMap<_, _> = projection
        .identifier_assignments
        .iter()
        .map(|assignment| (assignment.assignment_id.as_str(), assignment))
        .collect();
    let surface_ids: HashSet<_> = projection
        .retrieval_surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect();
    let class_descriptors: BTreeMap<_, _> = projection
        .object_classes
        .iter()
        .map(|class| (class.class_name.as_str(), class))
        .collect();
    let mut unit_keys = HashSet::new();
    for unit in &projection.units {
        if !objects.contains(&unit.parent_object_id)
            || !regions.contains_key(&unit.parent_region_address)
            || !unit_keys.insert((unit.parent_region_address.clone(), unit.block_ordinal))
        {
            metrics.unit_parent_failures += 1;
        }
        if let Some(region) = regions.get(&unit.parent_region_address) {
            if occurrence_count(&region.contained_unit_ids, &unit.unit_id) != 1 {
                metrics.region_containment_failures += 1;
            }
        }
    }
    for region in &projection.regions {
        for assignment_id in &region.inherited_identifier_assignment_ids {
            let Some(assignment) = assignments.get(assignment_id.as_str()) else {
                metrics.region_inherited_assignment_failures += 1;
                continue;
            };
            let valid_provenance = matches!(
                assignment.provenance,
                RecordProvenance::ObjectField { ref object_id, .. }
                    if object_id == &region.address.object_id
            );
            let excluded = EXCLUDED_FIELDS.contains(&assignment.identifier_name.as_str());
            let applicable = descriptors
                .get(assignment.identifier_name.as_str())
                .is_some_and(|descriptor| {
                    descriptor
                        .applicable_address_kinds
                        .contains(&AddressKind::SemanticRegion)
                });
            if !valid_provenance || !applicable {
                metrics.region_inherited_assignment_failures += 1;
            }
            if excluded {
                metrics.excluded_region_inheritance += 1;
            }
        }
    }
    for region in &projection.regions {
        let mut mapping_ids = HashSet::new();
        for mapping in &region.block_target_mappings {
            if !mapping_ids.insert(mapping.authored_block_id.clone()) {
                metrics.block_mapping_duplicates += 1;
            }
            let Some(unit) = units.get(&mapping.target_unit_id) else {
                metrics.block_mapping_missing_target += 1;
                continue;
            };
            if unit.parent_region_address != region.address {
                metrics.block_mapping_wrong_region += 1;
            }
            if occurrence_count(&region.contained_unit_ids, &mapping.target_unit_id) != 1 {
                metrics.block_mapping_wrong_region += 1;
            }
        }
    }
    for unit in &projection.units {
        if let Some(block_id) = &unit.explicit_block_id {
            let mapping_count = regions
                .get(&unit.parent_region_address)
                .map(|region| {
                    region
                        .block_target_mappings
                        .iter()
                        .filter(|mapping| {
                            mapping.authored_block_id == *block_id
                                && mapping.target_unit_id == unit.unit_id
                        })
                        .count()
                })
                .unwrap_or(0);
            if mapping_count == 0 {
                metrics.block_mapping_missing += 1;
            } else if mapping_count > 1 {
                metrics.block_mapping_duplicates += 1;
            }
        }
    }
    for object in &projection.objects {
        let Some(class) = class_descriptors.get(object.object_class.as_str()) else {
            metrics.class_applicability_failures += 1;
            continue;
        };
        for assignment_id in &object.identifier_assignment_ids {
            if let Some(assignment) = assignments.get(assignment_id.as_str()) {
                if !class
                    .applicable_identifier_names
                    .contains(&assignment.identifier_name)
                {
                    metrics.class_applicability_failures += 1;
                }
            }
        }
    }
    for descriptor in &projection.identifier_descriptors {
        let expected_mode = descriptor_assignment_mode(&descriptor.semantic_role);
        if descriptor.assignment_mode != expected_mode {
            metrics.assignment_mode_failures += 1;
        }
        for surface_id in &descriptor.retrieval_surface_ids {
            if !surface_ids.contains(surface_id.as_str())
                || !projection
                    .retrieval_surfaces
                    .iter()
                    .find(|surface| surface.surface_id == *surface_id)
                    .is_some_and(|surface| {
                        surface
                            .visible_address_kinds
                            .contains(&AddressKind::Identifier)
                    })
            {
                metrics.retrieval_affordance_failures += 1;
            }
        }
    }
    let temporal_null_assignments: HashSet<_> = projection
        .identifier_assignments
        .iter()
        .filter(|assignment| matches!(assignment.value, IdentifierValue::Null))
        .filter(|assignment| {
            descriptors
                .get(assignment.identifier_name.as_str())
                .is_some_and(|descriptor| {
                    matches!(
                        descriptor.temporal_affordance,
                        TemporalAffordance::CreatesAnchor
                    )
                })
        })
        .filter_map(
            |assignment| match (&assignment.subject, &assignment.provenance) {
                (
                    SemanticAddress::Object(object_id),
                    RecordProvenance::ObjectField { field_path, .. },
                ) => Some(format!("{object_id}:{field_path}")),
                _ => None,
            },
        )
        .collect();
    for anchor in &projection.temporal_anchors {
        if let RecordProvenance::ObjectField {
            object_id,
            field_path,
        } = &anchor.provenance
        {
            if temporal_null_assignments.contains(&format!("{object_id}:{field_path}")) {
                metrics.present_null_temporal_anchor_failures += 1;
            }
        }
    }
    let mut unit_ids = HashSet::new();
    for unit in &projection.units {
        if !unit_ids.insert(unit.unit_id.clone()) {
            metrics.unit_identity_duplicates += 1;
        }
        for assignment_id in &unit.inherited_identifier_assignment_ids {
            let Some(assignment) = assignments.get(assignment_id.as_str()) else {
                metrics.inherited_assignment_reference_failures += 1;
                continue;
            };
            let valid_provenance = matches!(
                assignment.provenance,
                RecordProvenance::ObjectField { ref object_id, .. } if object_id == &unit.parent_object_id
            );
            let applicable = descriptors
                .get(assignment.identifier_name.as_str())
                .is_some_and(|descriptor| {
                    descriptor
                        .applicable_address_kinds
                        .contains(&AddressKind::SemanticUnit)
                });
            if !valid_provenance || !applicable {
                metrics.inherited_assignment_reference_failures += 1;
            }
        }
    }
    for assignment in &projection.identifier_assignments {
        let valid_subject = match &assignment.subject {
            SemanticAddress::Object(object_id) => objects.contains(object_id),
            SemanticAddress::Unit(unit_id) => units.contains_key(unit_id),
            SemanticAddress::Region(address) => regions.contains_key(address),
            _ => false,
        };
        let valid_descriptor = descriptors
            .get(assignment.identifier_name.as_str())
            .is_some_and(|descriptor| {
                identifier_value_matches(
                    &assignment.value,
                    &descriptor.value_shape,
                    &descriptor.cardinality,
                )
            });
        if !valid_subject || !valid_descriptor {
            metrics.identifier_descriptor_assignment_failures += 1;
        }
    }
    for occurrence in &projection.occurrences {
        match &occurrence.source {
            OccurrenceSource::ObjectField { object_id, .. } => {
                if projection
                    .objects
                    .iter()
                    .find(|object| &object.object_id == object_id)
                    .is_none_or(|object| {
                        occurrence_count(
                            &object.object_field_occurrence_ids,
                            &occurrence.occurrence_id,
                        ) != 1
                    })
                {
                    metrics.target_incidence_failures += 1;
                }
            }
            OccurrenceSource::SemanticRegion { region_address } => {
                if regions.get(region_address).is_none_or(|region| {
                    occurrence_count(&region.outgoing_occurrence_ids, &occurrence.occurrence_id)
                        != 1
                }) {
                    metrics.region_source_incidence_failures += 1;
                }
            }
            OccurrenceSource::SemanticUnit { unit_id } => {
                let valid = units.get(unit_id).is_some_and(|unit| {
                    let span_ok = match (&occurrence.source_span, &unit.source_provenance) {
                        (
                            Some(occurrence_span),
                            RecordProvenance::SemanticUnit {
                                source_span: Some(unit_span),
                                ..
                            },
                        ) => span_contains(unit_span, occurrence_span),
                        _ => false,
                    };
                    span_ok
                        && occurrence_count(
                            &unit.outgoing_occurrence_ids,
                            &occurrence.occurrence_id,
                        ) == 1
                });
                if !valid {
                    metrics.semantic_unit_source_attribution_failures += 1;
                }
            }
        }
        if let Some(target) = &occurrence.resolved_target {
            let valid = match target {
                SemanticAddress::Object(object_id) => projection.objects.iter().any(|object| {
                    &object.object_id == object_id
                        && occurrence_count(
                            &object.incoming_occurrence_ids,
                            &occurrence.occurrence_id,
                        ) == 1
                }),
                SemanticAddress::Region(address) => regions.get(address).is_some_and(|region| {
                    occurrence_count(&region.incoming_occurrence_ids, &occurrence.occurrence_id)
                        == 1
                }),
                SemanticAddress::Unit(unit_id) => units.get(unit_id).is_some_and(|unit| {
                    occurrence_count(&unit.incoming_occurrence_ids, &occurrence.occurrence_id) == 1
                }),
                _ => false,
            };
            if !valid {
                metrics.target_incidence_failures += 1;
            }
        }
    }
    if metrics.semantic_unit_source_attribution_failures
        + metrics.semantic_unit_outgoing_incidence_failures
        + metrics.unresolved_block_target_degradations
        + metrics.identifier_descriptor_assignment_failures
        + metrics.inherited_assignment_reference_failures
        + metrics.unit_identity_duplicates
        + metrics.region_source_incidence_failures
        + metrics.target_incidence_failures
        + metrics.unit_parent_failures
        + metrics.region_containment_failures
        + metrics.region_inherited_assignment_failures
        + metrics.excluded_region_inheritance
        + metrics.block_mapping_missing
        + metrics.block_mapping_duplicates
        + metrics.block_mapping_wrong_region
        + metrics.block_mapping_missing_target
        + metrics.assignment_mode_failures
        + metrics.retrieval_affordance_failures
        + metrics.class_applicability_failures
        + metrics.present_null_temporal_anchor_failures
        != 0
    {
        return Err(ConstructionError::Contract(format!(
            "projection closure failed: {metrics:?}"
        )));
    }
    Ok(metrics)
}
fn block_type(kind: &str, raw: &str) -> Result<AuthoredBlockType, ConstructionError> {
    match kind {
        "paragraph" => Ok(AuthoredBlockType::Paragraph),
        "list" => Ok(AuthoredBlockType::List),
        "blockquote_or_callout"
            if raw
                .lines()
                .any(|line| line.trim_start().starts_with("> [!")) =>
        {
            Ok(AuthoredBlockType::Callout)
        }
        "blockquote_or_callout" if raw.lines().any(|line| line.trim_start().starts_with('>')) => {
            Ok(AuthoredBlockType::BlockQuote)
        }
        "code_fence" => Ok(AuthoredBlockType::CodeBlock),
        "table" => Ok(AuthoredBlockType::Table),
        "equation" => Ok(AuthoredBlockType::Equation),
        "callout" => Ok(AuthoredBlockType::Callout),
        "embedded_media" => Ok(AuthoredBlockType::EmbeddedMedia),
        "heading" => Ok(AuthoredBlockType::Paragraph),
        _ => Err(ConstructionError::Contract(format!(
            "unsupported authored block kind: {kind}"
        ))),
    }
}
fn temporal_value(value: &Value) -> Option<TemporalValue> {
    match value {
        Value::String(value) if !value.is_empty() => Some(TemporalValue::Label(value.clone())),
        Value::Number(value) => value.as_i64().map(TemporalValue::Ordinal),
        _ => None,
    }
}
fn candidate_path(link: &Value) -> Option<String> {
    link.get("target_candidates")?
        .get("candidate_source_paths")?
        .as_array()?
        .first()?
        .as_str()
        .map(str::to_owned)
}

fn object_class_name(record: &Value) -> String {
    record
        .get("frontmatter")
        .and_then(|frontmatter| frontmatter.get("values"))
        .and_then(|values| values.get("note_type"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| "admitted_markdown".into())
}

/// Constructs a semantic-unit identity from the accepted canonical structure.
/// Source paths, byte offsets, and block content remain provenance or hydration
/// data and never substitute for the canonical object/region/ordinal tuple.
pub fn canonical_unit_id(
    object_id: &SemanticObjectId,
    region_address: &SemanticRegionAddress,
    block_ordinal: u32,
    explicit_block_id: Option<&str>,
) -> Result<SemanticUnitId, EmptyIdentityError> {
    fn component(value: &str) -> String {
        format!("{}:{}", value.len(), value)
    }
    let explicit = explicit_block_id
        .map(|value| format!("1:{}", component(value)))
        .unwrap_or_else(|| "0".into());
    SemanticUnitId::parse(format!(
        "unit-v2:{}:{}:{}:{}",
        component(&object_id.to_string()),
        component(&region_address.authored_structural_address),
        block_ordinal,
        explicit
    ))
}

/// Join accepted heading-match provenance to an existing materialized region.
pub fn join_region_by_exact_span(
    regions: &[SemanticRegionRecord],
    object_id: &SemanticObjectId,
    matched_span: &SourceSpan,
) -> Result<SemanticRegionAddress, ConstructionError> {
    let matches: Vec<_> = regions
        .iter()
        .filter(|region| {
            &region.address.object_id == object_id
                && region.source_span.as_ref().is_some_and(|span| {
                    span.start_byte == matched_span.start_byte
                        && span.end_byte == matched_span.end_byte
                })
        })
        .collect();
    match matches.as_slice() {
        [region] => Ok(region.address.clone()),
        [] => Err(ConstructionError::Contract(
            "heading target span matched zero materialized semantic regions".into(),
        )),
        _ => Err(ConstructionError::Contract(
            "heading target span matched multiple materialized semantic regions".into(),
        )),
    }
}

/// Resolve one ordinary body occurrence to the unique authored unit whose
/// factual source span contains it. The caller must not substitute a parent
/// object, heading region, or arbitrary neighboring unit on failure.
pub fn containing_unit_for_span(
    units: &[(SemanticUnitId, SourceSpan)],
    occurrence_span: &SourceSpan,
) -> Result<SemanticUnitId, ConstructionError> {
    let matches: Vec<_> = units
        .iter()
        .filter(|(_, unit_span)| span_contains(unit_span, occurrence_span))
        .map(|(unit_id, _)| unit_id.clone())
        .collect();
    match matches.as_slice() {
        [unit_id] => Ok(unit_id.clone()),
        [] => Err(ConstructionError::Contract(
            "ordinary body occurrence has zero containing semantic units".into(),
        )),
        _ => Err(ConstructionError::Contract(
            "ordinary body occurrence has multiple containing semantic units".into(),
        )),
    }
}

/// Resolve an explicit block target without degrading an unknown block to its
/// parent object. Duplicate claims are rejected while the mapping is built.
pub fn resolve_explicit_block_target(
    mapping: &BTreeMap<(String, String), SemanticUnitId>,
    target_path: &str,
    block_id: &str,
) -> Option<SemanticUnitId> {
    mapping
        .get(&(target_path.to_owned(), block_id.to_owned()))
        .cloned()
}

/// Construct a private projection and a repository-safe numerical report.
pub fn construct(observation_path: &Path, output_path: &Path) -> Result<Value, ConstructionError> {
    let root: Value = serde_json::from_str(&fs::read_to_string(observation_path)?)?;
    if text(&root, "observation_schema_version") != "vault-observation/v2" {
        return Err(ConstructionError::Contract(
            "observer schema is not vault-observation/v2".into(),
        ));
    }
    let snapshot = text(&root, "vault_resident_snapshot_identity");
    let markdown = array(&root, "markdown_observations");
    let markdown_count = markdown.len();
    let mut admitted = Vec::new();
    for m in markdown {
        let path = text(m.get("source").unwrap_or(&Value::Null), "relative_path");
        if !path.starts_with("VAULT DESIGN/") {
            admitted.push(m);
        }
    }
    let mut by_path = BTreeMap::<String, SemanticObjectId>::new();
    let mut objects = Vec::new();
    for m in &admitted {
        let source = m.get("source").unwrap();
        let uuid = text(m.get("uuid").unwrap(), "parsed_value");
        let object_id: SemanticObjectId = id(uuid, "object UUID")?;
        let path = text(source, "relative_path");
        by_path.insert(path.clone(), object_id.clone());
        let fm = m.get("frontmatter").unwrap();
        let values = fm
            .get("values")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let title = values
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_else(|| {
                source
                    .get("basename")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            })
            .to_owned();
        let aliases: Vec<String> = values
            .get("aliases")
            .map(|v| {
                v.as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(Value::as_str)
                            .map(str::to_owned)
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let region = SemanticRegionAddress::parse(object_id.clone(), "root")
            .map_err(|e| ConstructionError::Contract(e.to_string()))?;
        objects.push((m, object_id, path, title, aliases, region));
    }
    let mut regions = Vec::new();
    let mut units = Vec::new();
    let mut assignments = Vec::new();
    let mut occurrences = Vec::new();
    let mut temporal_anchors = Vec::new();
    let mut object_assignments: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut object_units: BTreeMap<String, Vec<SemanticUnitId>> = BTreeMap::new();
    let mut object_body_occ: BTreeMap<String, Vec<OccurrenceId>> = BTreeMap::new();
    let mut object_field_occ: BTreeMap<String, Vec<OccurrenceId>> = BTreeMap::new();
    let mut object_temporal: BTreeMap<String, Vec<TemporalAnchorId>> = BTreeMap::new();
    let mut region_by_object_span: BTreeMap<(String, u64, u64), Vec<usize>> = BTreeMap::new();
    let mut region_index_by_object_address: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut region_bounds_by_object: BTreeMap<
        String,
        Vec<(SemanticRegionAddress, u64, u64, Vec<String>)>,
    > = BTreeMap::new();
    let mut region_heading_spans_by_object: BTreeMap<
        String,
        Vec<(SemanticRegionAddress, Option<SourceSpan>)>,
    > = BTreeMap::new();
    let mut unit_bounds_by_object: BTreeMap<String, Vec<(SemanticUnitId, SourceSpan)>> =
        BTreeMap::new();
    let mut unit_by_path_block_id: BTreeMap<(String, String), SemanticUnitId> = BTreeMap::new();
    let mut field_names = BTreeSet::new();
    let mut semantic_unit_source_count = 0usize;
    let mut zero_containing_unit_failures = 0usize;
    let mut multiple_containing_unit_failures = 0usize;
    let mut block_fragment_count = 0usize;
    let mut resolved_block_target_count = 0usize;
    let mut unresolved_block_target_count = 0usize;
    let mut explicit_block_id_count = 0usize;
    let mut region_block_mapping_count = 0usize;
    let mut block_kind_counts: BTreeMap<String, usize> = BTreeMap::new();
    // Materialize every region before resolving any cross-object occurrence.
    // Target objects may appear later in authored order than their sources.
    for (_m, oid, path, _title, _aliases, root_region) in &objects {
        let root_index = regions.len();
        region_index_by_object_address.insert(
            (
                oid.to_string(),
                root_region.authored_structural_address.clone(),
            ),
            root_index,
        );
        regions.push(SemanticRegionRecord {
            address: root_region.clone(),
            heading_path: vec![],
            heading_identity: format!(
                "region:{}",
                fnv(root_region.authored_structural_address.as_bytes())
            ),
            source_span: None,
            child_region_addresses: vec![],
            contained_unit_ids: vec![],
            block_target_mappings: vec![],
            incoming_occurrence_ids: vec![],
            outgoing_occurrence_ids: vec![],
            inherited_identifier_assignment_ids: vec![],
            retrieval_surface_ids: vec![],
        });
        let headings = array(_m, "headings");
        let mut authored_headings = Vec::with_capacity(headings.len());
        let mut parent_heading_indices = Vec::with_capacity(headings.len());
        let mut heading_stack: Vec<(u8, usize)> = Vec::new();
        for heading in &headings {
            let level = heading
                .get("level")
                .and_then(Value::as_u64)
                .ok_or_else(|| {
                    ConstructionError::Contract("heading observation lacks a positive level".into())
                })?;
            let level = u8::try_from(level).map_err(|_| {
                ConstructionError::Contract("heading level exceeds constructor domain".into())
            })?;
            while heading_stack
                .last()
                .is_some_and(|(parent_level, _)| *parent_level >= level)
            {
                heading_stack.pop();
            }
            parent_heading_indices.push(heading_stack.last().map(|(_, index)| *index));
            authored_headings.push(AuthoredRegionHeading {
                level,
                authored_structural_address: heading
                    .get("address_key")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        ConstructionError::Contract(
                            "heading observation lacks an authored structural address".into(),
                        )
                    })?
                    .to_owned(),
                source_span: span(heading.get("source_span").unwrap_or(&Value::Null), path),
            });
            heading_stack.push((level, authored_headings.len() - 1));
        }
        let identities = canonical_region_identities(oid.clone(), &authored_headings)
            .map_err(|error| ConstructionError::Contract(error.to_string()))?;
        for (index, (_heading, identity)) in headings.iter().zip(&identities).enumerate() {
            let address = identity.address.clone();
            let heading_span = identity.source_span.clone();
            let region_index = regions.len();
            region_index_by_object_address.insert(
                (oid.to_string(), address.authored_structural_address.clone()),
                region_index,
            );
            if let Some(heading_span) = heading_span.as_ref() {
                if let (Some(start), Some(end)) = (heading_span.start_byte, heading_span.end_byte) {
                    region_by_object_span
                        .entry((oid.to_string(), start, end))
                        .or_default()
                        .push(region_index);
                }
            }
            regions.push(SemanticRegionRecord {
                address: address.clone(),
                heading_path: identity.heading_path.clone(),
                heading_identity: format!(
                    "region:{}",
                    fnv(address.authored_structural_address.as_bytes())
                ),
                source_span: heading_span.clone(),
                child_region_addresses: vec![],
                contained_unit_ids: vec![],
                block_target_mappings: vec![],
                incoming_occurrence_ids: vec![],
                outgoing_occurrence_ids: vec![],
                inherited_identifier_assignment_ids: vec![],
                retrieval_surface_ids: vec![],
            });
            region_heading_spans_by_object
                .entry(oid.to_string())
                .or_default()
                .push((address.clone(), heading_span.clone()));
            let parent_index = parent_heading_indices[index]
                .map(|parent| root_index + 1 + parent)
                .unwrap_or(root_index);
            if !regions[parent_index]
                .child_region_addresses
                .contains(&address)
            {
                regions[parent_index]
                    .child_region_addresses
                    .push(address.clone());
            }
            if let Some(start) = heading_span.as_ref().and_then(|span| span.start_byte) {
                let level = authored_headings[index].level as u64;
                let end = headings
                    .iter()
                    .skip(index + 1)
                    .filter(|next| {
                        next.get("level")
                            .and_then(Value::as_u64)
                            .is_some_and(|next_level| next_level <= level)
                    })
                    .find_map(|next| {
                        span(next.get("source_span").unwrap_or(&Value::Null), path)
                            .and_then(|span| span.start_byte)
                    })
                    .unwrap_or_else(|| {
                        _m.get("raw_markdown")
                            .and_then(Value::as_str)
                            .map_or(start, |markdown| markdown.len() as u64)
                    });
                region_bounds_by_object
                    .entry(oid.to_string())
                    .or_default()
                    .push((address, start, end, identity.heading_path.clone()));
            }
        }
    }
    for (m, oid, path, _title, _aliases, root_region) in &objects {
        let fm = m.get("frontmatter").unwrap();
        let values = fm
            .get("values")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        for key in values.keys() {
            field_names.insert(key.clone());
        }
        let heading_regions = region_heading_spans_by_object
            .get(&oid.to_string())
            .cloned()
            .unwrap_or_default();
        let mut local_units = Vec::new();
        let mut region_ordinals: BTreeMap<String, u32> = BTreeMap::new();
        for block in array(m, "block_candidates")
            .into_iter()
            .filter(|b| text(b, "block_kind_observation") != "heading")
        {
            let block_kind = text(&block, "block_kind_observation");
            *block_kind_counts.entry(block_kind.clone()).or_default() += 1;
            let raw_block = text(&block, "raw_markdown");
            let authored_block_type = block_type(&block_kind, &raw_block)?;
            let source_span = span(block.get("source_span").unwrap_or(&Value::Null), path);
            let (parent_region_address, heading_path) = source_span
                .as_ref()
                .and_then(|span| span.start_byte)
                .and_then(|start| {
                    region_bounds_by_object
                        .get(&oid.to_string())
                        .and_then(|bounds| {
                            bounds
                                .iter()
                                .filter(|(_, region_start, region_end, _)| {
                                    *region_start <= start && start < *region_end
                                })
                                .max_by_key(|(_, region_start, _, _)| *region_start)
                                .map(|(address, _, _, heading_path)| {
                                    (address.clone(), heading_path.clone())
                                })
                        })
                })
                .unwrap_or_else(|| (root_region.clone(), Vec::new()));
            let ordinal_key = parent_region_address.authored_structural_address.clone();
            let block_ordinal = {
                let next = region_ordinals.entry(ordinal_key).or_insert(0);
                *next += 1;
                *next
            };
            let explicit_block_id = array(&block, "explicit_block_ids")
                .first()
                .and_then(Value::as_str)
                .map(str::to_owned);
            if explicit_block_id.is_some() {
                explicit_block_id_count += 1;
            }
            let unit_id: SemanticUnitId = id(
                canonical_unit_id(
                    oid,
                    &parent_region_address,
                    block_ordinal,
                    explicit_block_id.as_deref(),
                )
                .map_err(|error| ConstructionError::Contract(error.to_string()))?
                .to_string(),
                "unit identity",
            )?;
            if let Some(source_span) = source_span.clone() {
                unit_bounds_by_object
                    .entry(oid.to_string())
                    .or_default()
                    .push((unit_id.clone(), source_span));
            }
            local_units.push(unit_id.clone());
            for block_id in array(&block, "explicit_block_ids") {
                if let Some(block_id) = block_id.as_str() {
                    if unit_by_path_block_id
                        .insert((path.clone(), block_id.to_owned()), unit_id.clone())
                        .is_some()
                    {
                        return Err(ConstructionError::Contract(format!(
                            "multiple semantic units claim explicit block target in {path}"
                        )));
                    }
                    let region = regions
                        .iter_mut()
                        .find(|region| region.address == parent_region_address)
                        .ok_or_else(|| {
                            ConstructionError::Contract(
                                "explicit block target has no canonical parent region".into(),
                            )
                        })?;
                    region.block_target_mappings.push(BlockTargetMapping {
                        authored_block_id: block_id.to_owned(),
                        target_unit_id: unit_id.clone(),
                    });
                    region_block_mapping_count += 1;
                }
            }
            units.push(SemanticUnitRecord {
                unit_id: unit_id.clone(),
                parent_object_id: oid.clone(),
                parent_region_address,
                authored_block_type,
                heading_path,
                block_ordinal,
                explicit_block_id,
                content: SemanticUnitContent::HydrationAddress {
                    address: format!(
                        "source:{}#bytes:{}:{}",
                        path,
                        source_span.as_ref().and_then(|s| s.start_byte).unwrap_or(0),
                        source_span.as_ref().and_then(|s| s.end_byte).unwrap_or(0)
                    ),
                    content_hash: text(m.get("source").unwrap(), "source_byte_hash"),
                },
                inherited_identifier_assignment_ids: vec![],
                unit_local_identifier_assignment_ids: vec![],
                outgoing_occurrence_ids: vec![],
                incoming_occurrence_ids: vec![],
                temporal_anchor_ids: vec![],
                retrieval_surface_ids: vec![],
                source_provenance: RecordProvenance::SemanticUnit {
                    unit_id: unit_id.clone(),
                    source_span,
                },
                transport_segments: vec![],
            });
        }
        object_units.insert(oid.to_string(), local_units);
        for (key, raw) in values {
            if EXCLUDED_FIELDS.contains(&key.as_str()) {
                continue;
            }
            let aid = format!("assignment:{}:{}", oid, key);
            assignments.push(IdentifierAssignment {
                assignment_id: aid.clone(),
                identifier_name: key.clone(),
                subject: SemanticAddress::Object(oid.clone()),
                value: typed_value(&raw)?,
                authored_raw_value: Some(raw.clone()),
                provenance: RecordProvenance::ObjectField {
                    object_id: oid.clone(),
                    field_path: key.clone(),
                },
            });
            if [
                "journal_entry_date",
                "birthday",
                "first_met",
                "original_year_published",
            ]
            .contains(&key.as_str())
            {
                if let Some(value) = temporal_value(&raw) {
                    let anchor_id: TemporalAnchorId = id(
                        format!("anchor:{}:{}", oid, key),
                        "temporal anchor identity",
                    )?;
                    temporal_anchors.push(TemporalAnchorRecord {
                        anchor_id: anchor_id.clone(),
                        subject: SemanticAddress::Object(oid.clone()),
                        value,
                        provenance: RecordProvenance::ObjectField {
                            object_id: oid.clone(),
                            field_path: key.clone(),
                        },
                    });
                    object_temporal
                        .entry(oid.to_string())
                        .or_default()
                        .push(anchor_id);
                }
            }
            object_assignments
                .entry(oid.to_string())
                .or_default()
                .push(aid);
        }
        for link in array(m, "authored_links") {
            let source_span = span(link.get("source_span").unwrap_or(&Value::Null), path);
            let candidate_source_path = candidate_path(&link);
            let target_object = candidate_source_path
                .as_ref()
                .and_then(|p| by_path.get(p).cloned());
            let occurrence_id: OccurrenceId = id(
                format!(
                    "occurrence:{}:{}",
                    oid,
                    fnv(serde_json::to_string(&link).unwrap_or_default().as_bytes())
                ),
                "occurrence identity",
            )?;
            let source_surface = text(&link, "source_surface");
            let source = if source_surface == "frontmatter" {
                OccurrenceSource::ObjectField {
                    object_id: oid.clone(),
                    field_path: text(&link, "frontmatter_key_path"),
                }
            } else if let Some((region_address, _)) =
                heading_regions.iter().find(|(_, marker_span)| {
                    marker_span.as_ref().zip(source_span.as_ref()).is_some_and(
                        |(marker, occurrence)| {
                            marker.start_byte <= occurrence.start_byte
                                && marker.end_byte >= occurrence.end_byte
                        },
                    )
                })
            {
                OccurrenceSource::SemanticRegion {
                    region_address: region_address.clone(),
                }
            } else {
                let occurrence_span = source_span.as_ref().ok_or_else(|| {
                    ConstructionError::Contract(format!(
                        "ordinary body occurrence lacks source span for {path}"
                    ))
                })?;
                let unit_id = containing_unit_for_span(
                    unit_bounds_by_object
                        .get(&oid.to_string())
                        .map(Vec::as_slice)
                        .unwrap_or(&[]),
                    occurrence_span,
                )
                .map_err(|error| {
                    if error.to_string().contains("zero") {
                        zero_containing_unit_failures += 1;
                    } else {
                        multiple_containing_unit_failures += 1;
                    }
                    ConstructionError::Contract(format!("{error} for {path}"))
                })?;
                semantic_unit_source_count += 1;
                OccurrenceSource::SemanticUnit { unit_id }
            };
            let target = if let Some(target_object) = target_object {
                if link
                    .get("heading_fragment")
                    .and_then(Value::as_str)
                    .is_some()
                {
                    let matches = array(&link, "heading_target_matches");
                    let matched_span = match matches.as_slice() {
                        [matched_heading] => {
                            span_bounds(matched_heading.get("source_span").unwrap_or(&Value::Null))
                                .ok_or_else(|| {
                                    ConstructionError::Contract(
                                        "heading target match lacks an exact source span".into(),
                                    )
                                })?
                        }
                        [] => {
                            return Err(ConstructionError::Contract(
                                "heading target has no accepted heading match".into(),
                            ));
                        }
                        _ => {
                            return Err(ConstructionError::Contract(
                                "heading target has multiple accepted heading matches".into(),
                            ));
                        }
                    };
                    let key = (target_object.to_string(), matched_span.0, matched_span.1);
                    let region_indices = region_by_object_span.get(&key).ok_or_else(|| {
                        ConstructionError::Contract(
                            "heading target span matched zero materialized semantic regions".into(),
                        )
                    })?;
                    let region_index = match region_indices.as_slice() {
                        [region_index] => *region_index,
                        _ => {
                            return Err(ConstructionError::Contract(
                                "heading target span matched multiple materialized semantic regions".into(),
                            ));
                        }
                    };
                    Some(SemanticAddress::Region(
                        regions[region_index].address.clone(),
                    ))
                } else if let Some(fragment) = link.get("block_fragment").and_then(Value::as_str) {
                    block_fragment_count += 1;
                    let mapping = candidate_source_path.as_ref().and_then(|p| {
                        resolve_explicit_block_target(&unit_by_path_block_id, p, fragment)
                    });
                    match mapping {
                        Some(unit_id) => {
                            resolved_block_target_count += 1;
                            Some(SemanticAddress::Unit(unit_id))
                        }
                        None => {
                            unresolved_block_target_count += 1;
                            None
                        }
                    }
                } else {
                    Some(SemanticAddress::Object(target_object))
                }
            } else {
                None
            };
            let mode = if link
                .get("embedded")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                OccurrencePresentation::Embed
            } else {
                OccurrencePresentation::Link
            };
            if let OccurrenceSource::SemanticRegion { region_address } = &source {
                if let Some(region) = regions
                    .iter_mut()
                    .find(|region| &region.address == region_address)
                {
                    region.outgoing_occurrence_ids.push(occurrence_id.clone());
                }
            }
            if source_surface == "frontmatter" {
                object_field_occ
                    .entry(oid.to_string())
                    .or_default()
                    .push(occurrence_id.clone());
            } else {
                object_body_occ
                    .entry(oid.to_string())
                    .or_default()
                    .push(occurrence_id.clone());
            }
            occurrences.push(OccurrenceRecord {
                occurrence_id,
                source,
                authored_target_text: text(&link, "raw_target"),
                display_alias: link
                    .get("display_alias")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                resolved_target: target,
                presentation_mode: mode,
                direction: Direction::Outgoing,
                source_span,
            });
        }
    }
    let admitted_fields: Vec<String> = field_names
        .iter()
        .filter(|k| !EXCLUDED_FIELDS.contains(&k.as_str()))
        .cloned()
        .collect();
    if field_names.len() != 60 {
        return Err(ConstructionError::Contract(format!(
            "observed field universe is {}, expected 60",
            field_names.len()
        )));
    }
    let descriptors = admitted_fields
        .iter()
        .map(|name| {
            let role = descriptor_role(name);
            IdentifierDescriptor {
                identifier_name: name.clone(),
                semantic_role: role.clone(),
                value_shape: descriptor_shape(&assignments, name),
                cardinality: descriptor_cardinality(&assignments, name),
                applicable_address_kinds: vec![
                    AddressKind::SemanticObject,
                    AddressKind::SemanticRegion,
                    AddressKind::SemanticUnit,
                ],
                assignment_mode: descriptor_assignment_mode(&role),
                source_surface: format!("frontmatter.{name}"),
                may_contain_canonical_links: ["book_read_today", "dream_motif"]
                    .contains(&name.as_str()),
                temporal_affordance: if [
                    "journal_entry_date",
                    "birthday",
                    "first_met",
                    "original_year_published",
                ]
                .contains(&name.as_str())
                {
                    TemporalAffordance::CreatesAnchor
                } else {
                    TemporalAffordance::None
                },
                retrieval_surface_ids: {
                    let mut surfaces = vec!["surface:exact".into(), "surface:lexical".into()];
                    if matches!(role, IdentifierRole::TemporalAnchoring) {
                        surfaces.push("surface:temporal".into());
                    }
                    surfaces
                },
                enabled_transition_ids: vec!["transition:identifier".into()],
            }
        })
        .collect();
    let surfaces = [
        (
            "surface:exact",
            RetrievalSurfaceKind::Exact,
            SurfaceMatchMode::Literal,
        ),
        (
            "surface:lexical",
            RetrievalSurfaceKind::Lexical,
            SurfaceMatchMode::Terms,
        ),
        (
            "surface:vector",
            RetrievalSurfaceKind::Vector,
            SurfaceMatchMode::NearestNeighbours,
        ),
        (
            "surface:graph",
            RetrievalSurfaceKind::Graph,
            SurfaceMatchMode::Incidence,
        ),
        (
            "surface:temporal",
            RetrievalSurfaceKind::Temporal,
            SurfaceMatchMode::Temporal,
        ),
    ]
    .into_iter()
    .map(|(id, kind, mode)| RetrievalSurfaceDescriptor {
        surface_id: id.into(),
        kind,
        available: false,
        visible_address_kinds: vec![
            AddressKind::SemanticObject,
            AddressKind::SemanticRegion,
            AddressKind::SemanticUnit,
            AddressKind::Identifier,
            AddressKind::Occurrence,
            AddressKind::TemporalAnchor,
        ],
        match_modes: vec![mode],
        default_candidate_limit: 0,
        hard_candidate_limit: 0,
        returned_identity: AddressKind::SemanticUnit,
        hydrates_to_semantic_units: false,
        coverage_semantics: CoverageSemantics::AvailabilityOnly,
        exhaustive_total_count_supported: false,
        continuation_supported: false,
        technical_limitations: vec![
            "Phase 5 representation only; no executable index or provider.".into(),
        ],
    })
    .collect();
    for unit in &units {
        if let Some(region) = regions
            .iter_mut()
            .find(|region| region.address == unit.parent_region_address)
        {
            region.contained_unit_ids.push(unit.unit_id.clone());
        }
    }
    for unit in &mut units {
        unit.inherited_identifier_assignment_ids = object_assignments
            .get(&unit.parent_object_id.to_string())
            .cloned()
            .unwrap_or_default();
    }
    for region in &mut regions {
        region.inherited_identifier_assignment_ids = object_assignments
            .get(&region.address.object_id.to_string())
            .cloned()
            .unwrap_or_default();
    }
    let mut object_class_applicability: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut object_classes_by_id = BTreeMap::new();
    for (m, oid, ..) in &objects {
        let class_name = object_class_name(m);
        object_classes_by_id.insert(oid.to_string(), class_name.clone());
        let class_fields = object_class_applicability.entry(class_name).or_default();
        if let Some(values) = m
            .get("frontmatter")
            .and_then(|frontmatter| frontmatter.get("values"))
            .and_then(Value::as_object)
        {
            class_fields.extend(
                values
                    .keys()
                    .filter(|field| !EXCLUDED_FIELDS.contains(&field.as_str()))
                    .cloned(),
            );
        }
    }
    let object_class_descriptors: Vec<_> = object_class_applicability
        .into_iter()
        .map(|(class_name, fields)| SemanticObjectClassDescriptor {
            class_name,
            applicable_identifier_names: fields.into_iter().collect(),
            permitted_source_kinds: vec![SourceKind::Markdown],
        })
        .collect();
    let mut object_records = Vec::new();
    for (_m, oid, path, title, aliases, _region) in &objects {
        object_records.push(SemanticObjectRecord {
            object_id: oid.clone(),
            source_identity: format!("source:{}", fnv(path.as_bytes())),
            source_kind: SourceKind::Markdown,
            canonical_path: path.clone(),
            filename: Path::new(path)
                .file_name()
                .and_then(|p| p.to_str())
                .unwrap_or_default()
                .into(),
            title: title.clone(),
            aliases: aliases.clone(),
            object_class: object_classes_by_id
                .get(&oid.to_string())
                .cloned()
                .unwrap_or_else(|| "admitted_markdown".into()),
            region_addresses: regions
                .iter()
                .filter(|region| region.address.object_id == *oid)
                .map(|region| region.address.clone())
                .collect(),
            unit_ids: object_units.remove(&oid.to_string()).unwrap_or_default(),
            identifier_assignment_ids: object_assignments
                .remove(&oid.to_string())
                .unwrap_or_default(),
            object_field_occurrence_ids: object_field_occ
                .remove(&oid.to_string())
                .unwrap_or_default(),
            body_occurrence_ids: object_body_occ.remove(&oid.to_string()).unwrap_or_default(),
            incoming_occurrence_ids: vec![],
            temporal_anchor_ids: object_temporal.remove(&oid.to_string()).unwrap_or_default(),
            retrieval_surface_ids: vec![],
        });
    }
    let mut projection = SemanticSpaceProjection {
        projection_snapshot_id: format!("projection:phase5:{snapshot}"),
        ingest_identity: format!("observer:99d0d4556684000f0ed585e47158a5f7fe9ce7e1"),
        schema_version: "semantic-space-projection/v1".into(),
        logical_hash: String::new(),
        corpus_snapshot_identity: snapshot.clone(),
        configuration_snapshot_id: "phase5:construction:no-indexes".into(),
        validation_status: ProjectionValidationStatus::Unvalidated,
        object_classes: object_class_descriptors,
        objects: object_records,
        regions,
        units,
        identifier_descriptors: descriptors,
        identifier_assignments: assignments,
        occurrences,
        temporal_anchors,
        retrieval_surfaces: surfaces,
        valid_transitions: vec![],
    };
    for occurrence in &projection.occurrences {
        if let Some(target) = &occurrence.resolved_target {
            match target {
                SemanticAddress::Object(object_id) => {
                    if let Some(object) = projection
                        .objects
                        .iter_mut()
                        .find(|object| &object.object_id == object_id)
                    {
                        object
                            .incoming_occurrence_ids
                            .push(occurrence.occurrence_id.clone());
                    }
                }
                SemanticAddress::Region(region_address) => {
                    if let Some(region) = projection
                        .regions
                        .iter_mut()
                        .find(|region| &region.address == region_address)
                    {
                        region
                            .incoming_occurrence_ids
                            .push(occurrence.occurrence_id.clone());
                    }
                }
                SemanticAddress::Unit(unit_id) => {
                    if let Some(unit) = projection
                        .units
                        .iter_mut()
                        .find(|unit| &unit.unit_id == unit_id)
                    {
                        unit.incoming_occurrence_ids
                            .push(occurrence.occurrence_id.clone());
                    }
                }
                _ => {}
            }
        }
        match &occurrence.source {
            OccurrenceSource::SemanticUnit { unit_id } => {
                if let Some(unit) = projection
                    .units
                    .iter_mut()
                    .find(|unit| &unit.unit_id == unit_id)
                {
                    unit.outgoing_occurrence_ids
                        .push(occurrence.occurrence_id.clone());
                }
            }
            OccurrenceSource::SemanticRegion { .. } | OccurrenceSource::ObjectField { .. } => {}
        }
    }
    let closure = validate_projection(&projection)?;
    let canonical = serde_json::to_vec(&projection)?;
    projection.logical_hash = format!("fnv1a:{}", fnv(&canonical));
    fs::write(output_path, serde_json::to_vec_pretty(&projection)?)?;
    let unresolved = projection
        .occurrences
        .iter()
        .filter(|o| o.resolved_target.is_none())
        .count();
    let resolved = projection.occurrences.len() - unresolved;
    let object_field_count = projection
        .occurrences
        .iter()
        .filter(|o| matches!(o.source, OccurrenceSource::ObjectField { .. }))
        .count();
    let region_count = projection
        .occurrences
        .iter()
        .filter(|o| matches!(o.source, OccurrenceSource::SemanticRegion { .. }))
        .count();
    let unit_count = projection
        .occurrences
        .iter()
        .filter(|o| matches!(o.source, OccurrenceSource::SemanticUnit { .. }))
        .count();
    let whole_resident_source_count = root
        .get("file_observations")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let inherited_assignment_reference_count: usize = projection
        .units
        .iter()
        .map(|unit| unit.inherited_identifier_assignment_ids.len())
        .sum();
    let region_inherited_assignment_reference_count: usize = projection
        .regions
        .iter()
        .map(|region| region.inherited_identifier_assignment_ids.len())
        .sum();
    let present_null_temporal_assignment_count = projection
        .identifier_assignments
        .iter()
        .filter(|assignment| matches!(assignment.value, IdentifierValue::Null))
        .filter(|assignment| {
            projection
                .identifier_descriptors
                .iter()
                .find(|descriptor| descriptor.identifier_name == assignment.identifier_name)
                .is_some_and(|descriptor| {
                    matches!(
                        descriptor.temporal_affordance,
                        TemporalAffordance::CreatesAnchor
                    )
                })
        })
        .count();
    let block_mapping_failures = closure.block_mapping_missing
        + closure.block_mapping_duplicates
        + closure.block_mapping_wrong_region
        + closure.block_mapping_missing_target;
    let report = serde_json::json!({
        "report_title": "PHASE 5 CONSTRUCTION EVIDENCE",
        "input_identity": {
            "observer_repository": "duck-lint/semantic-traversal",
            "observer_commit": "99d0d4556684000f0ed585e47158a5f7fe9ce7e1",
            "observer_schema": "vault-observation/v2",
            "corpus_snapshot_identity": snapshot
        },
        "admission": {
            "whole_resident_source_count": whole_resident_source_count,
            "resident_markdown_count": markdown_count,
            "admission_eligible_count": projection.objects.len(),
            "admitted_object_count": projection.objects.len(),
            "excluded_source_count": markdown_count.saturating_sub(admitted.len())
        },
        "projection_construction": {
            "canonical_object_count": projection.objects.len(),
            "canonical_region_count": projection.regions.len(),
            "canonical_semantic_unit_count": projection.units.len(),
            "unit_identity_basis": "canonical object UUID + canonical parent region + region-local block ordinal + explicit authored block ID when present",
            "explicit_block_id_participation": true
        },
        "identifier_materialization": {
            "descriptor_count": projection.identifier_descriptors.len(),
            "assignment_count": projection.identifier_assignments.len(),
            "present_null_assignment_count": projection.identifier_assignments.iter().filter(|a| matches!(a.value, IdentifierValue::Null)).count(),
            "admitted_field_coverage": projection.identifier_descriptors.len(),
            "excluded_field_count": EXCLUDED_FIELDS.len(),
            "inherited_assignment_reference_count": inherited_assignment_reference_count,
            "region_inherited_assignment_reference_count": region_inherited_assignment_reference_count
        },
        "occurrences": {
            "authored_occurrence_count": projection.occurrences.len(),
            "object_field_count": object_field_count,
            "semantic_region_count": region_count,
            "semantic_unit_count": unit_count,
            "resolved_count": resolved,
            "unresolved_count": unresolved,
            "ambiguous_count": 0,
            "semantic_unit_source_attribution_failures": zero_containing_unit_failures + multiple_containing_unit_failures,
            "correct_containing_unit_attributions": semantic_unit_source_count,
            "zero_containing_unit_failures": zero_containing_unit_failures,
            "multiple_containing_unit_failures": multiple_containing_unit_failures,
            "block_fragment_count": block_fragment_count,
            "resolved_block_target_count": resolved_block_target_count,
            "unresolved_block_target_count": unresolved_block_target_count,
            "object_fallback_degradations": closure.unresolved_block_target_degradations
        },
        "authored_block_kinds": {
            "distribution": block_kind_counts,
            "explicit_block_ids": explicit_block_id_count,
            "region_block_target_mappings": region_block_mapping_count,
            "unsupported_block_kinds": 0,
            "collapsed_block_kinds": 0
        },
        "closure": {
            "semantic_unit_source_attribution_failures": closure.semantic_unit_source_attribution_failures,
            "semantic_unit_outgoing_incidence_failures": closure.semantic_unit_outgoing_incidence_failures,
            "unresolved_block_target_degradations": closure.unresolved_block_target_degradations,
            "identifier_descriptor_assignment_conformance_failures": closure.identifier_descriptor_assignment_failures,
            "inherited_assignment_reference_failures": closure.inherited_assignment_reference_failures,
            "unit_identity_duplicates": closure.unit_identity_duplicates,
            "region_source_incidence_failures": closure.region_source_incidence_failures,
            "target_incidence_failures": closure.target_incidence_failures,
            "unit_parent_failures": closure.unit_parent_failures,
            "region_containment_failures": closure.region_containment_failures,
            "region_inherited_assignment_failures": closure.region_inherited_assignment_failures,
            "excluded_region_inheritance": closure.excluded_region_inheritance,
            "block_mapping_missing": closure.block_mapping_missing,
            "block_mapping_duplicates": closure.block_mapping_duplicates,
            "block_mapping_wrong_region": closure.block_mapping_wrong_region,
            "block_mapping_missing_target": closure.block_mapping_missing_target,
            "block_mapping_failures": block_mapping_failures,
            "assignment_mode_failures": closure.assignment_mode_failures,
            "retrieval_affordance_failures": closure.retrieval_affordance_failures,
            "class_applicability_failures": closure.class_applicability_failures,
            "present_null_temporal_anchor_failures": closure.present_null_temporal_anchor_failures
        },
        "temporal": {
            "temporally_capable_descriptors": projection.identifier_descriptors.iter().filter(|d| matches!(d.temporal_affordance, TemporalAffordance::CreatesAnchor)).count(),
            "materially_created_temporal_anchor_count": projection.temporal_anchors.len(),
            "present_null_temporal_assignments_creating_no_anchor": present_null_temporal_assignment_count,
            "present_null_temporal_assignments_incorrectly_anchored": closure.present_null_temporal_anchor_failures
        },
        "construction_status": "produced",
        "contract_contact_failures": 0,
        "projection": {
            "schema_version": projection.schema_version,
            "logical_hash": projection.logical_hash,
            "validation_status": "unvalidated"
        },
        "determinism": {
            "first_run_logical_hash": projection.logical_hash,
            "second_run_logical_hash": "rerun-required",
            "equality_result": "not_run_by_library"
        }
    });
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_block_kinds_do_not_collapse_or_fallback() {
        assert_eq!(
            block_type("paragraph", "body").unwrap(),
            AuthoredBlockType::Paragraph
        );
        assert_eq!(
            block_type("blockquote_or_callout", "> quoted").unwrap(),
            AuthoredBlockType::BlockQuote
        );
        assert_eq!(
            block_type("blockquote_or_callout", "> [!NOTE]\n> callout").unwrap(),
            AuthoredBlockType::Callout
        );
        assert!(block_type("unknown_kind", "body").is_err());
    }

    #[test]
    fn assignment_mode_separates_authorship_from_visibility() {
        assert_eq!(
            descriptor_assignment_mode(&IdentifierRole::ObjectClass),
            IdentifierAssignmentMode::Intrinsic
        );
        assert_eq!(
            descriptor_assignment_mode(&IdentifierRole::BridgeConstitutive),
            IdentifierAssignmentMode::Intrinsic
        );
        assert_eq!(
            descriptor_assignment_mode(&IdentifierRole::ContextualRelation),
            IdentifierAssignmentMode::Relational
        );
    }
}
