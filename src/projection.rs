//! Frozen semantic-space projection contracts.
//!
//! These records represent corpus-derived schema and canonical instances for
//! one immutable snapshot. They do not ingest a corpus, build or validate a
//! projection, activate a working view, choose semantic access, or execute a
//! retrieval surface.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::model::{
    AddressKind, Direction, OccurrenceId, RecordProvenance, RetrievalSurfaceKind, SemanticAddress,
    SemanticObjectId, SemanticRegionAddress, SemanticUnitId, SourceSpan, TemporalAnchorId,
    TransportSegmentId,
};

/// Frozen projection snapshot of the admitted semantic substrate.
///
/// It contains schema-level possibility and instance-level actuality for
/// canonical objects, regions, units, identifiers, occurrences, anchors,
/// surfaces, and transitions. It is structural authority for a snapshot only;
/// it may not infer, activate, query, conform, retrieve, or synthesize.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticSpaceProjection {
    /// Immutable snapshot identity used throughout one turn.
    pub projection_snapshot_id: String,
    /// Identity of the ingest state from which this projection was materialized.
    pub ingest_identity: String,
    /// Version of the projected schema.
    pub schema_version: String,
    /// Stable logical hash for equivalent projected input state.
    pub logical_hash: String,
    /// Identity of the admitted corpus snapshot.
    pub corpus_snapshot_identity: String,
    /// Configuration snapshot governing represented bounds and capabilities.
    pub configuration_snapshot_id: String,
    /// Declared validation state of the frozen representation.
    pub validation_status: ProjectionValidationStatus,
    /// Schema-level classes of semantic object represented in the snapshot.
    pub object_classes: Vec<SemanticObjectClassDescriptor>,
    /// Canonical semantic-object instances.
    pub objects: Vec<SemanticObjectRecord>,
    /// Canonical authored structural regions.
    pub regions: Vec<SemanticRegionRecord>,
    /// Canonical authored semantic units.
    pub units: Vec<SemanticUnitRecord>,
    /// Descriptors for admitted identifiers.
    pub identifier_descriptors: Vec<IdentifierDescriptor>,
    /// Actual identifier assignments and provenance.
    pub identifier_assignments: Vec<IdentifierAssignment>,
    /// Actual authored occurrences and resolved canonical targets.
    pub occurrences: Vec<OccurrenceRecord>,
    /// Materially sourced temporal anchors.
    pub temporal_anchors: Vec<TemporalAnchorRecord>,
    /// Retrieval-surface capabilities and bounds.
    pub retrieval_surfaces: Vec<RetrievalSurfaceDescriptor>,
    /// Valid structural transitions represented by the snapshot.
    pub valid_transitions: Vec<StructuralTransition>,
}

/// Validation state attached to a frozen projection representation.
///
/// This is recorded projection metadata. The enum does not validate anything
/// and does not convert an invalid snapshot into a usable one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionValidationStatus {
    /// Snapshot has passed the accepted structural validation process.
    Validated,
    /// Snapshot exists but has not yet been structurally validated.
    Unvalidated,
    /// Snapshot is retained for diagnostics but is structurally invalid.
    Invalid,
}

/// Schema-level descriptor of a represented semantic-object class.
///
/// It declares possible identifiers and source kinds for conformance. It does
/// not instantiate objects or hard-code corpus-specific semantic content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticObjectClassDescriptor {
    /// Snapshot-local class name.
    pub class_name: String,
    /// Identifier names structurally applicable to this class.
    pub applicable_identifier_names: Vec<String>,
    /// Source kinds from which this class may be materialized.
    pub permitted_source_kinds: Vec<SourceKind>,
}

/// Canonical semantic-object instance in one projection snapshot.
///
/// It preserves identity, source topology, contained addresses, occurrence
/// incidence, anchors, and surface affordances. It does not select access or
/// collapse discovery surfaces into canonical identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticObjectRecord {
    /// Canonical object UUID.
    pub object_id: SemanticObjectId,
    /// Stable source identity from which the object was materialized.
    pub source_identity: String,
    /// Kind of admitted authored source.
    pub source_kind: SourceKind,
    /// Canonical source path or topology address.
    pub canonical_path: String,
    /// Authored filename or source display surface.
    pub filename: String,
    /// Canonical or authored title surface.
    pub title: String,
    /// Non-canonical aliases retained for discovery.
    pub aliases: Vec<String>,
    /// Snapshot-level semantic-object class.
    pub object_class: String,
    /// Contained authored semantic-region addresses.
    pub region_addresses: Vec<SemanticRegionAddress>,
    /// Contained canonical semantic-unit identities.
    pub unit_ids: Vec<SemanticUnitId>,
    /// Identifier assignments carried by the object.
    pub identifier_assignment_ids: Vec<String>,
    /// Occurrences authored in object fields.
    pub object_field_occurrence_ids: Vec<OccurrenceId>,
    /// Occurrences authored in body units.
    pub body_occurrence_ids: Vec<OccurrenceId>,
    /// Incoming occurrences targeting this object.
    pub incoming_occurrence_ids: Vec<OccurrenceId>,
    /// Temporal anchors materially attached to the object.
    pub temporal_anchor_ids: Vec<TemporalAnchorId>,
    /// Retrieval surfaces structurally capable of inspecting this object.
    pub retrieval_surface_ids: Vec<String>,
}

