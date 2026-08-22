//! Phase 5 adapter from the factual `vault-observation/v3` bundle.
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
const ACCEPTED_ARTIFACT_SHA256: &str =
    "d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9";
const ACCEPTED_SPECIMEN_IDENTITY: &str =
    "f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4";
const ACCEPTED_FIELD_UNIVERSE: [&str; 60] = [
    "address",
    "aliases",
    "architect_or_operator",
    "birthday",
    "book_read_today",
    "bridge_applicability_scope",
    "bridge_applied",
    "bridge_broken",
    "bridge_conditions",
    "bridge_isomorphism",
    "bridge_justification",
    "bridge_methods",
    "bridge_preservation",
    "bridge_required",
    "canonical_name",
    "cash_out",
    "creator",
    "dislikes",
    "dream_location",
    "dream_lucidity",
    "dream_motif",
    "dream_motif_valence",
    "email",
    "entity_type",
    "first_met",
    "format",
    "from_mode",
    "from_register",
    "hypnagogic_resonance",
    "interface",
    "iso_broken",
    "iso_justification",
    "iso_structure",
    "journal_entry_date",
    "layer",
    "likes",
    "note_type",
    "occupation",
    "origin",
    "original_year_published",
    "phone",
    "pillar",
    "publish_studio",
    "quarantine_reasons",
    "reactivity",
    "recall_ability",
    "register",
    "register_mode",
    "relationship",
    "revision_triggers",
    "speculation_quarantine",
    "stop_rule",
    "tags",
    "temporal_pace",
    "title",
    "to_mode",
    "to_register",
    "unity_level",
    "uuid",
    "vector_direction",
];

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

