//! Independent Phase 6 correspondence validation for a frozen Phase 5 projection.
//!
//! The validator deliberately reads the observation as `serde_json::Value` and
//! derives its expected record surface locally. It does not call the Phase 5
//! constructor or its closure helper: construction evidence and validation
//! evidence must remain separate.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    construction::sha256,
    model::{
        RecordProvenance, SemanticAddress, SemanticObjectId, SemanticRegionAddress, SemanticUnitId,
        SourceSpan,
    },
    projection::{
        AuthoredBlockType, IdentifierAssignment, IdentifierValue, OccurrenceResolutionState,
        OccurrenceSource, SemanticSpaceProjection, SemanticUnitContent, TemporalValue,
    },
    region_identity::{AuthoredRegionHeading, canonical_region_identities},
};

const OBSERVATION_SHA256: &str = "d3a340a1b203a64b2455f71a8d4f17003d5bfdba8be0583cbec1529692320bb9";
const PROJECTION_SHA256: &str = "4870244e512f996aff1280e7d2690a9053bcbb4c4be4df7b8b9f33eab58eea5a";
const CORPUS: &str = "f6e3e4672560d294b0c303f21a063c2943f6ead0cb365ea93a66d0d9526c9ce4";
const OBSERVER_COMMIT: &str = "e9bb2d95c14b1beb334dc2b8d83420f5998b9a53";
const EXCLUDED_FIELDS: [&str; 5] = ["address", "email", "phone", "likes", "dislikes"];
const IDENTIFIER_FIELDS: [&str; 55] = [
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
    "dream_location",
    "dream_lucidity",
    "dream_motif",
    "dream_motif_valence",
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
    "note_type",
    "occupation",
    "origin",
    "original_year_published",
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationSummary {
    pub status: String,
    pub observation_sha256: String,
    pub phase5_projection_sha256: String,
    pub phase5_logical_hash: String,
    pub phase5_snapshot_id: String,
    pub phase6_snapshot_id: String,
    pub phase6_logical_hash: String,
    pub phase6_projection_sha256: String,
    pub counts: BTreeMap<String, usize>,
    pub failure_counts: BTreeMap<String, usize>,
    pub violations: Vec<String>,
}

#[derive(Debug)]
pub enum ValidationError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Input(String),
    Violations(Box<ValidationSummary>),
}
impl From<std::io::Error> for ValidationError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<serde_json::Error> for ValidationError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ValidationError {}

fn text(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}
fn array(value: &Value, key: &str) -> Vec<Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}
fn span(value: &Value, source: &str) -> Option<SourceSpan> {
    let values = value.as_array()?;
    Some(SourceSpan {
        source: source.to_owned(),
        start_byte: values.first()?.as_u64(),
        end_byte: values.get(1)?.as_u64(),
    })
}
fn fnv(bytes: &[u8]) -> String {
    let mut hash = 14695981039346656037u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}