/// Canonical addressable authored region within one semantic object.
///
/// It preserves heading topology, contained regions and units, target mappings,
/// inherited identifiers, incidence, and affordances. It cannot arbitrarily
/// collapse a multi-unit heading to one unit or become a separate ontology.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticRegionRecord {
    /// Canonical parent object plus authored structural address.
    pub address: SemanticRegionAddress,
    /// Authored hierarchy from the object root to this region.
    pub heading_path: Vec<String>,
    /// Stable heading or region identity within the projection snapshot.
    pub heading_identity: String,
    /// Source span of the authored region marker when known.
    pub source_span: Option<SourceSpan>,
    /// Directly contained child-region addresses.
    pub child_region_addresses: Vec<SemanticRegionAddress>,
    /// Canonical semantic units contained by the region.
    pub contained_unit_ids: Vec<SemanticUnitId>,
    /// Deterministic block-target mappings declared inside the region.
    pub block_target_mappings: Vec<BlockTargetMapping>,
    /// Incoming occurrences targeting this region.
    pub incoming_occurrence_ids: Vec<OccurrenceId>,
    /// Inherited identifier assignments visible at the region.
    pub inherited_identifier_assignment_ids: Vec<String>,
    /// Retrieval surfaces capable of inspecting the region.
    pub retrieval_surface_ids: Vec<String>,
}

/// Canonical independently addressable authored semantic unit.
///
/// It preserves object and region belonging, authored block structure, content
/// or hydration address, inherited and local identifiers, occurrences, anchors,
/// source provenance, and subordinate transport segments. It cannot be
/// manufactured from a provider token limit or semantically filtered here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SemanticUnitRecord {
    /// Canonical semantic-unit identity.
    pub unit_id: SemanticUnitId,
    /// Canonical parent semantic object.
    pub parent_object_id: SemanticObjectId,
    /// Canonical authored region containing the unit.
    pub parent_region_address: SemanticRegionAddress,
    /// Authored Markdown or source block category.
    pub authored_block_type: AuthoredBlockType,
    /// Complete authored heading path.
    pub heading_path: Vec<String>,
    /// One-based ordinal within the authored region.
    pub block_ordinal: u32,
    /// Explicit authored block identifier when present.
    pub explicit_block_id: Option<String>,
    /// Authored content or deterministic hydration address.
    pub content: SemanticUnitContent,
    /// Identifier assignments inherited from the parent object.
    pub inherited_identifier_assignment_ids: Vec<String>,
    /// Identifier assignments authored locally on the unit.
    pub unit_local_identifier_assignment_ids: Vec<String>,
    /// Authored outgoing occurrences from the unit.
    pub outgoing_occurrence_ids: Vec<OccurrenceId>,
    /// Incoming occurrences targeting the unit.
    pub incoming_occurrence_ids: Vec<OccurrenceId>,
    /// Temporal anchors materially attached to the unit.
    pub temporal_anchor_ids: Vec<TemporalAnchorId>,
    /// Retrieval surfaces capable of inspecting or hydrating the unit.
    pub retrieval_surface_ids: Vec<String>,
    /// Materialization provenance for the canonical unit.
    pub source_provenance: RecordProvenance,
    /// Technical segments subordinate to this one canonical unit.
    pub transport_segments: Vec<TransportSegmentRecord>,
}

/// Authored source kinds admitted by the projection contract.
///
/// These values describe source form only. They do not determine semantic
/// relevance, register, or corpus admission by themselves.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceKind {
    /// Markdown note or document.
    Markdown,
    /// Another explicitly admitted source kind named by configuration.
    Admitted {
        /// Stable source-kind name.
        name: String,
    },
}

