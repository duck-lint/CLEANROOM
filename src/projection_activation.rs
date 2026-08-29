//! Deterministic initial projection activation over current access artifacts.
//!
//! This runtime consumes the Phase 7 projection-access artifacts directly. It
//! does not construct a parallel retrieval architecture, hydrate prose, bind
//! problem-space referents to canonical addresses, or make negative claims from
//! omitted records.

use std::{collections::HashSet, fmt};

use crate::{
    access::{
        AccessOperand, EmbeddingProvider, ProjectionAccessArtifacts, ProjectionAccessProbe,
        ProjectionAccessProbeResult, TemporalPrecision, TemporalQuery, VectorProviderState,
    },
    activation::{
        ActivatedEdge, ActivatedIdentifierAssignmentRecord, ActivatedObjectRecord,
        ActivatedOccurrenceRecord, ActivatedProjection, ActivatedRecordKind, ActivatedRegionRecord,
        ActivatedTemporalAnchorRecord, ActivatedTextPreview, ActivatedUnitRecord,
        ActivationProvenance, ActivationUtterance, CandidateCount, ContinuationAccess,
        ContinuationHandle, ContinuationOrdering, ContinuationOrigin, CountByLabel,
        ProjectionActivationConfig, ProjectionActivationViolation, ProjectionTelemetry,
        TruncationState,
    },
    model::{AddressKind, Direction, RetrievalSurfaceKind, SemanticAddress},
    problem_space::{
        ActivationBand, OpenTension, ProblemConstraintApplicability, ProblemSpaceState,
        RecordLifecycle, RegionPersistenceState, TensionLifecycle,
    },
    projection::{
        IdentifierValue, OccurrenceSource, ProjectionValidationStatus, RetrievalSurfaceDescriptor,
        SemanticSpaceProjection, SemanticUnitContent, SurfaceMatchMode, TemporalValue,
    },
};

/// Current activation-access boundary over Phase 7 access artifacts.
///
/// The optional query-embedding provider is used only for vector probes and
/// must resolve to the same provider identity recorded in the access artifact.
pub struct ProjectionActivationAccess<'a> {
    artifacts: &'a ProjectionAccessArtifacts,
    query_embedding_provider: Option<&'a dyn EmbeddingProvider>,
}

impl<'a> ProjectionActivationAccess<'a> {
    pub fn new(artifacts: &'a ProjectionAccessArtifacts) -> Self {
        Self {
            artifacts,
            query_embedding_provider: None,
        }
    }

    pub fn with_query_embedding_provider(
        artifacts: &'a ProjectionAccessArtifacts,
        query_embedding_provider: &'a dyn EmbeddingProvider,
    ) -> Self {
        Self {
            artifacts,
            query_embedding_provider: Some(query_embedding_provider),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ActivationProbeBand {
    Unbanded,
    Attention(ActivationBand),
}

#[derive(Clone, Debug)]
struct Seed {
    text: String,
    band: ActivationProbeBand,
    provenance: Vec<ActivationProvenance>,
    region_id: Option<String>,
}

#[derive(Clone, Debug)]
struct QueuedAddress {
    address: SemanticAddress,
    band: ActivationProbeBand,
    provenance: Vec<ActivationProvenance>,
    region_id: Option<String>,
    depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EdgeTuple {
    source: SemanticAddress,
    transition_id: String,
    direction: Direction,
    target: SemanticAddress,
}

#[derive(Clone, Debug)]
struct Exposure {
    provenance: Vec<ActivationProvenance>,
}

#[derive(Default)]
struct Work {
    objects: Vec<ActivatedObjectRecord>,
    regions: Vec<ActivatedRegionRecord>,
    units: Vec<ActivatedUnitRecord>,
    assignments: Vec<ActivatedIdentifierAssignmentRecord>,
    occurrences: Vec<ActivatedOccurrenceRecord>,
    anchors: Vec<ActivatedTemporalAnchorRecord>,
    edges: Vec<ActivatedEdge>,
    telemetry: Vec<ProjectionTelemetry>,
    handles: Vec<ContinuationHandle>,
    queued_addresses: Vec<QueuedAddress>,
    bounded_probe_ids: HashSet<String>,
    probe_counter: u64,
    telemetry_counter: u64,
    edge_counter: u64,
    handle_counter: u64,
}

/// Build the positive initial activated projection for one turn.
pub fn activate_projection(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
) -> Result<ActivatedProjection, ProjectionActivationViolation> {
    preflight(projection, problem_space, utterance, config, access)?;

    let mut work = Work::default();
    for seed in build_seeds(problem_space, utterance, config)? {
        dispatch_text_seed(
            projection,
            problem_space,
            utterance,
            config,
            access,
            &mut work,
            seed,
        )?;
        drain_queued_addresses(
            projection,
            problem_space,
            utterance,
            config,
            access,
            &mut work,
        )?;
    }
    add_visible_context_edges(projection, config, access, &mut work)?;
    mark_bounded_telemetry(&mut work);

    Ok(ActivatedProjection {
        projection_snapshot_id: projection.projection_snapshot_id.clone(),
        configuration_snapshot_id: config.configuration_snapshot_id.clone(),
        problem_space_thread_id: problem_space.thread_id.clone(),
        problem_space_version: problem_space.version,
        newest_utterance_id: utterance.utterance_id.clone(),
        activated_objects: work.objects,
        activated_regions: work.regions,
        activated_units: work.units,
        activated_identifier_assignments: work.assignments,
        activated_occurrences: work.occurrences,
        activated_temporal_anchors: work.anchors,
        edges: work.edges,
        telemetry: work.telemetry,
        continuation_handles: work.handles,
    })
}

fn preflight(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
) -> Result<(), ProjectionActivationViolation> {
    for (field, value) in [
        (
            "projection.projection_snapshot_id",
            projection.projection_snapshot_id.as_str(),
        ),
        ("problem_space.thread_id", problem_space.thread_id.as_str()),
        ("utterance.utterance_id", utterance.utterance_id.as_str()),
        (
            "config.configuration_snapshot_id",
            config.configuration_snapshot_id.as_str(),
        ),
    ] {
        if value.trim().is_empty() {
            return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                field: field.into(),
            });
        }
    }
    if projection.validation_status != ProjectionValidationStatus::Validated {
        return Err(ProjectionActivationViolation::ProjectionNotValidated {
            status: projection.validation_status.clone(),
        });
    }
    access
        .artifacts
        .validate_against(projection)
        .map_err(
            |error| ProjectionActivationViolation::InvalidActivatedReference {
                context: format!("access artifacts do not validate: {error}"),
            },
        )?;
    validate_surface_configuration(projection, config)?;
    validate_supported_match_modes(projection)?;
    validate_lens(problem_space)
}

fn validate_surface_configuration(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
) -> Result<(), ProjectionActivationViolation> {
    let mut seen = HashSet::new();
    for surface in &config.surface_limits {
        if surface.surface_id.trim().is_empty() {
            return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                field: "config.surface_limits.surface_id".into(),
            });
        }
        if !seen.insert(surface.surface_id.clone()) {
            return Err(
                ProjectionActivationViolation::DuplicateSurfaceConfiguration {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
        if !projection
            .retrieval_surfaces
            .iter()
            .any(|descriptor| descriptor.surface_id == surface.surface_id)
        {
            return Err(
                ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
                    surface_id: surface.surface_id.clone(),
                },
            );
        }
    }
    for descriptor in &projection.retrieval_surfaces {
        if descriptor.surface_id.trim().is_empty() {
            return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                field: "projection.retrieval_surfaces.surface_id".into(),
            });
        }
        if !seen.contains(&descriptor.surface_id) {
            return Err(
                ProjectionActivationViolation::MissingAvailableSurfaceConfiguration {
                    surface_id: descriptor.surface_id.clone(),
                },
            );
        }
    }
    Ok(())
}

fn validate_supported_match_modes(
    projection: &SemanticSpaceProjection,
) -> Result<(), ProjectionActivationViolation> {
    for surface in &projection.retrieval_surfaces {
        for mode in &surface.match_modes {
            if let SurfaceMatchMode::Declared { name } = mode {
                return Err(ProjectionActivationViolation::SurfaceAccessFailed {
                    surface_id: surface.surface_id.clone(),
                    probe_id: "activation-probe:unsupported-declared-mode".into(),
                    context: format!(
                        "declared match mode {name} is unsupported by current Phase 7 access"
                    ),
                });
            }
        }
    }
    Ok(())
}

fn validate_lens(problem_space: &ProblemSpaceState) -> Result<(), ProjectionActivationViolation> {
    let mut lens = Vec::<(&String, ActivationBand)>::new();
    for id in &problem_space.attention_lens.primary_region_ids {
        lens.push((id, ActivationBand::Primary));
    }
    for id in &problem_space.attention_lens.secondary_region_ids {
        lens.push((id, ActivationBand::Secondary));
    }
    for id in &problem_space.attention_lens.tertiary_region_ids {
        lens.push((id, ActivationBand::Tertiary));
    }
    for id in &problem_space.attention_lens.background_region_ids {
        lens.push((id, ActivationBand::Background));
    }

    let mut seen = HashSet::new();
    for (region_id, band) in lens {
        if !seen.insert(region_id.clone()) {
            return invalid_reference(format!("attention_lens duplicates region {region_id}"));
        }
        let region = problem_space
            .regions
            .iter()
            .find(|region| &region.region_id == region_id)
            .ok_or_else(
                || ProjectionActivationViolation::InvalidActivatedReference {
                    context: format!("attention_lens references unknown region {region_id}"),
                },
            )?;
        if !operational_region(&region.persistence_state) {
            return invalid_reference(format!(
                "attention_lens references nonoperational region {region_id}"
            ));
        }
        if region.activation_band != band {
            return invalid_reference(format!(
                "attention_lens band mismatch for region {region_id}"
            ));
        }
    }
    for region in &problem_space.regions {
        if operational_region(&region.persistence_state) && !seen.contains(&region.region_id) {
            return invalid_reference(format!(
                "operational region {} missing from attention_lens",
                region.region_id
            ));
        }
    }
    for constraint in &problem_space.constraints {
        if constraint.lifecycle == RecordLifecycle::Active
            && let ProblemConstraintApplicability::Regions { region_ids } =
                &constraint.applicability
        {
            for region_id in region_ids {
                let region = problem_space
                    .regions
                    .iter()
                    .find(|region| &region.region_id == region_id)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: format!(
                                "active constraint {} references unknown region {region_id}",
                                constraint.constraint_id
                            ),
                        },
                    )?;
                if !operational_region(&region.persistence_state) {
                    return invalid_reference(format!(
                        "active constraint {} references nonoperational region {region_id}",
                        constraint.constraint_id
                    ));
                }
            }
        }
    }
    for relation in &problem_space.relations {
        if relation.lifecycle == RecordLifecycle::Active {
            if !problem_space
                .regions
                .iter()
                .any(|region| region.region_id == relation.source_region_id)
            {
                return invalid_reference(format!(
                    "active relation {} references unknown source region {}",
                    relation.relation_id, relation.source_region_id
                ));
            }
            if let Some(target_region_id) = &relation.target_region_id
                && !problem_space
                    .regions
                    .iter()
                    .any(|region| &region.region_id == target_region_id)
            {
                return invalid_reference(format!(
                    "active relation {} references unknown target region {target_region_id}",
                    relation.relation_id
                ));
            }
        }
    }
    for tension in &problem_space.open_tensions {
        if tension.lifecycle == TensionLifecycle::Open
            && !problem_space
                .regions
                .iter()
                .any(|region| region.region_id == tension.region_id)
        {
            return invalid_reference(format!(
                "open tension {} references unknown region {}",
                tension.tension_id, tension.region_id
            ));
        }
    }
    Ok(())
}

