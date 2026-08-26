//! Read-only access artifacts over one validated semantic projection.
//!
//! This module is deliberately downstream of [`crate::projection`].  It does
//! not create canonical identities, infer relations, or decide relevance.  A
//! projection is the only source of addressable records; observation data is
//! used only to hydrate a projected unit whose content hash is checked.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::{Duration, Instant},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    model::{AddressKind, Direction, RetrievalSurfaceKind, SemanticAddress, SemanticRegionAddress},
    projection::{
        IdentifierAssignment, IdentifierValue, OccurrenceSource, ProjectionValidationStatus,
        SemanticSpaceProjection, SemanticUnitContent, TemporalValue,
    },
};

/// Version of the serialized access-artifact contract.
pub const ACCESS_SCHEMA_VERSION: &str = "projection-access-artifacts/v1";
/// Technical baseline requested for the prototype vector provider.
pub const VECTOR_MODEL: &str = "qwen3-embedding:0.6b";
pub const VECTOR_DIMENSION: usize = 1024;

/// A deterministic, technical description of the lexical index tokenizer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FtsTokenizerConfiguration {
    pub implementation: String,
    pub tokenization: String,
    pub case_folding: String,
    pub stop_words: String,
    pub punctuation: String,
}

impl Default for FtsTokenizerConfiguration {
    fn default() -> Self {
        Self {
            implementation: "cleanroom-unicode61-compatible".into(),
            tokenization: "unicode-alphanumeric-runs".into(),
            case_folding: "unicode-lowercase".into(),
            stop_words: "none".into(),
            punctuation: "delimiters".into(),
        }
    }
}

/// Technical vector-provider contract.  None of these fields are projection
/// facts and changing them cannot mutate the projection snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VectorProviderContract {
    pub provider: String,
    pub requested_model: String,
    pub dimension: usize,
    pub dtype: String,
    pub normalization: String,
    pub similarity: String,
    pub truncation: String,
}

impl Default for VectorProviderContract {
    fn default() -> Self {
        Self {
            provider: "Ollama".into(),
            requested_model: VECTOR_MODEL.into(),
            dimension: VECTOR_DIMENSION,
            dtype: "float32".into(),
            normalization: "L2".into(),
            similarity: "cosine".into(),
            truncation: "disabled".into(),
        }
    }
}