fn component(value: &str) -> String {
    format!("{}:{value}", value.len())
}
fn independent_unit_id(
    object: &SemanticObjectId,
    region: &str,
    ordinal: u32,
    explicit: Option<&str>,
) -> String {
    let suffix = explicit
        .map(|value| format!("1:{}", component(value)))
        .unwrap_or_else(|| "0".into());
    format!(
        "unit-v2:{}:{}:{ordinal}:{suffix}",
        component(&object.to_string()),
        component(region)
    )
}
fn admitted(path: &str) -> bool {
    !path.starts_with("VAULT DESIGN/")
}
fn map_value(value: &Value) -> BTreeMap<String, Value> {
    value
        .as_object()
        .map(|object| object.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}
fn expected_block_type(kind: &str, raw: &str) -> Option<AuthoredBlockType> {
    Some(match kind {
        "paragraph" => AuthoredBlockType::Paragraph,
        "list" => AuthoredBlockType::List,
        "block_quote" => AuthoredBlockType::BlockQuote,
        "blockquote_or_callout"
            if raw
                .lines()
                .any(|line| line.trim_start().starts_with("> [!")) =>
        {
            AuthoredBlockType::Callout
        }
        "blockquote_or_callout" if raw.lines().any(|line| line.trim_start().starts_with('>')) => {
            AuthoredBlockType::BlockQuote
        }
        "table" => AuthoredBlockType::Table,
        "code_fence" => AuthoredBlockType::CodeBlock,
        "display_equation" => AuthoredBlockType::Equation,
        "callout" => AuthoredBlockType::Callout,
        "embedded_media" => AuthoredBlockType::EmbeddedMedia,
        _ => return None,
    })
}

#[derive(Clone)]
struct ExpectedRegion {
    address: SemanticRegionAddress,
    heading_path: Vec<String>,
    span: Option<SourceSpan>,
    parent: Option<SemanticRegionAddress>,
}
#[derive(Clone)]
struct ExpectedUnit {
    id: String,
    region: SemanticRegionAddress,
    heading_path: Vec<String>,
    ordinal: u32,
    explicit: Option<String>,
    kind: AuthoredBlockType,
    path: String,
    span: Option<SourceSpan>,
    raw: String,
}

fn expected_regions(
    object: &SemanticObjectId,
    markdown: &Value,
    path: &str,
) -> Result<Vec<ExpectedRegion>, String> {
    let root = SemanticRegionAddress::parse(object.clone(), "root").map_err(|e| e.to_string())?;
    let mut result = vec![ExpectedRegion {
        address: root,
        heading_path: vec![],
        span: None,
        parent: None,
    }];
    let headings = array(markdown, "headings");
    let authored: Vec<_> = headings
        .iter()
        .map(|heading| {
            Ok(AuthoredRegionHeading {
                level: heading
                    .get("level")
                    .and_then(Value::as_u64)
                    .ok_or("heading level missing")? as u8,
                authored_structural_address: text(heading, "address_key")
                    .ok_or("heading address missing")?,
                source_span: span(heading.get("source_span").unwrap_or(&Value::Null), path),
            })
        })
        .collect::<Result<_, String>>()?;
    let identities =
        canonical_region_identities(object.clone(), &authored).map_err(|e| e.to_string())?;
    let mut stack: Vec<(u8, usize)> = Vec::new();
    for (index, identity) in identities.into_iter().enumerate() {
        let level = authored[index].level;
        while stack.last().is_some_and(|(parent, _)| *parent >= level) {
            stack.pop();
        }
        let parent = stack
            .last()
            .map(|(_, idx)| result[idx + 1].address.clone())
            .or_else(|| Some(result[0].address.clone()));
        result.push(ExpectedRegion {
            address: identity.address,
            heading_path: identity.heading_path,
            span: identity.source_span,
            parent,
        });
        stack.push((level, index));
    }
    Ok(result)
}

fn expected_units(
    object: &SemanticObjectId,
    markdown: &Value,
    path: &str,
    regions: &[ExpectedRegion],
) -> Result<Vec<ExpectedUnit>, String> {
    let source_len = markdown
        .get("raw_markdown")
        .and_then(Value::as_str)
        .map_or(0, str::len) as u64;
    let mut region_bounds: Vec<(&ExpectedRegion, u64, u64)> = Vec::new();
    for (index, region) in regions.iter().enumerate().skip(1) {
        let start = region.span.as_ref().and_then(|s| s.start_byte).unwrap_or(0);
        let level = markdown
            .get("headings")
            .and_then(Value::as_array)
            .and_then(|h| h.get(index - 1))
            .and_then(|h| h.get("level"))
            .and_then(Value::as_u64)
            .unwrap_or(1);
        let end = markdown
            .get("headings")
            .and_then(Value::as_array)
            .and_then(|headings| {
                headings.iter().skip(index).find_map(|next| {
                    (next.get("level").and_then(Value::as_u64).unwrap_or(1) <= level)
                        .then(|| {
                            next.get("source_span")
                                .and_then(|s| s.as_array())
                                .and_then(|a| a.first())
                                .and_then(Value::as_u64)
                        })
                        .flatten()
                })
            })
            .unwrap_or(source_len);
        region_bounds.push((region, start, end));
    }
    let root = &regions[0].address;
    let mut ordinals: BTreeMap<String, u32> = BTreeMap::new();
    let mut output = Vec::new();
    for block in array(markdown, "block_candidates")
        .into_iter()
        .filter(|block| text(block, "block_kind_observation").as_deref() != Some("heading"))
    {
        let raw = text(&block, "raw_markdown").unwrap_or_default();
        let source_span = span(block.get("source_span").unwrap_or(&Value::Null), path);
        let start = source_span.as_ref().and_then(|s| s.start_byte).unwrap_or(0);
        let selected = region_bounds
            .iter()
            .filter(|(_, a, b)| *a <= start && start < *b)
            .max_by_key(|(_, a, _)| *a)
            .map(|(r, _, _)| *r)
            .unwrap_or(&regions[0]);
        let key = selected.address.authored_structural_address.clone();
        let ordinal = ordinals
            .entry(key.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        let explicit = array(&block, "explicit_block_ids")
            .first()
            .and_then(Value::as_str)
            .map(str::to_owned);
        let kind = expected_block_type(
            text(&block, "block_kind_observation")
                .as_deref()
                .unwrap_or(""),
            &raw,
        )
        .ok_or_else(|| "unsupported authored block kind".to_owned())?;
        let id = independent_unit_id(
            object,
            &selected.address.authored_structural_address,
            *ordinal,
            explicit.as_deref(),
        );
        output.push(ExpectedUnit {
            id,
            region: selected.address.clone(),
            heading_path: selected.heading_path.clone(),
            ordinal: *ordinal,
            explicit,
            kind,
            path: path.to_owned(),
            span: source_span,
            raw,
        });
    }
    let _ = root;
    Ok(output)
}

fn occurrence_id(object: &SemanticObjectId, link: &Value) -> String {
    let encoded = serde_json::to_string(link).unwrap_or_default();
    format!("occurrence:{}:{}", object, fnv(encoded.as_bytes()))
}
fn assignment_raw(assignment: &IdentifierAssignment) -> Option<&Value> {
    assignment.authored_raw_value.as_ref()
}
fn object_id_from_markdown(markdown: &Value) -> Result<SemanticObjectId, String> {
    markdown
        .get("uuid")
        .and_then(|u| u.get("parsed_value"))
        .and_then(Value::as_str)
        .ok_or_else(|| "missing parsed UUID".into())
        .and_then(|id| SemanticObjectId::parse(id).map_err(|e| e.to_string()))
}

fn check_typed_topology(
    projection: &SemanticSpaceProjection,
    failures: &mut BTreeMap<String, usize>,
    violations: &mut Vec<String>,
) {
    let mut bump = |domain: &str, message: String| {
        *failures.entry(domain.into()).or_default() += 1;
        violations.push(message);
    };
    let object_ids: HashSet<_> = projection
        .objects
        .iter()
        .map(|o| o.object_id.clone())
        .collect();
    if object_ids.len() != projection.objects.len() {
        bump(
            "deterministic_identity",
            "duplicate semantic object identity".into(),
        );
    }
    let region_ids: HashSet<_> = projection
        .regions
        .iter()
        .map(|r| r.address.clone())
        .collect();
    if region_ids.len() != projection.regions.len() {
        bump(
            "deterministic_identity",
            "duplicate semantic region identity".into(),
        );
    }
    let unit_ids: HashSet<_> = projection.units.iter().map(|u| u.unit_id.clone()).collect();
    if unit_ids.len() != projection.units.len() {
        bump(
            "deterministic_identity",
            "duplicate semantic unit identity".into(),
        );
    }
    let assignment_ids: HashSet<_> = projection
        .identifier_assignments
        .iter()
        .map(|a| a.assignment_id.clone())
        .collect();
    if assignment_ids.len() != projection.identifier_assignments.len() {
        bump(
            "deterministic_identity",
            "duplicate identifier assignment identity".into(),
        );
    }
    let occurrence_ids: HashSet<_> = projection
        .occurrences
        .iter()
        .map(|o| o.occurrence_id.clone())
        .collect();
    if occurrence_ids.len() != projection.occurrences.len() {
        bump(
            "deterministic_identity",
            "duplicate occurrence identity".into(),
        );
    }
    let temporal_ids: HashSet<_> = projection
        .temporal_anchors
        .iter()
        .map(|a| a.anchor_id.clone())
        .collect();
    if temporal_ids.len() != projection.temporal_anchors.len() {
        bump(
            "deterministic_identity",
            "duplicate temporal anchor identity".into(),
        );
    }
    let surface_ids: HashSet<_> = projection
        .retrieval_surfaces
        .iter()
        .map(|s| s.surface_id.clone())
        .collect();
    let transition_ids: HashSet<_> = projection
        .valid_transitions
        .iter()
        .map(|t| t.transition_id.clone())
        .collect();
    if surface_ids.len() != projection.retrieval_surfaces.len()
        || transition_ids.len() != projection.valid_transitions.len()
    {
        bump(
            "deterministic_identity",
            "duplicate capability identity".into(),
        );
    }
    for region in &projection.regions {
        if !object_ids.contains(&region.address.object_id) {
            bump(
                "region",
                format!(
                    "region references absent object: {}",
                    region.address.authored_structural_address
                ),
            );
        }
        if region
            .contained_unit_ids
            .iter()
            .any(|id| !unit_ids.contains(id))
        {
            bump(
                "reverse_incidence",
                "region contains dangling unit reference".into(),
            );
        }
    }
    for unit in &projection.units {
        if !object_ids.contains(&unit.parent_object_id)
            || !region_ids.contains(&unit.parent_region_address)
        {
            bump(
                "unit",
                format!("unit has dangling parent: {}", unit.unit_id),
            );
        }
        match &unit.content {
            SemanticUnitContent::HydrationAddress {
                address,
                content_hash,
            } if address.is_empty() || !content_hash.starts_with("sha256:") => bump(
                "provenance",
                format!("invalid hydration provenance: {}", unit.unit_id),
            ),
            _ => {}
        }
        if unit
            .transport_segments
            .iter()
            .any(|segment| segment.parent_unit_id != unit.unit_id)
        {
            bump(
                "transport_segmentation",
                format!("transport segment has wrong parent: {}", unit.unit_id),
            );
        }
    }
    for assignment in &projection.identifier_assignments {
        if EXCLUDED_FIELDS.contains(&assignment.identifier_name.as_str()) {
            bump(
                "identifier",
                format!(
                    "excluded field leaked into assignment: {}",
                    assignment.identifier_name
                ),
            );
        }
        if let SemanticAddress::Object(id) = &assignment.subject
            && !object_ids.contains(id)
        {
            bump("identifier", "assignment references absent object".into());
        }
        if let Some(raw) = assignment_raw(assignment)
            && matches!(assignment.value, IdentifierValue::Null) != raw.is_null()
        {
            bump(
                "identifier",
                format!("null shape mismatch: {}", assignment.assignment_id),
            );
        }
    }
    for occurrence in &projection.occurrences {
        let source_exists = match &occurrence.source {
            OccurrenceSource::ObjectField { object_id, .. } => object_ids.contains(object_id),
            OccurrenceSource::SemanticRegion { region_address } => {
                region_ids.contains(region_address)
            }
            OccurrenceSource::SemanticUnit { unit_id } => unit_ids.contains(unit_id),
        };
        if !source_exists {
            bump(
                "occurrence",
                format!("occurrence has absent source: {}", occurrence.occurrence_id),
            );
        }
        let source_incidence_ok = match &occurrence.source {
            OccurrenceSource::ObjectField { object_id, .. } => projection
                .objects
                .iter()
                .find(|o| &o.object_id == object_id)
                .is_some_and(|o| {
                    o.object_field_occurrence_ids
                        .iter()
                        .filter(|id| *id == &occurrence.occurrence_id)
                        .count()
                        == 1
                }),
            OccurrenceSource::SemanticRegion { region_address } => projection
                .regions
                .iter()
                .find(|r| &r.address == region_address)
                .is_some_and(|r| {
                    r.outgoing_occurrence_ids
                        .iter()
                        .filter(|id| *id == &occurrence.occurrence_id)
                        .count()
                        == 1
                }),
            OccurrenceSource::SemanticUnit { unit_id } => projection
                .units
                .iter()
                .find(|u| &u.unit_id == unit_id)
                .is_some_and(|u| {
                    u.outgoing_occurrence_ids
                        .iter()
                        .filter(|id| *id == &occurrence.occurrence_id)
                        .count()
                        == 1
                }),
        };
        if !source_incidence_ok {
            bump(
                "reverse_incidence",
                format!(
                    "occurrence source incidence mismatch: {}",
                    occurrence.occurrence_id
                ),
            );
        }
        if occurrence.resolved_target.is_some()
            && !matches!(
                occurrence.resolution_state,
                OccurrenceResolutionState::Resolved
            )
        {
            bump(
                "target",
                format!(
                    "resolved target has non-resolved state: {}",
                    occurrence.occurrence_id
                ),
            );
        }
        if occurrence.resolved_target.is_none()
            && matches!(
                occurrence.resolution_state,
                OccurrenceResolutionState::Resolved
            )
        {
            bump(
                "target",
                format!("resolved state lacks target: {}", occurrence.occurrence_id),
            );
        }
        if let Some(target) = &occurrence.resolved_target {
            let incoming_ok = match target {
                SemanticAddress::Object(id) => projection
                    .objects
                    .iter()
                    .find(|o| &o.object_id == id)
                    .is_some_and(|o| {
                        o.incoming_occurrence_ids
                            .iter()
                            .filter(|value| *value == &occurrence.occurrence_id)
                            .count()
                            == 1
                    }),
                SemanticAddress::Region(address) => projection
                    .regions
                    .iter()
                    .find(|r| &r.address == address)
                    .is_some_and(|r| {
                        r.incoming_occurrence_ids
                            .iter()
                            .filter(|value| *value == &occurrence.occurrence_id)
                            .count()
                            == 1
                    }),
                SemanticAddress::Unit(id) => projection
                    .units
                    .iter()
                    .find(|u| &u.unit_id == id)
                    .is_some_and(|u| {
                        u.incoming_occurrence_ids
                            .iter()
                            .filter(|value| *value == &occurrence.occurrence_id)
                            .count()
                            == 1
                    }),
                _ => false,
            };
            if !incoming_ok {
                bump(
                    "reverse_incidence",
                    format!(
                        "occurrence target incidence mismatch: {}",
                        occurrence.occurrence_id
                    ),
                );
            }
        }
    }
    for anchor in &projection.temporal_anchors {
        if !matches!(
            anchor.value,
            TemporalValue::FullDate(_)
                | TemporalValue::DateTime(_)
                | TemporalValue::ExactYear(_)
                | TemporalValue::MonthDay(_)
                | TemporalValue::ApproximateYear(_)
        ) {
            bump("temporal", "unknown temporal precision".into());
        }
    }
    for surface in &projection.retrieval_surfaces {
        if surface.default_candidate_limit > surface.hard_candidate_limit
            || surface.surface_id.is_empty()
        {
            bump(
                "bounds",
                format!("invalid surface bounds: {}", surface.surface_id),
            );
        }
    }
    for transition in &projection.valid_transitions {
        if let Some(surface) = &transition.retrieval_surface_id
            && !surface_ids.contains(surface)
        {
            bump(
                "surface",
                format!(
                    "transition references absent surface: {}",
                    transition.transition_id
                ),
            );
        }
    }
}

fn compare_observation(
    projection: &SemanticSpaceProjection,
    root: &Value,
    failures: &mut BTreeMap<String, usize>,
    violations: &mut Vec<String>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let markdown = array(root, "markdown_observations");
    let admitted_notes: Vec<_> = markdown
        .iter()
        .filter(|m| {
            text(m.get("source").unwrap_or(&Value::Null), "relative_path")
                .is_some_and(|p| admitted(&p))
        })
        .collect();
    let mut bump = |domain: &str, message: String| {
        *failures.entry(domain.into()).or_default() += 1;
        violations.push(message);
    };
    counts.insert("resident_source_records".into(), markdown.len());
    counts.insert("resident_markdown".into(), markdown.len());
    counts.insert("admitted_sources".into(), admitted_notes.len());
    counts.insert(
        "excluded_markdown".into(),
        markdown.len() - admitted_notes.len(),
    );
    counts.insert("objects".into(), projection.objects.len());
    counts.insert("regions".into(), projection.regions.len());
    counts.insert("semantic_units".into(), projection.units.len());
    counts.insert(
        "identifier_descriptors".into(),
        projection.identifier_descriptors.len(),
    );
    counts.insert(
        "identifier_assignments".into(),
        projection.identifier_assignments.len(),
    );
    counts.insert("occurrences".into(), projection.occurrences.len());
    counts.insert("temporal_anchors".into(), projection.temporal_anchors.len());
    counts.insert(
        "retrieval_surfaces".into(),
        projection.retrieval_surfaces.len(),
    );
    counts.insert(
        "structural_transitions".into(),
        projection.valid_transitions.len(),
    );
    counts.insert(
        "transport_segments".into(),
        projection
            .units
            .iter()
            .map(|u| u.transport_segments.len())
            .sum(),
    );
    let admitted_paths: HashSet<String> = admitted_notes
        .iter()
        .filter_map(|note| text(note.get("source").unwrap_or(&Value::Null), "relative_path"))
        .collect();
    let mut objects_by_id = HashMap::new();
    let mut regions_by_address = HashMap::new();
    let mut units_by_id = HashMap::new();
    let mut occurrence_ids = HashSet::new();
    for object in &projection.objects {
        if objects_by_id
            .insert(object.object_id.clone(), object)
            .is_some()
        {
            bump("object", format!("duplicate object: {}", object.object_id));
        }
    }
    for region in &projection.regions {
        regions_by_address.insert(region.address.clone(), region);
    }
    for unit in &projection.units {
        units_by_id.insert(unit.unit_id.clone(), unit);
    }
    let expected_object_ids: HashSet<SemanticObjectId> = admitted_notes
        .iter()
        .filter_map(|note| object_id_from_markdown(note).ok())
        .collect();
    for object in &projection.objects {
        if !expected_object_ids.contains(&object.object_id) {
            bump("object", format!("invented object: {}", object.object_id));
        }
    }
    for note in &admitted_notes {
        let path =
            text(note.get("source").unwrap_or(&Value::Null), "relative_path").unwrap_or_default();
        let Ok(id) = object_id_from_markdown(note) else {
            bump("admission", format!("missing object UUID for {path}"));
            continue;
        };
        let Some(object) = objects_by_id.get(&id) else {
            bump("object", format!("admitted source missing object: {path}"));
            continue;
        };
        if object.canonical_path != path
            || object.source_identity.is_empty()
            || !matches!(object.source_kind, crate::projection::SourceKind::Markdown)
        {
            bump("object", format!("object correspondence mismatch: {path}"));
        }
        let values = note
            .get("frontmatter")
            .and_then(|f| f.get("values"))
            .unwrap_or(&Value::Null);
        let expected_title = values
            .get("title")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| {
                text(note.get("source").unwrap_or(&Value::Null), "basename").unwrap_or_default()
            });
        if object.title != expected_title {
            bump("object", format!("object title mismatch: {path}"));
        }
        let expected_regions = match expected_regions(&id, note, &path) {
            Ok(value) => value,
            Err(error) => {
                bump(
                    "region",
                    format!("cannot derive regions for {path}: {error}"),
                );
                continue;
            }
        };
        for expected in &expected_regions {
            let Some(region) = regions_by_address.get(&expected.address) else {
                bump(
                    "region",
                    format!(
                        "missing region: {}",
                        expected.address.authored_structural_address
                    ),
                );
                continue;
            };
            if region.heading_path != expected.heading_path || region.source_span != expected.span {
                bump(
                    "region",
                    format!(
                        "region correspondence mismatch: {}",
                        expected.address.authored_structural_address
                    ),
                );
            }
            if let Some(parent) = &expected.parent
                && !regions_by_address
                    .get(parent)
                    .is_some_and(|p| p.child_region_addresses.contains(&expected.address))
            {
                bump(
                    "reverse_incidence",
                    format!(
                        "region parent incidence mismatch: {}",
                        expected.address.authored_structural_address
                    ),
                );
            }
        }
        let expected_units = match expected_units(&id, note, &path, &expected_regions) {
            Ok(value) => value,
            Err(error) => {
                bump("unit", format!("cannot derive units for {path}: {error}"));
                continue;
            }
        };
        for expected in &expected_units {
            let lookup = SemanticUnitId::parse(expected.id.clone())
                .expect("independently constructed unit identity");
            let Some(unit) = units_by_id.get(&lookup) else {
                bump("unit", format!("missing unit: {}", expected.id));
                continue;
            };
            let content_ok = match &unit.content {
                SemanticUnitContent::HydrationAddress {
                    address,
                    content_hash,
                } => {
                    address
                        == &format!(
                            "source:{}#bytes:{}:{}",
                            expected.path,
                            expected
                                .span
                                .as_ref()
                                .and_then(|s| s.start_byte)
                                .unwrap_or(0),
                            expected.span.as_ref().and_then(|s| s.end_byte).unwrap_or(0)
                        )
                        && content_hash == &format!("sha256:{}", sha256(expected.raw.as_bytes()))
                }
                _ => false,
            };
            let provenance_ok = matches!(&unit.source_provenance, RecordProvenance::SemanticUnit { unit_id, source_span } if unit_id == &lookup && source_span == &expected.span);
            if unit.parent_region_address != expected.region
                || unit.block_ordinal != expected.ordinal
                || unit.heading_path != expected.heading_path
                || unit.explicit_block_id != expected.explicit
                || unit.authored_block_type != expected.kind
                || !content_ok
                || !provenance_ok
            {
                bump(
                    "unit",
                    format!("unit correspondence mismatch: {}", expected.id),
                );
            }
        }
        let values = note
            .get("frontmatter")
            .and_then(|f| f.get("values"))
            .unwrap_or(&Value::Null);
        let value_map = map_value(values);
        for (name, raw) in value_map {
            if EXCLUDED_FIELDS.contains(&name.as_str()) {
                if projection
                    .identifier_descriptors
                    .iter()
                    .any(|d| d.identifier_name == name)
                {
                    bump("identifier", format!("excluded descriptor leaked: {name}"));
                }
                continue;
            }
            let id_text = format!("assignment:{id}:{name}");
            let assignment = projection
                .identifier_assignments
                .iter()
                .find(|a| a.assignment_id == id_text);
            if let Some(assignment) = assignment {
                if !raw.is_null() && assignment_raw(assignment) != Some(&raw) {
                    bump("identifier", format!("authored value mismatch: {id_text}"));
                }
            } else {
                bump("identifier", format!("missing assignment: {id_text}"));
            }
        }
        for link in array(note, "authored_links") {
            let oid = occurrence_id(&id, &link);
            if !occurrence_ids.insert(oid.clone()) {
                bump(
                    "deterministic_identity",
                    format!("duplicate authored occurrence: {oid}"),
                );
            }
            let Some(observed) = projection
                .occurrences
                .iter()
                .find(|o| o.occurrence_id.to_string() == oid)
            else {
                bump("occurrence", format!("missing occurrence: {oid}"));
                continue;
            };
            if observed.authored_target_text != text(&link, "raw_target").unwrap_or_default() {
                bump(
                    "occurrence",
                    format!("occurrence target text mismatch: {oid}"),
                );
            }
            let candidates: Vec<String> = array(
                link.get("target_candidates").unwrap_or(&Value::Null),
                "candidate_source_paths",
            )
            .iter()
            .filter_map(Value::as_str)
            .filter(|path| admitted_paths.contains(*path))
            .map(str::to_owned)
            .collect();
            let has_fragment = link
                .get("heading_fragment")
                .and_then(Value::as_str)
                .is_some()
                || link.get("block_fragment").and_then(Value::as_str).is_some();
            if !has_fragment {
                let expected_state = match candidates.len() {
                    1 => OccurrenceResolutionState::Resolved,
                    0 => OccurrenceResolutionState::Unresolved,
                    _ => OccurrenceResolutionState::Ambiguous {
                        candidate_source_paths: candidates,
                    },
                };
                if observed.resolution_state != expected_state {
                    bump("target", format!("occurrence resolution mismatch: {oid}"));
                }
            }
        }
    }
    for occurrence in &projection.occurrences {
        if !occurrence_ids.contains(&occurrence.occurrence_id.to_string()) {
            bump(
                "occurrence",
                format!("invented occurrence: {}", occurrence.occurrence_id),
            );
        }
    }
    let observed_block_fragments: Vec<_> = admitted_notes
        .iter()
        .flat_map(|note| array(note, "authored_links"))
        .filter(|link| link.get("block_fragment").and_then(Value::as_str).is_some())
        .collect();
    counts.insert(
        "block_fragment_occurrences".into(),
        observed_block_fragments.len(),
    );
    counts.insert(
        "resolved_block_targets".into(),
        observed_block_fragments
            .iter()
            .filter(|link| {
                link.get("block_target_evaluation").and_then(Value::as_str) == Some("resolved")
            })
            .count(),
    );
    counts.insert(
        "unresolved_block_targets".into(),
        observed_block_fragments
            .iter()
            .filter(|link| {
                link.get("block_target_evaluation").and_then(Value::as_str) != Some("resolved")
            })
            .count(),
    );
    counts.insert(
        "object_field_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| matches!(o.source, OccurrenceSource::ObjectField { .. }))
            .count(),
    );
    counts.insert(
        "semantic_region_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| matches!(o.source, OccurrenceSource::SemanticRegion { .. }))
            .count(),
    );
    counts.insert(
        "semantic_unit_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| matches!(o.source, OccurrenceSource::SemanticUnit { .. }))
            .count(),
    );
    counts.insert("object_classes".into(), projection.object_classes.len());
    counts.insert(
        "present_null_temporal_assignments".into(),
        admitted_notes
            .iter()
            .filter_map(|note| {
                note.get("frontmatter")
                    .and_then(|f| f.get("values"))
                    .and_then(Value::as_object)
            })
            .flat_map(|values| {
                [
                    "birthday",
                    "first_met",
                    "original_year_published",
                    "journal_entry_date",
                ]
                .into_iter()
                .filter(move |field| values.get(*field).is_some_and(Value::is_null))
            })
            .count(),
    );
    counts.insert(
        "resolved_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| matches!(o.resolution_state, OccurrenceResolutionState::Resolved))
            .count(),
    );
    counts.insert(
        "unresolved_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| matches!(o.resolution_state, OccurrenceResolutionState::Unresolved))
            .count(),
    );
    counts.insert(
        "ambiguous_occurrences".into(),
        projection
            .occurrences
            .iter()
            .filter(|o| {
                matches!(
                    o.resolution_state,
                    OccurrenceResolutionState::Ambiguous { .. }
                )
            })
            .count(),
    );
    counts
}

pub fn validate(
    observation_path: &Path,
    projection_path: &Path,
    output_path: &Path,
) -> Result<ValidationSummary, ValidationError> {
    if observation_path == projection_path
        || output_path == projection_path
        || output_path == observation_path
    {
        return Err(ValidationError::Input(
            "observation, Phase 5 input, and Phase 6 output paths must differ".into(),
        ));
    }
    let observation_bytes = fs::read(observation_path)?;
    let projection_bytes = fs::read(projection_path)?;
    let observation_hash = sha256(&observation_bytes);
    let projection_hash = sha256(&projection_bytes);
    if observation_hash != OBSERVATION_SHA256 {
        return Err(ValidationError::Input(format!(
            "observation byte hash mismatch: {observation_hash}"
        )));
    }
    if projection_hash != PROJECTION_SHA256 {
        return Err(ValidationError::Input(format!(
            "Phase 5 projection byte hash mismatch: {projection_hash}"
        )));
    }
    let root: Value = serde_json::from_slice(&observation_bytes)?;
    let projection: SemanticSpaceProjection = serde_json::from_slice(&projection_bytes)?;
    if text(&root, "observation_schema_version").as_deref() != Some("vault-observation/v3")
        || root
            .get("observer_provenance")
            .and_then(|p| text(p, "commit"))
            .as_deref()
            != Some(OBSERVER_COMMIT)
        || text(&root, "vault_resident_snapshot_identity").as_deref() != Some(CORPUS)
    {
        return Err(ValidationError::Input(
            "pinned observation identity mismatch".into(),
        ));
    }
    if projection.validation_status != crate::projection::ProjectionValidationStatus::Unvalidated {
        return Err(ValidationError::Input(
            "Phase 5 projection is not Unvalidated".into(),
        ));
    }
    if projection.corpus_snapshot_identity != CORPUS
        || projection.projection_snapshot_id != format!("projection:phase5:{CORPUS}")
    {
        return Err(ValidationError::Input(
            "pinned Phase 5 projection identity mismatch".into(),
        ));
    }
    let mut failures = BTreeMap::new();
    let mut violations = Vec::new();
    let counts = compare_observation(&projection, &root, &mut failures, &mut violations);
    check_typed_topology(&projection, &mut failures, &mut violations);
    let expected_descriptors: BTreeSet<_> =
        IDENTIFIER_FIELDS.iter().map(|s| s.to_string()).collect();
    let actual_descriptors: BTreeSet<_> = projection
        .identifier_descriptors
        .iter()
        .map(|d| d.identifier_name.clone())
        .collect();
    if actual_descriptors != expected_descriptors {
        *failures.entry("identifier".into()).or_default() += 1;
        violations.push("identifier descriptor universe mismatch".into());
    }
    let mut promoted = projection.clone();
    promoted.projection_snapshot_id = format!("projection:phase6:{CORPUS}");
    promoted.validation_status = crate::projection::ProjectionValidationStatus::Validated;
    promoted.logical_hash.clear();
    let canonical = serde_json::to_vec(&promoted)?;
    promoted.logical_hash = format!("sha256:{}", sha256(&canonical));
    let output_bytes = serde_json::to_vec(&promoted)?;
    let phase5_logical_hash = projection.logical_hash.clone();
    let mut summary = ValidationSummary {
        status: if violations.is_empty() {
            "Validated".into()
        } else {
            "Invalid".into()
        },
        observation_sha256: observation_hash,
        phase5_projection_sha256: projection_hash,
        phase5_logical_hash,
        phase5_snapshot_id: format!("projection:phase5:{CORPUS}"),
        phase6_snapshot_id: promoted.projection_snapshot_id.clone(),
        phase6_logical_hash: promoted.logical_hash.clone(),
        phase6_projection_sha256: sha256(&output_bytes),
        counts,
        failure_counts: failures,
        violations,
    };
    if summary.status != "Validated" {
        return Err(ValidationError::Violations(Box::new(summary)));
    }
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, &output_bytes)?;
    summary.counts.insert(
        "temporal_full_date".into(),
        promoted
            .temporal_anchors
            .iter()
            .filter(|a| matches!(a.value, TemporalValue::FullDate(_)))
            .count(),
    );
    summary.counts.insert(
        "temporal_datetime".into(),
        promoted
            .temporal_anchors
            .iter()
            .filter(|a| matches!(a.value, TemporalValue::DateTime(_)))
            .count(),
    );
    summary.counts.insert(
        "temporal_exact_year".into(),
        promoted
            .temporal_anchors
            .iter()
            .filter(|a| matches!(a.value, TemporalValue::ExactYear(_)))
            .count(),
    );
    summary.counts.insert(
        "temporal_month_day".into(),
        promoted
            .temporal_anchors
            .iter()
            .filter(|a| matches!(a.value, TemporalValue::MonthDay(_)))
            .count(),
    );
    summary.counts.insert(
        "temporal_approximate_year".into(),
        promoted
            .temporal_anchors
            .iter()
            .filter(|a| matches!(a.value, TemporalValue::ApproximateYear(_)))
            .count(),
    );
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AddressKind, Direction};
    use crate::projection::{
        CoverageSemantics, RetrievalSurfaceDescriptor, StructuralTransition,
        StructuralTransitionOperation,
    };

    fn base_projection() -> SemanticSpaceProjection {
        let object_id = SemanticObjectId::parse("019dcf5c-ded1-70df-ab68-c25bbc4e8eb1").unwrap();
        let region_address = SemanticRegionAddress::parse(object_id.clone(), "root").unwrap();
        let unit_id = SemanticUnitId::parse("unit-test").unwrap();
        let occurrence_id = crate::model::OccurrenceId::parse("occurrence-test").unwrap();
        let object = crate::projection::SemanticObjectRecord {
            object_id: object_id.clone(),
            source_identity: "source:test".into(),
            source_kind: crate::projection::SourceKind::Markdown,
            canonical_path: "note.md".into(),
            filename: "note.md".into(),
            title: "note.md".into(),
            aliases: vec![],
            object_class: "note".into(),
            region_addresses: vec![region_address.clone()],
            unit_ids: vec![unit_id.clone()],
            identifier_assignment_ids: vec![],
            object_field_occurrence_ids: vec![],
            body_occurrence_ids: vec![occurrence_id.clone()],
            incoming_occurrence_ids: vec![],
            temporal_anchor_ids: vec![],
            retrieval_surface_ids: vec!["surface:test".into()],
        };
        let region = crate::projection::SemanticRegionRecord {
            address: region_address.clone(),
            heading_path: vec![],
            heading_identity: "region:test".into(),
            source_span: None,
            child_region_addresses: vec![],
            contained_unit_ids: vec![unit_id.clone()],
            block_target_mappings: vec![],
            incoming_occurrence_ids: vec![],
            outgoing_occurrence_ids: vec![],
            inherited_identifier_assignment_ids: vec![],
            retrieval_surface_ids: vec!["surface:test".into()],
        };
        let unit = crate::projection::SemanticUnitRecord {
            unit_id: unit_id.clone(),
            parent_object_id: object_id.clone(),
            parent_region_address: region_address.clone(),
            authored_block_type: AuthoredBlockType::Paragraph,
            heading_path: vec![],
            block_ordinal: 1,
            explicit_block_id: None,
            content: SemanticUnitContent::Inline {
                authored_markdown: "x".into(),
                normalized_text: "x".into(),
            },
            inherited_identifier_assignment_ids: vec![],
            unit_local_identifier_assignment_ids: vec![],
            outgoing_occurrence_ids: vec![occurrence_id.clone()],
            incoming_occurrence_ids: vec![],
            temporal_anchor_ids: vec![],
            retrieval_surface_ids: vec!["surface:test".into()],
            source_provenance: RecordProvenance::SemanticUnit {
                unit_id: unit_id.clone(),
                source_span: None,
            },
            transport_segments: vec![],
        };
        SemanticSpaceProjection {
            projection_snapshot_id: "projection:test".into(),
            ingest_identity: "ingest:test".into(),
            schema_version: "projection/v1".into(),
            logical_hash: "sha256:test".into(),
            corpus_snapshot_identity: "corpus:test".into(),
            configuration_snapshot_id: "config:test".into(),
            validation_status: crate::projection::ProjectionValidationStatus::Unvalidated,
            object_classes: vec![],
            objects: vec![object],
            regions: vec![region],
            units: vec![unit],
            identifier_descriptors: vec![],
            identifier_assignments: vec![],
            occurrences: vec![],
            temporal_anchors: vec![],
            retrieval_surfaces: vec![],
            valid_transitions: vec![],
        }
    }
    fn detects(projection: &SemanticSpaceProjection, domain: &str) -> bool {
        let mut failures = BTreeMap::new();
        let mut violations = Vec::new();
        check_typed_topology(projection, &mut failures, &mut violations);
        failures.get(domain).copied().unwrap_or_default() > 0
    }

    #[test]
    fn detects_missing_or_duplicate_object_identity() {
        let mut p = base_projection();
        p.objects.push(p.objects[0].clone());
        assert!(detects(&p, "deterministic_identity"));
    }
    #[test]
    fn detects_invalid_region_and_unit_parent() {
        let mut p = base_projection();
        p.units[0].parent_object_id =
            SemanticObjectId::parse("019dcf5c-ded1-70df-ab68-c25bbc4e8eb2").unwrap();
        assert!(detects(&p, "unit"));
    }
    #[test]
    fn detects_excluded_identifier_and_null_shape() {
        let mut p = base_projection();
        p.identifier_assignments.push(IdentifierAssignment {
            assignment_id: "assignment:test".into(),
            identifier_name: "phone".into(),
            subject: SemanticAddress::Object(p.objects[0].object_id.clone()),
            value: IdentifierValue::String("x".into()),
            authored_raw_value: Some(Value::String("x".into())),
            provenance: RecordProvenance::ObjectField {
                object_id: p.objects[0].object_id.clone(),
                field_path: "phone".into(),
            },
        });
        assert!(detects(&p, "identifier"));
    }
    #[test]
    fn detects_invalid_target_and_reverse_incidence() {
        let mut p = base_projection();
        let unit = SemanticUnitId::parse("missing-unit").unwrap();
        p.units[0].outgoing_occurrence_ids.clear();
        p.occurrences.push(crate::projection::OccurrenceRecord {
            occurrence_id: crate::model::OccurrenceId::parse("occurrence-x").unwrap(),
            source: OccurrenceSource::SemanticUnit { unit_id: unit },
            authored_target_text: "x".into(),
            display_alias: None,
            resolved_target: Some(SemanticAddress::Object(p.objects[0].object_id.clone())),
            resolution_state: OccurrenceResolutionState::Resolved,
            presentation_mode: crate::projection::OccurrencePresentation::Link,
            direction: Direction::Outgoing,
            source_span: None,
        });
        assert!(
            detects(&p, "occurrence") || detects(&p, "target") || detects(&p, "reverse_incidence")
        );
    }
    #[test]
    fn detects_invalid_surface_bound_and_transition_reference() {
        let mut p = base_projection();
        p.retrieval_surfaces.push(RetrievalSurfaceDescriptor {
            surface_id: "surface:test".into(),
            kind: crate::model::RetrievalSurfaceKind::Exact,
            available: false,
            visible_address_kinds: vec![AddressKind::SemanticObject],
            match_modes: vec![],
            default_candidate_limit: 10,
            hard_candidate_limit: 1,
            returned_identity: AddressKind::SemanticObject,
            hydrates_to_semantic_units: false,
            coverage_semantics: CoverageSemantics::Bounded,
            exhaustive_total_count_supported: false,
            continuation_supported: false,
            technical_limitations: vec![],
        });
        p.valid_transitions.push(StructuralTransition {
            transition_id: "transition:test".into(),
            from: AddressKind::SemanticObject,
            operation: StructuralTransitionOperation::Containment,
            direction: Direction::Outgoing,
            to: AddressKind::SemanticUnit,
            retrieval_surface_id: Some("missing".into()),
        });
        assert!(
            detects(&p, "bounds")
                || detects(&p, "surface")
                || detects(&p, "deterministic_identity")
        );
    }
    #[test]
    fn unit_identity_changes_for_ordinal_and_explicit_id() {
        let id = SemanticObjectId::parse("019dcf5c-ded1-70df-ab68-c25bbc4e8eb1").unwrap();
        assert_ne!(
            independent_unit_id(&id, "root", 1, None),
            independent_unit_id(&id, "root", 2, None)
        );
        assert_ne!(
            independent_unit_id(&id, "root", 1, None),
            independent_unit_id(&id, "root", 1, Some("x"))
        );
    }
}