/// Authored block categories that may materialize one semantic unit.
///
/// The variants preserve source form and cannot split a canonical unit based on
/// provider limits.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthoredBlockType {
    /// Authored paragraph.
    Paragraph,
    /// Authored list block.
    List,
    /// Authored block quotation.
    BlockQuote,
    /// Authored table.
    Table,
    /// Authored code fence.
    CodeBlock,
    /// Authored display equation.
    Equation,
    /// Authored callout.
    Callout,
    /// Authored embedded media reference.
    EmbeddedMedia,
}

/// Canonical unit content representation.
///
/// It either carries authored material directly or a deterministic hydration
/// address. It does not carry normalized text as a replacement authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum SemanticUnitContent {
    /// Authored Markdown or source block is present in the projection.
    Inline {
        /// Raw authored block.
        authored_markdown: String,
        /// Deterministic normalized representation used by capable surfaces.
        normalized_text: String,
    },
    /// Full content is available through a deterministic read-only address.
    HydrationAddress {
        /// Stable address understood by the substrate adapter.
        address: String,
        /// Integrity hash of the hydrated authored content.
        content_hash: String,
    },
}

/// Mapping from an authored block target to one canonical semantic unit.
///
/// It preserves target resolution only and cannot invent a block target from a
/// heuristic string match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlockTargetMapping {
    /// Authored block target text.
    pub authored_block_id: String,
    /// Canonical semantic unit resolved at ingest or projection time.
    pub target_unit_id: SemanticUnitId,
}

/// Technical transport segment subordinate to one canonical semantic unit.
///
/// It preserves parent identity, order, source span, and reconstruction data.
/// It may serve provider transport but can never become independent evidence or
/// a canonical semantic unit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TransportSegmentRecord {
    /// Opaque technical segment identity.
    pub segment_id: TransportSegmentId,
    /// Canonical semantic unit that owns the segment.
    pub parent_unit_id: SemanticUnitId,
    /// Zero-based deterministic segment ordinal.
    pub segment_ordinal: u32,
    /// Source span covered by the segment.
    pub source_span: SourceSpan,
    /// Total segment count needed for complete reconstruction.
    pub total_segments: u32,
    /// Stable reconstruction group identity.
    pub reconstruction_group: String,
}

/// Descriptor of one admitted identifier.
///
/// It states role, shape, applicability, provenance mode, temporal and link
/// affordances, capable surfaces, and enabled transitions. It does not assign a
/// value or create a relation by itself.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IdentifierDescriptor {
    /// Stable identifier name.
    pub identifier_name: String,
    /// Semantic role in the authored substrate.
    pub semantic_role: IdentifierRole,
    /// Represented value shape.
    pub value_shape: IdentifierValueShape,
    /// Scalar or collection cardinality.
    pub cardinality: IdentifierCardinality,
    /// Address kinds to which this identifier may structurally apply.
    pub applicable_address_kinds: Vec<AddressKind>,
    /// How assignment provenance participates in inheritance and relation.
    pub assignment_mode: IdentifierAssignmentMode,
    /// Authored or materialized source surface.
    pub source_surface: String,
    /// Whether values may contain canonical authored links.
    pub may_contain_canonical_links: bool,
    /// Temporal affordance supplied by this identifier.
    pub temporal_affordance: TemporalAffordance,
    /// Surfaces structurally capable of inspecting the representation.
    pub retrieval_surface_ids: Vec<String>,
    /// Transition identities enabled by this descriptor.
    pub enabled_transition_ids: Vec<String>,
}

/// Semantic role of an admitted identifier.
///
/// Roles preserve typing discipline. They do not determine truth, relevance,
/// or whether an assignment answers a current problem.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum IdentifierRole {
    /// Canonical individuation anchor.
    Individuation,
    /// Object or content class.
    ObjectClass,
    /// Organon or authored topological position.
    FrameworkPosition,
    /// Public or indexical register typing.
    RegisterTyping,
    /// Canonical naming, title, alias, or attribution.
    CanonicalNaming,
    /// Material temporal anchoring.
    TemporalAnchoring,
    /// Contextual relation to another canonical address.
    ContextualRelation,
    /// Non-individuating grouping.
    Grouping,
    /// Indexical telemetry carried by authored source material.
    IndexicalTelemetry,
    /// Named admitted role not hard-coded as corpus content.
    Declared {
        /// Stable role name.
        name: String,
    },
}