/// Immutable provider identity resolved outside the projection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VectorProviderIdentity {
    pub contract: VectorProviderContract,
    pub endpoint: String,
    pub resolved_model: String,
    pub model_digest: String,
    pub max_input_chars: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AccessFailure {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl AccessFailure {
    fn new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
}

/// Provider state is technical state, not corpus or projection state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum VectorProviderState {
    Ready {
        identity: VectorProviderIdentity,
    },
    Unavailable {
        contract: VectorProviderContract,
        failure: AccessFailure,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExactIndexEntry {
    pub literal: String,
    pub unit_id: crate::model::SemanticUnitId,
    pub representation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExactIndex {
    pub index_identity: String,
    pub entries: Vec<ExactIndexEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LexicalPosting {
    pub token: String,
    pub unit_id: crate::model::SemanticUnitId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LexicalIndex {
    pub index_identity: String,
    pub tokenizer: FtsTokenizerConfiguration,
    pub postings: Vec<LexicalPosting>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GraphEdge {
    pub source: SemanticAddress,
    pub target: SemanticAddress,
    pub transition_id: String,
    pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GraphIndex {
    pub index_identity: String,
    pub edges: Vec<GraphEdge>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum TemporalPrecision {
    FullDate,
    DateTime,
    ExactYear,
    MonthDay,
    ApproximateYear,
}

impl TemporalPrecision {
    fn rank(&self) -> u8 {
        match self {
            Self::FullDate => 0,
            Self::DateTime => 1,
            Self::ExactYear => 2,
            Self::MonthDay => 3,
            Self::ApproximateYear => 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TemporalIndexEntry {
    pub anchor_id: crate::model::TemporalAnchorId,
    pub subject: SemanticAddress,
    pub precision: TemporalPrecision,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TemporalIndex {
    pub index_identity: String,
    pub entries: Vec<TemporalIndexEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VectorSegmentRecord {
    pub segment_id: String,
    pub parent_unit_id: crate::model::SemanticUnitId,
    pub segment_ordinal: u32,
    pub total_segments: u32,
    pub embedding: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VectorIndex {
    pub index_identity: String,
    pub provider: VectorProviderState,
    pub segments: Vec<VectorSegmentRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionAccessManifest {
    pub access_schema_version: String,
    pub projection_snapshot_id: String,
    pub projection_logical_hash: String,
    pub corpus_snapshot_identity: String,
    pub exact_index_identity: String,
    pub lexical_index_identity: String,
    pub graph_index_identity: String,
    pub temporal_index_identity: String,
    pub vector_index_identity: String,
    pub vector_provider: VectorProviderState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionAccessArtifacts {
    pub artifact_identity: String,
    pub manifest: ProjectionAccessManifest,
    pub exact: ExactIndex,
    pub lexical: LexicalIndex,
    pub graph: GraphIndex,
    pub temporal: TemporalIndex,
    pub vector: VectorIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessError {
    InvalidProjection(String),
    Hydration(String),
    Serialization(String),
    Probe(String),
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProjection(message) => write!(f, "invalid projection: {message}"),
            Self::Hydration(message) => write!(f, "hydration failed: {message}"),
            Self::Serialization(message) => write!(f, "serialization failed: {message}"),
            Self::Probe(message) => write!(f, "probe failed: {message}"),
        }
    }
}

impl std::error::Error for AccessError {}

/// The provider seam is intentionally tiny.  It may be replaced by a test
/// double without changing access or projection contracts.
pub trait EmbeddingProvider: Sync {
    fn identity(&self) -> Result<VectorProviderIdentity, AccessFailure>;
    fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, AccessFailure>;
}

/// Build all read-only indexes from one validated projection.
pub fn build_projection_access_artifacts(
    projection: &SemanticSpaceProjection,
    observation: Option<&Value>,
    provider: Option<&dyn EmbeddingProvider>,
) -> Result<ProjectionAccessArtifacts, AccessError> {
    validate_projection(projection)?;
    let hydrated = hydrate_units(projection, observation)?;
    let exact = build_exact_index(projection, &hydrated)?;
    let lexical = build_lexical_index(&exact, &hydrated)?;
    let graph = build_graph_index(projection)?;
    let temporal = build_temporal_index(projection)?;
    let vector = build_vector_index(&hydrated, provider);
    let manifest = ProjectionAccessManifest {
        access_schema_version: ACCESS_SCHEMA_VERSION.into(),
        projection_snapshot_id: projection.projection_snapshot_id.clone(),
        projection_logical_hash: projection.logical_hash.clone(),
        corpus_snapshot_identity: projection.corpus_snapshot_identity.clone(),
        exact_index_identity: exact.index_identity.clone(),
        lexical_index_identity: lexical.index_identity.clone(),
        graph_index_identity: graph.index_identity.clone(),
        temporal_index_identity: temporal.index_identity.clone(),
        vector_index_identity: vector.index_identity.clone(),
        vector_provider: vector.provider.clone(),
    };
    let mut artifacts = ProjectionAccessArtifacts {
        artifact_identity: String::new(),
        manifest,
        exact,
        lexical,
        graph,
        temporal,
        vector,
    };
    artifacts.artifact_identity = hash_json(&artifacts)?;
    Ok(artifacts)
}

impl ProjectionAccessArtifacts {
    /// Verify both artifact self-integrity and exact projection binding before
    /// any probe is allowed to inspect an index.
    pub fn validate_against(
        &self,
        projection: &SemanticSpaceProjection,
    ) -> Result<(), AccessError> {
        validate_projection(projection)?;
        if self.manifest.access_schema_version != ACCESS_SCHEMA_VERSION {
            return Err(AccessError::InvalidProjection(
                "unsupported access artifact schema version".into(),
            ));
        }
        if self.manifest.projection_snapshot_id != projection.projection_snapshot_id
            || self.manifest.projection_logical_hash != projection.logical_hash
            || self.manifest.corpus_snapshot_identity != projection.corpus_snapshot_identity
        {
            return Err(AccessError::InvalidProjection(
                "access artifact is bound to a different projection".into(),
            ));
        }
        if self.manifest.exact_index_identity != self.exact.index_identity
            || self.manifest.lexical_index_identity != self.lexical.index_identity
            || self.manifest.graph_index_identity != self.graph.index_identity
            || self.manifest.temporal_index_identity != self.temporal.index_identity
            || self.manifest.vector_index_identity != self.vector.index_identity
            || self.manifest.vector_provider != self.vector.provider
        {
            return Err(AccessError::InvalidProjection(
                "access manifest does not match its indexes".into(),
            ));
        }
        if hash_json(&self.exact.entries)? != self.exact.index_identity
            || hash_json(&self.lexical.postings)? != self.lexical.index_identity
            || hash_json(&self.graph.edges)? != self.graph.index_identity
            || hash_json(&self.temporal.entries)? != self.temporal.index_identity
            || vector_index_identity(&self.vector.provider, &self.vector.segments)?
                != self.vector.index_identity
        {
            return Err(AccessError::InvalidProjection(
                "access index identity does not verify".into(),
            ));
        }
        let mut copy = self.clone();
        copy.artifact_identity.clear();
        if hash_json(&copy)? != self.artifact_identity {
            return Err(AccessError::InvalidProjection(
                "access artifact identity mismatch".into(),
            ));
        }
        Ok(())
    }

    pub fn probe(
        &self,
        projection: &SemanticSpaceProjection,
        probe: &ProjectionAccessProbe,
    ) -> Result<ProjectionAccessProbeResult, AccessError> {
        self.validate_against(projection)?;
        if probe.projection_snapshot_id != projection.projection_snapshot_id {
            return Err(AccessError::Probe(
                "probe projection snapshot does not match the projection".into(),
            ));
        }
        let started = Instant::now();
        let descriptor = projection
            .retrieval_surfaces
            .iter()
            .find(|surface| surface.surface_id == probe.surface_id)
            .ok_or_else(|| AccessError::Probe("unknown retrieval surface".into()))?;
        if descriptor.kind != probe.surface_kind
            || !descriptor.match_modes.contains(&probe.match_mode)
        {
            return Err(AccessError::Probe(
                "surface family or match mode is not declared by the projection".into(),
            ));
        }
        if probe.page_size == 0 {
            return Err(AccessError::Probe("page_size must be positive".into()));
        }

        let (mut candidates, failure, provider_identity) =
            match (&probe.surface_kind, &probe.operand) {
                (RetrievalSurfaceKind::Exact, AccessOperand::ExactLiteral(value)) => {
                    (self.exact_candidates(value), None, None)
                }
                (RetrievalSurfaceKind::Lexical, AccessOperand::LexicalTerms(values)) => {
                    match lexical_candidates(&self.lexical, values) {
                        Ok(values) => (values, None, None),
                        Err(message) => (
                            Vec::new(),
                            Some(AccessFailure::new("invalid_terms", message, false)),
                            None,
                        ),
                    }
                }
                (
                    RetrievalSurfaceKind::Graph,
                    AccessOperand::Graph {
                        seed,
                        direction,
                        transition_ids,
                    },
                ) => (
                    self.graph_candidates(seed, direction, transition_ids),
                    None,
                    None,
                ),
                (RetrievalSurfaceKind::Temporal, AccessOperand::Temporal(query)) => {
                    (temporal_candidates(&self.temporal, query), None, None)
                }
                (RetrievalSurfaceKind::Vector, AccessOperand::Vector(vector)) => {
                    match &self.vector.provider {
                        VectorProviderState::Unavailable { failure, .. } => (
                            Vec::new(),
                            Some(failure.clone()),
                            Some(self.vector.index_identity.clone()),
                        ),
                        VectorProviderState::Ready { identity } => (
                            vector_candidates(&self.vector, vector, identity.contract.dimension)?,
                            None,
                            Some(self.vector.index_identity.clone()),
                        ),
                    }
                }
                _ => {
                    return Err(AccessError::Probe(
                        "typed operand does not match the requested surface".into(),
                    ));
                }
            };
        candidates.sort_by(|left, right| {
            left.order
                .cmp(&right.order)
                .then_with(|| address_key(&left.identity).cmp(&address_key(&right.identity)))
        });
        for (order, candidate) in candidates.iter_mut().enumerate() {
            candidate.order = order;
        }
        let total = if failure.is_none() {
            Some(candidates.len())
        } else {
            None
        };
        let offset = parse_cursor(probe.cursor.as_deref(), &probe_fingerprint(probe))?;
        let end = offset.saturating_add(probe.page_size).min(candidates.len());
        let page = candidates[offset.min(candidates.len())..end].to_vec();
        let truncated = end < candidates.len();
        let continuation = truncated.then(|| AccessContinuation {
            cursor: format!("offset={end};fingerprint={}", probe_fingerprint(probe)),
        });
        Ok(ProjectionAccessProbeResult {
            probe_id: probe.probe_id.clone(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: probe.surface_id.clone(),
            surface_kind: probe.surface_kind.clone(),
            match_mode: probe.match_mode.clone(),
            candidates: page,
            returned_count: end.saturating_sub(offset.min(candidates.len())),
            total_candidate_count: total,
            truncated,
            continuation,
            index_identity: provider_identity.unwrap_or_else(|| match probe.surface_kind {
                RetrievalSurfaceKind::Exact => self.exact.index_identity.clone(),
                RetrievalSurfaceKind::Lexical => self.lexical.index_identity.clone(),
                RetrievalSurfaceKind::Graph => self.graph.index_identity.clone(),
                RetrievalSurfaceKind::Temporal => self.temporal.index_identity.clone(),
                RetrievalSurfaceKind::Vector => self.vector.index_identity.clone(),
            }),
            failure,
            duration_micros: started.elapsed().as_micros(),
        })
    }

    fn exact_candidates(&self, literal: &str) -> Vec<AccessCandidate> {
        let mut seen = HashSet::new();
        self.exact
            .entries
            .iter()
            .filter(|entry| entry.literal == literal && seen.insert(entry.unit_id.to_string()))
            .map(|entry| AccessCandidate {
                identity: SemanticAddress::Unit(entry.unit_id.clone()),
                order: 0,
                mechanical_score: None,
                transition_id: None,
            })
            .collect()
    }

    fn graph_candidates(
        &self,
        seed: &SemanticAddress,
        direction: &Direction,
        transition_ids: &[String],
    ) -> Vec<AccessCandidate> {
        self.graph
            .edges
            .iter()
            .filter(|edge| {
                edge.direction == *direction
                    && (transition_ids.is_empty() || transition_ids.contains(&edge.transition_id))
                    && match direction {
                        Direction::Outgoing => edge.source == *seed,
                        Direction::Incoming => edge.target == *seed,
                    }
            })
            .map(|edge| AccessCandidate {
                identity: match direction {
                    Direction::Outgoing => edge.target.clone(),
                    Direction::Incoming => edge.source.clone(),
                },
                order: 0,
                mechanical_score: None,
                transition_id: Some(edge.transition_id.clone()),
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionAccessProbe {
    pub probe_id: String,
    pub projection_snapshot_id: String,
    pub surface_id: String,
    pub surface_kind: RetrievalSurfaceKind,
    pub match_mode: crate::projection::SurfaceMatchMode,
    pub operand: AccessOperand,
    pub page_size: usize,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum AccessOperand {
    ExactLiteral(String),
    LexicalTerms(Vec<String>),
    Vector(Vec<f32>),
    Graph {
        seed: SemanticAddress,
        direction: Direction,
        transition_ids: Vec<String>,
    },
    Temporal(TemporalQuery),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum TemporalQuery {
    Exact {
        precision: TemporalPrecision,
        value: String,
    },
    Range {
        precision: TemporalPrecision,
        start: Option<String>,
        end: Option<String>,
    },
    Ordered {
        precision: Option<TemporalPrecision>,
        descending: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AccessCandidate {
    pub identity: SemanticAddress,
    pub order: usize,
    pub mechanical_score: Option<f32>,
    pub transition_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AccessContinuation {
    pub cursor: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionAccessProbeResult {
    pub probe_id: String,
    pub projection_snapshot_id: String,
    pub surface_id: String,
    pub surface_kind: RetrievalSurfaceKind,
    pub match_mode: crate::projection::SurfaceMatchMode,
    pub candidates: Vec<AccessCandidate>,
    pub returned_count: usize,
    pub total_candidate_count: Option<usize>,
    pub truncated: bool,
    pub continuation: Option<AccessContinuation>,
    pub index_identity: String,
    pub failure: Option<AccessFailure>,
    pub duration_micros: u128,
}

#[derive(Clone, Debug)]
struct HydratedUnitText {
    unit_id: crate::model::SemanticUnitId,
    raw: String,
    lexical: String,
}

fn validate_projection(projection: &SemanticSpaceProjection) -> Result<(), AccessError> {
    if projection.validation_status != ProjectionValidationStatus::Validated {
        return Err(AccessError::InvalidProjection(
            "access requires a validated projection".into(),
        ));
    }
    let mut canonical = projection.clone();
    canonical.logical_hash.clear();
    let expected = hash_json(&canonical)?;
    if projection.logical_hash != expected {
        return Err(AccessError::InvalidProjection(
            "projection logical hash does not verify".into(),
        ));
    }
    Ok(())
}

fn hydrate_units(
    projection: &SemanticSpaceProjection,
    observation: Option<&Value>,
) -> Result<Vec<HydratedUnitText>, AccessError> {
    let sources = observation.map(index_observation).transpose()?;
    projection
        .units
        .iter()
        .map(|unit| match &unit.content {
            SemanticUnitContent::Inline {
                authored_markdown,
                normalized_text,
            } => Ok(HydratedUnitText {
                unit_id: unit.unit_id.clone(),
                raw: authored_markdown.clone(),
                lexical: normalized_text.clone(),
            }),
            SemanticUnitContent::HydrationAddress {
                address,
                content_hash,
            } => {
                let sources = sources.as_ref().ok_or_else(|| {
                    AccessError::Hydration(format!("observation required for {}", unit.unit_id))
                })?;
                let (path, start, end) = parse_hydration_address(address)?;
                let provenance = match &unit.source_provenance {
                    crate::model::RecordProvenance::SemanticUnit {
                        unit_id,
                        source_span: Some(span),
                    } if unit_id == &unit.unit_id => span,
                    _ => {
                        return Err(AccessError::Hydration(format!(
                            "unit {} has no matching semantic-unit source provenance",
                            unit.unit_id
                        )));
                    }
                };
                if provenance.source != path
                    || provenance.start_byte != Some(start as u64)
                    || provenance.end_byte != Some(end as u64)
                {
                    return Err(AccessError::Hydration(format!(
                        "unit {} hydration address disagrees with provenance",
                        unit.unit_id
                    )));
                }
                let source = sources.get(&path).ok_or_else(|| {
                    AccessError::Hydration(format!("observation has no source {path}"))
                })?;
                let block = source
                    .blocks
                    .get(&(start as u64, end as u64))
                    .ok_or_else(|| {
                        AccessError::Hydration(format!(
                            "unit {} has no observation block at its projected span",
                            unit.unit_id
                        ))
                    })?;
                // These coordinates belong to the parser-native observation
                // blocks.  Do not reinterpret them as UTF-8 byte offsets in
                // serialized JSON text; the exact block is the factual join
                // surface and its bytes are what the projection hash covers.
                let raw = block.raw.clone();
                if format!("sha256:{}", sha256_bytes(raw.as_bytes())) != *content_hash {
                    return Err(AccessError::Hydration(format!(
                        "unit {} content hash mismatch",
                        unit.unit_id
                    )));
                }
                Ok(HydratedUnitText {
                    unit_id: unit.unit_id.clone(),
                    raw,
                    lexical: block
                        .parsed_text
                        .clone()
                        .unwrap_or_else(|| block.raw.clone()),
                })
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct ObservedBlock {
    raw: String,
    parsed_text: Option<String>,
}

#[derive(Clone, Debug)]
struct ObservedSource {
    blocks: HashMap<(u64, u64), ObservedBlock>,
}

fn index_observation(value: &Value) -> Result<HashMap<String, ObservedSource>, AccessError> {
    let notes = value
        .get("markdown_observations")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            AccessError::Hydration("observation markdown_observations is missing".into())
        })?;
    let mut output = HashMap::new();
    for note in notes {
        let source = note
            .get("source")
            .and_then(Value::as_object)
            .ok_or_else(|| AccessError::Hydration("observation source is missing".into()))?;
        let path = source
            .get("relative_path")
            .and_then(Value::as_str)
            .ok_or_else(|| AccessError::Hydration("observation relative_path is missing".into()))?
            .to_owned();
        let _source_raw = note
            .get("raw_markdown")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                AccessError::Hydration(format!("observation raw_markdown missing for {path}"))
            })?
            .to_owned();
        // The observation's source_byte_hash describes the resident source
        // bytes, while parser-native raw_markdown may normalize line endings.
        // Phase 7 therefore verifies the smaller joined block against the
        // projection's content_hash instead of inventing a whole-source byte
        // equivalence that the observation contract does not guarantee.
        let mut blocks = HashMap::new();
        for block in note
            .get("block_candidates")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let span = block
                .get("source_span")
                .and_then(span_pair)
                .ok_or_else(|| {
                    AccessError::Hydration(format!("observation block span missing for {path}"))
                })?;
            let raw_block = block
                .get("raw_markdown")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    AccessError::Hydration(format!("observation block text missing for {path}"))
                })?
                .to_owned();
            blocks.insert(
                span,
                ObservedBlock {
                    raw: raw_block,
                    parsed_text: block
                        .get("parsed_text")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                },
            );
        }
        output.insert(path, ObservedSource { blocks });
    }
    Ok(output)
}

fn span_pair(value: &Value) -> Option<(u64, u64)> {
    let values = value.as_array()?;
    Some((values.first()?.as_u64()?, values.get(1)?.as_u64()?))
}

fn parse_hydration_address(address: &str) -> Result<(String, usize, usize), AccessError> {
    let value = address.strip_prefix("source:").ok_or_else(|| {
        AccessError::Hydration(format!("unsupported hydration address {address}"))
    })?;
    let (path, bytes) = value.split_once("#bytes:").ok_or_else(|| {
        AccessError::Hydration(format!("hydration address has no byte span {address}"))
    })?;
    let mut parts = bytes.split(':');
    let start = parts
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| AccessError::Hydration(format!("invalid hydration start {address}")))?;
    let end = parts
        .next()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| AccessError::Hydration(format!("invalid hydration end {address}")))?;
    if parts.next().is_some() || start > end {
        return Err(AccessError::Hydration(format!(
            "invalid hydration span {address}"
        )));
    }
    Ok((path.to_owned(), start, end))
}

fn build_exact_index(
    projection: &SemanticSpaceProjection,
    hydrated: &[HydratedUnitText],
) -> Result<ExactIndex, AccessError> {
    let units_by_object: HashMap<_, _> = projection
        .objects
        .iter()
        .map(|object| (object.object_id.clone(), object.unit_ids.clone()))
        .collect();
    let units_by_region: HashMap<_, _> = projection
        .regions
        .iter()
        .map(|region| (region.address.clone(), region.contained_unit_ids.clone()))
        .collect();
    let hydrated_by_unit: HashMap<_, _> = hydrated
        .iter()
        .map(|text| (text.unit_id.clone(), text))
        .collect();
    let mut entries = Vec::new();
    let mut add =
        |literal: String, units: &[crate::model::SemanticUnitId], representation: &str| {
            if literal.is_empty() {
                return;
            }
            for unit_id in units {
                if hydrated_by_unit.contains_key(unit_id) {
                    entries.push(ExactIndexEntry {
                        literal: literal.clone(),
                        unit_id: unit_id.clone(),
                        representation: representation.into(),
                    });
                }
            }
        };
    for object in &projection.objects {
        let units = units_by_object
            .get(&object.object_id)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        add(object.object_id.to_string(), units, "object_id");
        add(object.canonical_path.clone(), units, "canonical_path");
        add(object.filename.clone(), units, "filename");
        add(object.title.clone(), units, "title");
        for alias in &object.aliases {
            add(alias.clone(), units, "alias");
        }
    }
    for region in &projection.regions {
        let units = units_by_region
            .get(&region.address)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        add(
            address_key(&SemanticAddress::Region(region.address.clone())),
            units,
            "region_address",
        );
        add(region.heading_identity.clone(), units, "heading_identity");
        for heading in &region.heading_path {
            add(heading.clone(), units, "heading");
        }
    }
    for unit in &projection.units {
        add(
            unit.unit_id.to_string(),
            std::slice::from_ref(&unit.unit_id),
            "unit_id",
        );
        if let Some(block_id) = &unit.explicit_block_id {
            add(
                block_id.clone(),
                std::slice::from_ref(&unit.unit_id),
                "explicit_block_id",
            );
        }
        if let Some(text) = hydrated_by_unit.get(&unit.unit_id) {
            add(
                text.raw.clone(),
                std::slice::from_ref(&unit.unit_id),
                "hydrated_text",
            );
        }
    }
    for assignment in &projection.identifier_assignments {
        let units = units_for_address(&assignment.subject, &units_by_object, &units_by_region);
        for value in identifier_strings(assignment) {
            add(
                value,
                &units,
                &format!("identifier:{}", assignment.identifier_name),
            );
        }
    }
    entries.sort_by(|left, right| {
        left.literal
            .cmp(&right.literal)
            .then_with(|| left.unit_id.to_string().cmp(&right.unit_id.to_string()))
            .then_with(|| left.representation.cmp(&right.representation))
    });
    entries.dedup_by(|left, right| {
        left.literal == right.literal
            && left.unit_id == right.unit_id
            && left.representation == right.representation
    });
    let index_identity = hash_json(&entries)?;
    Ok(ExactIndex {
        index_identity,
        entries,
    })
}

fn build_lexical_index(
    exact: &ExactIndex,
    hydrated: &[HydratedUnitText],
) -> Result<LexicalIndex, AccessError> {
    let tokenizer = FtsTokenizerConfiguration::default();
    let mut postings = Vec::new();
    for entry in &exact.entries {
        for token in tokenize(&entry.literal) {
            postings.push(LexicalPosting {
                token,
                unit_id: entry.unit_id.clone(),
            });
        }
    }
    for text in hydrated {
        for token in tokenize(&text.lexical) {
            postings.push(LexicalPosting {
                token,
                unit_id: text.unit_id.clone(),
            });
        }
    }
    postings.sort_by(|left, right| {
        left.token
            .cmp(&right.token)
            .then_with(|| left.unit_id.to_string().cmp(&right.unit_id.to_string()))
    });
    postings.dedup();
    let index_identity = hash_json(&postings)?;
    Ok(LexicalIndex {
        index_identity,
        tokenizer,
        postings,
    })
}

fn tokenize(value: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut token = String::new();
    for character in value.chars() {
        if character.is_alphanumeric() {
            for lower in character.to_lowercase() {
                token.push(lower);
            }
        } else if !token.is_empty() {
            output.push(std::mem::take(&mut token));
        }
    }
    if !token.is_empty() {
        output.push(token);
    }
    output.sort();
    output.dedup();
    output
}

fn lexical_candidates(
    index: &LexicalIndex,
    values: &[String],
) -> Result<Vec<AccessCandidate>, String> {
    if values.is_empty() {
        return Err("terms operand must not be empty".into());
    }
    let mut required = Vec::new();
    for value in values {
        let tokens = tokenize(value);
        if tokens.len() != 1 || tokens[0] != value.to_lowercase() {
            return Err(format!(
                "terms operand must contain exactly one configured token: {value}"
            ));
        }
        required.push(tokens[0].clone());
    }
    required.sort();
    required.dedup();
    let mut units: Option<HashSet<String>> = None;
    for token in required {
        let current: HashSet<_> = index
            .postings
            .iter()
            .filter(|posting| posting.token == token)
            .map(|posting| posting.unit_id.to_string())
            .collect();
        units = Some(match units {
            None => current,
            Some(previous) => previous.intersection(&current).cloned().collect(),
        });
    }
    let mut output: Vec<_> = units
        .unwrap_or_default()
        .into_iter()
        .filter_map(|id| crate::model::SemanticUnitId::parse(id).ok())
        .map(|unit_id| AccessCandidate {
            identity: SemanticAddress::Unit(unit_id),
            order: 0,
            mechanical_score: None,
            transition_id: None,
        })
        .collect();
    output.sort_by(|left, right| address_key(&left.identity).cmp(&address_key(&right.identity)));
    Ok(output)
}

fn build_graph_index(projection: &SemanticSpaceProjection) -> Result<GraphIndex, AccessError> {
    let mut edges = Vec::new();
    for object in &projection.objects {
        let object_address = SemanticAddress::Object(object.object_id.clone());
        for region in &object.region_addresses {
            edges.push(GraphEdge {
                source: object_address.clone(),
                target: SemanticAddress::Region(region.clone()),
                transition_id: "transition:object-region".into(),
                direction: Direction::Outgoing,
            });
            edges.push(GraphEdge {
                source: SemanticAddress::Region(region.clone()),
                target: object_address.clone(),
                transition_id: "transition:unit-region".into(),
                direction: Direction::Incoming,
            });
        }
        for unit in &object.unit_ids {
            edges.push(GraphEdge {
                source: object_address.clone(),
                target: SemanticAddress::Unit(unit.clone()),
                transition_id: "transition:object-unit".into(),
                direction: Direction::Outgoing,
            });
            edges.push(GraphEdge {
                source: SemanticAddress::Unit(unit.clone()),
                target: object_address.clone(),
                transition_id: "transition:unit-object".into(),
                direction: Direction::Outgoing,
            });
        }
        for occurrence in object
            .object_field_occurrence_ids
            .iter()
            .chain(object.body_occurrence_ids.iter())
        {
            let occurrence = SemanticAddress::Occurrence(occurrence.clone());
            edges.push(GraphEdge {
                source: object_address.clone(),
                target: occurrence.clone(),
                transition_id: "transition:object-occurrence-outgoing".into(),
                direction: Direction::Outgoing,
            });
            edges.push(GraphEdge {
                source: occurrence,
                target: object_address.clone(),
                transition_id: "transition:object-occurrence-incoming".into(),
                direction: Direction::Incoming,
            });
        }
    }
    for region in &projection.regions {
        let region_address = SemanticAddress::Region(region.address.clone());
        for child in &region.child_region_addresses {
            edges.push(GraphEdge {
                source: region_address.clone(),
                target: SemanticAddress::Region(child.clone()),
                transition_id: "transition:object-region".into(),
                direction: Direction::Outgoing,
            });
        }
        for unit in &region.contained_unit_ids {
            edges.push(GraphEdge {
                source: region_address.clone(),
                target: SemanticAddress::Unit(unit.clone()),
                transition_id: "transition:region-unit".into(),
                direction: Direction::Outgoing,
            });
            edges.push(GraphEdge {
                source: SemanticAddress::Unit(unit.clone()),
                target: region_address.clone(),
                transition_id: "transition:unit-region".into(),
                direction: Direction::Outgoing,
            });
        }
        for occurrence_id in &region.outgoing_occurrence_ids {
            edges.push(GraphEdge {
                source: region_address.clone(),
                target: SemanticAddress::Occurrence(occurrence_id.clone()),
                transition_id: "transition:region-occurrence-outgoing".into(),
                direction: Direction::Outgoing,
            });
        }
    }
    for unit in &projection.units {
        let unit_address = SemanticAddress::Unit(unit.unit_id.clone());
        for occurrence_id in &unit.outgoing_occurrence_ids {
            edges.push(GraphEdge {
                source: unit_address.clone(),
                target: SemanticAddress::Occurrence(occurrence_id.clone()),
                transition_id: "transition:unit-occurrence-outgoing".into(),
                direction: Direction::Outgoing,
            });
        }
    }
    for occurrence in &projection.occurrences {
        let occurrence_address = SemanticAddress::Occurrence(occurrence.occurrence_id.clone());
        edges.push(GraphEdge {
            source: occurrence_source_address(&occurrence.source),
            target: occurrence_address.clone(),
            transition_id: source_transition(&occurrence.source),
            direction: Direction::Outgoing,
        });
        if let Some(target) = &occurrence.resolved_target {
            edges.push(GraphEdge {
                source: occurrence_address.clone(),
                target: target.clone(),
                transition_id: target_transition(target),
                direction: Direction::Outgoing,
            });
        }
    }
    let outgoing_edges = edges
        .iter()
        .filter(|edge| edge.direction == Direction::Outgoing)
        .cloned()
        .collect::<Vec<_>>();
    for edge in outgoing_edges {
        edges.push(GraphEdge {
            source: edge.target,
            target: edge.source,
            transition_id: incoming_transition(&edge.transition_id),
            direction: Direction::Incoming,
        });
    }
    edges.sort_by(|left, right| {
        address_key(&left.source)
            .cmp(&address_key(&right.source))
            .then_with(|| direction_key(&left.direction).cmp(&direction_key(&right.direction)))
            .then_with(|| left.transition_id.cmp(&right.transition_id))
            .then_with(|| address_key(&left.target).cmp(&address_key(&right.target)))
    });
    edges.dedup();
    Ok(GraphIndex {
        index_identity: hash_json(&edges)?,
        edges,
    })
}

fn occurrence_source_address(source: &OccurrenceSource) -> SemanticAddress {
    match source {
        OccurrenceSource::ObjectField { object_id, .. } => {
            SemanticAddress::Object(object_id.clone())
        }
        OccurrenceSource::SemanticRegion { region_address } => {
            SemanticAddress::Region(region_address.clone())
        }
        OccurrenceSource::SemanticUnit { unit_id } => SemanticAddress::Unit(unit_id.clone()),
    }
}

fn source_transition(source: &OccurrenceSource) -> String {
    match source {
        OccurrenceSource::ObjectField { .. } => "transition:object-occurrence-outgoing",
        OccurrenceSource::SemanticRegion { .. } => "transition:region-occurrence-outgoing",
        OccurrenceSource::SemanticUnit { .. } => "transition:unit-occurrence-outgoing",
    }
    .into()
}

fn target_transition(target: &SemanticAddress) -> String {
    match target.kind() {
        AddressKind::SemanticObject => "transition:occurrence-object-target",
        AddressKind::SemanticRegion => "transition:occurrence-region-target",
        AddressKind::SemanticUnit => "transition:occurrence-unit-target",
        _ => "transition:occurrence-unit-target",
    }
    .into()
}

fn incoming_transition(transition_id: &str) -> String {
    match transition_id {
        "transition:object-occurrence-outgoing" => "transition:object-occurrence-incoming",
        "transition:region-occurrence-outgoing" => "transition:region-occurrence-incoming",
        "transition:unit-occurrence-outgoing" => "transition:unit-occurrence-incoming",
        "transition:occurrence-object-target" => "transition:object-occurrence-incoming",
        "transition:occurrence-region-target" => "transition:region-occurrence-incoming",
        "transition:occurrence-unit-target" => "transition:unit-occurrence-incoming",
        "transition:object-unit" => "transition:unit-object",
        "transition:region-unit" => "transition:unit-region",
        "transition:unit-object" => "transition:object-unit",
        "transition:unit-region" => "transition:region-unit",
        other => other,
    }
    .into()
}

fn direction_key(direction: &Direction) -> u8 {
    match direction {
        Direction::Outgoing => 0,
        Direction::Incoming => 1,
    }
}

fn build_temporal_index(
    projection: &SemanticSpaceProjection,
) -> Result<TemporalIndex, AccessError> {
    let mut entries = projection
        .temporal_anchors
        .iter()
        .map(|anchor| {
            let (precision, value) = temporal_value(&anchor.value);
            TemporalIndexEntry {
                anchor_id: anchor.anchor_id.clone(),
                subject: anchor.subject.clone(),
                precision,
                value,
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        left.precision
            .rank()
            .cmp(&right.precision.rank())
            .then_with(|| left.value.cmp(&right.value))
            .then_with(|| left.anchor_id.to_string().cmp(&right.anchor_id.to_string()))
    });
    Ok(TemporalIndex {
        index_identity: hash_json(&entries)?,
        entries,
    })
}

fn temporal_value(value: &TemporalValue) -> (TemporalPrecision, String) {
    match value {
        TemporalValue::FullDate(value) => (TemporalPrecision::FullDate, value.clone()),
        TemporalValue::DateTime(value) => (TemporalPrecision::DateTime, value.clone()),
        TemporalValue::ExactYear(value) => (TemporalPrecision::ExactYear, value.to_string()),
        TemporalValue::MonthDay(value) => (TemporalPrecision::MonthDay, value.clone()),
        TemporalValue::ApproximateYear(value) => {
            (TemporalPrecision::ApproximateYear, value.clone())
        }
    }
}

fn temporal_candidates(index: &TemporalIndex, query: &TemporalQuery) -> Vec<AccessCandidate> {
    let entries: Vec<_> = match query {
        TemporalQuery::Exact { precision, value } => index
            .entries
            .iter()
            .filter(|entry| &entry.precision == precision && &entry.value == value)
            .collect(),
        TemporalQuery::Range {
            precision,
            start,
            end,
        } => index
            .entries
            .iter()
            .filter(|entry| {
                &entry.precision == precision
                    && start
                        .as_ref()
                        .map(|start| &entry.value >= start)
                        .unwrap_or(true)
                    && end.as_ref().map(|end| &entry.value <= end).unwrap_or(true)
            })
            .collect(),
        TemporalQuery::Ordered {
            precision,
            descending,
        } => {
            let mut values: Vec<_> = index
                .entries
                .iter()
                .filter(|entry| {
                    precision
                        .as_ref()
                        .map(|precision| &entry.precision == precision)
                        .unwrap_or(true)
                })
                .collect();
            values.sort_by(|left, right| {
                left.precision
                    .rank()
                    .cmp(&right.precision.rank())
                    .then_with(|| left.value.cmp(&right.value))
                    .then_with(|| left.anchor_id.to_string().cmp(&right.anchor_id.to_string()))
            });
            if *descending {
                values.reverse();
            }
            values
        }
    };
    entries
        .iter()
        .enumerate()
        .map(|(order, entry)| AccessCandidate {
            identity: SemanticAddress::TemporalAnchor(entry.anchor_id.clone()),
            order,
            mechanical_score: None,
            transition_id: None,
        })
        .collect()
}

fn build_vector_index(
    hydrated: &[HydratedUnitText],
    provider: Option<&dyn EmbeddingProvider>,
) -> VectorIndex {
    let Some(provider) = provider else {
        return unavailable_vector_index(
            VectorProviderContract::default(),
            AccessFailure::new(
                "provider_not_configured",
                "Ollama provider was not configured",
                false,
            ),
        );
    };
    let identity = match provider.identity() {
        Ok(identity) => identity,
        Err(failure) => {
            return unavailable_vector_index(VectorProviderContract::default(), failure);
        }
    };
    if identity.contract.dimension == 0 {
        return unavailable_vector_index(
            identity.contract.clone(),
            AccessFailure::new(
                "invalid_provider_dimension",
                "provider dimension must be positive",
                false,
            ),
        );
    }
    let unit_inputs = hydrated
        .iter()
        .map(|text| text.raw.clone())
        .collect::<Vec<_>>();
    let results =
        match embed_units_concurrently(provider, &unit_inputs, identity.contract.dimension) {
            Ok(results) => results,
            Err(failure) => {
                return unavailable_vector_index(identity.contract.clone(), failure);
            }
        };
    let mut segments = Vec::new();
    for (unit_index, (result, text)) in results.into_iter().zip(hydrated).enumerate() {
        let unit_context = format!(
            "unit_index={unit_index}; parent_unit_id={}; input_bytes={}",
            text.unit_id,
            text.raw.len()
        );
        let unit_segments = match result {
            Ok(unit_segments) => unit_segments,
            Err(mut failure) => {
                failure.message = format!("{unit_context}; {}", failure.message);
                return unavailable_vector_index(identity.contract.clone(), failure);
            }
        };
        if unit_segments.is_empty() {
            return unavailable_vector_index(
                identity.contract.clone(),
                AccessFailure::new(
                    "provider_segmentation_failed",
                    format!("{unit_context}; provider returned no transport segments"),
                    false,
                ),
            );
        }
        let total_segments = unit_segments.len() as u32;
        for (segment_ordinal, segment) in unit_segments.into_iter().enumerate() {
            segments.push(VectorSegmentRecord {
                segment_id: format!("vector-segment:v1:{}:{segment_ordinal}", text.unit_id),
                parent_unit_id: text.unit_id.clone(),
                segment_ordinal: segment_ordinal as u32,
                total_segments,
                embedding: segment.embedding,
            });
        }
    }
    let state = VectorProviderState::Ready { identity };
    VectorIndex {
        index_identity: vector_index_identity(&state, &segments)
            .unwrap_or_else(|_| "vector-index:ready".into()),
        provider: state,
        segments,
    }
}

const VECTOR_PROVIDER_MAX_WORKERS: usize = 8;

#[derive(Clone, Debug)]
struct ProviderVectorSegment {
    text: String,
    embedding: Vec<f32>,
}

fn unavailable_vector_index(
    contract: VectorProviderContract,
    failure: AccessFailure,
) -> VectorIndex {
    let state = VectorProviderState::Unavailable { contract, failure };
    VectorIndex {
        index_identity: vector_index_identity(&state, &[])
            .unwrap_or_else(|_| "vector-index:unavailable".into()),
        provider: state,
        segments: Vec::new(),
    }
}

fn embed_units_concurrently(
    provider: &dyn EmbeddingProvider,
    inputs: &[String],
    dimension: usize,
) -> Result<Vec<Result<Vec<ProviderVectorSegment>, AccessFailure>>, AccessFailure> {
    if inputs.is_empty() {
        return Ok(Vec::new());
    }
    let worker_count = VECTOR_PROVIDER_MAX_WORKERS.min(inputs.len());
    let (job_sender, job_receiver) = std::sync::mpsc::channel::<(usize, String)>();
    let (result_sender, result_receiver) =
        std::sync::mpsc::channel::<(usize, Result<Vec<ProviderVectorSegment>, AccessFailure>)>();
    let job_receiver = std::sync::Arc::new(std::sync::Mutex::new(job_receiver));

    let results = std::thread::scope(|scope| {
        for _ in 0..worker_count {
            let job_receiver = std::sync::Arc::clone(&job_receiver);
            let result_sender = result_sender.clone();
            scope.spawn(move || {
                loop {
                    let job = job_receiver
                        .lock()
                        .expect("vector provider job mutex is not poisoned")
                        .recv();
                    let Ok((index, input)) = job else {
                        break;
                    };
                    let result = segment_and_embed(provider, &input, dimension);
                    result_sender
                        .send((index, result))
                        .expect("vector provider result receiver remains active");
                }
            });
        }
        drop(result_sender);
        for (index, input) in inputs.iter().cloned().enumerate() {
            job_sender.send((index, input)).map_err(|_| {
                AccessFailure::new(
                    "provider_concurrency_failed",
                    "vector provider worker queue closed",
                    false,
                )
            })?;
        }
        drop(job_sender);

        let mut results: Vec<Option<Result<Vec<ProviderVectorSegment>, AccessFailure>>> =
            (0..inputs.len()).map(|_| None).collect();
        for _ in 0..inputs.len() {
            let (index, result) = result_receiver.recv().map_err(|_| {
                AccessFailure::new(
                    "provider_concurrency_failed",
                    "vector provider worker result channel closed",
                    false,
                )
            })?;
            results[index] = Some(result);
        }
        Ok::<_, AccessFailure>(
            results
                .into_iter()
                .map(|result| result.expect("every vector segment has one provider result"))
                .collect(),
        )
    })?;
    Ok(results)
}

fn vector_index_identity(
    provider: &VectorProviderState,
    segments: &[VectorSegmentRecord],
) -> Result<String, AccessError> {
    hash_json(&(provider, segments))
}

fn segment_and_embed(
    provider: &dyn EmbeddingProvider,
    text: &str,
    dimension: usize,
) -> Result<Vec<ProviderVectorSegment>, AccessFailure> {
    if text.is_empty() {
        return Err(AccessFailure::new(
            "provider_request_shape",
            "vector input must be a non-empty string",
            false,
        ));
    }
    let mut accepted = HashMap::<String, Result<Vec<f32>, AccessFailure>>::new();
    let mut try_input = |input: &str| -> Result<Option<Vec<f32>>, AccessFailure> {
        if let Some(result) = accepted.get(input) {
            return match result {
                Ok(embedding) => Ok(Some(embedding.clone())),
                Err(failure) => Err(failure.clone()),
            };
        }
        let result = provider.embed(&[input.to_owned()]).and_then(|embeddings| {
            if embeddings.len() != 1 {
                return Err(AccessFailure::new(
                    "provider_shape_mismatch",
                    format!(
                        "expected exactly one embedding for one input; \
                         returned_embeddings={}",
                        embeddings.len()
                    ),
                    false,
                ));
            }
            let embedding = embeddings.into_iter().next().expect("count checked");
            if embedding.len() != dimension {
                return Err(AccessFailure::new(
                    "provider_shape_mismatch",
                    format!(
                        "expected_dimension={dimension}; returned_dimension={}",
                        embedding.len()
                    ),
                    false,
                ));
            }
            Ok(embedding)
        });
        match result {
            Ok(embedding) => {
                accepted.insert(input.to_owned(), Ok(embedding.clone()));
                Ok(Some(embedding))
            }
            Err(failure) if is_provider_capacity_rejection(&failure) => Ok(None),
            Err(failure) => Err(failure),
        }
    };

    if let Some(embedding) = try_input(text)? {
        return Ok(vec![ProviderVectorSegment {
            text: text.to_owned(),
            embedding,
        }]);
    }

    let mut segments = Vec::new();
    let mut remainder = text.to_owned();
    while !remainder.is_empty() {
        if let Some(embedding) = try_input(&remainder)? {
            segments.push(ProviderVectorSegment {
                text: remainder,
                embedding,
            });
            break;
        }
        let mut chosen = None;
        for boundary in preferred_boundaries(&remainder) {
            if let Some(embedding) = try_input(&remainder[..boundary])? {
                chosen = Some((boundary, embedding));
                break;
            }
        }
        let Some((boundary, embedding)) = chosen else {
            return Err(AccessFailure::new(
                "provider_segmentation_failed",
                format!(
                    "provider rejected every non-empty prefix of remainder; \
                     remainder_bytes={}",
                    remainder.len()
                ),
                false,
            ));
        };
        let segment_text = remainder[..boundary].to_owned();
        segments.push(ProviderVectorSegment {
            text: segment_text,
            embedding,
        });
        remainder = remainder[boundary..].to_owned();
    }
    if segments
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<String>()
        != text
    {
        return Err(AccessFailure::new(
            "provider_segmentation_failed",
            "transport segments do not reconstruct the exact provider input",
            false,
        ));
    }
    Ok(segments)
}

fn is_provider_capacity_rejection(failure: &AccessFailure) -> bool {
    failure.code == "provider_capacity_exceeded"
}

fn preferred_boundaries(value: &str) -> Vec<usize> {
    let mut newline = Vec::new();
    let mut whitespace = Vec::new();
    let mut code_point = Vec::new();
    for (byte_index, character) in value.char_indices() {
        let boundary = byte_index + character.len_utf8();
        if boundary >= value.len() {
            continue;
        }
        code_point.push(boundary);
        if character == '\n' {
            newline.push(boundary);
        } else if character.is_whitespace() {
            whitespace.push(boundary);
        }
    }
    newline.sort_unstable_by(|left, right| right.cmp(left));
    whitespace.sort_unstable_by(|left, right| right.cmp(left));
    code_point.sort_unstable_by(|left, right| right.cmp(left));
    newline.extend(whitespace);
    newline.extend(code_point);
    newline
}

fn vector_candidates(
    index: &VectorIndex,
    query: &[f32],
    dimension: usize,
) -> Result<Vec<AccessCandidate>, AccessError> {
    if query.len() != dimension {
        return Err(AccessError::Probe(format!(
            "vector operand dimension {} does not match provider dimension {dimension}",
            query.len()
        )));
    }
    let query_norm = query.iter().map(|value| value * value).sum::<f32>().sqrt();
    if query_norm == 0.0 {
        return Err(AccessError::Probe(
            "zero vector operand is not searchable".into(),
        ));
    }
    let mut by_unit: HashMap<String, (crate::model::SemanticUnitId, f32)> = HashMap::new();
    for segment in &index.segments {
        let norm = segment
            .embedding
            .iter()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();
        if norm == 0.0 {
            continue;
        }
        let score = segment
            .embedding
            .iter()
            .zip(query)
            .map(|(left, right)| left * right)
            .sum::<f32>()
            / (norm * query_norm);
        let key = segment.parent_unit_id.to_string();
        if by_unit
            .get(&key)
            .map(|(_, current)| score > *current)
            .unwrap_or(true)
        {
            by_unit.insert(key, (segment.parent_unit_id.clone(), score));
        }
    }
    let mut output: Vec<_> = by_unit
        .into_values()
        .map(|(unit_id, score)| AccessCandidate {
            identity: SemanticAddress::Unit(unit_id),
            order: 0,
            mechanical_score: Some(score),
            transition_id: None,
        })
        .collect();
    output.sort_by(|left, right| {
        right
            .mechanical_score
            .partial_cmp(&left.mechanical_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| address_key(&left.identity).cmp(&address_key(&right.identity)))
    });
    Ok(output)
}

fn units_for_address(
    address: &SemanticAddress,
    objects: &HashMap<crate::model::SemanticObjectId, Vec<crate::model::SemanticUnitId>>,
    regions: &HashMap<SemanticRegionAddress, Vec<crate::model::SemanticUnitId>>,
) -> Vec<crate::model::SemanticUnitId> {
    match address {
        SemanticAddress::Object(id) => objects.get(id).cloned().unwrap_or_default(),
        SemanticAddress::Region(address) => regions.get(address).cloned().unwrap_or_default(),
        SemanticAddress::Unit(id) => vec![id.clone()],
        _ => Vec::new(),
    }
}

fn identifier_strings(assignment: &IdentifierAssignment) -> Vec<String> {
    let mut output = Vec::new();
    fn visit(value: &IdentifierValue, output: &mut Vec<String>) {
        match value {
            IdentifierValue::String(value) => output.push(value.clone()),
            IdentifierValue::Integer(value) => output.push(value.to_string()),
            IdentifierValue::Boolean(value) => output.push(value.to_string()),
            IdentifierValue::SemanticAddress(value) => output.push(address_key(value)),
            IdentifierValue::Strings(values) => output.extend(values.iter().cloned()),
            IdentifierValue::Integers(values) => {
                output.extend(values.iter().map(ToString::to_string))
            }
            IdentifierValue::Booleans(values) => {
                output.extend(values.iter().map(ToString::to_string))
            }
            IdentifierValue::Values(values) => values.iter().for_each(|value| visit(value, output)),
            IdentifierValue::SemanticAddresses(values) => values
                .iter()
                .for_each(|value| output.push(address_key(value))),
            IdentifierValue::Null => {}
        }
    }
    visit(&assignment.value, &mut output);
    if let Some(value) = &assignment.authored_raw_value {
        raw_value_strings(value, &mut output);
    }
    output
}

fn raw_value_strings(value: &Value, output: &mut Vec<String>) {
    match value {
        Value::String(value) => output.push(value.clone()),
        Value::Number(value) => output.push(value.to_string()),
        Value::Bool(value) => output.push(value.to_string()),
        Value::Array(values) => values
            .iter()
            .for_each(|value| raw_value_strings(value, output)),
        _ => {}
    }
}

fn address_key(address: &SemanticAddress) -> String {
    serde_json::to_string(address).unwrap_or_else(|_| format!("{address:?}"))
}

fn probe_fingerprint(probe: &ProjectionAccessProbe) -> String {
    let value = serde_json::json!({
        "projection_snapshot_id": probe.projection_snapshot_id,
        "surface_id": probe.surface_id,
        "surface_kind": probe.surface_kind,
        "match_mode": probe.match_mode,
        "operand": probe.operand,
    });
    hash_json(&value).unwrap_or_else(|_| "probe".into())
}

fn parse_cursor(cursor: Option<&str>, fingerprint: &str) -> Result<usize, AccessError> {
    let Some(cursor) = cursor else { return Ok(0) };
    let Some(cursor) = cursor.strip_prefix("offset=") else {
        return Err(AccessError::Probe("invalid continuation cursor".into()));
    };
    let Some((offset, actual_fingerprint)) = cursor.split_once(";fingerprint=") else {
        return Err(AccessError::Probe("invalid continuation cursor".into()));
    };
    if actual_fingerprint != fingerprint {
        return Err(AccessError::Probe(
            "continuation cursor belongs to another probe".into(),
        ));
    }
    offset
        .parse()
        .map_err(|_| AccessError::Probe("invalid continuation offset".into()))
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, AccessError> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| AccessError::Serialization(error.to_string()))?;
    Ok(format!("sha256:{}", sha256_bytes(&bytes)))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Minimal native HTTP client for the local Ollama endpoint.  HTTPS is not
/// guessed or silently downgraded; this prototype accepts only plain HTTP and
/// reports a technical provider failure otherwise.
#[derive(Clone, Debug)]
pub struct OllamaEmbeddingProvider {
    pub endpoint: String,
    pub requested_model: String,
    pub dimension: usize,
}

impl Default for OllamaEmbeddingProvider {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:11434".into(),
            requested_model: VECTOR_MODEL.into(),
            dimension: VECTOR_DIMENSION,
        }
    }
}

impl OllamaEmbeddingProvider {
    fn contract(&self) -> VectorProviderContract {
        VectorProviderContract {
            requested_model: self.requested_model.clone(),
            dimension: self.dimension,
            ..VectorProviderContract::default()
        }
    }

    fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, AccessFailure> {
        let body_bytes = body
            .map(|body| {
                serde_json::to_vec(body).map_err(|error| {
                    AccessFailure::new("request_serialization_failed", error.to_string(), false)
                })
            })
            .transpose()?
            .unwrap_or_default();
        let request_bytes = body_bytes.len();
        let target = self.endpoint.strip_prefix("http://").ok_or_else(|| {
            AccessFailure::new(
                "unsupported_endpoint",
                "Ollama prototype requires an http:// endpoint",
                false,
            )
        })?;
        let target = target.trim_end_matches('/');
        let mut address_parts = target.splitn(2, ':');
        let host = address_parts.next().unwrap_or_default();
        let port = address_parts
            .next()
            .and_then(|port| port.parse().ok())
            .unwrap_or(80u16);
        let address = (host, port)
            .to_socket_addrs()
            .map_err(|error| {
                provider_transport_failure(
                    "provider_connect_failed",
                    &error.to_string(),
                    request_bytes,
                )
            })?
            .next()
            .ok_or_else(|| {
                provider_transport_failure(
                    "provider_connect_failed",
                    "endpoint did not resolve",
                    request_bytes,
                )
            })?;
        let mut stream =
            TcpStream::connect_timeout(&address, Duration::from_secs(3)).map_err(|error| {
                provider_transport_failure(
                    "provider_connect_failed",
                    &error.to_string(),
                    request_bytes,
                )
            })?;
        stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
        let request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body_bytes.len()
        );
        stream.write_all(request.as_bytes()).map_err(|error| {
            provider_transport_failure("provider_write_failed", &error.to_string(), request_bytes)
        })?;
        stream.write_all(&body_bytes).map_err(|error| {
            provider_transport_failure("provider_write_failed", &error.to_string(), request_bytes)
        })?;
        let mut response = Vec::new();
        stream.read_to_end(&mut response).map_err(|error| {
            if response.is_empty() {
                provider_transport_failure(
                    "provider_read_failed",
                    &error.to_string(),
                    request_bytes,
                )
            } else {
                provider_response_failure(
                    "provider_read_failed",
                    None,
                    None,
                    &response,
                    &[],
                    request_bytes,
                    &format!("partial response; transport_error={}", error),
                    true,
                )
            }
        })?;
        let Some(header_end) = response.windows(4).position(|window| window == b"\r\n\r\n") else {
            return Err(provider_response_failure(
                "provider_invalid_response",
                None,
                None,
                &response,
                &[],
                request_bytes,
                "response has no HTTP header terminator",
                false,
            ));
        };
        let headers = match std::str::from_utf8(&response[..header_end]) {
            Ok(headers) => headers,
            Err(error) => {
                return Err(provider_response_failure(
                    "provider_invalid_response",
                    None,
                    None,
                    &response,
                    &[],
                    request_bytes,
                    &error.to_string(),
                    false,
                ));
            }
        };
        let status = headers
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse::<u16>().ok());
        let content_type = headers.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-type")
                .then_some(value.trim())
        });
        let wire_body = &response[header_end + 4..];
        let response_body = match decode_response_body(headers, wire_body) {
            Ok(body) => body,
            Err(error) => {
                return Err(provider_response_failure(
                    "provider_invalid_chunked_response",
                    status,
                    content_type,
                    wire_body,
                    &[],
                    request_bytes,
                    &error,
                    false,
                ));
            }
        };
        if !status.is_some_and(|status| (200..300).contains(&status)) {
            return Err(provider_response_failure(
                "provider_http_error",
                status,
                content_type,
                wire_body,
                &response_body,
                request_bytes,
                "HTTP response status was not successful",
                status.is_some_and(|status| status >= 500),
            ));
        }
        let value: Value = serde_json::from_slice(&response_body).map_err(|error| {
            provider_response_failure(
                "provider_invalid_json",
                status,
                content_type,
                wire_body,
                &response_body,
                request_bytes,
                &error.to_string(),
                false,
            )
        })?;
        Ok(value)
    }
}

fn provider_transport_failure(code: &str, error: &str, request_bytes: usize) -> AccessFailure {
    AccessFailure::new(
        code,
        format!(
            "http_status=none; content_type=none; response_wire_bytes=0; response_entity_bytes=0; body_diagnostic=<none>; wire_body_diagnostic=<none>; request_bytes={request_bytes}; transport_error={}",
            safe_diagnostic(error.as_bytes())
        ),
        true,
    )
}

fn provider_response_failure(
    code: &str,
    status: Option<u16>,
    content_type: Option<&str>,
    wire_body: &[u8],
    entity_body: &[u8],
    request_bytes: usize,
    cause: &str,
    retryable: bool,
) -> AccessFailure {
    AccessFailure::new(
        code,
        format!(
            "http_status={}; content_type={}; response_wire_bytes={}; response_entity_bytes={}; body_diagnostic={}; wire_body_diagnostic={}; request_bytes={request_bytes}; failure={}",
            status.map_or_else(|| "none".into(), |status| status.to_string()),
            content_type.map_or_else(|| "none".into(), |value| safe_diagnostic(value.as_bytes())),
            wire_body.len(),
            entity_body.len(),
            safe_diagnostic(entity_body),
            safe_diagnostic(wire_body),
            safe_diagnostic(cause.as_bytes())
        ),
        retryable,
    )
}

fn decode_response_body(headers: &str, wire_body: &[u8]) -> Result<Vec<u8>, String> {
    if transfer_encoding_is_chunked(headers) {
        decode_chunked_body(wire_body)
    } else {
        Ok(wire_body.to_vec())
    }
}

fn transfer_encoding_is_chunked(headers: &str) -> bool {
    headers
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once(':'))
        .any(|(name, value)| {
            name.eq_ignore_ascii_case("transfer-encoding")
                && value
                    .split(',')
                    .any(|encoding| encoding.trim().eq_ignore_ascii_case("chunked"))
        })
}

fn decode_chunked_body(wire_body: &[u8]) -> Result<Vec<u8>, String> {
    let mut cursor = 0usize;
    let mut entity_body = Vec::new();

    loop {
        let size_line_end =
            find_crlf(wire_body, cursor).ok_or_else(|| "truncated chunk-size line".to_string())?;
        let size_line = &wire_body[cursor..size_line_end];
        cursor = size_line_end + 2;
        let size_token = size_line
            .split(|byte| *byte == b';')
            .next()
            .unwrap_or_default();
        let chunk_size = parse_hex_chunk_size(size_token)?;

        if chunk_size == 0 {
            loop {
                let trailer_end = find_crlf(wire_body, cursor)
                    .ok_or_else(|| "truncated chunk trailer section".to_string())?;
                let trailer = &wire_body[cursor..trailer_end];
                cursor = trailer_end + 2;
                if trailer.is_empty() {
                    if cursor != wire_body.len() {
                        return Err("bytes follow the terminating chunk trailers".into());
                    }
                    return Ok(entity_body);
                }
                validate_trailer_field(trailer)?;
            }
        }

        let chunk_end = cursor
            .checked_add(chunk_size)
            .ok_or_else(|| "chunk size overflows response bounds".to_string())?;
        if chunk_end > wire_body.len() {
            return Err("truncated chunk data".into());
        }
        entity_body.extend_from_slice(&wire_body[cursor..chunk_end]);
        cursor = chunk_end;
        if wire_body.get(cursor..cursor + 2) != Some(b"\r\n") {
            return Err("chunk data is not followed by CRLF".into());
        }
        cursor += 2;
    }
}

fn find_crlf(bytes: &[u8], start: usize) -> Option<usize> {
    bytes
        .get(start..)?
        .windows(2)
        .position(|window| window == b"\r\n")
        .map(|offset| start + offset)
}

fn parse_hex_chunk_size(bytes: &[u8]) -> Result<usize, String> {
    if bytes.is_empty() {
        return Err("empty chunk size".into());
    }
    let mut size = 0usize;
    for byte in bytes {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err("malformed hexadecimal chunk size".into()),
        } as usize;
        size = size
            .checked_mul(16)
            .and_then(|size| size.checked_add(digit))
            .ok_or_else(|| "chunk size overflows usize".to_string())?;
    }
    Ok(size)
}

fn validate_trailer_field(trailer: &[u8]) -> Result<(), String> {
    let Some(colon) = trailer.iter().position(|byte| *byte == b':') else {
        return Err("malformed chunk trailer field".into());
    };
    if colon == 0 || !trailer[..colon].iter().copied().all(is_http_token_byte) {
        return Err("malformed chunk trailer field name".into());
    }
    Ok(())
}

fn is_http_token_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'0'..=b'9'
            | b'a'..=b'z'
            | b'A'..=b'Z'
            | b'!'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~'
    )
}

fn safe_diagnostic(bytes: &[u8]) -> String {
    const MAX_DIAGNOSTIC_BYTES: usize = 512;
    let mut diagnostic = String::new();
    for byte in bytes.iter().copied().take(MAX_DIAGNOSTIC_BYTES) {
        match byte {
            b'\r' => diagnostic.push_str("\\r"),
            b'\n' => diagnostic.push_str("\\n"),
            b'\t' => diagnostic.push_str("\\t"),
            0x20..=0x7e => diagnostic.push(byte as char),
            byte => diagnostic.push_str(&format!("\\x{byte:02x}")),
        }
    }
    if bytes.is_empty() {
        diagnostic.push_str("<empty>");
    } else if bytes.len() > MAX_DIAGNOSTIC_BYTES {
        diagnostic.push_str("...(truncated)");
    }
    diagnostic
}

impl EmbeddingProvider for OllamaEmbeddingProvider {
    fn identity(&self) -> Result<VectorProviderIdentity, AccessFailure> {
        let response = self.request("GET", "/api/tags", None)?;
        let model = response
            .get("models")
            .and_then(Value::as_array)
            .and_then(|models| {
                models.iter().find(|model| {
                    model.get("name").and_then(Value::as_str) == Some(self.requested_model.as_str())
                })
            })
            .ok_or_else(|| {
                AccessFailure::new(
                    "model_not_found",
                    format!("Ollama model {} is not installed", self.requested_model),
                    false,
                )
            })?;
        let resolved_model = model
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(&self.requested_model)
            .to_owned();
        let digest = model
            .get("digest")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                AccessFailure::new(
                    "model_digest_unavailable",
                    "Ollama did not report an immutable model digest",
                    false,
                )
            })?
            .to_owned();
        Ok(VectorProviderIdentity {
            contract: self.contract(),
            endpoint: self.endpoint.clone(),
            resolved_model,
            model_digest: digest,
            max_input_chars: None,
        })
    }

    fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, AccessFailure> {
        if inputs.len() != 1 {
            return Err(AccessFailure::new(
                "provider_request_shape",
                format!(
                    "Ollama /api/embed requires exactly one input per request; \
                     received_inputs={}",
                    inputs.len()
                ),
                false,
            ));
        }
        let request_body = serde_json::json!({
            "model": self.requested_model,
            "input": &inputs[0],
            "truncate": false
        });
        let input_count = inputs.len();
        let total_input_bytes = inputs.iter().map(String::len).sum::<usize>();
        let max_input_bytes = inputs.iter().map(String::len).max().unwrap_or(0);
        let response = self
            .request("POST", "/api/embed", Some(&request_body))
            .map_err(|mut failure| {
                if failure.code == "provider_http_error"
                    && failure
                        .message
                        .to_ascii_lowercase()
                        .contains("the input length exceeds the context length")
                {
                    failure.code = "provider_capacity_exceeded".into();
                }
                failure.message = format!(
                    "{}; input_count={input_count}; total_input_bytes={total_input_bytes}; max_input_bytes={max_input_bytes}",
                    failure.message
                );
                failure
            })?;
        let embeddings = response
            .get("embeddings")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                AccessFailure::new(
                    "provider_response_shape",
                    "Ollama response did not contain embeddings",
                    false,
                )
            })?;
        if embeddings.len() != 1 {
            return Err(AccessFailure::new(
                "provider_response_shape",
                format!(
                    "Ollama /api/embed returned {} embeddings for one input",
                    embeddings.len()
                ),
                false,
            ));
        }
        embeddings
            .iter()
            .map(|embedding| {
                serde_json::from_value(embedding.clone()).map_err(|error| {
                    AccessFailure::new("provider_response_shape", error.to_string(), false)
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod transport_tests {
    use super::{EmbeddingProvider, OllamaEmbeddingProvider, VECTOR_MODEL, decode_response_body};
    use serde_json::Value;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    const CHUNKED_HEADERS: &str =
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Type: application/json\r\n";

    #[test]
    fn decodes_valid_single_chunk_json_and_detects_headers_case_insensitively() {
        let wire = b"7\r\n{\"a\":1}\r\n0\r\n\r\n";
        assert_eq!(
            decode_response_body("HTTP/1.1 200 OK\r\ntRaNsFeR-EnCoDiNg: ChUnKeD\r\n", wire)
                .unwrap(),
            br#"{"a":1}"#
        );
    }

    #[test]
    fn decodes_multi_chunk_json_split_inside_json_token() {
        let wire = b"3\r\n{\"a\r\n4\r\n\":1}\r\n0\r\n\r\n";
        assert_eq!(
            decode_response_body(CHUNKED_HEADERS, wire).unwrap(),
            br#"{"a":1}"#
        );
    }

    #[test]
    fn accepts_uppercase_and_lowercase_hexadecimal_chunk_sizes() {
        let uppercase = b"A\r\n0123456789\r\n0\r\n\r\n";
        let lowercase = b"a\r\n0123456789\r\n0\r\n\r\n";
        assert_eq!(
            decode_response_body(CHUNKED_HEADERS, uppercase).unwrap(),
            b"0123456789"
        );
        assert_eq!(
            decode_response_body(CHUNKED_HEADERS, lowercase).unwrap(),
            b"0123456789"
        );
    }

    #[test]
    fn ignores_chunk_extensions() {
        let wire = b"7;foo=bar\r\n{\"a\":1}\r\n0;done=yes\r\n\r\n";
        assert_eq!(
            decode_response_body(CHUNKED_HEADERS, wire).unwrap(),
            br#"{"a":1}"#
        );
    }

    #[test]
    fn consumes_legal_trailers_after_terminal_chunk() {
        let wire = b"3\r\nabc\r\n0\r\nX-Trace: yes\r\nAnother: value\r\n\r\n";
        assert_eq!(decode_response_body(CHUNKED_HEADERS, wire).unwrap(), b"abc");
    }

    #[test]
    fn rejects_malformed_chunk_size() {
        let wire = b"g\r\nabc\r\n0\r\n\r\n";
        let error = decode_response_body(CHUNKED_HEADERS, wire).unwrap_err();
        assert!(error.contains("hexadecimal chunk size"));
    }

    #[test]
    fn rejects_truncated_chunk_data() {
        let wire = b"5\r\nabc";
        let error = decode_response_body(CHUNKED_HEADERS, wire).unwrap_err();
        assert!(error.contains("truncated chunk data"));
    }

    #[test]
    fn rejects_missing_terminal_zero_chunk() {
        let wire = b"3\r\nabc\r\n";
        let error = decode_response_body(CHUNKED_HEADERS, wire).unwrap_err();
        assert!(error.contains("chunk-size line"));
    }

    #[test]
    fn preserves_non_chunked_content_length_body() {
        let wire = br#"{"a":1}"#;
        assert_eq!(
            decode_response_body(
                "HTTP/1.1 200 OK\r\nContent-Length: 7\r\nContent-Type: application/json\r\n",
                wire
            )
            .unwrap(),
            wire
        );
    }

    #[test]
    fn ollama_embed_uses_one_scalar_input_and_keeps_truncation_disabled() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("test listener binds");
        let address = listener.local_addr().expect("test listener has address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider request arrives");
            let mut request = Vec::new();
            let header_end = loop {
                let mut buffer = [0u8; 4096];
                let count = stream.read(&mut buffer).expect("request reads");
                assert!(count > 0, "request ended before headers");
                request.extend_from_slice(&buffer[..count]);
                if let Some(end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
                    break end;
                }
            };
            let headers = std::str::from_utf8(&request[..header_end]).expect("headers are UTF-8");
            let content_length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length").then(|| {
                        value
                            .trim()
                            .parse::<usize>()
                            .expect("content length parses")
                    })
                })
                .expect("content length is present");
            let body_start = header_end + 4;
            while request.len() < body_start + content_length {
                let mut buffer = [0u8; 4096];
                let count = stream.read(&mut buffer).expect("request body reads");
                assert!(count > 0, "request ended before body");
                request.extend_from_slice(&buffer[..count]);
            }
            let body: Value =
                serde_json::from_slice(&request[body_start..body_start + content_length])
                    .expect("request body is JSON");
            assert_eq!(body["model"], VECTOR_MODEL);
            assert_eq!(body["input"], "transport test input");
            assert_eq!(body["truncate"], false);
            let response = br#"{"embeddings":[[1.0,0.0]]}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                response.len()
            )
            .expect("response headers write");
            stream.write_all(response).expect("response body write");
        });

        let provider = OllamaEmbeddingProvider {
            endpoint: format!("http://{}", address),
            requested_model: VECTOR_MODEL.into(),
            dimension: 2,
        };
        let embeddings = provider
            .embed(&["transport test input".into()])
            .expect("one-input embedding succeeds");
        assert_eq!(embeddings, vec![vec![1.0, 0.0]]);
        server.join().expect("test provider exits");
    }
}

#[cfg(test)]
mod vector_segmentation_tests {
    use super::{
        AccessFailure, EmbeddingProvider, HydratedUnitText, ProviderVectorSegment,
        VectorProviderContract, VectorProviderIdentity, build_vector_index, segment_and_embed,
    };
    use std::sync::{Arc, Mutex};

    struct CapacityProvider {
        max_bytes: usize,
        non_capacity_failure: bool,
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl CapacityProvider {
        fn new(max_bytes: usize) -> Self {
            Self {
                max_bytes,
                non_capacity_failure: false,
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn with_non_capacity_failure() -> Self {
            Self {
                max_bytes: usize::MAX,
                non_capacity_failure: true,
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn identity_value(&self) -> VectorProviderIdentity {
            VectorProviderIdentity {
                contract: VectorProviderContract {
                    provider: "test".into(),
                    requested_model: "test-model".into(),
                    dimension: 2,
                    dtype: "float32".into(),
                    normalization: "L2".into(),
                    similarity: "cosine".into(),
                    truncation: "disabled".into(),
                },
                endpoint: "test://capacity".into(),
                resolved_model: "test-model@fixed".into(),
                model_digest: "sha256:capacity".into(),
                max_input_chars: None,
            }
        }
    }

    impl EmbeddingProvider for CapacityProvider {
        fn identity(&self) -> Result<VectorProviderIdentity, AccessFailure> {
            Ok(self.identity_value())
        }

        fn embed(&self, inputs: &[String]) -> Result<Vec<Vec<f32>>, AccessFailure> {
            assert_eq!(inputs.len(), 1);
            let input = inputs[0].clone();
            self.calls
                .lock()
                .expect("capacity calls are not poisoned")
                .push(input.clone());
            if self.non_capacity_failure {
                return Err(AccessFailure {
                    code: "provider_http_error".into(),
                    message: "HTTP 500 provider failure".into(),
                    retryable: true,
                });
            }
            if input.len() > self.max_bytes {
                return Err(AccessFailure {
                    code: "provider_capacity_exceeded".into(),
                    message: "the input length exceeds the context length".into(),
                    retryable: false,
                });
            }
            Ok(vec![vec![input.len() as f32, 1.0]])
        }
    }

    fn texts(segments: &[ProviderVectorSegment]) -> Vec<&str> {
        segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect()
    }

    #[test]
    fn whole_input_acceptance_uses_one_provider_request() {
        let provider = CapacityProvider::new(100);
        let segments = segment_and_embed(&provider, "whole unit", 2).expect("whole input accepts");
        assert_eq!(texts(&segments), vec!["whole unit"]);
        assert_eq!(
            provider
                .calls
                .lock()
                .expect("calls are not poisoned")
                .as_slice(),
            ["whole unit"]
        );
    }

    #[test]
    fn capacity_rejection_activates_newline_preference() {
        let provider = CapacityProvider::new(8);
        let segments = segment_and_embed(&provider, "aa\nbbbb cc", 2).expect("splits");
        assert_eq!(texts(&segments), vec!["aa\n", "bbbb cc"]);
    }

    #[test]
    fn whitespace_is_used_when_no_newline_boundary_exists() {
        let provider = CapacityProvider::new(5);
        let segments = segment_and_embed(&provider, "aaaa bbbb", 2).expect("splits");
        assert_eq!(texts(&segments), vec!["aaaa ", "bbbb"]);
    }

    #[test]
    fn unicode_code_point_fallback_is_used_without_whitespace() {
        let provider = CapacityProvider::new(3);
        let segments = segment_and_embed(&provider, "abcdef", 2).expect("splits");
        assert_eq!(texts(&segments), vec!["abc", "def"]);
    }

    #[test]
    fn segmentation_reconstructs_exact_input_without_overlap() {
        let provider = CapacityProvider::new(5);
        let input = "one two three";
        let segments = segment_and_embed(&provider, input, 2).expect("splits");
        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<String>(),
            input
        );
    }

    #[test]
    fn non_capacity_provider_failure_does_not_activate_segmentation() {
        let provider = CapacityProvider::with_non_capacity_failure();
        let error = segment_and_embed(&provider, "provider failure", 2).unwrap_err();
        assert_eq!(error.code, "provider_http_error");
        assert_eq!(
            provider.calls.lock().expect("calls are not poisoned").len(),
            1
        );
    }

    #[test]
    fn oversized_real_shaped_unit_splits_at_provider_capacity() {
        let provider = CapacityProvider::new(24_000);
        let input = format!("{}\n{}", "a".repeat(23_999), "b".repeat(767));
        assert_eq!(input.len(), 24_767);
        let segments = segment_and_embed(&provider, &input, 2).expect("oversized input splits");
        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.text.len())
                .collect::<Vec<_>>(),
            vec![24_000, 767]
        );
        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<String>(),
            input
        );
    }

    #[test]
    fn segment_ordinals_are_dense_and_parented_to_one_unit() {
        let provider = CapacityProvider::new(5);
        let unit_id =
            crate::model::SemanticUnitId::parse("unit:test:oversized").expect("unit identity");
        let vector = build_vector_index(
            &[HydratedUnitText {
                unit_id: unit_id.clone(),
                raw: "aaaa bbbb".into(),
                lexical: "aaaa bbbb".into(),
            }],
            Some(&provider),
        );
        assert_eq!(
            vector
                .segments
                .iter()
                .map(|segment| segment.segment_ordinal)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
        assert!(
            vector.segments.iter().all(|segment| {
                segment.parent_unit_id == unit_id && segment.total_segments == 2
            })
        );
    }
}