fn operational_region(state: &RegionPersistenceState) -> bool {
    matches!(
        state,
        RegionPersistenceState::Active
            | RegionPersistenceState::Background
            | RegionPersistenceState::Unresolved
    )
}

fn invalid_reference<T>(context: String) -> Result<T, ProjectionActivationViolation> {
    Err(ProjectionActivationViolation::InvalidActivatedReference { context })
}

fn build_seeds(
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
) -> Result<Vec<Seed>, ProjectionActivationViolation> {
    let mut seeds = Vec::new();
    let mut unbanded = 0;
    let unbanded_limit = config.unbanded.maximum_textual_seeds;
    if unbanded < unbanded_limit {
        seeds.push(Seed {
            text: utterance.text.clone(),
            band: ActivationProbeBand::Unbanded,
            provenance: vec![ActivationProvenance::NewestUtterance {
                utterance_id: utterance.utterance_id.clone(),
            }],
            region_id: None,
        });
        unbanded += 1;
    }
    for constraint in &problem_space.constraints {
        if unbanded >= unbanded_limit {
            break;
        }
        if constraint.lifecycle == RecordLifecycle::Active
            && matches!(
                constraint.applicability,
                ProblemConstraintApplicability::WholeProblemSpace
            )
        {
            seeds.push(Seed {
                text: constraint.expression.clone(),
                band: ActivationProbeBand::Unbanded,
                provenance: vec![ActivationProvenance::Constraint {
                    constraint_id: constraint.constraint_id.clone(),
                }],
                region_id: None,
            });
            unbanded += 1;
        }
    }
    add_band_seeds(
        problem_space,
        config,
        ActivationBand::Primary,
        &problem_space.attention_lens.primary_region_ids,
        &mut seeds,
    )?;
    add_band_seeds(
        problem_space,
        config,
        ActivationBand::Secondary,
        &problem_space.attention_lens.secondary_region_ids,
        &mut seeds,
    )?;
    add_band_seeds(
        problem_space,
        config,
        ActivationBand::Tertiary,
        &problem_space.attention_lens.tertiary_region_ids,
        &mut seeds,
    )?;
    add_band_seeds(
        problem_space,
        config,
        ActivationBand::Background,
        &problem_space.attention_lens.background_region_ids,
        &mut seeds,
    )?;
    Ok(seeds)
}

fn add_band_seeds(
    problem_space: &ProblemSpaceState,
    config: &ProjectionActivationConfig,
    band: ActivationBand,
    region_ids: &[String],
    seeds: &mut Vec<Seed>,
) -> Result<(), ProjectionActivationViolation> {
    let probe_band = ActivationProbeBand::Attention(band.clone());
    let limit = band_config(config, &probe_band).maximum_textual_seeds;
    let mut used = 0;
    for region_id in region_ids {
        if used >= limit {
            break;
        }
        let region = problem_space
            .regions
            .iter()
            .find(|region| &region.region_id == region_id)
            .expect("attention lens was preflight validated");
        for referent in &region.anchor_referents {
            if used >= limit {
                break;
            }
            seeds.push(Seed {
                text: referent.expression.clone(),
                band: probe_band.clone(),
                provenance: vec![
                    ActivationProvenance::ProblemRegion {
                        region_id: region_id.clone(),
                    },
                    ActivationProvenance::ProblemReferent {
                        region_id: region_id.clone(),
                        referent_id: referent.referent_id.clone(),
                    },
                    ActivationProvenance::AttentionBand {
                        region_id: region_id.clone(),
                        band: band.clone(),
                    },
                ],
                region_id: Some(region_id.clone()),
            });
            used += 1;
        }
        for constraint in &problem_space.constraints {
            if used >= limit {
                break;
            }
            if constraint.lifecycle == RecordLifecycle::Active
                && let ProblemConstraintApplicability::Regions { region_ids } =
                    &constraint.applicability
                && region_ids.contains(region_id)
            {
                seeds.push(Seed {
                    text: constraint.expression.clone(),
                    band: probe_band.clone(),
                    provenance: vec![
                        ActivationProvenance::ProblemRegion {
                            region_id: region_id.clone(),
                        },
                        ActivationProvenance::Constraint {
                            constraint_id: constraint.constraint_id.clone(),
                        },
                        ActivationProvenance::AttentionBand {
                            region_id: region_id.clone(),
                            band: band.clone(),
                        },
                    ],
                    region_id: Some(region_id.clone()),
                });
                used += 1;
            }
        }
        for tension in &problem_space.open_tensions {
            if used >= limit {
                break;
            }
            if tension.region_id == *region_id && tension.lifecycle == TensionLifecycle::Open {
                add_tension_seeds(
                    tension,
                    region_id,
                    &band,
                    &probe_band,
                    seeds,
                    &mut used,
                    limit,
                )?;
            }
        }
    }
    Ok(())
}

fn add_tension_seeds(
    tension: &OpenTension,
    region_id: &str,
    band: &ActivationBand,
    probe_band: &ActivationProbeBand,
    seeds: &mut Vec<Seed>,
    used: &mut u32,
    limit: u32,
) -> Result<(), ProjectionActivationViolation> {
    if let Some(expression) = &tension.unresolved_expression
        && *used < limit
    {
        seeds.push(Seed {
            text: expression.clone(),
            band: probe_band.clone(),
            provenance: vec![
                ActivationProvenance::ProblemRegion {
                    region_id: region_id.into(),
                },
                ActivationProvenance::OpenTension {
                    tension_id: tension.tension_id.clone(),
                },
                ActivationProvenance::AttentionBand {
                    region_id: region_id.into(),
                    band: band.clone(),
                },
            ],
            region_id: Some(region_id.into()),
        });
        *used += 1;
    }
    for (index, candidate) in tension.candidate_bindings.iter().enumerate() {
        if *used >= limit {
            break;
        }
        seeds.push(Seed {
            text: candidate.clone(),
            band: probe_band.clone(),
            provenance: vec![
                ActivationProvenance::ProblemRegion {
                    region_id: region_id.into(),
                },
                ActivationProvenance::OpenTension {
                    tension_id: tension.tension_id.clone(),
                },
                ActivationProvenance::OpenTensionCandidate {
                    tension_id: tension.tension_id.clone(),
                    candidate_index: u32::try_from(index)
                        .map_err(|_| ProjectionActivationViolation::CountOverflow)?,
                },
                ActivationProvenance::AttentionBand {
                    region_id: region_id.into(),
                    band: band.clone(),
                },
            ],
            region_id: Some(region_id.into()),
        });
        *used += 1;
    }
    Ok(())
}

fn dispatch_text_seed(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
    work: &mut Work,
    seed: Seed,
) -> Result<(), ProjectionActivationViolation> {
    for surface in &projection.retrieval_surfaces {
        for mode in &surface.match_modes {
            if !text_mode_applies(surface, mode) {
                continue;
            }
            let mut provenance = with_default(seed.provenance.clone(), "automatic_surface_fan_out");
            append_relation_provenance(&mut provenance, problem_space, seed.region_id.as_ref());
            let limit = surface_limit(config, &surface.surface_id, &seed.band);
            if limit == 0 {
                record_zero_candidate_probe(config, work, surface, mode.clone(), 0, provenance)?;
                continue;
            }
            let operands =
                text_operands(access, surface, mode, &seed.text, &mut work.probe_counter)?;
            if operands.is_empty() {
                continue;
            }
            for operand in operands {
                execute_access_probe(
                    projection,
                    problem_space,
                    utterance,
                    config,
                    access,
                    work,
                    surface,
                    mode.clone(),
                    operand,
                    limit,
                    0,
                    provenance.clone(),
                    ProbeOrigin::Text(seed.text.clone()),
                    seed.band.clone(),
                    seed.region_id.clone(),
                    None,
                )?;
            }
        }
    }
    Ok(())
}

fn text_mode_applies(surface: &RetrievalSurfaceDescriptor, match_mode: &SurfaceMatchMode) -> bool {
    matches!(
        (&surface.kind, match_mode),
        (RetrievalSurfaceKind::Exact, SurfaceMatchMode::Literal)
            | (RetrievalSurfaceKind::Lexical, SurfaceMatchMode::Terms)
            | (
                RetrievalSurfaceKind::Vector,
                SurfaceMatchMode::NearestNeighbours
            )
            | (_, SurfaceMatchMode::Declared { .. })
    )
}