/// JSON-compatible value shapes admitted for identifier assignments.
///
/// The shape constrains serialization and conformance only. It does not assign
/// semantic meaning beyond the descriptor's declared role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IdentifierValueShape {
    /// UTF-8 string.
    String,
    /// Signed integer.
    Integer,
    /// Boolean.
    Boolean,
    /// Canonical projected address.
    SemanticAddress,
}

/// Cardinality of an identifier assignment.
///
/// It is a structural schema constraint only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IdentifierCardinality {
    /// One represented value.
    Scalar,
    /// Zero or more represented values.
    Collection,
}

/// Provenance and inheritance mode of an identifier assignment.
///
/// This distinction prevents contextual participation from being flattened into
/// intrinsic object typing. It does not itself propagate assignments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IdentifierAssignmentMode {
    /// Intrinsic to the addressed record.
    Intrinsic,
    /// Inherited from a canonical parent object with provenance retained.
    Inherited,
    /// Authored locally on a region or unit.
    Local,
    /// Contextual relation whose source and target remain distinct.
    Relational,
}

/// Temporal affordance of one identifier descriptor.
///
/// It declares structural time-bearing capacity only and does not evaluate or
/// compare temporal values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemporalAffordance {
    /// No temporal anchor is created or referenced.
    None,
    /// Assignment may materialize a temporal anchor.
    CreatesAnchor,
    /// Assignment may point to an existing anchor.
    ReferencesAnchor,
}

/// Actual assignment of one admitted identifier to one projected address.
///
/// It preserves descriptor name, subject, represented value, and provenance.
/// It cannot change the subject's canonical identity or upgrade contextual
/// participation into intrinsic typing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IdentifierAssignment {
    /// Snapshot-local assignment identity.
    pub assignment_id: String,
    /// Identifier descriptor name.
    pub identifier_name: String,
    /// Canonical subject that carries or participates in the assignment.
    pub subject: SemanticAddress,
    /// Scalar or collection value matching the descriptor's declared shape.
    pub value: IdentifierValue,
    /// Exact assignment provenance.
    pub provenance: RecordProvenance,
}

/// Value carried by one identifier assignment.
///
/// This is a closed structural union for exchange. It cannot be interpreted as
/// evidence or a relation without its descriptor and provenance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum IdentifierValue {
    /// One string value.
    String(String),
    /// One signed integer value.
    Integer(i64),
    /// One boolean value.
    Boolean(bool),
    /// One canonical address value.
    SemanticAddress(SemanticAddress),
    /// Collection of strings.
    Strings(Vec<String>),
    /// Collection of canonical addresses.
    SemanticAddresses(Vec<SemanticAddress>),
}

/// One authored occurrence with its resolved canonical target and provenance.
///
/// It is an addressable record rather than an untyped edge. It preserves
/// authored text, source surface, direction, and target identity but cannot
/// invent or semantically label a relation after retrieval.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OccurrenceRecord {
    /// Canonical occurrence identity.
    pub occurrence_id: OccurrenceId,
    /// Authored source of the occurrence.
    pub source: OccurrenceSource,
    /// Authored target text before canonical resolution.
    pub authored_target_text: String,
    /// Optional authored display alias.
    pub display_alias: Option<String>,
    /// Canonical object, region, or unit target resolved by projection.
    pub resolved_target: SemanticAddress,
    /// Link or embed presentation mode.
    pub presentation_mode: OccurrencePresentation,
    /// Authored incidence direction.
    pub direction: Direction,
    /// Exact source span when represented.
    pub source_span: Option<SourceSpan>,
}

/// Authored source surface of one occurrence.
///
/// It distinguishes object-field context from semantic-unit body context and
/// cannot transfer identifiers between source and target.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum OccurrenceSource {
    /// Occurrence authored in an object field.
    ObjectField {
        /// Source canonical object.
        object_id: SemanticObjectId,
        /// Authored field path.
        field_path: String,
    },
    /// Occurrence authored in a semantic unit body.
    SemanticUnit {
        /// Source canonical semantic unit.
        unit_id: SemanticUnitId,
    },
}

/// Presentation form of an authored occurrence.
///
/// Presentation does not create a second target object or change canonical
/// identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OccurrencePresentation {
    /// Authored link occurrence.
    Link,
    /// Authored embed occurrence.
    Embed,
}

/// Materially sourced temporal anchor attached to a canonical address.
///
/// It preserves temporal value and provenance. It cannot compare dates,
/// generate chronology, or make a contextual date intrinsic to a target object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TemporalAnchorRecord {
    /// Canonical temporal-anchor identity.
    pub anchor_id: TemporalAnchorId,
    /// Canonical object or unit to which the anchor is materially attached.
    pub subject: SemanticAddress,
    /// Structured temporal value.
    pub value: TemporalValue,
    /// Exact source provenance of the anchor.
    pub provenance: RecordProvenance,
}