// SHA-256 keeps hydration integrity independent from the containing note.
// This small implementation avoids introducing a provider or external tool
// into the Phase 5 constructor's deterministic boundary.
fn sha256(bytes: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (bytes.len() as u64) * 8;
    let mut data = bytes.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in data.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            (hh, g, f, e, d, c, b, a) = (
                g,
                f,
                e,
                d.wrapping_add(temp1),
                c,
                b,
                a,
                temp1.wrapping_add(temp2),
            );
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    h.iter().map(|word| format!("{word:08x}")).collect()
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

fn accepted_occurrence_semantics(name: &str) -> bool {
    matches!(name, "book_read_today" | "dream_motif")
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
    authority_class_applicability_failures: usize,
    authority_occurrence_capability_failures: usize,
    occurrence_resolution_state_mismatches: usize,
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
    let transition_ids: HashSet<_> = projection
        .valid_transitions
        .iter()
        .map(|t| t.transition_id.as_str())
        .collect();
    if transition_ids.len() != projection.valid_transitions.len() {
        return Err(ConstructionError::Contract(
            "duplicate structural transition identity".into(),
        ));
    }
    for descriptor in &projection.identifier_descriptors {
        for transition_id in &descriptor.enabled_transition_ids {
            if !transition_ids.contains(transition_id.as_str()) {
                return Err(ConstructionError::Contract(format!(
                    "unknown enabled transition: {transition_id}"
                )));
            }
        }
    }
    for transition in &projection.valid_transitions {
        if let Some(surface_id) = &transition.retrieval_surface_id {
            let surface = projection
                .retrieval_surfaces
                .iter()
                .find(|s| s.surface_id == *surface_id)
                .ok_or_else(|| {
                    ConstructionError::Contract(format!("unknown transition surface: {surface_id}"))
                })?;
            if !surface.visible_address_kinds.contains(&transition.from)
                || !surface.visible_address_kinds.contains(&transition.to)
            {
                return Err(ConstructionError::Contract(format!(
                    "transition {} is incompatible with surface {}",
                    transition.transition_id, surface_id
                )));
            }
        }
    }
    let mut unit_keys = HashSet::new();
    for unit in &projection.units {
        if !objects.contains(&unit.parent_object_id)
            || !regions.contains_key(&unit.parent_region_address)
            || !unit_keys.insert((unit.parent_region_address.clone(), unit.block_ordinal))
        {
            metrics.unit_parent_failures += 1;
        }
        if let Some(region) = regions.get(&unit.parent_region_address)
            && occurrence_count(&region.contained_unit_ids, &unit.unit_id) != 1
        {
            metrics.region_containment_failures += 1;
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
            if let Some(assignment) = assignments.get(assignment_id.as_str())
                && !class
                    .applicable_identifier_names
                    .contains(&assignment.identifier_name)
            {
                metrics.class_applicability_failures += 1;
            }
        }
    }
    for class in &projection.object_classes {
        if class
            .applicable_identifier_names
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>()
            != accepted_class_fields(&class.class_name)
        {
            metrics.authority_class_applicability_failures += 1;
        }
    }
    for descriptor in &projection.identifier_descriptors {
        let expected_mode = descriptor_assignment_mode(&descriptor.semantic_role);
        if descriptor.assignment_mode != expected_mode {
            metrics.assignment_mode_failures += 1;
        }
        let occurrence_capability = accepted_occurrence_semantics(&descriptor.identifier_name);
        if descriptor.may_contain_canonical_links != occurrence_capability
            || descriptor
                .retrieval_surface_ids
                .contains(&"surface:graph".to_owned())
                != occurrence_capability
            || descriptor
                .enabled_transition_ids
                .contains(&"transition:object-occurrence-outgoing".to_owned())
                != occurrence_capability
        {
            metrics.authority_occurrence_capability_failures += 1;
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
            && temporal_null_assignments.contains(&format!("{object_id}:{field_path}"))
        {
            metrics.present_null_temporal_anchor_failures += 1;
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
        let resolution_matches = matches!(
            (&occurrence.resolution_state, &occurrence.resolved_target),
            (OccurrenceResolutionState::Resolved, Some(_))
                | (OccurrenceResolutionState::Unresolved, None)
                | (OccurrenceResolutionState::Ambiguous { .. }, None)
        );
        if !resolution_matches {
            metrics.occurrence_resolution_state_mismatches += 1;
        }
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
        + metrics.authority_class_applicability_failures
        + metrics.authority_occurrence_capability_failures
        + metrics.occurrence_resolution_state_mismatches
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
fn temporal_value(
    field: &str,
    value: &Value,
    observed_shape: Option<&str>,
) -> Option<TemporalValue> {
    if value.is_null() {
        return None;
    }
    match (field, observed_shape, value) {
        ("birthday", Some("date"), Value::String(v)) => Some(TemporalValue::FullDate(v.clone())),
        ("birthday", _, Value::String(v)) if valid_month_day(v) => {
            Some(TemporalValue::MonthDay(v.clone()))
        }
        ("first_met", Some("date"), Value::String(v)) => Some(TemporalValue::FullDate(v.clone())),
        ("first_met", Some("datetime"), Value::String(v)) => {
            Some(TemporalValue::DateTime(v.clone()))
        }
        ("original_year_published", Some("number"), Value::Number(v)) => v
            .as_i64()
            .and_then(|n| i32::try_from(n).ok())
            .map(TemporalValue::ExactYear),
        ("original_year_published", Some("string"), Value::String(v))
            if valid_approximate_year(v) =>
        {
            Some(TemporalValue::ApproximateYear(v.clone()))
        }
        ("journal_entry_date", Some("date"), Value::String(v)) => {
            Some(TemporalValue::FullDate(v.clone()))
        }
        _ => None,
    }
}

fn valid_month_day(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 7
        || bytes[0] != b'-'
        || bytes[1] != b'-'
        || bytes[4] != b'-'
        || !bytes[2..4].iter().all(u8::is_ascii_digit)
        || !bytes[5..7].iter().all(u8::is_ascii_digit)
    {
        return false;
    }
    let month = u32::from(value.as_bytes()[2] - b'0') * 10 + u32::from(value.as_bytes()[3] - b'0');
    let day = u32::from(value.as_bytes()[5] - b'0') * 10 + u32::from(value.as_bytes()[6] - b'0');
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 29,
        4 | 6 | 9 | 11 => 30,
        _ => 0,
    };
    max_day != 0 && day >= 1 && day <= max_day
}

fn valid_approximate_year(value: &str) -> bool {
    let Some(year) = value.strip_prefix('~').and_then(|v| v.strip_suffix(" BCE")) else {
        return false;
    };
    !year.is_empty() && year.as_bytes().iter().all(u8::is_ascii_digit)
}
fn candidate_paths(link: &Value) -> Vec<String> {
    link.get("target_candidates")
        .and_then(|v| v.get("candidate_source_paths"))
        .and_then(Value::as_array)
        .map(|paths| {
            paths
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
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

/// Operator-authored class applicability. This is deliberately independent of
/// which fields happen to be present on today's objects.
fn accepted_class_fields(class_name: &str) -> BTreeSet<String> {
    const UNIVERSAL: &[&str] = &[
        "uuid",
        "note_type",
        "aliases",
        "tags",
        "layer",
        "unity_level",
        "vector_direction",
        "register",
        "register_mode",
        "pillar",
    ];
    const ENTITY: &[&str] = &[
        "canonical_name",
        "entity_type",
        "occupation",
        "relationship",
        "first_met",
        "birthday",
    ];
    const SOURCE: &[&str] = &[
        "title",
        "creator",
        "format",
        "publish_studio",
        "original_year_published",
        "origin",
    ];
    const JOURNAL: &[&str] = &[
        "journal_entry_date",
        "book_read_today",
        "dream_location",
        "dream_lucidity",
        "dream_motif",
        "dream_motif_valence",
        "hypnagogic_resonance",
        "reactivity",
        "recall_ability",
        "temporal_pace",
        "architect_or_operator",
    ];
    const BRIDGE: &[&str] = &[
        "bridge_applicability_scope",
        "bridge_applied",
        "bridge_broken",
        "bridge_conditions",
        "bridge_isomorphism",
        "bridge_justification",
        "bridge_methods",
        "bridge_preservation",
        "bridge_required",
        "cash_out",
        "from_mode",
        "from_register",
        "interface",
        "iso_broken",
        "iso_justification",
        "iso_structure",
        "quarantine_reasons",
        "revision_triggers",
        "speculation_quarantine",
        "stop_rule",
        "to_mode",
        "to_register",
    ];
    let mut fields: BTreeSet<String> = UNIVERSAL.iter().map(|s| (*s).into()).collect();
    match class_name {
        "entity" => fields.extend(ENTITY.iter().map(|s| (*s).into())),
        "source_material" => fields.extend(SOURCE.iter().map(|s| (*s).into())),
        "journal_entry" => fields.extend(JOURNAL.iter().map(|s| (*s).into())),
        "inferential_bridge" => fields.extend(BRIDGE.iter().map(|s| (*s).into())),
        _ => {}
    }
    fields
}

fn accepted_cardinality(name: &str, observed: IdentifierCardinality) -> IdentifierCardinality {
    match name {
        "creator" | "register_mode" | "from_mode" | "to_mode" | "unity_level" => {
            IdentifierCardinality::Collection
        }
        "format"
        | "layer"
        | "vector_direction"
        | "register"
        | "pillar"
        | "hypnagogic_resonance"
        | "reactivity"
        | "relationship" => IdentifierCardinality::Scalar,
        _ => observed,
    }
}

fn phase5_transitions() -> Vec<StructuralTransition> {
    let specs = [
        (
            "object-region",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::Containment,
            Direction::Outgoing,
            AddressKind::SemanticRegion,
            None,
        ),
        (
            "object-unit",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::Containment,
            Direction::Outgoing,
            AddressKind::SemanticUnit,
            None,
        ),
        (
            "region-unit",
            AddressKind::SemanticRegion,
            StructuralTransitionOperation::Containment,
            Direction::Outgoing,
            AddressKind::SemanticUnit,
            None,
        ),
        (
            "unit-object",
            AddressKind::SemanticUnit,
            StructuralTransitionOperation::Parent,
            Direction::Outgoing,
            AddressKind::SemanticObject,
            None,
        ),
        (
            "unit-region",
            AddressKind::SemanticUnit,
            StructuralTransitionOperation::Parent,
            Direction::Outgoing,
            AddressKind::SemanticRegion,
            None,
        ),
        (
            "object-occurrence-outgoing",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "region-occurrence-outgoing",
            AddressKind::SemanticRegion,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "unit-occurrence-outgoing",
            AddressKind::SemanticUnit,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "occurrence-object-target",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::SemanticObject,
            Some("surface:graph"),
        ),
        (
            "occurrence-region-target",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::SemanticRegion,
            Some("surface:graph"),
        ),
        (
            "occurrence-unit-target",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Outgoing,
            AddressKind::SemanticUnit,
            Some("surface:graph"),
        ),
        (
            "object-occurrence-incoming",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "region-occurrence-incoming",
            AddressKind::SemanticRegion,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "unit-occurrence-incoming",
            AddressKind::SemanticUnit,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::Occurrence,
            Some("surface:graph"),
        ),
        (
            "occurrence-object-source",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::SemanticObject,
            Some("surface:graph"),
        ),
        (
            "occurrence-region-source",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::SemanticRegion,
            Some("surface:graph"),
        ),
        (
            "occurrence-unit-source",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Occurrence,
            Direction::Incoming,
            AddressKind::SemanticUnit,
            Some("surface:graph"),
        ),
        (
            "identifier",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::Identifier,
            Direction::Outgoing,
            AddressKind::Identifier,
            Some("surface:exact"),
        ),
        (
            "temporal-anchor",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::TemporalAnchor,
            Direction::Outgoing,
            AddressKind::TemporalAnchor,
            Some("surface:temporal"),
        ),
        (
            "occurrence-unit-hydration",
            AddressKind::Occurrence,
            StructuralTransitionOperation::Hydration,
            Direction::Outgoing,
            AddressKind::SemanticUnit,
            Some("surface:graph"),
        ),
        (
            "unit-unit-hydration",
            AddressKind::SemanticUnit,
            StructuralTransitionOperation::Hydration,
            Direction::Outgoing,
            AddressKind::SemanticUnit,
            Some("surface:vector"),
        ),
        (
            "surface-invocation",
            AddressKind::SemanticObject,
            StructuralTransitionOperation::RetrievalSurface,
            Direction::Outgoing,
            AddressKind::RetrievalSurface,
            None,
        ),
    ];
    specs
        .into_iter()
        .map(
            |(id, from, operation, direction, to, surface)| StructuralTransition {
                transition_id: format!("transition:{id}"),
                from,
                operation,
                direction,
                to,
                retrieval_surface_id: surface.map(str::to_owned),
            },
        )
        .collect()
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

fn finalize_occurrence_resolution(
    candidate_count: usize,
    candidate_source_paths: &[String],
    target: Option<SemanticAddress>,
) -> Result<(Option<SemanticAddress>, OccurrenceResolutionState), ConstructionError> {
    match (candidate_count, target) {
        (0, None) => Ok((None, OccurrenceResolutionState::Unresolved)),
        (0, Some(_)) => Err(ConstructionError::Contract(
            "resolved occurrence target has no canonical candidate".into(),
        )),
        (1, Some(target)) => Ok((Some(target), OccurrenceResolutionState::Resolved)),
        (1, None) => Ok((None, OccurrenceResolutionState::Unresolved)),
        (_, None) => Ok((
            None,
            OccurrenceResolutionState::Ambiguous {
                candidate_source_paths: candidate_source_paths.to_vec(),
            },
        )),
        (_, Some(_)) => Err(ConstructionError::Contract(
            "ambiguous occurrence received a resolved target".into(),
        )),
    }
}

fn validate_accepted_field_universe(observed: &BTreeSet<String>) -> Result<(), ConstructionError> {
    let accepted: BTreeSet<_> = ACCEPTED_FIELD_UNIVERSE
        .iter()
        .map(|field| (*field).to_owned())
        .collect();
    let missing: Vec<_> = accepted.difference(observed).cloned().collect();
    let unexpected: Vec<_> = observed.difference(&accepted).cloned().collect();
    if !missing.is_empty() || !unexpected.is_empty() {
        return Err(ConstructionError::Contract(format!(
            "accepted field universe mismatch; missing={missing:?}; unexpected={unexpected:?}"
        )));
    }
    Ok(())
}

/// Construct a private projection and a repository-safe numerical report.
pub fn construct(observation_path: &Path, output_path: &Path) -> Result<Value, ConstructionError> {
    let input_bytes = fs::read(observation_path)?;
    let input_artifact_sha256 = sha256(&input_bytes);
    if input_artifact_sha256 != ACCEPTED_ARTIFACT_SHA256 {
        return Err(ConstructionError::Contract(format!(
            "accepted observation artifact SHA-256 mismatch: {input_artifact_sha256}"
        )));
    }
    let input_text = String::from_utf8(input_bytes).map_err(|error| {
        ConstructionError::Contract(format!("accepted observation is not UTF-8: {error}"))
    })?;
    let root: Value = serde_json::from_str(&input_text)?;
    if text(&root, "observation_schema_version") != "vault-observation/v3" {
        return Err(ConstructionError::Contract(
            "observer schema is not vault-observation/v3".into(),
        ));
    }
    if root
        .get("observer_provenance")
        .and_then(|p| p.get("commit"))
        .and_then(Value::as_str)
        != Some("e9bb2d95c14b1beb334dc2b8d83420f5998b9a53")
    {
        return Err(ConstructionError::Contract(
            "observer specimen commit is not accepted".into(),
        ));
    }
    let snapshot = text(&root, "vault_resident_snapshot_identity");
    if snapshot != ACCEPTED_SPECIMEN_IDENTITY {
        return Err(ConstructionError::Contract(
            "observer specimen identity is not accepted".into(),
        ));
    }
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
    type RegionBoundsByObject =
        BTreeMap<String, Vec<(SemanticRegionAddress, u64, u64, Vec<String>)>>;
    let mut region_bounds_by_object: RegionBoundsByObject = BTreeMap::new();
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
            if let Some(heading_span) = heading_span.as_ref()
                && let (Some(start), Some(end)) = (heading_span.start_byte, heading_span.end_byte)
            {
                region_by_object_span
                    .entry((oid.to_string(), start, end))
                    .or_default()
                    .push(region_index);
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
        let observed_value_shapes = fm
            .get("value_shapes")
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
                    content_hash: format!("sha256:{}", sha256(raw_block.as_bytes())),
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
                let observed_shape = observed_value_shapes.get(&key).and_then(Value::as_str);
                if let Some(value) = temporal_value(&key, &raw, observed_shape) {
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
            let candidate_source_paths = candidate_paths(&link);
            let canonical_candidates: Vec<_> = candidate_source_paths
                .iter()
                .filter_map(|p| by_path.get(p).cloned())
                .collect();
            let target_object = match canonical_candidates.as_slice() {
                [candidate] => Some(candidate.clone()),
                _ => None,
            };
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
                    let mapping = match candidate_source_paths.as_slice() {
                        [p] => resolve_explicit_block_target(&unit_by_path_block_id, p, fragment),
                        _ => None,
                    };
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
            if let OccurrenceSource::SemanticRegion { region_address } = &source
                && let Some(region) = regions
                    .iter_mut()
                    .find(|region| &region.address == region_address)
            {
                region.outgoing_occurrence_ids.push(occurrence_id.clone());
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
            let (resolved_target, resolution_state) = finalize_occurrence_resolution(
                canonical_candidates.len(),
                &candidate_source_paths,
                target,
            )?;
            occurrences.push(OccurrenceRecord {
                occurrence_id,
                source,
                authored_target_text: text(&link, "raw_target"),
                display_alias: link
                    .get("display_alias")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                resolved_target,
                resolution_state,
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
    validate_accepted_field_universe(&field_names)?;
    let descriptors = admitted_fields
        .iter()
        .map(|name| {
            let role = descriptor_role(name);
            IdentifierDescriptor {
                identifier_name: name.clone(),
                semantic_role: role.clone(),
                value_shape: descriptor_shape(&assignments, name),
                cardinality: accepted_cardinality(name, descriptor_cardinality(&assignments, name)),
                applicable_address_kinds: vec![
                    AddressKind::SemanticObject,
                    AddressKind::SemanticRegion,
                    AddressKind::SemanticUnit,
                ],
                assignment_mode: descriptor_assignment_mode(&role),
                source_surface: format!("frontmatter.{name}"),
                may_contain_canonical_links: accepted_occurrence_semantics(name),
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
                    if accepted_occurrence_semantics(name) {
                        surfaces.push("surface:graph".into());
                    }
                    surfaces
                },
                enabled_transition_ids: {
                    let mut transitions = vec!["transition:identifier".into()];
                    if matches!(role, IdentifierRole::TemporalAnchoring) {
                        transitions.push("transition:temporal-anchor".into());
                    }
                    if accepted_occurrence_semantics(name) {
                        transitions.push("transition:object-occurrence-outgoing".into());
                    }
                    transitions
                },
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
        visible_address_kinds: match id {
            "surface:exact" | "surface:lexical" => vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::Identifier,
            ],
            "surface:vector" => vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
            ],
            "surface:graph" => vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticRegion,
                AddressKind::SemanticUnit,
                AddressKind::Identifier,
                AddressKind::Occurrence,
            ],
            "surface:temporal" => vec![
                AddressKind::SemanticObject,
                AddressKind::SemanticUnit,
                AddressKind::Identifier,
                AddressKind::TemporalAnchor,
            ],
            _ => vec![],
        },
        match_modes: vec![mode],
        returned_identity: if id == "surface:graph" {
            AddressKind::Occurrence
        } else if id == "surface:temporal" {
            AddressKind::TemporalAnchor
        } else {
            AddressKind::SemanticUnit
        },
        hydrates_to_semantic_units: false,
        coverage_semantics: CoverageSemantics::Bounded,
        exhaustive_total_count_supported: false,
        continuation_supported: false,
        technical_limitations: vec![],
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
        object_class_applicability
            .entry(class_name.clone())
            .or_insert_with(|| accepted_class_fields(&class_name));
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
        projection_snapshot_id: format!("projection:phase5:v2:{snapshot}"),
        ingest_identity: "observer:e9bb2d95c14b1beb334dc2b8d83420f5998b9a53".to_string(),
        schema_version: "semantic-space-projection/v2".into(),
        logical_hash: String::new(),
        corpus_snapshot_identity: snapshot.clone(),
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
        valid_transitions: phase5_transitions(),
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
    for object in &mut projection.objects {
        object.retrieval_surface_ids = vec![
            "surface:exact".into(),
            "surface:lexical".into(),
            "surface:vector".into(),
        ];
        if !object.incoming_occurrence_ids.is_empty()
            || !object.body_occurrence_ids.is_empty()
            || !object.object_field_occurrence_ids.is_empty()
        {
            object.retrieval_surface_ids.push("surface:graph".into());
        }
        if !object.temporal_anchor_ids.is_empty() {
            object.retrieval_surface_ids.push("surface:temporal".into());
        }
    }
    for region in &mut projection.regions {
        region.retrieval_surface_ids = vec!["surface:exact".into(), "surface:lexical".into()];
        if !region.incoming_occurrence_ids.is_empty() || !region.outgoing_occurrence_ids.is_empty()
        {
            region.retrieval_surface_ids.push("surface:graph".into());
        }
    }
    for unit in &mut projection.units {
        unit.retrieval_surface_ids = vec![
            "surface:exact".into(),
            "surface:lexical".into(),
            "surface:vector".into(),
        ];
        if !unit.incoming_occurrence_ids.is_empty() || !unit.outgoing_occurrence_ids.is_empty() {
            unit.retrieval_surface_ids.push("surface:graph".into());
        }
        if !unit.temporal_anchor_ids.is_empty() {
            unit.retrieval_surface_ids.push("surface:temporal".into());
        }
    }
    let closure = validate_projection(&projection)?;
    let canonical = serde_json::to_vec(&projection)?;
    projection.logical_hash = format!("sha256:{}", sha256(&canonical));
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
            "observer_commit": "e9bb2d95c14b1beb334dc2b8d83420f5998b9a53",
            "observer_schema": "vault-observation/v3",
            "corpus_snapshot_identity": snapshot,
            "specimen_identity": snapshot,
            "pinned_input_artifact_sha256": input_artifact_sha256,
            "accepted_artifact_sha256_gate": true,
            "accepted_observer_commit_gate": true,
            "accepted_schema_gate": true,
            "accepted_specimen_gate": true,
        },
        "admission": {
            "whole_resident_source_count": markdown_count,
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
            "observed_field_universe_count": field_names.len(),
            "accepted_field_universe_count": ACCEPTED_FIELD_UNIVERSE.len(),
            "exact_field_universe_match": true,
            "missing_fields": Vec::<String>::new(),
            "unexpected_fields": Vec::<String>::new(),
            "admitted_field_count": ACCEPTED_FIELD_UNIVERSE.len() - EXCLUDED_FIELDS.len(),
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
            "ambiguous_count": projection.occurrences.iter().filter(|o| matches!(o.resolution_state, OccurrenceResolutionState::Ambiguous { .. })).count(),
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
            "authority_class_applicability_failures": closure.authority_class_applicability_failures,
            "authority_occurrence_capability_failures": closure.authority_occurrence_capability_failures,
            "occurrence_resolution_state_mismatches": closure.occurrence_resolution_state_mismatches,
            "present_null_temporal_anchor_failures": closure.present_null_temporal_anchor_failures
        },
        "temporal": {
            "month_day_grammar_validated": true,
            "approximate_year_grammar_validated": true,
            "temporally_capable_descriptors": projection.identifier_descriptors.iter().filter(|d| matches!(d.temporal_affordance, TemporalAffordance::CreatesAnchor)).count(),
            "materially_created_temporal_anchor_count": projection.temporal_anchors.len(),
            "representation_counts": {
                "FullDate": projection.temporal_anchors.iter().filter(|a| matches!(a.value, TemporalValue::FullDate(_))).count(),
                "DateTime": projection.temporal_anchors.iter().filter(|a| matches!(a.value, TemporalValue::DateTime(_))).count(),
                "ExactYear": projection.temporal_anchors.iter().filter(|a| matches!(a.value, TemporalValue::ExactYear(_))).count(),
                "MonthDay": projection.temporal_anchors.iter().filter(|a| matches!(a.value, TemporalValue::MonthDay(_))).count(),
                "ApproximateYear": projection.temporal_anchors.iter().filter(|a| matches!(a.value, TemporalValue::ApproximateYear(_))).count()
            },
            "present_null_temporal_assignments_creating_no_anchor": present_null_temporal_assignment_count,
            "present_null_temporal_assignments_incorrectly_anchored": closure.present_null_temporal_anchor_failures
        },
        "construction_status": "produced",
        "contract_contact_failures": 0,
        "projection": {
            "schema_version": projection.schema_version,
            "logical_hash": projection.logical_hash,
            "validation_status": "unvalidated",
            "retrieval_surface_count": projection.retrieval_surfaces.len(),
            "valid_transition_count": projection.valid_transitions.len(),
            "object_class_count": projection.object_classes.len()
        },
        "determinism": {
            "first_run_logical_hash": projection.logical_hash,
            "second_run_logical_hash": projection.logical_hash,
            "equality_result": "single-run; external byte comparison required"
        }
    });
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    #[test]
    fn temporal_parser_preserves_licensed_precision_and_rejects_generic_strings() {
        assert!(matches!(
            temporal_value("birthday", &json!("--10-19"), Some("string")),
            Some(TemporalValue::MonthDay(_))
        ));
        assert!(matches!(
            temporal_value("first_met", &json!("2022-10-26"), Some("date")),
            Some(TemporalValue::FullDate(_))
        ));
        assert!(matches!(
            temporal_value("first_met", &json!("10/26/22 14:10:00"), Some("datetime")),
            Some(TemporalValue::DateTime(_))
        ));
        assert!(matches!(
            temporal_value("original_year_published", &json!(1867), Some("number")),
            Some(TemporalValue::ExactYear(1867))
        ));
        assert!(matches!(
            temporal_value(
                "original_year_published",
                &json!("~400 BCE"),
                Some("string")
            ),
            Some(TemporalValue::ApproximateYear(_))
        ));
        assert!(temporal_value("birthday", &json!("sometime"), Some("string")).is_none());
        assert!(temporal_value("birthday", &json!("--02-29"), Some("string")).is_some());
        assert!(temporal_value("birthday", &json!("--99-99"), Some("string")).is_none());
        assert!(temporal_value("birthday", &json!("--13-01"), Some("string")).is_none());
        assert!(temporal_value("birthday", &json!("--12-aa"), Some("string")).is_none());
        assert!(
            temporal_value(
                "original_year_published",
                &json!("~400 BCE"),
                Some("string")
            )
            .is_some()
        );
        assert!(
            temporal_value(
                "original_year_published",
                &json!("~banana BCE"),
                Some("string")
            )
            .is_none()
        );
        assert!(
            temporal_value("original_year_published", &json!("~400 BC"), Some("string")).is_none()
        );
        assert!(
            temporal_value(
                "original_year_published",
                &json!("about 400 BCE"),
                Some("string")
            )
            .is_none()
        );
        assert!(temporal_value("journal_entry_date", &Value::Null, Some("date")).is_none());
    }

    #[test]
    fn target_candidates_never_rank_ambiguity() {
        let link = json!({"target_candidates": {"candidate_source_paths": ["a.md", "b.md"]}});
        let candidates = candidate_paths(&link);
        assert_eq!(candidates, ["a.md", "b.md"]);
        let state = OccurrenceResolutionState::Ambiguous {
            candidate_source_paths: candidates,
        };
        assert!(
            matches!(state, OccurrenceResolutionState::Ambiguous { candidate_source_paths } if candidate_source_paths.len() == 2)
        );
    }

    #[test]
    fn class_applicability_is_contract_derived() {
        assert!(accepted_class_fields("entity").contains("birthday"));
        assert!(!accepted_class_fields("entity").contains("title"));
        assert!(accepted_class_fields("source_material").contains("title"));
        assert!(accepted_class_fields("journal_entry").contains("architect_or_operator"));
        assert!(accepted_class_fields("journal_entry").contains("dream_motif"));
        assert!(!accepted_class_fields("dream_motif").contains("architect_or_operator"));
        assert!(!accepted_class_fields("dream_motif").contains("journal_entry_date"));
        assert!(accepted_occurrence_semantics("book_read_today"));
        assert!(accepted_occurrence_semantics("dream_motif"));
        assert!(!accepted_occurrence_semantics("relationship"));
    }

    #[test]
    fn exact_field_universe_rejects_substitution_and_missing_members() {
        let accepted: BTreeSet<_> = ACCEPTED_FIELD_UNIVERSE
            .iter()
            .map(|field| (*field).into())
            .collect();
        assert!(validate_accepted_field_universe(&accepted).is_ok());
        let mut unknown = accepted.clone();
        unknown.remove("address");
        unknown.insert("unknown_field".into());
        assert!(validate_accepted_field_universe(&unknown).is_err());
        let mut missing = accepted;
        missing.remove("dream_motif");
        assert!(validate_accepted_field_universe(&missing).is_err());
    }

    #[test]
    fn final_occurrence_resolution_state_matches_final_target() {
        let paths = vec!["one.md".into()];
        let object =
            SemanticAddress::Object("00000000-0000-0000-0000-000000000001".parse().unwrap());
        assert!(matches!(
            finalize_occurrence_resolution(0, &[], None),
            Ok((None, OccurrenceResolutionState::Unresolved))
        ));
        assert!(matches!(
            finalize_occurrence_resolution(1, &paths, Some(object.clone())),
            Ok((
                Some(SemanticAddress::Object(_)),
                OccurrenceResolutionState::Resolved
            ))
        ));
        assert!(matches!(
            finalize_occurrence_resolution(2, &["one.md".into(), "two.md".into()], None),
            Ok((None, OccurrenceResolutionState::Ambiguous { .. }))
        ));
        assert!(finalize_occurrence_resolution(0, &[], Some(object)).is_err());
    }

    #[test]
    fn exact_unit_hash_changes_only_with_unit_bytes() {
        assert_eq!(sha256(b"unit-a"), sha256(b"unit-a"));
        assert_ne!(sha256(b"unit-a"), sha256(b"unit-b"));
        assert_ne!(sha256(b"unit-a\noutside"), sha256(b"unit-a"));
    }

    #[test]
    fn structural_transition_set_is_unique_and_surface_closed() {
        let transitions = phase5_transitions();
        let ids: HashSet<_> = transitions.iter().map(|t| &t.transition_id).collect();
        assert_eq!(ids.len(), transitions.len());
        assert!(
            transitions
                .iter()
                .any(|t| t.transition_id == "transition:surface-invocation")
        );
        assert!(transitions.iter().all(|t| {
            t.retrieval_surface_id.is_none()
                || [
                    "surface:exact",
                    "surface:graph",
                    "surface:temporal",
                    "surface:vector",
                ]
                .contains(&t.retrieval_surface_id.as_deref().unwrap())
        }));
    }
}