#[allow(clippy::too_many_arguments)]
fn drain_queued_addresses(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
    work: &mut Work,
) -> Result<(), ProjectionActivationViolation> {
    let mut drained = Vec::<(SemanticAddress, u32)>::new();
    while !work.queued_addresses.is_empty() {
        let item = work.queued_addresses.remove(0);
        if drained.contains(&(item.address.clone(), item.depth)) {
            continue;
        }
        drained.push((item.address.clone(), item.depth));
        for surface in &projection.retrieval_surfaces {
            if !surface_applies_to_address(projection, surface, &item.address) {
                continue;
            }
            for mode in &surface.match_modes {
                match (&surface.kind, mode) {
                    (RetrievalSurfaceKind::Graph, SurfaceMatchMode::Incidence) => {
                        for direction in [Direction::Outgoing, Direction::Incoming] {
                            let mut provenance = item.provenance.clone();
                            append_relation_provenance(
                                &mut provenance,
                                problem_space,
                                item.region_id.as_ref(),
                            );
                            push_unique(
                                &mut provenance,
                                ActivationProvenance::ConfiguredDefault {
                                    configuration_key: "automatic_surface_fan_out".into(),
                                },
                            );
                            let limit = surface_limit(config, &surface.surface_id, &item.band);
                            execute_access_probe(
                                projection,
                                problem_space,
                                utterance,
                                config,
                                access,
                                work,
                                surface,
                                mode.clone(),
                                AccessOperand::Graph {
                                    seed: item.address.clone(),
                                    direction: direction.clone(),
                                    transition_ids: vec![],
                                },
                                limit,
                                item.depth,
                                provenance,
                                ProbeOrigin::Structural {
                                    subject: item.address.clone(),
                                    direction: direction.clone(),
                                },
                                item.band.clone(),
                                item.region_id.clone(),
                                Some((item.address.clone(), direction)),
                            )?;
                        }
                    }
                    (RetrievalSurfaceKind::Temporal, SurfaceMatchMode::Temporal) => {
                        let queries = temporal_queries_for_address(projection, &item.address);
                        if queries.is_empty() {
                            continue;
                        }
                        let provenance =
                            with_default(item.provenance.clone(), "automatic_surface_fan_out");
                        let limit = surface_limit(config, &surface.surface_id, &item.band);
                        for query in queries {
                            if limit == 0 {
                                record_zero_candidate_probe(
                                    config,
                                    work,
                                    surface,
                                    mode.clone(),
                                    item.depth,
                                    provenance.clone(),
                                )?;
                                continue;
                            }
                            execute_access_probe(
                                projection,
                                problem_space,
                                utterance,
                                config,
                                access,
                                work,
                                surface,
                                mode.clone(),
                                AccessOperand::Temporal(query.clone()),
                                limit,
                                item.depth,
                                provenance.clone(),
                                ProbeOrigin::Temporal { query },
                                item.band.clone(),
                                item.region_id.clone(),
                                None,
                            )?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

enum ProbeOrigin {
    Text(String),
    Structural {
        subject: SemanticAddress,
        direction: Direction,
    },
    Temporal {
        query: TemporalQuery,
    },
}

#[allow(clippy::too_many_arguments)]
fn execute_access_probe(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
    work: &mut Work,
    surface: &RetrievalSurfaceDescriptor,
    match_mode: SurfaceMatchMode,
    operand: AccessOperand,
    candidate_limit: u32,
    depth: u32,
    provenance: Vec<ActivationProvenance>,
    origin: ProbeOrigin,
    band: ActivationProbeBand,
    region_id: Option<String>,
    direct_edge_source: Option<(SemanticAddress, Direction)>,
) -> Result<(), ProjectionActivationViolation> {
    let probe_id = next_id("activation-probe", &mut work.probe_counter)?;
    let telemetry_id = reserve_telemetry(config, work)?;
    let structural_neighbor_limit = match &origin {
        ProbeOrigin::Structural { .. } => {
            band_config(config, &band).maximum_structural_neighbors_per_record
        }
        ProbeOrigin::Text(_) | ProbeOrigin::Temporal { .. } => u32::MAX,
    };
    let probe_limit = candidate_limit.min(structural_neighbor_limit);
    let high_degree = matches!(&origin, ProbeOrigin::Structural { subject, .. }
        if structural_degree(access, subject) >= config.hub_degree_threshold);
    let mut telemetry_provenance = provenance.clone();
    if high_degree {
        push_unique(
            &mut telemetry_provenance,
            ActivationProvenance::ConfiguredDefault {
                configuration_key: "high_degree_summary".into(),
            },
        );
    }
    if probe_limit == 0 {
        if let Some(telemetry_id) = telemetry_id {
            push_zero_candidate_telemetry(
                config,
                work,
                surface,
                match_mode,
                depth,
                telemetry_provenance,
                probe_id,
                telemetry_id,
            );
        }
        return Ok(());
    }
    let probe = ProjectionAccessProbe {
        probe_id: probe_id.clone(),
        projection_snapshot_id: projection.projection_snapshot_id.clone(),
        surface_id: surface.surface_id.clone(),
        surface_kind: surface.kind.clone(),
        match_mode: match_mode.clone(),
        operand,
        page_size: probe_limit as usize,
        cursor: None,
    };
    let result = access
        .artifacts
        .probe(projection, &probe)
        .map_err(|error| ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface.surface_id.clone(),
            probe_id: probe_id.clone(),
            context: error.to_string(),
        })?;
    if let Some(failure) = &result.failure {
        return Err(ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface.surface_id.clone(),
            probe_id: probe_id.clone(),
            context: format!("{}: {}", failure.code, failure.message),
        });
    }
    validate_probe_result(projection, access, surface, &probe, &result)?;
    if result.candidates.len() > probe_limit as usize {
        return Err(ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface.surface_id.clone(),
            probe_id: probe_id.clone(),
            context: "returned candidate count exceeds probe limit".into(),
        });
    }

    let mut bounded = result.truncated;
    let exposure = Exposure {
        provenance: provenance.clone(),
    };
    for candidate in &result.candidates {
        validate_candidate_address_exists(projection, &candidate.identity).map_err(|context| {
            ProjectionActivationViolation::SurfaceAccessFailed {
                surface_id: surface.surface_id.clone(),
                probe_id: probe_id.clone(),
                context,
            }
        })?;
        let direct_edge = direct_edge_source.as_ref().and_then(|(source, direction)| {
            candidate
                .transition_id
                .as_ref()
                .map(|transition_id| EdgeTuple {
                    source: source.clone(),
                    transition_id: transition_id.clone(),
                    direction: direction.clone(),
                    target: candidate.identity.clone(),
                })
        });
        if !insert_candidate_bundle(
            projection,
            config,
            work,
            &candidate.identity,
            &exposure,
            &band,
        )? {
            bounded = true;
            work.bounded_probe_ids.insert(probe_id.clone());
            continue;
        }
        if let Some(edge) = direct_edge {
            add_edge(config, work, edge, provenance.clone(), &probe_id)?;
        }
        enqueue_candidate_followups(
            projection,
            config,
            work,
            &candidate.identity,
            &band,
            &provenance,
            region_id.clone(),
            depth,
        )?;
    }

    let continuation_available = add_surface_continuation(
        problem_space,
        utterance,
        config,
        work,
        &probe_id,
        &result,
        &origin,
        surface,
        match_mode.clone(),
        &telemetry_provenance,
        probe_limit,
    )?;
    let returned_count = u64::try_from(result.returned_count)
        .map_err(|_| ProjectionActivationViolation::CountOverflow)?;
    if let Some(telemetry_id) = telemetry_id {
        work.telemetry.push(ProjectionTelemetry {
            telemetry_id,
            probe_id,
            match_mode,
            surface_kind: surface.kind.clone(),
            surface_id: surface.surface_id.clone(),
            candidate_count: access_candidate_count(&result)?,
            current_depth: depth,
            maximum_depth: config.maximum_initial_relation_depth,
            returned_count,
            remaining_expansion_budget: config.maximum_expansion_budget,
            truncation_state: if bounded {
                TruncationState::Bounded
            } else {
                TruncationState::Complete
            },
            identifier_type_distribution: identifier_type_distribution(
                projection,
                &result.candidates,
            ),
            temporal_anchor_count: result
                .candidates
                .iter()
                .filter(|candidate| {
                    matches!(candidate.identity, SemanticAddress::TemporalAnchor(_))
                })
                .count() as u64,
            unresolved_target_count: unresolved_target_count(projection, &result.candidates),
            continuation_available,
            activation_provenance: telemetry_provenance,
        });
    }
    Ok(())
}

fn record_zero_candidate_probe(
    config: &ProjectionActivationConfig,
    work: &mut Work,
    surface: &RetrievalSurfaceDescriptor,
    match_mode: SurfaceMatchMode,
    depth: u32,
    provenance: Vec<ActivationProvenance>,
) -> Result<(), ProjectionActivationViolation> {
    let probe_id = next_id("activation-probe", &mut work.probe_counter)?;
    let telemetry_id = reserve_telemetry(config, work)?;
    if let Some(telemetry_id) = telemetry_id {
        push_zero_candidate_telemetry(
            config,
            work,
            surface,
            match_mode,
            depth,
            provenance,
            probe_id,
            telemetry_id,
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn push_zero_candidate_telemetry(
    config: &ProjectionActivationConfig,
    work: &mut Work,
    surface: &RetrievalSurfaceDescriptor,
    match_mode: SurfaceMatchMode,
    depth: u32,
    provenance: Vec<ActivationProvenance>,
    probe_id: String,
    telemetry_id: String,
) {
    work.telemetry.push(ProjectionTelemetry {
        telemetry_id,
        probe_id,
        match_mode,
        surface_kind: surface.kind.clone(),
        surface_id: surface.surface_id.clone(),
        candidate_count: CandidateCount::Exact(0),
        current_depth: depth,
        maximum_depth: config.maximum_initial_relation_depth,
        returned_count: 0,
        remaining_expansion_budget: config.maximum_expansion_budget,
        truncation_state: TruncationState::Complete,
        identifier_type_distribution: vec![],
        temporal_anchor_count: 0,
        unresolved_target_count: 0,
        continuation_available: false,
        activation_provenance: provenance,
    });
}

fn text_operands(
    access: &ProjectionActivationAccess<'_>,
    surface: &RetrievalSurfaceDescriptor,
    match_mode: &SurfaceMatchMode,
    text: &str,
    probe_counter: &mut u64,
) -> Result<Vec<AccessOperand>, ProjectionActivationViolation> {
    match (&surface.kind, match_mode) {
        (RetrievalSurfaceKind::Exact, SurfaceMatchMode::Literal) => {
            Ok(vec![AccessOperand::ExactLiteral(text.to_owned())])
        }
        (RetrievalSurfaceKind::Lexical, SurfaceMatchMode::Terms) => {
            let terms = lexical_terms(text);
            if terms.is_empty() {
                Ok(vec![])
            } else {
                Ok(vec![AccessOperand::LexicalTerms(terms)])
            }
        }
        (RetrievalSurfaceKind::Vector, SurfaceMatchMode::NearestNeighbours) => {
            vector_text_operand(access, &surface.surface_id, text, probe_counter)
                .map(|operand| vec![operand])
        }
        (_, SurfaceMatchMode::Declared { name }) => {
            let probe_id = next_id("activation-probe", probe_counter)?;
            Err(ProjectionActivationViolation::SurfaceAccessFailed {
                surface_id: surface.surface_id.clone(),
                probe_id,
                context: format!(
                    "declared match mode {name} is unsupported by current Phase 7 access"
                ),
            })
        }
        _ => Ok(vec![]),
    }
}

fn vector_text_operand(
    access: &ProjectionActivationAccess<'_>,
    surface_id: &str,
    text: &str,
    probe_counter: &mut u64,
) -> Result<AccessOperand, ProjectionActivationViolation> {
    let Some(provider) = access.query_embedding_provider else {
        if matches!(
            &access.artifacts.vector.provider,
            VectorProviderState::Unavailable { .. }
        ) {
            return Ok(AccessOperand::Vector(vec![0.0]));
        }
        let probe_id = next_id("activation-probe", probe_counter)?;
        return Err(ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface_id.into(),
            probe_id,
            context: "vector textual activation requires a query embedding provider matching the access artifact".into(),
        });
    };
    let query_identity = provider.identity().map_err(|failure| {
        ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface_id.into(),
            probe_id: next_id("activation-probe", probe_counter)
                .unwrap_or_else(|_| "activation-probe:overflow".into()),
            context: format!(
                "query embedding provider identity failed: {}",
                failure.message
            ),
        }
    })?;
    match &access.artifacts.vector.provider {
        VectorProviderState::Ready { identity } if identity == &query_identity => {}
        VectorProviderState::Ready { .. } => {
            let probe_id = next_id("activation-probe", probe_counter)?;
            return Err(ProjectionActivationViolation::SurfaceAccessFailed {
                surface_id: surface_id.into(),
                probe_id,
                context: "query embedding provider identity does not match vector access artifact"
                    .into(),
            });
        }
        VectorProviderState::Unavailable { failure, .. } => {
            let probe_id = next_id("activation-probe", probe_counter)?;
            return Err(ProjectionActivationViolation::SurfaceAccessFailed {
                surface_id: surface_id.into(),
                probe_id,
                context: format!("vector access provider unavailable: {}", failure.message),
            });
        }
    }
    let embeddings = provider.embed(&[text.to_owned()]).map_err(|failure| {
        ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface_id.into(),
            probe_id: next_id("activation-probe", probe_counter)
                .unwrap_or_else(|_| "activation-probe:overflow".into()),
            context: format!("query embedding failed: {}", failure.message),
        }
    })?;
    if embeddings.len() != 1 {
        let probe_id = next_id("activation-probe", probe_counter)?;
        return Err(ProjectionActivationViolation::SurfaceAccessFailed {
            surface_id: surface_id.into(),
            probe_id,
            context: format!(
                "query embedding returned {} vectors for one textual seed",
                embeddings.len()
            ),
        });
    }
    Ok(AccessOperand::Vector(
        embeddings.into_iter().next().expect("count checked"),
    ))
}

#[allow(clippy::too_many_arguments)]
fn enqueue_candidate_followups(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    address: &SemanticAddress,
    band: &ActivationProbeBand,
    provenance: &[ActivationProvenance],
    region_id: Option<String>,
    depth: u32,
) -> Result<(), ProjectionActivationViolation> {
    let mut context = Vec::new();
    closure_into(projection, address, &mut context)?;
    for context_address in context {
        if depth < config.maximum_initial_relation_depth {
            let next_depth = depth
                .checked_add(1)
                .ok_or(ProjectionActivationViolation::CountOverflow)?;
            push_queued_address(
                work,
                QueuedAddress {
                    address: context_address.clone(),
                    band: band.clone(),
                    provenance: provenance.to_vec(),
                    region_id: region_id.clone(),
                    depth: next_depth,
                },
            );
        }
        if !temporal_queries_for_address(projection, &context_address).is_empty() {
            push_queued_address(
                work,
                QueuedAddress {
                    address: context_address,
                    band: band.clone(),
                    provenance: provenance.to_vec(),
                    region_id: region_id.clone(),
                    depth,
                },
            );
        }
    }
    Ok(())
}

fn push_queued_address(work: &mut Work, queued: QueuedAddress) {
    if !work.queued_addresses.iter().any(|known| {
        known.address == queued.address && known.depth == queued.depth && known.band == queued.band
    }) {
        work.queued_addresses.push(queued);
    }
}

fn insert_candidate_bundle(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    candidate: &SemanticAddress,
    exposure: &Exposure,
    band: &ActivationProbeBand,
) -> Result<bool, ProjectionActivationViolation> {
    let Some(required) = closure_addresses(projection, candidate)? else {
        return Ok(false);
    };
    if !bundle_fits(config, work, projection, &required)? {
        return Ok(false);
    }
    for address in &required {
        let provenance = if address == candidate {
            exposure.provenance.clone()
        } else {
            with_default(exposure.provenance.clone(), "bounded_structural_context")
        };
        insert_one(projection, config, work, address, provenance, band)?;
    }
    refresh_visible_previews(projection, config, work);
    Ok(true)
}

fn closure_addresses(
    projection: &SemanticSpaceProjection,
    address: &SemanticAddress,
) -> Result<Option<Vec<SemanticAddress>>, ProjectionActivationViolation> {
    if let SemanticAddress::Occurrence(occurrence_id) = address
        && projection
            .occurrences
            .iter()
            .find(|occurrence| &occurrence.occurrence_id == occurrence_id)
            .is_some_and(|occurrence| occurrence.resolved_target.is_none())
    {
        return Ok(None);
    }
    let mut required = Vec::new();
    closure_into(projection, address, &mut required)?;
    Ok(Some(required))
}

fn closure_into(
    projection: &SemanticSpaceProjection,
    address: &SemanticAddress,
    required: &mut Vec<SemanticAddress>,
) -> Result<(), ProjectionActivationViolation> {
    match address {
        SemanticAddress::Object(_) => push_unique(required, address.clone()),
        SemanticAddress::Region(region) => {
            push_unique(required, SemanticAddress::Object(region.object_id.clone()));
            push_unique(required, address.clone());
        }
        SemanticAddress::Unit(unit_id) => {
            let unit = projection
                .units
                .iter()
                .find(|unit| &unit.unit_id == unit_id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: format!("unit {unit_id} missing"),
                    },
                )?;
            push_unique(
                required,
                SemanticAddress::Object(unit.parent_object_id.clone()),
            );
            push_unique(
                required,
                SemanticAddress::Region(unit.parent_region_address.clone()),
            );
            push_unique(required, address.clone());
        }
        SemanticAddress::Identifier(_) => {
            let assignment =
                resolve_identifier_assignment(projection, address).map_err(|context| {
                    ProjectionActivationViolation::InvalidActivatedReference { context }
                })?;
            closure_into(projection, &assignment.subject, required)?;
            push_unique(required, address.clone());
        }
        SemanticAddress::Occurrence(occurrence_id) => {
            let occurrence = projection
                .occurrences
                .iter()
                .find(|occurrence| &occurrence.occurrence_id == occurrence_id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: format!("occurrence {occurrence_id} missing"),
                    },
                )?;
            match &occurrence.source {
                OccurrenceSource::ObjectField { object_id, .. } => {
                    closure_into(
                        projection,
                        &SemanticAddress::Object(object_id.clone()),
                        required,
                    )?;
                }
                OccurrenceSource::SemanticRegion { region_address } => {
                    closure_into(
                        projection,
                        &SemanticAddress::Region(region_address.clone()),
                        required,
                    )?;
                }
                OccurrenceSource::SemanticUnit { unit_id } => {
                    closure_into(
                        projection,
                        &SemanticAddress::Unit(unit_id.clone()),
                        required,
                    )?;
                }
            }
            let Some(target) = &occurrence.resolved_target else {
                return Ok(());
            };
            closure_into(projection, target, required)?;
            push_unique(required, address.clone());
        }
        SemanticAddress::TemporalAnchor(anchor_id) => {
            let anchor = projection
                .temporal_anchors
                .iter()
                .find(|anchor| &anchor.anchor_id == anchor_id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: format!("temporal anchor {anchor_id} missing"),
                    },
                )?;
            closure_into(projection, &anchor.subject, required)?;
            push_unique(required, address.clone());
        }
        SemanticAddress::RetrievalSurface(_) => {}
    }
    Ok(())
}

fn bundle_fits(
    config: &ProjectionActivationConfig,
    work: &Work,
    projection: &SemanticSpaceProjection,
    required: &[SemanticAddress],
) -> Result<bool, ProjectionActivationViolation> {
    let mut objects = 0usize;
    let mut regions = 0usize;
    let mut units = 0usize;
    let mut assignments = 0usize;
    let mut occurrences = 0usize;
    let mut anchors = 0usize;
    for address in required {
        match address {
            SemanticAddress::Object(id) if !work.objects.iter().any(|r| &r.object_id == id) => {
                objects += 1;
            }
            SemanticAddress::Region(id) if !work.regions.iter().any(|r| &r.address == id) => {
                regions += 1;
            }
            SemanticAddress::Unit(id) if !work.units.iter().any(|r| &r.unit_id == id) => {
                units += 1;
            }
            SemanticAddress::Identifier(_)
                if !resolve_identifier_assignment(projection, address).is_ok_and(
                    |assignment| {
                        work.assignments
                            .iter()
                            .any(|record| record.assignment_id == assignment.assignment_id)
                    },
                ) =>
            {
                assignments += 1;
            }
            SemanticAddress::Occurrence(id)
                if !work
                    .occurrences
                    .iter()
                    .any(|record| &record.occurrence_id == id) =>
            {
                occurrences += 1;
            }
            SemanticAddress::TemporalAnchor(id)
                if !work.anchors.iter().any(|record| &record.anchor_id == id) =>
            {
                anchors += 1;
            }
            _ => {}
        }
    }
    Ok(
        work.objects.len().saturating_add(objects) <= config.maximum_activated_objects as usize
            && work.regions.len().saturating_add(regions)
                <= config.maximum_activated_regions as usize
            && work.units.len().saturating_add(units) <= config.maximum_activated_units as usize
            && work.assignments.len().saturating_add(assignments)
                <= config.maximum_activated_identifier_assignments as usize
            && work.occurrences.len().saturating_add(occurrences)
                <= config.maximum_activated_occurrences as usize
            && work.anchors.len().saturating_add(anchors)
                <= config.maximum_activated_temporal_anchors as usize,
    )
}

fn insert_one(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    address: &SemanticAddress,
    provenance: Vec<ActivationProvenance>,
    band: &ActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    let structural_context_provenance =
        with_default(provenance.clone(), "bounded_structural_context");
    match address {
        SemanticAddress::Object(object_id) => {
            if let Some(record) = work
                .objects
                .iter_mut()
                .find(|record| &record.object_id == object_id)
            {
                merge(&mut record.activation_provenance, &provenance);
            } else {
                let object = projection
                    .objects
                    .iter()
                    .find(|object| &object.object_id == object_id)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: format!("object {object_id} missing"),
                        },
                    )?;
                work.objects.push(ActivatedObjectRecord {
                    object_id: object_id.clone(),
                    title: object.title.clone(),
                    aliases: object.aliases.clone(),
                    object_class: object.object_class.clone(),
                    visible_region_addresses: vec![],
                    visible_unit_ids: vec![],
                    visible_identifier_assignment_ids: vec![],
                    contained_region_count: object.region_addresses.len() as u64,
                    contained_unit_count: object.unit_ids.len() as u64,
                    incoming_occurrence_count: object.incoming_occurrence_ids.len() as u64,
                    outgoing_occurrence_count: (object.object_field_occurrence_ids.len()
                        + object.body_occurrence_ids.len())
                        as u64,
                    available_surface_ids: object.retrieval_surface_ids.clone(),
                    activation_provenance: provenance,
                });
            }
            insert_assignment_ids(
                projection,
                config,
                work,
                &projection
                    .objects
                    .iter()
                    .find(|object| &object.object_id == object_id)
                    .map(|object| object.identifier_assignment_ids.clone())
                    .unwrap_or_default(),
                address,
                &structural_context_provenance,
            )?;
        }
        SemanticAddress::Region(region_address) => {
            if let Some(record) = work
                .regions
                .iter_mut()
                .find(|record| &record.address == region_address)
            {
                merge(&mut record.activation_provenance, &provenance);
            } else {
                let region = projection
                    .regions
                    .iter()
                    .find(|region| &region.address == region_address)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: "region missing".into(),
                        },
                    )?;
                work.regions.push(ActivatedRegionRecord {
                    address: region_address.clone(),
                    heading_path: region.heading_path.clone(),
                    heading_identity: region.heading_identity.clone(),
                    visible_identifier_assignment_ids: vec![],
                    visible_unit_ids: vec![],
                    contained_unit_count: region.contained_unit_ids.len() as u64,
                    available_surface_ids: region.retrieval_surface_ids.clone(),
                    activation_provenance: provenance,
                });
            }
            insert_assignment_ids(
                projection,
                config,
                work,
                &projection
                    .regions
                    .iter()
                    .find(|region| &region.address == region_address)
                    .map(|region| region.inherited_identifier_assignment_ids.clone())
                    .unwrap_or_default(),
                address,
                &structural_context_provenance,
            )?;
        }
        SemanticAddress::Unit(unit_id) => {
            if let Some(record) = work
                .units
                .iter_mut()
                .find(|record| &record.unit_id == unit_id)
            {
                merge(&mut record.activation_provenance, &provenance);
                let unit = projection
                    .units
                    .iter()
                    .find(|unit| &unit.unit_id == unit_id)
                    .expect("existing activated unit has projection record");
                record.text_preview = larger_preview(
                    record.text_preview.clone(),
                    &unit.content,
                    band_config(config, band).text_preview_character_limit,
                );
            } else {
                let unit = projection
                    .units
                    .iter()
                    .find(|unit| &unit.unit_id == unit_id)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: format!("unit {unit_id} missing"),
                        },
                    )?;
                work.units.push(ActivatedUnitRecord {
                    unit_id: unit_id.clone(),
                    parent_object_id: unit.parent_object_id.clone(),
                    parent_region_address: unit.parent_region_address.clone(),
                    authored_block_type: unit.authored_block_type.clone(),
                    heading_path: unit.heading_path.clone(),
                    visible_inherited_identifier_assignment_ids: vec![],
                    visible_unit_local_identifier_assignment_ids: vec![],
                    text_preview: preview(
                        &unit.content,
                        band_config(config, band).text_preview_character_limit,
                    ),
                    incoming_occurrence_count: unit.incoming_occurrence_ids.len() as u64,
                    outgoing_occurrence_count: unit.outgoing_occurrence_ids.len() as u64,
                    temporal_anchor_count: unit.temporal_anchor_ids.len() as u64,
                    available_surface_ids: unit.retrieval_surface_ids.clone(),
                    activation_provenance: provenance,
                });
            }
            let unit = projection
                .units
                .iter()
                .find(|unit| &unit.unit_id == unit_id)
                .expect("unit was just resolved");
            let ids = unit
                .inherited_identifier_assignment_ids
                .iter()
                .chain(unit.unit_local_identifier_assignment_ids.iter())
                .cloned()
                .collect::<Vec<_>>();
            insert_assignment_ids(
                projection,
                config,
                work,
                &ids,
                address,
                &structural_context_provenance,
            )?;
        }
        SemanticAddress::Identifier(_) => {
            let assignment = resolve_identifier_assignment(projection, address)
                .map_err(
                    |context| ProjectionActivationViolation::InvalidActivatedReference { context },
                )?
                .clone();
            insert_assignment_record(
                projection,
                config,
                work,
                &assignment.assignment_id,
                provenance,
            )?;
        }
        SemanticAddress::Occurrence(occurrence_id) => {
            if let Some(record) = work
                .occurrences
                .iter_mut()
                .find(|record| &record.occurrence_id == occurrence_id)
            {
                merge(&mut record.activation_provenance, &provenance);
            } else {
                let occurrence = projection
                    .occurrences
                    .iter()
                    .find(|occurrence| &occurrence.occurrence_id == occurrence_id)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: format!("occurrence {occurrence_id} missing"),
                        },
                    )?;
                let Some(resolved_target) = &occurrence.resolved_target else {
                    return Ok(());
                };
                work.occurrences.push(ActivatedOccurrenceRecord {
                    occurrence_id: occurrence_id.clone(),
                    source: occurrence.source.clone(),
                    authored_target_text: occurrence.authored_target_text.clone(),
                    display_alias: occurrence.display_alias.clone(),
                    resolved_target: resolved_target.clone(),
                    presentation_mode: occurrence.presentation_mode.clone(),
                    direction: occurrence.direction.clone(),
                    source_span: occurrence.source_span.clone(),
                    available_surface_ids: capable_surface_ids(projection, AddressKind::Occurrence),
                    activation_provenance: provenance,
                });
            }
        }
        SemanticAddress::TemporalAnchor(anchor_id) => {
            if let Some(record) = work
                .anchors
                .iter_mut()
                .find(|record| &record.anchor_id == anchor_id)
            {
                merge(&mut record.activation_provenance, &provenance);
            } else {
                let anchor = projection
                    .temporal_anchors
                    .iter()
                    .find(|anchor| &anchor.anchor_id == anchor_id)
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: format!("temporal anchor {anchor_id} missing"),
                        },
                    )?;
                work.anchors.push(ActivatedTemporalAnchorRecord {
                    anchor_id: anchor_id.clone(),
                    subject: anchor.subject.clone(),
                    value: anchor.value.clone(),
                    record_provenance: anchor.provenance.clone(),
                    available_surface_ids: capable_surface_ids(
                        projection,
                        AddressKind::TemporalAnchor,
                    ),
                    activation_provenance: provenance,
                });
            }
        }
        SemanticAddress::RetrievalSurface(_) => {}
    }
    Ok(())
}