/// Structured temporal value represented by a temporal anchor.
///
/// It preserves authored precision and does not perform ordering or inference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "precision",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TemporalValue {
    /// Calendar date in ISO-like authored form.
    Date(String),
    /// Date and time with an explicit offset or zone in authored form.
    DateTime(String),
    /// Calendar year.
    Year(i32),
    /// Relative or source-defined ordinal.
    Ordinal(i64),
    /// Named temporal label when no stronger syntax is admitted.
    Label(String),
}

/// Descriptor of one retrieval-surface capability in a frozen projection.
///
/// It declares availability, visible address kinds, modes, bounds, returned
/// identity, hydration, coverage semantics, continuation, and limitations. It
/// does not execute a query or judge semantic adequacy.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RetrievalSurfaceDescriptor {
    /// Snapshot-local surface identity.
    pub surface_id: String,
    /// Stable surface family.
    pub kind: RetrievalSurfaceKind,
    /// Whether the concrete surface is available in this configuration.
    pub available: bool,
    /// Address kinds the surface may structurally inspect.
    pub visible_address_kinds: Vec<AddressKind>,
    /// Query or match modes accepted by the surface.
    pub match_modes: Vec<SurfaceMatchMode>,
    /// Configured default candidate bound.
    pub default_candidate_limit: u32,
    /// Configured hard candidate bound.
    pub hard_candidate_limit: u32,
    /// Canonical or projected identity returned directly by the surface.
    pub returned_identity: AddressKind,
    /// Whether results deterministically hydrate to canonical semantic units.
    pub hydrates_to_semantic_units: bool,
    /// Measurable coverage behavior of the surface.
    pub coverage_semantics: CoverageSemantics,
    /// Whether a complete eligible-scope total count is supported.
    pub exhaustive_total_count_supported: bool,
    /// Whether bounded continuation is structurally available.
    pub continuation_supported: bool,
    /// Explicit technical limitations.
    pub technical_limitations: Vec<String>,
}

/// Query or match modes exposed by a retrieval surface.
///
/// Modes constrain execution shape only and do not imply semantic relevance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum SurfaceMatchMode {
    /// Exact literal match.
    Literal,
    /// Token or term match.
    Terms,
    /// Nearest-neighbour candidate lookup.
    NearestNeighbours,
    /// Typed edge or occurrence navigation.
    Incidence,
    /// Temporal range or ordering access.
    Temporal,
    /// Named configured mode not hard-coded as corpus meaning.
    Declared {
        /// Stable configured mode name.
        name: String,
    },
}

/// Measurable coverage behavior declared for a surface.
///
/// Coverage constrains later claim scope; it does not reinterpret returned
/// units or score problem-space coherence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CoverageSemantics {
    /// Eligible scope can be exhaustively enumerated and counted.
    Exhaustive,
    /// Results are deterministically bounded and may be incomplete.
    Bounded,
    /// Surface reports availability but no stronger coverage guarantee.
    AvailabilityOnly,
}

/// Valid structural transition represented by a projection snapshot.
///
/// It states a possible typed move between address kinds and optional surface.
/// It does not perform the move or determine whether a concrete instance path
/// answers the current problem.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StructuralTransition {
    /// Snapshot-local transition identity.
    pub transition_id: String,
    /// Required input address kind.
    pub from: AddressKind,
    /// Structural operation represented by the transition.
    pub operation: StructuralTransitionOperation,
    /// Explicit incidence direction.
    pub direction: Direction,
    /// Emitted address kind.
    pub to: AddressKind,
    /// Surface required by the transition when applicable.
    pub retrieval_surface_id: Option<String>,
}

/// Structural operation families represented by projection transitions.
///
/// Variants are possibility grammar, not execution methods or semantic judges.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StructuralTransitionOperation {
    /// Object-to-region or object-to-unit containment.
    Containment,
    /// Unit or region to canonical parent.
    Parent,
    /// Incoming or outgoing authored occurrence incidence.
    Occurrence,
    /// Identifier inheritance or assignment access.
    Identifier,
    /// Temporal-anchor incidence.
    TemporalAnchor,
    /// Deterministic hydration to canonical semantic units.
    Hydration,
    /// Retrieval-surface invocation.
    RetrievalSurface,
}
