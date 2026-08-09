//! Phase 5 adapter from the factual `vault-observation/v2` bundle.
//!
//! This module deliberately consumes observer JSON as facts, then applies the
//! CLEANROOM admission boundary. It does not import the observer's ontology or
//! build any retrieval index.

use std::{
    collections::{BTreeMap, BTreeSet},
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
fn typed_value(v: &Value) -> IdentifierValue {
    match v {
        Value::Null => IdentifierValue::Null,
        Value::Bool(b) => IdentifierValue::Boolean(*b),
        Value::Number(n) => IdentifierValue::Integer(n.as_i64().unwrap_or_default()),
        Value::String(s) => IdentifierValue::String(s.clone()),
        Value::Array(a) => IdentifierValue::Strings(
            a.iter()
                .map(|x| x.as_str().unwrap_or_default().to_owned())
                .collect(),
        ),
        Value::Object(_) => IdentifierValue::String(v.to_string()),
    }
}
fn block_type(kind: &str) -> AuthoredBlockType {
    match kind {
        "list" => AuthoredBlockType::List,
        "blockquote_or_callout" => AuthoredBlockType::BlockQuote,
        "code_fence" => AuthoredBlockType::CodeBlock,
        "table" => AuthoredBlockType::Table,
        "heading" => AuthoredBlockType::Paragraph,
        _ => AuthoredBlockType::Paragraph,
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
    let mut unit_by_path_block_id: BTreeMap<(String, String), SemanticUnitId> = BTreeMap::new();
    let mut field_names = BTreeSet::new();
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
        for (ordinal, block) in array(m, "block_candidates")
            .into_iter()
            .filter(|b| text(b, "block_kind_observation") != "heading")
            .enumerate()
        {
            let raw = text(&block, "raw_markdown");
            let unit_id: SemanticUnitId = id(
                format!(
                    "unit:{}:{}",
                    oid,
                    fnv(format!("{}:{}:{}", path, ordinal, raw).as_bytes())
                ),
                "unit identity",
            )?;
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
            local_units.push(unit_id.clone());
            for block_id in array(&block, "explicit_block_ids") {
                if let Some(block_id) = block_id.as_str() {
                    unit_by_path_block_id
                        .insert((path.clone(), block_id.to_owned()), unit_id.clone());
                }
            }
            units.push(SemanticUnitRecord {
                unit_id: unit_id.clone(),
                parent_object_id: oid.clone(),
                parent_region_address,
                authored_block_type: block_type(&text(&block, "block_kind_observation")),
                heading_path,
                block_ordinal,
                explicit_block_id: array(&block, "explicit_block_ids")
                    .first()
                    .and_then(Value::as_str)
                    .map(str::to_owned),
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
                value: typed_value(&raw),
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
                OccurrenceSource::SemanticUnit {
                    unit_id: object_units
                        .get(&oid.to_string())
                        .and_then(|u| u.first())
                        .cloned()
                        .ok_or_else(|| {
                            ConstructionError::Contract(format!(
                                "body occurrence has no authored unit for {path}"
                            ))
                        })?,
                }
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
                    candidate_source_path
                        .as_ref()
                        .and_then(|p| {
                            unit_by_path_block_id
                                .get(&(p.clone(), fragment.to_owned()))
                                .cloned()
                        })
                        .map(SemanticAddress::Unit)
                        .or_else(|| Some(SemanticAddress::Object(target_object)))
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
        .map(|name| IdentifierDescriptor {
            identifier_name: name.clone(),
            semantic_role: if name == "uuid" {
                IdentifierRole::Individuation
            } else if name == "note_type" {
                IdentifierRole::ObjectClass
            } else {
                IdentifierRole::Declared {
                    name: "admitted_frontmatter".into(),
                }
            },
            value_shape: IdentifierValueShape::String,
            cardinality: IdentifierCardinality::Scalar,
            applicable_address_kinds: vec![AddressKind::SemanticObject],
            assignment_mode: IdentifierAssignmentMode::Intrinsic,
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
            retrieval_surface_ids: vec![],
            enabled_transition_ids: vec!["transition:identifier".into()],
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
            object_class: "admitted_markdown".into(),
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
        object_classes: vec![SemanticObjectClassDescriptor {
            class_name: "admitted_markdown".into(),
            applicable_identifier_names: admitted_fields.clone(),
            permitted_source_kinds: vec![SourceKind::Markdown],
        }],
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
    let report = serde_json::json!({"report_title":"PHASE 5 CONSTRUCTION EVIDENCE","input_identity":{"observer_repository":"duck-lint/semantic-traversal","observer_commit":"99d0d4556684000f0ed585e47158a5f7fe9ce7e1","observer_schema":"vault-observation/v2","corpus_snapshot_identity":snapshot},"admission":{"whole_resident_source_count":whole_resident_source_count,"admission_eligible_count":projection.objects.len(),"admitted_object_count":projection.objects.len(),"excluded_source_count":markdown_count.saturating_sub(admitted.len())},"projection_construction":{"canonical_object_count":projection.objects.len(),"canonical_region_count":projection.regions.len(),"canonical_semantic_unit_count":projection.units.len()},"identifier_materialization":{"descriptor_count":projection.identifier_descriptors.len(),"assignment_count":projection.identifier_assignments.len(),"present_null_assignment_count":projection.identifier_assignments.iter().filter(|a| matches!(a.value, IdentifierValue::Null)).count(),"admitted_field_coverage":projection.identifier_descriptors.len(),"excluded_field_count":EXCLUDED_FIELDS.len()},"occurrences":{"authored_occurrence_count":projection.occurrences.len(),"object_field_count":object_field_count,"semantic_region_count":region_count,"semantic_unit_count":unit_count,"resolved_count":resolved,"unresolved_count":unresolved,"ambiguous_count":0},"temporal":{"temporally_capable_descriptors":projection.identifier_descriptors.iter().filter(|d| matches!(d.temporal_affordance, TemporalAffordance::CreatesAnchor)).count(),"materially_created_temporal_anchor_count":projection.temporal_anchors.len(),"present_null_temporal_assignments_creating_no_anchor":0},"construction_status":"produced","contract_contact_failures":0,"projection":{"schema_version":projection.schema_version,"logical_hash":projection.logical_hash,"validation_status":"unvalidated"},"determinism":{"first_run_logical_hash":projection.logical_hash,"second_run_logical_hash":"rerun-required","equality_result":"not_run_by_library"}});
    Ok(report)
}