fn insert_assignment_ids(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    assignment_ids: &[String],
    visible_subject: &SemanticAddress,
    provenance: &[ActivationProvenance],
) -> Result<(), ProjectionActivationViolation> {
    for assignment_id in assignment_ids {
        if !insert_assignment_record(projection, config, work, assignment_id, provenance.to_vec())?
        {
            break;
        }
        match visible_subject {
            SemanticAddress::Object(object_id) => {
                if let Some(record) = work
                    .objects
                    .iter_mut()
                    .find(|record| &record.object_id == object_id)
                {
                    push_unique(
                        &mut record.visible_identifier_assignment_ids,
                        assignment_id.clone(),
                    );
                }
            }
            SemanticAddress::Region(region_address) => {
                if let Some(record) = work
                    .regions
                    .iter_mut()
                    .find(|record| &record.address == region_address)
                {
                    push_unique(
                        &mut record.visible_identifier_assignment_ids,
                        assignment_id.clone(),
                    );
                }
            }
            SemanticAddress::Unit(unit_id) => {
                if let Some(unit) = projection
                    .units
                    .iter()
                    .find(|unit| &unit.unit_id == unit_id)
                    && let Some(record) = work
                        .units
                        .iter_mut()
                        .find(|record| &record.unit_id == unit_id)
                {
                    if unit
                        .inherited_identifier_assignment_ids
                        .contains(assignment_id)
                    {
                        push_unique(
                            &mut record.visible_inherited_identifier_assignment_ids,
                            assignment_id.clone(),
                        );
                    } else {
                        push_unique(
                            &mut record.visible_unit_local_identifier_assignment_ids,
                            assignment_id.clone(),
                        );
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn insert_assignment_record(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    assignment_id: &str,
    provenance: Vec<ActivationProvenance>,
) -> Result<bool, ProjectionActivationViolation> {
    if let Some(existing) = work
        .assignments
        .iter_mut()
        .find(|record| record.assignment_id == assignment_id)
    {
        merge(&mut existing.activation_provenance, &provenance);
        return Ok(true);
    }
    if work.assignments.len() >= config.maximum_activated_identifier_assignments as usize {
        return Ok(false);
    }
    let assignment = projection
        .identifier_assignments
        .iter()
        .find(|assignment| assignment.assignment_id == assignment_id)
        .ok_or_else(
            || ProjectionActivationViolation::InvalidActivatedReference {
                context: format!("assignment {assignment_id} missing"),
            },
        )?;
    let descriptor_surfaces = projection
        .identifier_descriptors
        .iter()
        .find(|descriptor| descriptor.identifier_name == assignment.identifier_name)
        .map(|descriptor| descriptor.retrieval_surface_ids.clone())
        .unwrap_or_default();
    work.assignments.push(ActivatedIdentifierAssignmentRecord {
        assignment_id: assignment.assignment_id.clone(),
        identifier_name: assignment.identifier_name.clone(),
        subject: assignment.subject.clone(),
        value: assignment.value.clone(),
        record_provenance: assignment.provenance.clone(),
        available_surface_ids: descriptor_surfaces,
        activation_provenance: provenance,
    });
    Ok(true)
}

fn refresh_visible_previews(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    work: &mut Work,
) {
    let visible_regions = work
        .regions
        .iter()
        .map(|record| record.address.clone())
        .collect::<HashSet<_>>();
    let visible_units = work
        .units
        .iter()
        .map(|record| record.unit_id.clone())
        .collect::<HashSet<_>>();
    for object in &mut work.objects {
        if let Some(projected) = projection
            .objects
            .iter()
            .find(|projected| projected.object_id == object.object_id)
        {
            let limit = band_config(config, &provenance_band(&object.activation_provenance))
                .maximum_structural_neighbors_per_record as usize;
            object.visible_region_addresses = projected
                .region_addresses
                .iter()
                .filter(|address| visible_regions.contains(*address))
                .take(limit)
                .cloned()
                .collect();
            let remaining = limit.saturating_sub(object.visible_region_addresses.len());
            object.visible_unit_ids = projected
                .unit_ids
                .iter()
                .filter(|unit_id| visible_units.contains(*unit_id))
                .take(remaining)
                .cloned()
                .collect();
        }
    }
    for region in &mut work.regions {
        if let Some(projected) = projection
            .regions
            .iter()
            .find(|projected| projected.address == region.address)
        {
            let limit = band_config(config, &provenance_band(&region.activation_provenance))
                .maximum_visible_units_per_region as usize;
            region.visible_unit_ids = projected
                .contained_unit_ids
                .iter()
                .filter(|unit_id| visible_units.contains(*unit_id))
                .take(limit)
                .cloned()
                .collect();
        }
    }
}

fn add_visible_context_edges(
    projection: &SemanticSpaceProjection,
    config: &ProjectionActivationConfig,
    access: &ProjectionActivationAccess<'_>,
    work: &mut Work,
) -> Result<(), ProjectionActivationViolation> {
    for graph_edge in &access.artifacts.graph.edges {
        let edge = match graph_edge.direction {
            Direction::Outgoing => EdgeTuple {
                source: graph_edge.source.clone(),
                transition_id: graph_edge.transition_id.clone(),
                direction: Direction::Outgoing,
                target: graph_edge.target.clone(),
            },
            Direction::Incoming => EdgeTuple {
                source: graph_edge.source.clone(),
                transition_id: graph_edge.transition_id.clone(),
                direction: Direction::Incoming,
                target: graph_edge.target.clone(),
            },
        };
        if edge.source.kind() == AddressKind::RetrievalSurface
            || edge.target.kind() == AddressKind::RetrievalSurface
        {
            continue;
        }
        if visible(projection, work, &edge.source) && visible(projection, work, &edge.target) {
            let Some(provenance) =
                edge_context_provenance(projection, work, &edge.source, &edge.target)
            else {
                continue;
            };
            add_edge(config, work, edge, provenance, "context-edge")?;
        }
    }
    Ok(())
}

fn edge_context_provenance(
    projection: &SemanticSpaceProjection,
    work: &Work,
    source: &SemanticAddress,
    target: &SemanticAddress,
) -> Option<Vec<ActivationProvenance>> {
    let mut provenance = Vec::new();
    if let Some(source_provenance) = activation_provenance_for_address(projection, work, source) {
        merge(&mut provenance, source_provenance);
    }
    if let Some(target_provenance) = activation_provenance_for_address(projection, work, target) {
        merge(&mut provenance, target_provenance);
    }
    if provenance.is_empty() {
        None
    } else {
        Some(with_default(provenance, "bounded_structural_context"))
    }
}

fn activation_provenance_for_address<'a>(
    projection: &SemanticSpaceProjection,
    work: &'a Work,
    address: &SemanticAddress,
) -> Option<&'a [ActivationProvenance]> {
    match address {
        SemanticAddress::Object(id) => work
            .objects
            .iter()
            .find(|record| &record.object_id == id)
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::Region(id) => work
            .regions
            .iter()
            .find(|record| &record.address == id)
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::Unit(id) => work
            .units
            .iter()
            .find(|record| &record.unit_id == id)
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::Occurrence(id) => work
            .occurrences
            .iter()
            .find(|record| &record.occurrence_id == id)
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::TemporalAnchor(id) => work
            .anchors
            .iter()
            .find(|record| &record.anchor_id == id)
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::Identifier(_) => resolve_identifier_assignment(projection, address)
            .ok()
            .and_then(|assignment| {
                work.assignments
                    .iter()
                    .find(|record| record.assignment_id == assignment.assignment_id)
            })
            .map(|record| record.activation_provenance.as_slice()),
        SemanticAddress::RetrievalSurface(_) => None,
    }
}

fn add_edge(
    config: &ProjectionActivationConfig,
    work: &mut Work,
    edge: EdgeTuple,
    provenance: Vec<ActivationProvenance>,
    probe_id: &str,
) -> Result<(), ProjectionActivationViolation> {
    if let Some(existing) = work.edges.iter_mut().find(|record| {
        record.source == edge.source
            && record.transition_id == edge.transition_id
            && record.direction == edge.direction
            && record.target == edge.target
    }) {
        merge(&mut existing.activation_provenance, &provenance);
        return Ok(());
    }
    if work.edges.len() >= config.maximum_activated_edges as usize {
        work.bounded_probe_ids.insert(probe_id.into());
        return Ok(());
    }
    let edge_id = next_id("activated-edge", &mut work.edge_counter)?;
    work.edges.push(ActivatedEdge {
        edge_id,
        source: edge.source,
        transition_id: edge.transition_id,
        direction: edge.direction,
        target: edge.target,
        activation_provenance: provenance,
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn add_surface_continuation(
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    work: &mut Work,
    probe_id: &str,
    result: &ProjectionAccessProbeResult,
    origin: &ProbeOrigin,
    surface: &RetrievalSurfaceDescriptor,
    match_mode: SurfaceMatchMode,
    provenance: &[ActivationProvenance],
    candidate_limit: u32,
) -> Result<bool, ProjectionActivationViolation> {
    let Some(continuation) = &result.continuation else {
        return Ok(false);
    };
    if config.continuation_page_limit == 0
        || work.handles.len() >= config.maximum_continuation_handles as usize
    {
        work.bounded_probe_ids.insert(probe_id.into());
        return Ok(false);
    }
    let Some(next_offset) = continuation_offset(&continuation.cursor) else {
        return Err(ProjectionActivationViolation::InvalidContinuationHandle {
            handle_id: probe_id.into(),
            context: "access continuation cursor did not expose an offset".into(),
        });
    };
    let remaining_count = result
        .total_candidate_count
        .and_then(|total| total.checked_sub(next_offset as usize))
        .map(|remaining| remaining as u64);
    let handle_id = next_id("activation-continuation", &mut work.handle_counter)?;
    let origin = match origin {
        ProbeOrigin::Text(text) => ContinuationOrigin::TextProbe {
            query_text: text.clone(),
            match_mode,
        },
        ProbeOrigin::Structural { subject, direction } => {
            ContinuationOrigin::StructuralNeighbourhood {
                subject: subject.clone(),
                transition_id: None,
                direction: Some(direction.clone()),
            }
        }
        ProbeOrigin::Temporal { query } => {
            let (start, end) = temporal_query_bounds(query);
            ContinuationOrigin::TemporalProbe { start, end }
        }
    };
    work.handles.push(ContinuationHandle {
        handle_id,
        projection_snapshot_id: result.projection_snapshot_id.clone(),
        configuration_snapshot_id: config.configuration_snapshot_id.clone(),
        problem_space_thread_id: problem_space.thread_id.clone(),
        problem_space_version: problem_space.version,
        newest_utterance_id: utterance.utterance_id.clone(),
        origin,
        access: ContinuationAccess::RetrievalSurface {
            surface_id: surface.surface_id.clone(),
            surface_kind: surface.kind.clone(),
        },
        filters: vec![],
        ordering: ContinuationOrdering::SurfaceDeclared {
            ordering_key: result.index_identity.clone(),
        },
        next_offset,
        remaining_count,
        next_page_limit: config.continuation_page_limit.min(candidate_limit),
        activation_provenance: provenance.to_vec(),
    });
    Ok(true)
}

fn continuation_offset(cursor: &str) -> Option<u64> {
    cursor
        .split(';')
        .find_map(|part| part.strip_prefix("offset="))
        .and_then(|value| value.parse().ok())
}

fn access_candidate_count(
    result: &ProjectionAccessProbeResult,
) -> Result<CandidateCount, ProjectionActivationViolation> {
    match result.total_candidate_count {
        Some(total) => Ok(CandidateCount::Exact(
            u64::try_from(total).map_err(|_| ProjectionActivationViolation::CountOverflow)?,
        )),
        None => Ok(CandidateCount::Estimated(
            u64::try_from(result.returned_count)
                .map_err(|_| ProjectionActivationViolation::CountOverflow)?,
        )),
    }
}

fn validate_probe_result(
    projection: &SemanticSpaceProjection,
    access: &ProjectionActivationAccess<'_>,
    surface: &RetrievalSurfaceDescriptor,
    probe: &ProjectionAccessProbe,
    result: &ProjectionAccessProbeResult,
) -> Result<(), ProjectionActivationViolation> {
    let fail = |context: String| ProjectionActivationViolation::SurfaceAccessFailed {
        surface_id: surface.surface_id.clone(),
        probe_id: probe.probe_id.clone(),
        context,
    };
    if result.probe_id != probe.probe_id {
        return Err(fail(
            "access result probe identity does not match request".into(),
        ));
    }
    if result.projection_snapshot_id != probe.projection_snapshot_id {
        return Err(fail(
            "access result projection snapshot does not match request".into(),
        ));
    }
    if result.surface_id != probe.surface_id
        || result.surface_kind != probe.surface_kind
        || result.match_mode != probe.match_mode
    {
        return Err(fail(
            "access result surface identity does not match request".into(),
        ));
    }
    if result.returned_count != result.candidates.len() {
        return Err(fail(
            "access result returned count does not match candidates".into(),
        ));
    }
    if result.candidates.len() > probe.page_size {
        return Err(fail(
            "returned candidate count exceeds probe page size".into(),
        ));
    }
    if result
        .total_candidate_count
        .is_some_and(|total| total < result.candidates.len())
    {
        return Err(fail(
            "access total candidate count is smaller than returned page".into(),
        ));
    }
    if result.truncated != result.continuation.is_some() {
        return Err(fail(
            "access truncation and continuation facts disagree".into(),
        ));
    }
    if let Some(continuation) = &result.continuation {
        if !surface.continuation_supported {
            return Err(fail("continuation returned for unsupported surface".into()));
        }
        let Some(offset) = continuation_offset(&continuation.cursor) else {
            return Err(fail(
                "access continuation cursor did not expose an offset".into(),
            ));
        };
        if offset != result.returned_count as u64 {
            return Err(fail(
                "initial continuation offset does not match returned page".into(),
            ));
        }
        if result
            .total_candidate_count
            .is_some_and(|total| offset > total as u64)
        {
            return Err(fail(
                "continuation offset exceeds exact candidate count".into(),
            ));
        }
    }
    for candidate in &result.candidates {
        if !matches!(
            surface.kind,
            RetrievalSurfaceKind::Graph | RetrievalSurfaceKind::Temporal
        ) && candidate.identity.kind() != surface.returned_identity
        {
            return Err(fail(
                "access candidate identity kind does not match surface".into(),
            ));
        }
        validate_candidate_address_exists(projection, &candidate.identity).map_err(&fail)?;
        if matches!(probe.match_mode, SurfaceMatchMode::Incidence) {
            let Some(transition_id) = &candidate.transition_id else {
                return Err(fail(
                    "incidence candidate has no transition identity".into(),
                ));
            };
            let AccessOperand::Graph {
                seed, direction, ..
            } = &probe.operand
            else {
                return Err(fail("incidence probe has a non-graph operand".into()));
            };
            let edge_exists = access.artifacts.graph.edges.iter().any(|edge| {
                edge.direction == *direction
                    && edge.transition_id == *transition_id
                    && edge.source == *seed
                    && edge.target == candidate.identity
            });
            if !edge_exists {
                return Err(fail(
                    "incidence candidate is not a represented graph edge".into(),
                ));
            }
        } else if candidate.transition_id.is_some() {
            return Err(fail(
                "non-incidence candidate carries a transition identity".into(),
            ));
        }
    }
    Ok(())
}

fn structural_degree(access: &ProjectionActivationAccess<'_>, subject: &SemanticAddress) -> u64 {
    access
        .artifacts
        .graph
        .edges
        .iter()
        .filter(|edge| edge.source == *subject || edge.target == *subject)
        .count() as u64
}

fn identifier_type_distribution(
    projection: &SemanticSpaceProjection,
    candidates: &[crate::access::AccessCandidate],
) -> Vec<CountByLabel> {
    let mut distribution = Vec::new();
    for candidate in candidates {
        let label = match &candidate.identity {
            SemanticAddress::Object(object_id) => projection
                .objects
                .iter()
                .find(|object| &object.object_id == object_id)
                .map(|object| format!("semantic_object:{}", object.object_class))
                .unwrap_or_else(|| "semantic_object".into()),
            SemanticAddress::Region(_) => "semantic_region".into(),
            SemanticAddress::Unit(_) => "semantic_unit".into(),
            SemanticAddress::Identifier(identifier) => {
                format!("identifier:{}", identifier.identifier_name)
            }
            SemanticAddress::Occurrence(_) => "occurrence".into(),
            SemanticAddress::TemporalAnchor(_) => "temporal_anchor".into(),
            SemanticAddress::RetrievalSurface(_) => "retrieval_surface".into(),
        };
        if let Some(existing) = distribution
            .iter_mut()
            .find(|entry: &&mut CountByLabel| entry.label == label)
        {
            existing.count += 1;
        } else {
            distribution.push(CountByLabel { label, count: 1 });
        }
    }
    distribution
}

fn unresolved_target_count(
    projection: &SemanticSpaceProjection,
    candidates: &[crate::access::AccessCandidate],
) -> u64 {
    projection
        .occurrences
        .iter()
        .filter(|occurrence| {
            occurrence.resolved_target.is_none()
                && candidates.iter().any(|candidate| {
                    candidate.identity
                        == SemanticAddress::Occurrence(occurrence.occurrence_id.clone())
                })
        })
        .count() as u64
}

fn reserve_telemetry(
    config: &ProjectionActivationConfig,
    work: &mut Work,
) -> Result<Option<String>, ProjectionActivationViolation> {
    if config.maximum_telemetry_records == 0 {
        return Ok(None);
    }
    let next = u64::try_from(work.telemetry.len())
        .map_err(|_| ProjectionActivationViolation::CountOverflow)?
        .checked_add(1)
        .ok_or(ProjectionActivationViolation::CountOverflow)?;
    if next > u64::from(config.maximum_telemetry_records) {
        return Err(ProjectionActivationViolation::ActivatedViewBoundExceeded {
            kind: ActivatedRecordKind::Telemetry,
            actual: next,
            maximum: config.maximum_telemetry_records,
        });
    }
    next_id("activation-telemetry", &mut work.telemetry_counter).map(Some)
}

fn mark_bounded_telemetry(work: &mut Work) {
    for telemetry in &mut work.telemetry {
        if work.bounded_probe_ids.contains(&telemetry.probe_id) {
            telemetry.truncation_state = TruncationState::Bounded;
        }
    }
}

fn band_config<'a>(
    config: &'a ProjectionActivationConfig,
    band: &ActivationProbeBand,
) -> &'a crate::activation::ProjectionActivationBandConfig {
    match band {
        ActivationProbeBand::Unbanded => &config.unbanded,
        ActivationProbeBand::Attention(ActivationBand::Primary) => &config.primary,
        ActivationProbeBand::Attention(ActivationBand::Secondary) => &config.secondary,
        ActivationProbeBand::Attention(ActivationBand::Tertiary) => &config.tertiary,
        ActivationProbeBand::Attention(ActivationBand::Background) => &config.background,
    }
}

fn provenance_band(provenance: &[ActivationProvenance]) -> ActivationProbeBand {
    provenance
        .iter()
        .find_map(|entry| match entry {
            ActivationProvenance::AttentionBand { band, .. } => {
                Some(ActivationProbeBand::Attention(band.clone()))
            }
            _ => None,
        })
        .unwrap_or(ActivationProbeBand::Unbanded)
}

fn surface_limit(
    config: &ProjectionActivationConfig,
    surface_id: &str,
    band: &ActivationProbeBand,
) -> u32 {
    let surface = config
        .surface_limits
        .iter()
        .find(|surface| surface.surface_id == surface_id)
        .expect("surface config preflight validated");
    match band {
        ActivationProbeBand::Unbanded => surface.unbanded_candidate_limit,
        ActivationProbeBand::Attention(ActivationBand::Primary) => surface.primary_candidate_limit,
        ActivationProbeBand::Attention(ActivationBand::Secondary) => {
            surface.secondary_candidate_limit
        }
        ActivationProbeBand::Attention(ActivationBand::Tertiary) => {
            surface.tertiary_candidate_limit
        }
        ActivationProbeBand::Attention(ActivationBand::Background) => {
            surface.background_candidate_limit
        }
    }
}

fn lexical_terms(text: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut token = String::new();
    for character in text.chars() {
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

fn temporal_queries_for_address(
    projection: &SemanticSpaceProjection,
    address: &SemanticAddress,
) -> Vec<TemporalQuery> {
    projection
        .temporal_anchors
        .iter()
        .filter(|anchor| match address {
            SemanticAddress::TemporalAnchor(anchor_id) => &anchor.anchor_id == anchor_id,
            SemanticAddress::Object(_) | SemanticAddress::Unit(_) => &anchor.subject == address,
            _ => false,
        })
        .map(|anchor| {
            let (precision, value) = temporal_value(&anchor.value);
            TemporalQuery::Exact { precision, value }
        })
        .collect()
}

fn temporal_query_bounds(query: &TemporalQuery) -> (Option<TemporalValue>, Option<TemporalValue>) {
    match query {
        TemporalQuery::Exact { precision, value } => {
            let value = temporal_value_from_parts(precision, value);
            (Some(value.clone()), Some(value))
        }
        TemporalQuery::Range {
            precision,
            start,
            end,
        } => (
            start
                .as_ref()
                .map(|value| temporal_value_from_parts(precision, value)),
            end.as_ref()
                .map(|value| temporal_value_from_parts(precision, value)),
        ),
        TemporalQuery::Ordered { .. } => (None, None),
    }
}

fn temporal_value_from_parts(precision: &TemporalPrecision, value: &str) -> TemporalValue {
    match precision {
        TemporalPrecision::FullDate => TemporalValue::FullDate(value.into()),
        TemporalPrecision::DateTime => TemporalValue::DateTime(value.into()),
        TemporalPrecision::ExactYear => TemporalValue::ExactYear(value.parse().unwrap_or_default()),
        TemporalPrecision::MonthDay => TemporalValue::MonthDay(value.into()),
        TemporalPrecision::ApproximateYear => TemporalValue::ApproximateYear(value.into()),
    }
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

fn preview(content: &SemanticUnitContent, limit: u32) -> ActivatedTextPreview {
    match content {
        SemanticUnitContent::Inline {
            normalized_text, ..
        } => {
            let text = normalized_text
                .chars()
                .take(limit as usize)
                .collect::<String>();
            let truncated = normalized_text.chars().count() > limit as usize;
            ActivatedTextPreview::Inline { text, truncated }
        }
        SemanticUnitContent::HydrationAddress { .. } => {
            ActivatedTextPreview::UnavailableWithoutHydration
        }
    }
}

fn larger_preview(
    current: ActivatedTextPreview,
    content: &SemanticUnitContent,
    limit: u32,
) -> ActivatedTextPreview {
    let candidate = preview(content, limit);
    match (&current, &candidate) {
        (
            ActivatedTextPreview::Inline { text: left, .. },
            ActivatedTextPreview::Inline { text: right, .. },
        ) if left.chars().count() >= right.chars().count() => current,
        _ => candidate,
    }
}

fn capable_surface_ids(projection: &SemanticSpaceProjection, kind: AddressKind) -> Vec<String> {
    projection
        .retrieval_surfaces
        .iter()
        .filter(|surface| surface.visible_address_kinds.contains(&kind))
        .map(|surface| surface.surface_id.clone())
        .collect()
}

fn surface_applies_to_address(
    projection: &SemanticSpaceProjection,
    surface: &RetrievalSurfaceDescriptor,
    address: &SemanticAddress,
) -> bool {
    if !surface.visible_address_kinds.contains(&address.kind()) {
        return false;
    }
    match address {
        SemanticAddress::Object(object_id) => projection
            .objects
            .iter()
            .find(|object| &object.object_id == object_id)
            .is_some_and(|object| object.retrieval_surface_ids.contains(&surface.surface_id)),
        SemanticAddress::Region(region_address) => projection
            .regions
            .iter()
            .find(|region| &region.address == region_address)
            .is_some_and(|region| region.retrieval_surface_ids.contains(&surface.surface_id)),
        SemanticAddress::Unit(unit_id) => projection
            .units
            .iter()
            .find(|unit| &unit.unit_id == unit_id)
            .is_some_and(|unit| unit.retrieval_surface_ids.contains(&surface.surface_id)),
        SemanticAddress::Identifier(identifier) => projection
            .identifier_descriptors
            .iter()
            .find(|descriptor| descriptor.identifier_name == identifier.identifier_name)
            .is_some_and(|descriptor| {
                descriptor
                    .retrieval_surface_ids
                    .contains(&surface.surface_id)
            }),
        SemanticAddress::Occurrence(_) | SemanticAddress::TemporalAnchor(_) => true,
        SemanticAddress::RetrievalSurface(_) => false,
    }
}

fn visible(projection: &SemanticSpaceProjection, work: &Work, address: &SemanticAddress) -> bool {
    match address {
        SemanticAddress::Object(id) => work.objects.iter().any(|record| &record.object_id == id),
        SemanticAddress::Region(id) => work.regions.iter().any(|record| &record.address == id),
        SemanticAddress::Unit(id) => work.units.iter().any(|record| &record.unit_id == id),
        SemanticAddress::Occurrence(id) => work
            .occurrences
            .iter()
            .any(|record| &record.occurrence_id == id),
        SemanticAddress::TemporalAnchor(id) => {
            work.anchors.iter().any(|record| &record.anchor_id == id)
        }
        SemanticAddress::Identifier(_) => resolve_identifier_assignment(projection, address)
            .is_ok_and(|assignment| {
                work.assignments
                    .iter()
                    .any(|record| record.assignment_id == assignment.assignment_id)
            }),
        SemanticAddress::RetrievalSurface(_) => false,
    }
}

fn validate_candidate_address_exists(
    projection: &SemanticSpaceProjection,
    address: &SemanticAddress,
) -> Result<(), String> {
    match address {
        SemanticAddress::Object(id) if projection.objects.iter().any(|r| &r.object_id == id) => {
            Ok(())
        }
        SemanticAddress::Region(id) if projection.regions.iter().any(|r| &r.address == id) => {
            Ok(())
        }
        SemanticAddress::Unit(id) if projection.units.iter().any(|r| &r.unit_id == id) => Ok(()),
        SemanticAddress::Occurrence(id)
            if projection
                .occurrences
                .iter()
                .any(|r| &r.occurrence_id == id) =>
        {
            Ok(())
        }
        SemanticAddress::TemporalAnchor(id)
            if projection
                .temporal_anchors
                .iter()
                .any(|r| &r.anchor_id == id) =>
        {
            Ok(())
        }
        SemanticAddress::Identifier(_) => {
            resolve_identifier_assignment(projection, address).map(|_| ())
        }
        SemanticAddress::RetrievalSurface(_) => {
            Err("retrieval surface candidates are not activated records".into())
        }
        _ => Err("candidate address does not exist in projection".into()),
    }
}

fn resolve_identifier_assignment<'a>(
    projection: &'a SemanticSpaceProjection,
    address: &SemanticAddress,
) -> Result<&'a crate::projection::IdentifierAssignment, String> {
    let SemanticAddress::Identifier(identifier) = address else {
        return Err("address is not an identifier".into());
    };
    let mut matches = projection
        .identifier_assignments
        .iter()
        .filter(|assignment| {
            assignment.identifier_name == identifier.identifier_name
                && identifier
                    .represented_value
                    .as_ref()
                    .is_none_or(|represented| {
                        identifier_value_matches(&assignment.value, represented)
                    })
        });
    let Some(first) = matches.next() else {
        return Err("identifier address resolved zero assignments".into());
    };
    if matches.next().is_some() {
        return Err("identifier address resolved multiple assignments".into());
    }
    Ok(first)
}

fn identifier_value_matches(value: &IdentifierValue, represented: &str) -> bool {
    match value {
        IdentifierValue::Null => represented == "null",
        IdentifierValue::String(value) => value == represented,
        IdentifierValue::Integer(value) => value.to_string() == represented,
        IdentifierValue::Boolean(value) => value.to_string() == represented,
        IdentifierValue::SemanticAddress(value) => address_key(value) == represented,
        IdentifierValue::Strings(values) => values.len() == 1 && values[0] == represented,
        IdentifierValue::Integers(values) => {
            values.len() == 1 && values[0].to_string() == represented
        }
        IdentifierValue::Booleans(values) => {
            values.len() == 1 && values[0].to_string() == represented
        }
        IdentifierValue::SemanticAddresses(values) => {
            values.len() == 1 && address_key(&values[0]) == represented
        }
        IdentifierValue::Values(values) => {
            values.len() == 1 && identifier_value_matches(&values[0], represented)
        }
    }
}

fn address_key(address: &SemanticAddress) -> String {
    match address {
        SemanticAddress::Object(id) => format!("object:{id}"),
        SemanticAddress::Unit(id) => format!("unit:{id}"),
        SemanticAddress::Region(address) => format!(
            "region:{}#{}",
            address.object_id, address.authored_structural_address
        ),
        SemanticAddress::Identifier(identifier) => format!(
            "identifier:{}={}",
            identifier.identifier_name,
            identifier.represented_value.clone().unwrap_or_default()
        ),
        SemanticAddress::Occurrence(id) => format!("occurrence:{id}"),
        SemanticAddress::TemporalAnchor(id) => format!("temporal:{id}"),
        SemanticAddress::RetrievalSurface(surface) => format!("surface:{}", surface.surface_id),
    }
}

fn append_relation_provenance(
    provenance: &mut Vec<ActivationProvenance>,
    problem_space: &ProblemSpaceState,
    region_id: Option<&String>,
) {
    let Some(region_id) = region_id else {
        return;
    };
    for relation in problem_space.relations.iter().filter(|relation| {
        relation.lifecycle == RecordLifecycle::Active
            && (relation.source_region_id == *region_id
                || relation.target_region_id.as_ref() == Some(region_id))
    }) {
        push_unique(
            provenance,
            ActivationProvenance::ProblemRelation {
                relation_id: relation.relation_id.clone(),
            },
        );
    }
}

fn with_default(
    mut provenance: Vec<ActivationProvenance>,
    configuration_key: &str,
) -> Vec<ActivationProvenance> {
    push_unique(
        &mut provenance,
        ActivationProvenance::ConfiguredDefault {
            configuration_key: configuration_key.into(),
        },
    );
    provenance
}

fn next_id(prefix: &str, counter: &mut u64) -> Result<String, ProjectionActivationViolation> {
    let id = format!("{prefix}:{}", *counter);
    *counter = counter
        .checked_add(1)
        .ok_or(ProjectionActivationViolation::CountOverflow)?;
    Ok(id)
}

fn push_unique<T: PartialEq>(items: &mut Vec<T>, item: T) {
    if !items.contains(&item) {
        items.push(item);
    }
}

fn merge<T: Clone + PartialEq>(items: &mut Vec<T>, additional: &[T]) {
    for item in additional {
        push_unique(items, item.clone());
    }
}

impl fmt::Debug for ProjectionActivationAccess<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProjectionActivationAccess")
            .field("artifact_identity", &self.artifacts.artifact_identity)
            .field(
                "has_query_embedding_provider",
                &self.query_embedding_provider.is_some(),
            )
            .finish()
    }
}
