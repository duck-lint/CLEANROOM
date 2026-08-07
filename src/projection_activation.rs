//! Deterministic initial projection activation runtime.
#![allow(clippy::collapsible_if)]
//!
//! This module executes the frozen activation contracts against one validated
//! [`SemanticSpaceProjection`]. It deliberately contains no retrieval provider:
//! callers supply a synchronous, read-only [`ProjectionActivationAccess`] seam
//! that receives typed probes and returns only mechanical projected identities.

use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt,
};

use crate::{
    activation::{
        ActivatedEdge, ActivatedIdentifierAssignmentRecord, ActivatedObjectRecord,
        ActivatedOccurrenceRecord, ActivatedProjection, ActivatedRecordKind, ActivatedRegionRecord,
        ActivatedTemporalAnchorRecord, ActivatedTextPreview, ActivatedUnitRecord,
        ActivationProvenance, ActivationUtterance, CandidateCount, ContinuationAccess,
        ContinuationFilter, ContinuationHandle, ContinuationOrdering, ContinuationOrigin,
        ProjectionActivationConfig, ProjectionActivationViolation, ProjectionTelemetry,
        TruncationState,
    },
    model::{
        AddressKind, Direction, RetrievalSurfaceAddress, RetrievalSurfaceKind, SemanticAddress,
    },
    problem_space::{
        ActivationBand, OpenTension, ProblemConstraintApplicability, ProblemSpaceState,
        RecordLifecycle, RegionPersistenceState, TensionLifecycle,
    },
    projection::{
        IdentifierValue, OccurrenceSource, ProjectionValidationStatus, SemanticSpaceProjection,
        SemanticUnitContent, StructuralTransition, StructuralTransitionOperation, SurfaceMatchMode,
    },
};

/// Runtime probe band. This type is intentionally not part of the JSON schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionActivationProbeBand {
    Unbanded,
    Attention(ActivationBand),
}

/// Runtime source family for a probe or declared mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionActivationProbeSourceKind {
    Text,
    Address,
    Temporal,
}

/// Runtime source supplied to the activation-access seam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectionActivationProbeSource {
    Text { text: String },
    Address { address: SemanticAddress },
    Temporal { address: SemanticAddress },
}

/// Deterministic activation probe issued to a read-only access seam.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationProbe {
    pub probe_id: String,
    pub band: ProjectionActivationProbeBand,
    pub surface_id: String,
    pub surface_kind: RetrievalSurfaceKind,
    pub match_mode: SurfaceMatchMode,
    pub source: ProjectionActivationProbeSource,
    pub candidate_limit: u32,
    pub current_depth: u32,
    pub activation_provenance: Vec<ActivationProvenance>,
}

/// Transition attached to an incidence candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationCandidateTransition {
    pub transition_id: String,
    pub direction: Direction,
}

/// Candidate returned by deterministic access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationCandidate {
    pub address: SemanticAddress,
    pub transition: Option<ProjectionActivationCandidateTransition>,
}

/// Mechanical continuation fact returned by deterministic access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationProbeContinuation {
    pub next_offset: u64,
    pub remaining_count: Option<u64>,
    pub ordering_key: String,
}

/// Probe result returned by deterministic access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationProbeResult {
    pub candidates: Vec<ProjectionActivationCandidate>,
    pub candidate_count: CandidateCount,
    pub continuation: Option<ProjectionActivationProbeContinuation>,
    pub identifier_type_distribution: Vec<crate::activation::CountByLabel>,
    pub temporal_anchor_count: u64,
    pub unresolved_target_count: u64,
}

/// Mechanical access failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionActivationAccessFailure {
    pub context: String,
}

/// Synchronous, read-only seam between activation and deterministic projected access.
pub trait ProjectionActivationAccess {
    fn execute_probe(
        &self,
        projection: &SemanticSpaceProjection,
        probe: &ProjectionActivationProbe,
    ) -> Result<ProjectionActivationProbeResult, ProjectionActivationAccessFailure>;

    fn declared_mode_source(
        &self,
        _surface_id: &str,
        _mode_name: &str,
    ) -> Option<ProjectionActivationProbeSourceKind> {
        None
    }
}

#[derive(Clone)]
struct Seed {
    text: String,
    band: ProjectionActivationProbeBand,
    provenance: Vec<ActivationProvenance>,
    region_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EdgeTuple {
    source: SemanticAddress,
    transition_id: String,
    direction: Direction,
    target: SemanticAddress,
}

#[derive(Clone)]
struct ExposureContext {
    probe_id: String,
    activation_provenance: Vec<ActivationProvenance>,
}

#[derive(Clone)]
struct ExposureAggregate {
    probe_ids: Vec<String>,
    activation_provenance: Vec<ActivationProvenance>,
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
    order: Vec<SemanticAddress>,
    bounded_probe_ids: HashSet<String>,
    edge_provenance: Vec<(EdgeTuple, ExposureAggregate)>,
    address_exposure: Vec<(SemanticAddress, ExposureAggregate)>,
    probe_counter: u64,
    telemetry_counter: u64,
    continuation_counter: u64,
    edge_counter: u64,
}

pub fn activate_projection<A>(
    projection: &SemanticSpaceProjection,
    problem_space: &ProblemSpaceState,
    utterance: &ActivationUtterance,
    config: &ProjectionActivationConfig,
    access: &A,
) -> Result<ActivatedProjection, ProjectionActivationViolation>
where
    A: ProjectionActivationAccess + ?Sized,
{
    preflight(projection, problem_space, utterance, config)?;
    let seeds = build_seeds(problem_space, utterance, config)?;
    let mut work = Work::default();
    for seed in seeds {
        dispatch_root(
            projection,
            problem_space,
            utterance,
            config,
            access,
            &mut work,
            seed,
        )?;
    }
    build_visible_edges_and_structure_handles(
        projection,
        problem_space,
        utterance,
        config,
        &mut work,
    )?;
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
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
) -> Result<(), ProjectionActivationViolation> {
    for (field, value) in [
        (
            "projection.projection_snapshot_id",
            p.projection_snapshot_id.as_str(),
        ),
        (
            "projection.configuration_snapshot_id",
            p.configuration_snapshot_id.as_str(),
        ),
        ("problem_space.thread_id", ps.thread_id.as_str()),
        ("utterance.utterance_id", u.utterance_id.as_str()),
        (
            "config.configuration_snapshot_id",
            c.configuration_snapshot_id.as_str(),
        ),
    ] {
        if value.trim().is_empty() {
            return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                field: field.into(),
            });
        }
    }
    if p.validation_status != ProjectionValidationStatus::Validated {
        return Err(ProjectionActivationViolation::ProjectionNotValidated {
            status: p.validation_status.clone(),
        });
    }
    if p.configuration_snapshot_id != c.configuration_snapshot_id {
        return Err(
            ProjectionActivationViolation::ConfigurationSnapshotMismatch {
                projection_configuration_snapshot_id: p.configuration_snapshot_id.clone(),
                activation_configuration_snapshot_id: c.configuration_snapshot_id.clone(),
            },
        );
    }
    let mut seen = HashSet::new();
    for sc in &c.surface_limits {
        if sc.surface_id.trim().is_empty() {
            return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                field: "config.surface_limits.surface_id".into(),
            });
        }
        if !seen.insert(sc.surface_id.clone()) {
            return Err(
                ProjectionActivationViolation::DuplicateSurfaceConfiguration {
                    surface_id: sc.surface_id.clone(),
                },
            );
        }
        let d = p
            .retrieval_surfaces
            .iter()
            .find(|s| s.surface_id == sc.surface_id)
            .ok_or_else(|| {
                ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
                    surface_id: sc.surface_id.clone(),
                }
            })?;
        if !d.available {
            return Err(
                ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
                    surface_id: sc.surface_id.clone(),
                },
            );
        }
        let max = [
            sc.unbanded_candidate_limit,
            sc.primary_candidate_limit,
            sc.secondary_candidate_limit,
            sc.tertiary_candidate_limit,
            sc.background_candidate_limit,
        ]
        .into_iter()
        .max()
        .unwrap_or(0);
        if max > d.hard_candidate_limit {
            return Err(
                ProjectionActivationViolation::SurfaceCandidateLimitExceedsHardLimit {
                    surface_id: sc.surface_id.clone(),
                    requested: max,
                    hard_maximum: d.hard_candidate_limit,
                },
            );
        }
    }
    for s in &p.retrieval_surfaces {
        if s.available {
            if s.surface_id.trim().is_empty() {
                return Err(ProjectionActivationViolation::EmptyRequiredIdentity {
                    field: "projection.retrieval_surfaces.surface_id".into(),
                });
            }
            if !seen.contains(&s.surface_id) {
                return Err(
                    ProjectionActivationViolation::MissingAvailableSurfaceConfiguration {
                        surface_id: s.surface_id.clone(),
                    },
                );
            }
        } else if seen.contains(&s.surface_id) {
            return Err(
                ProjectionActivationViolation::UnknownOrUnavailableSurfaceConfiguration {
                    surface_id: s.surface_id.clone(),
                },
            );
        }
    }
    validate_lens(ps)
}

fn validate_lens(ps: &ProblemSpaceState) -> Result<(), ProjectionActivationViolation> {
    let mut lens = Vec::<(&String, ActivationBand)>::new();
    for id in &ps.attention_lens.primary_region_ids {
        lens.push((id, ActivationBand::Primary));
    }
    for id in &ps.attention_lens.secondary_region_ids {
        lens.push((id, ActivationBand::Secondary));
    }
    for id in &ps.attention_lens.tertiary_region_ids {
        lens.push((id, ActivationBand::Tertiary));
    }
    for id in &ps.attention_lens.background_region_ids {
        lens.push((id, ActivationBand::Background));
    }
    let mut seen = HashSet::new();
    for (id, band) in lens {
        if !seen.insert(id.clone()) {
            return invalid_ref(format!("attention_lens duplicates region {id}"));
        }
        let r = ps
            .regions
            .iter()
            .find(|r| &r.region_id == id)
            .ok_or_else(
                || ProjectionActivationViolation::InvalidActivatedReference {
                    context: format!("attention_lens references unknown region {id}"),
                },
            )?;
        if !operational(&r.persistence_state) {
            return invalid_ref(format!(
                "attention_lens references nonoperational region {id}"
            ));
        }
        if r.activation_band != band {
            return invalid_ref(format!("attention_lens band mismatch for region {id}"));
        }
    }
    for r in &ps.regions {
        if operational(&r.persistence_state) && !seen.contains(&r.region_id) {
            return invalid_ref(format!(
                "operational region {} missing from attention_lens",
                r.region_id
            ));
        }
    }
    for c in &ps.constraints {
        if c.lifecycle == RecordLifecycle::Active {
            if let ProblemConstraintApplicability::Regions { region_ids } = &c.applicability {
                for id in region_ids {
                    let r = ps
                        .regions
                        .iter()
                        .find(|r| &r.region_id == id)
                        .ok_or_else(|| {
                            ProjectionActivationViolation::InvalidActivatedReference {
                                context: format!(
                                    "active constraint {} references unknown region {id}",
                                    c.constraint_id
                                ),
                            }
                        })?;
                    if !operational(&r.persistence_state) {
                        return invalid_ref(format!(
                            "active constraint {} references nonoperational region {id}",
                            c.constraint_id
                        ));
                    }
                }
            }
        }
    }
    for rel in &ps.relations {
        if rel.lifecycle == RecordLifecycle::Active {
            if !ps
                .regions
                .iter()
                .any(|r| r.region_id == rel.source_region_id)
            {
                return invalid_ref(format!(
                    "active relation {} references unknown source region {}",
                    rel.relation_id, rel.source_region_id
                ));
            }
            if let Some(t) = &rel.target_region_id {
                if !ps.regions.iter().any(|r| &r.region_id == t) {
                    return invalid_ref(format!(
                        "active relation {} references unknown target region {t}",
                        rel.relation_id
                    ));
                }
            }
        }
    }
    for t in &ps.open_tensions {
        if t.lifecycle == TensionLifecycle::Open
            && !ps.regions.iter().any(|r| r.region_id == t.region_id)
        {
            return invalid_ref(format!(
                "open tension {} references unknown region {}",
                t.tension_id, t.region_id
            ));
        }
    }
    Ok(())
}
fn operational(s: &RegionPersistenceState) -> bool {
    matches!(
        s,
        RegionPersistenceState::Active
            | RegionPersistenceState::Background
            | RegionPersistenceState::Unresolved
    )
}
fn invalid_ref<T>(context: String) -> Result<T, ProjectionActivationViolation> {
    Err(ProjectionActivationViolation::InvalidActivatedReference { context })
}

fn band_config<'a>(
    c: &'a ProjectionActivationConfig,
    b: &ProjectionActivationProbeBand,
) -> &'a crate::activation::ProjectionActivationBandConfig {
    match b {
        ProjectionActivationProbeBand::Unbanded => &c.unbanded,
        ProjectionActivationProbeBand::Attention(ActivationBand::Primary) => &c.primary,
        ProjectionActivationProbeBand::Attention(ActivationBand::Secondary) => &c.secondary,
        ProjectionActivationProbeBand::Attention(ActivationBand::Tertiary) => &c.tertiary,
        ProjectionActivationProbeBand::Attention(ActivationBand::Background) => &c.background,
    }
}
fn surface_limit(
    c: &ProjectionActivationConfig,
    sid: &str,
    b: &ProjectionActivationProbeBand,
) -> u32 {
    let s = c
        .surface_limits
        .iter()
        .find(|s| s.surface_id == sid)
        .expect("preflight");
    match b {
        ProjectionActivationProbeBand::Unbanded => s.unbanded_candidate_limit,
        ProjectionActivationProbeBand::Attention(ActivationBand::Primary) => {
            s.primary_candidate_limit
        }
        ProjectionActivationProbeBand::Attention(ActivationBand::Secondary) => {
            s.secondary_candidate_limit
        }
        ProjectionActivationProbeBand::Attention(ActivationBand::Tertiary) => {
            s.tertiary_candidate_limit
        }
        ProjectionActivationProbeBand::Attention(ActivationBand::Background) => {
            s.background_candidate_limit
        }
    }
}

fn build_seeds(
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
) -> Result<Vec<Seed>, ProjectionActivationViolation> {
    let mut out = Vec::new();
    let mut un = 0u32;
    let max = c.unbanded.maximum_textual_seeds;
    if un < max {
        out.push(Seed {
            text: u.text.clone(),
            band: ProjectionActivationProbeBand::Unbanded,
            provenance: vec![ActivationProvenance::NewestUtterance {
                utterance_id: u.utterance_id.clone(),
            }],
            region_id: None,
        });
        un += 1;
    }
    for pc in &ps.constraints {
        if un >= max {
            break;
        }
        if pc.lifecycle == RecordLifecycle::Active
            && matches!(
                pc.applicability,
                ProblemConstraintApplicability::WholeProblemSpace
            )
        {
            out.push(Seed {
                text: pc.expression.clone(),
                band: ProjectionActivationProbeBand::Unbanded,
                provenance: vec![ActivationProvenance::Constraint {
                    constraint_id: pc.constraint_id.clone(),
                }],
                region_id: None,
            });
            un += 1;
        }
    }
    add_band_seeds(
        ps,
        c,
        ActivationBand::Primary,
        &ps.attention_lens.primary_region_ids,
        &mut out,
    )?;
    add_band_seeds(
        ps,
        c,
        ActivationBand::Secondary,
        &ps.attention_lens.secondary_region_ids,
        &mut out,
    )?;
    add_band_seeds(
        ps,
        c,
        ActivationBand::Tertiary,
        &ps.attention_lens.tertiary_region_ids,
        &mut out,
    )?;
    add_band_seeds(
        ps,
        c,
        ActivationBand::Background,
        &ps.attention_lens.background_region_ids,
        &mut out,
    )?;
    Ok(out)
}
fn add_band_seeds(
    ps: &ProblemSpaceState,
    c: &ProjectionActivationConfig,
    band: ActivationBand,
    ids: &[String],
    out: &mut Vec<Seed>,
) -> Result<(), ProjectionActivationViolation> {
    let pb = ProjectionActivationProbeBand::Attention(band.clone());
    let max = band_config(c, &pb).maximum_textual_seeds;
    let mut used = 0u32;
    for id in ids {
        if used >= max {
            break;
        }
        let r = ps
            .regions
            .iter()
            .find(|r| &r.region_id == id)
            .expect("lens");
        for rf in &r.anchor_referents {
            if used >= max {
                break;
            }
            out.push(Seed {
                text: rf.expression.clone(),
                band: pb.clone(),
                provenance: vec![
                    ActivationProvenance::ProblemRegion {
                        region_id: id.clone(),
                    },
                    ActivationProvenance::ProblemReferent {
                        region_id: id.clone(),
                        referent_id: rf.referent_id.clone(),
                    },
                    ActivationProvenance::AttentionBand {
                        region_id: id.clone(),
                        band: band.clone(),
                    },
                ],
                region_id: Some(id.clone()),
            });
            used += 1;
        }
        for pc in &ps.constraints {
            if used >= max {
                break;
            }
            if pc.lifecycle == RecordLifecycle::Active {
                if let ProblemConstraintApplicability::Regions { region_ids } = &pc.applicability {
                    if region_ids.contains(id) {
                        out.push(Seed {
                            text: pc.expression.clone(),
                            band: pb.clone(),
                            provenance: vec![
                                ActivationProvenance::ProblemRegion {
                                    region_id: id.clone(),
                                },
                                ActivationProvenance::Constraint {
                                    constraint_id: pc.constraint_id.clone(),
                                },
                                ActivationProvenance::AttentionBand {
                                    region_id: id.clone(),
                                    band: band.clone(),
                                },
                            ],
                            region_id: Some(id.clone()),
                        });
                        used += 1;
                    }
                }
            }
        }
        for t in &ps.open_tensions {
            if used >= max {
                break;
            }
            if t.region_id == *id && t.lifecycle == TensionLifecycle::Open {
                add_tension(t, id, &pb, &band, out, &mut used, max)?;
            }
        }
    }
    Ok(())
}
fn add_tension(
    t: &OpenTension,
    id: &str,
    pb: &ProjectionActivationProbeBand,
    band: &ActivationBand,
    out: &mut Vec<Seed>,
    used: &mut u32,
    max: u32,
) -> Result<(), ProjectionActivationViolation> {
    if let Some(expr) = &t.unresolved_expression {
        if *used < max {
            out.push(Seed {
                text: expr.clone(),
                band: pb.clone(),
                provenance: vec![
                    ActivationProvenance::ProblemRegion {
                        region_id: id.into(),
                    },
                    ActivationProvenance::OpenTension {
                        tension_id: t.tension_id.clone(),
                    },
                    ActivationProvenance::AttentionBand {
                        region_id: id.into(),
                        band: band.clone(),
                    },
                ],
                region_id: Some(id.into()),
            });
            *used += 1;
        }
    }
    for (i, cand) in t.candidate_bindings.iter().enumerate() {
        if *used >= max {
            break;
        }
        out.push(Seed {
            text: cand.clone(),
            band: pb.clone(),
            provenance: vec![
                ActivationProvenance::ProblemRegion {
                    region_id: id.into(),
                },
                ActivationProvenance::OpenTension {
                    tension_id: t.tension_id.clone(),
                },
                ActivationProvenance::OpenTensionCandidate {
                    tension_id: t.tension_id.clone(),
                    candidate_index: u32::try_from(i)
                        .map_err(|_| ProjectionActivationViolation::CountOverflow)?,
                },
                ActivationProvenance::AttentionBand {
                    region_id: id.into(),
                    band: band.clone(),
                },
            ],
            region_id: Some(id.into()),
        });
        *used += 1;
    }
    Ok(())
}

#[derive(Clone)]
struct QueuedSource {
    seed: Seed,
    source: ProjectionActivationProbeSource,
    depth: u32,
}

fn dispatch_root<A: ProjectionActivationAccess + ?Sized>(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
    access: &A,
    w: &mut Work,
    seed: Seed,
) -> Result<(), ProjectionActivationViolation> {
    let mut queue = VecDeque::from([QueuedSource {
        source: ProjectionActivationProbeSource::Text {
            text: seed.text.clone(),
        },
        depth: 0,
        seed,
    }]);
    let mut seen_incidence = Vec::<SemanticAddress>::new();
    let mut seen_temporal = Vec::<SemanticAddress>::new();

    while let Some(item) = queue.pop_front() {
        let derived = dispatch_one_source(p, ps, u, c, access, w, &item)?;
        for derived_item in derived {
            match &derived_item.source {
                ProjectionActivationProbeSource::Address { address } => {
                    if !seen_incidence.contains(address) {
                        seen_incidence.push(address.clone());
                        queue.push_back(derived_item);
                    }
                }
                ProjectionActivationProbeSource::Temporal { address } => {
                    if !seen_temporal.contains(address) {
                        seen_temporal.push(address.clone());
                        queue.push_back(derived_item);
                    }
                }
                ProjectionActivationProbeSource::Text { .. } => queue.push_back(derived_item),
            }
        }
    }
    Ok(())
}

fn dispatch_one_source<A: ProjectionActivationAccess + ?Sized>(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
    access: &A,
    w: &mut Work,
    item: &QueuedSource,
) -> Result<Vec<QueuedSource>, ProjectionActivationViolation> {
    let mut derived = Vec::new();
    for surface in p.retrieval_surfaces.iter().filter(|s| s.available) {
        for mode in &surface.match_modes {
            let Some(source_kind) = compatible_or_declared_failure(
                access,
                &surface.surface_id,
                mode,
                &item.source,
                &mut w.probe_counter,
            )?
            else {
                continue;
            };
            if !capable(p, surface, &item.source) {
                continue;
            }
            let probe_id = next_id("activation-probe", &mut w.probe_counter)?;
            let telemetry_id = reserve_telemetry(c, w)?;
            let mut probe_provenance = item.seed.provenance.clone();
            if matches!(source_kind, ProjectionActivationProbeSourceKind::Address)
                && matches!(
                    mode,
                    SurfaceMatchMode::Incidence | SurfaceMatchMode::Declared { .. }
                )
            {
                append_relation_provenance(&mut probe_provenance, ps, item.seed.region_id.as_ref());
            }
            push_unique(
                &mut probe_provenance,
                ActivationProvenance::ConfiguredDefault {
                    configuration_key: "automatic_surface_fan_out".into(),
                },
            );
            let probe = ProjectionActivationProbe {
                probe_id: probe_id.clone(),
                band: item.seed.band.clone(),
                surface_id: surface.surface_id.clone(),
                surface_kind: surface.kind.clone(),
                match_mode: mode.clone(),
                source: item.source.clone(),
                candidate_limit: surface_limit(c, &surface.surface_id, &item.seed.band),
                current_depth: item.depth,
                activation_provenance: probe_provenance.clone(),
            };
            let result = access.execute_probe(p, &probe).map_err(|e| {
                ProjectionActivationViolation::SurfaceAccessFailed {
                    surface_id: surface.surface_id.clone(),
                    probe_id: probe_id.clone(),
                    context: e.context,
                }
            })?;
            validate_result(p, surface, &probe, &result)?;
            let mut bounded = result.continuation.is_some()
                || candidate_count_value(&result.candidate_count) > result.candidates.len() as u64;
            let returned_count = u64::try_from(result.candidates.len())
                .map_err(|_| ProjectionActivationViolation::CountOverflow)?;
            let exposure = ExposureContext {
                probe_id: probe_id.clone(),
                activation_provenance: probe_provenance.clone(),
            };
            for candidate in &result.candidates {
                let direct_edge = candidate.transition.as_ref().map(|transition| {
                    let source = match &probe.source {
                        ProjectionActivationProbeSource::Address { address } => address.clone(),
                        _ => candidate.address.clone(),
                    };
                    EdgeTuple {
                        source,
                        transition_id: transition.transition_id.clone(),
                        direction: transition.direction.clone(),
                        target: candidate.address.clone(),
                    }
                });
                if !insert_bundle(
                    p,
                    c,
                    w,
                    &candidate.address,
                    &exposure,
                    &item.seed.band,
                    direct_edge.as_ref(),
                )? {
                    bounded = true;
                    w.bounded_probe_ids.insert(probe_id.clone());
                    continue;
                }
                if let Some(edge) = direct_edge {
                    remember_edge_provenance(w, edge, &exposure);
                }
                if matches!(
                    item.source,
                    ProjectionActivationProbeSource::Text { .. }
                        | ProjectionActivationProbeSource::Address { .. }
                ) && item.depth < c.maximum_initial_relation_depth
                {
                    derived.push(QueuedSource {
                        seed: item.seed.clone(),
                        source: ProjectionActivationProbeSource::Address {
                            address: candidate.address.clone(),
                        },
                        depth: item
                            .depth
                            .checked_add(1)
                            .ok_or(ProjectionActivationViolation::CountOverflow)?,
                    });
                }
                if matches!(
                    item.source,
                    ProjectionActivationProbeSource::Text { .. }
                        | ProjectionActivationProbeSource::Address { .. }
                ) && is_temporal_probe_root(&candidate.address)
                {
                    derived.push(QueuedSource {
                        seed: item.seed.clone(),
                        source: ProjectionActivationProbeSource::Temporal {
                            address: candidate.address.clone(),
                        },
                        depth: item.depth,
                    });
                }
            }
            let handle_emitted = add_surface_handle(p, ps, u, c, w, &probe, &result, &exposure)?;
            if result.continuation.is_some() && !handle_emitted {
                bounded = true;
                w.bounded_probe_ids.insert(probe_id.clone());
            }
            w.telemetry.push(ProjectionTelemetry {
                telemetry_id,
                probe_id: probe_id.clone(),
                match_mode: probe.match_mode,
                surface_kind: probe.surface_kind,
                surface_id: probe.surface_id,
                candidate_count: result.candidate_count,
                current_depth: item.depth,
                maximum_depth: c.maximum_initial_relation_depth,
                returned_count,
                remaining_expansion_budget: c.maximum_expansion_budget,
                truncation_state: if bounded {
                    TruncationState::Bounded
                } else {
                    TruncationState::Complete
                },
                identifier_type_distribution: result.identifier_type_distribution,
                temporal_anchor_count: result.temporal_anchor_count,
                unresolved_target_count: result.unresolved_target_count,
                continuation_available: handle_emitted,
                activation_provenance: probe.activation_provenance,
            });
        }
    }
    Ok(derived)
}

fn reserve_telemetry(
    c: &ProjectionActivationConfig,
    w: &mut Work,
) -> Result<String, ProjectionActivationViolation> {
    let next_actual = u64::try_from(w.telemetry.len())
        .map_err(|_| ProjectionActivationViolation::CountOverflow)?
        .checked_add(1)
        .ok_or(ProjectionActivationViolation::CountOverflow)?;
    if next_actual > u64::from(c.maximum_telemetry_records) {
        return Err(ProjectionActivationViolation::ActivatedViewBoundExceeded {
            kind: ActivatedRecordKind::Telemetry,
            actual: next_actual,
            maximum: c.maximum_telemetry_records,
        });
    }
    next_id("activation-telemetry", &mut w.telemetry_counter)
}

fn append_relation_provenance(
    provenance: &mut Vec<ActivationProvenance>,
    ps: &ProblemSpaceState,
    region_id: Option<&String>,
) {
    let Some(region_id) = region_id else {
        return;
    };
    for relation in ps.relations.iter().filter(|relation| {
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

fn source_kind(src: &ProjectionActivationProbeSource) -> ProjectionActivationProbeSourceKind {
    match src {
        ProjectionActivationProbeSource::Text { .. } => ProjectionActivationProbeSourceKind::Text,
        ProjectionActivationProbeSource::Address { .. } => {
            ProjectionActivationProbeSourceKind::Address
        }
        ProjectionActivationProbeSource::Temporal { .. } => {
            ProjectionActivationProbeSourceKind::Temporal
        }
    }
}

fn compatible_or_declared_failure<A: ProjectionActivationAccess + ?Sized>(
    access: &A,
    surface_id: &str,
    mode: &SurfaceMatchMode,
    src: &ProjectionActivationProbeSource,
    probe_counter: &mut u64,
) -> Result<Option<ProjectionActivationProbeSourceKind>, ProjectionActivationViolation> {
    let current = source_kind(src);
    let compatible = match (mode, &current) {
        (
            SurfaceMatchMode::Literal
            | SurfaceMatchMode::Terms
            | SurfaceMatchMode::NearestNeighbours,
            ProjectionActivationProbeSourceKind::Text,
        ) => true,
        (SurfaceMatchMode::Incidence, ProjectionActivationProbeSourceKind::Address) => true,
        (SurfaceMatchMode::Temporal, ProjectionActivationProbeSourceKind::Temporal) => true,
        (SurfaceMatchMode::Declared { name }, _) => {
            match access.declared_mode_source(surface_id, name) {
                Some(kind) if kind == current => true,
                Some(_) => false,
                None => {
                    let probe_id = next_id("activation-probe", probe_counter)?;
                    return Err(ProjectionActivationViolation::SurfaceAccessFailed {
                        surface_id: surface_id.to_owned(),
                        probe_id,
                        context: format!(
                            "declared match mode {name} is unsupported for surface {surface_id}"
                        ),
                    });
                }
            }
        }
        _ => false,
    };
    Ok(compatible.then_some(current))
}

fn capable(
    p: &SemanticSpaceProjection,
    s: &crate::projection::RetrievalSurfaceDescriptor,
    src: &ProjectionActivationProbeSource,
) -> bool {
    match src {
        ProjectionActivationProbeSource::Text { .. } => true,
        ProjectionActivationProbeSource::Address { address }
        | ProjectionActivationProbeSource::Temporal { address } => {
            s.visible_address_kinds.contains(&address.kind())
                && record_surfaces(p, address).is_none_or(|ids| ids.contains(&s.surface_id))
        }
    }
}

fn record_surfaces<'a>(
    p: &'a SemanticSpaceProjection,
    a: &SemanticAddress,
) -> Option<&'a Vec<String>> {
    match a {
        SemanticAddress::Object(id) => p
            .objects
            .iter()
            .find(|r| &r.object_id == id)
            .map(|r| &r.retrieval_surface_ids),
        SemanticAddress::Region(id) => p
            .regions
            .iter()
            .find(|r| &r.address == id)
            .map(|r| &r.retrieval_surface_ids),
        SemanticAddress::Unit(id) => p
            .units
            .iter()
            .find(|r| &r.unit_id == id)
            .map(|r| &r.retrieval_surface_ids),
        SemanticAddress::Identifier(ia) => p
            .identifier_descriptors
            .iter()
            .find(|d| d.identifier_name == ia.identifier_name)
            .map(|d| &d.retrieval_surface_ids),
        _ => None,
    }
}

fn next_id(prefix: &str, c: &mut u64) -> Result<String, ProjectionActivationViolation> {
    let id = format!("{prefix}:{}", *c);
    *c = c
        .checked_add(1)
        .ok_or(ProjectionActivationViolation::CountOverflow)?;
    Ok(id)
}

fn candidate_count_value(c: &CandidateCount) -> u64 {
    match c {
        CandidateCount::Exact(v) | CandidateCount::Estimated(v) => *v,
    }
}

fn validate_result(
    p: &SemanticSpaceProjection,
    s: &crate::projection::RetrievalSurfaceDescriptor,
    probe: &ProjectionActivationProbe,
    res: &ProjectionActivationProbeResult,
) -> Result<(), ProjectionActivationViolation> {
    let fail = |ctx: String| ProjectionActivationViolation::SurfaceAccessFailed {
        surface_id: s.surface_id.clone(),
        probe_id: probe.probe_id.clone(),
        context: ctx,
    };
    if u32::try_from(res.candidates.len())
        .map_err(|_| ProjectionActivationViolation::CountOverflow)?
        > probe.candidate_limit
    {
        return Err(fail("returned candidate count exceeds probe limit".into()));
    }
    if probe.candidate_limit == 0 && !res.candidates.is_empty() {
        return Err(fail("zero-limit probe returned candidates".into()));
    }
    if candidate_count_value(&res.candidate_count)
        < u64::try_from(res.candidates.len())
            .map_err(|_| ProjectionActivationViolation::CountOverflow)?
    {
        return Err(fail(
            "candidate count smaller than returned candidates".into(),
        ));
    }
    let mut seen = Vec::<SemanticAddress>::new();
    for cand in &res.candidates {
        if seen.contains(&cand.address) {
            return Err(fail("duplicate candidate address".into()));
        }
        seen.push(cand.address.clone());
        if cand.address.kind() != s.returned_identity {
            return Err(fail("returned address kind mismatch".into()));
        }
        validate_candidate_address_exists(p, &cand.address).map_err(&fail)?;
        let incidence = matches!(probe.match_mode, SurfaceMatchMode::Incidence)
            || matches!(
                (&probe.match_mode, &probe.source),
                (
                    SurfaceMatchMode::Declared { .. },
                    ProjectionActivationProbeSource::Address { .. }
                )
            );
        if incidence != cand.transition.is_some() {
            return Err(fail("incidence transition presence mismatch".into()));
        }
        if let Some(returned) = &cand.transition {
            let ProjectionActivationProbeSource::Address { address: source } = &probe.source else {
                return Err(fail(
                    "incidence candidate requires address probe source".into(),
                ));
            };
            let transition = p
                .valid_transitions
                .iter()
                .find(|t| t.transition_id == returned.transition_id)
                .ok_or_else(|| {
                    fail(format!(
                        "unknown incidence transition {}",
                        returned.transition_id
                    ))
                })?;
            if transition.from != source.kind()
                || transition.to != cand.address.kind()
                || transition.direction != returned.direction
            {
                return Err(fail("incidence transition shape mismatch".into()));
            }
            if transition
                .retrieval_surface_id
                .as_ref()
                .is_some_and(|required| required != &s.surface_id)
            {
                return Err(fail(
                    "incidence transition requires a different surface".into(),
                ));
            }
            if !edge_exists(
                p,
                source,
                &returned.transition_id,
                &returned.direction,
                &cand.address,
            ) {
                return Err(fail(
                    "incidence candidate is not an actual represented edge".into(),
                ));
            }
        }
    }
    if let Some(cont) = &res.continuation {
        if !s.continuation_supported {
            return Err(fail("continuation returned for unsupported surface".into()));
        }
        if cont.ordering_key.is_empty() {
            return Err(fail("continuation ordering key is empty".into()));
        }
        let returned = u64::try_from(res.candidates.len())
            .map_err(|_| ProjectionActivationViolation::CountOverflow)?;
        if cont.next_offset != returned {
            return Err(fail("initial continuation next_offset mismatch".into()));
        }
        if let CandidateCount::Exact(total) = res.candidate_count {
            if let Some(remaining) = cont.remaining_count {
                let known = cont
                    .next_offset
                    .checked_add(remaining)
                    .ok_or(ProjectionActivationViolation::CountOverflow)?;
                if total != known {
                    return Err(fail("exact total incompatible with continuation".into()));
                }
            } else if total < cont.next_offset {
                return Err(fail("exact total incompatible with continuation".into()));
            }
        }
    }
    Ok(())
}

fn validate_candidate_address_exists(
    p: &SemanticSpaceProjection,
    a: &SemanticAddress,
) -> Result<(), String> {
    match a {
        SemanticAddress::Object(id) if p.objects.iter().any(|r| &r.object_id == id) => Ok(()),
        SemanticAddress::Region(id) if p.regions.iter().any(|r| &r.address == id) => Ok(()),
        SemanticAddress::Unit(id) if p.units.iter().any(|r| &r.unit_id == id) => Ok(()),
        SemanticAddress::Occurrence(id) if p.occurrences.iter().any(|r| &r.occurrence_id == id) => {
            Ok(())
        }
        SemanticAddress::TemporalAnchor(id)
            if p.temporal_anchors.iter().any(|r| &r.anchor_id == id) =>
        {
            Ok(())
        }
        SemanticAddress::Identifier(_) => resolve_identifier_assignment(p, a).map(|_| ()),
        SemanticAddress::RetrievalSurface(_) => {
            Err("retrieval surface candidates are not representable in activated projection".into())
        }
        _ => Err("candidate address does not exist in projection".into()),
    }
}

fn resolve_identifier_assignment<'a>(
    p: &'a SemanticSpaceProjection,
    a: &SemanticAddress,
) -> Result<&'a crate::projection::IdentifierAssignment, String> {
    let SemanticAddress::Identifier(ia) = a else {
        return Err("address is not an identifier".into());
    };
    let mut matches = p.identifier_assignments.iter().filter(|assignment| {
        assignment.identifier_name == ia.identifier_name
            && ia
                .represented_value
                .as_ref()
                .is_none_or(|value| identifier_value_matches(&assignment.value, value))
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
        IdentifierValue::String(s) => s == represented,
        IdentifierValue::Integer(i) => i.to_string() == represented,
        IdentifierValue::Boolean(b) => b.to_string() == represented,
        IdentifierValue::SemanticAddress(address) => format!("{address:?}") == represented,
        IdentifierValue::Strings(values) => values.len() == 1 && values[0] == represented,
        IdentifierValue::SemanticAddresses(values) => {
            values.len() == 1 && format!("{:?}", values[0]) == represented
        }
    }
}

fn identifier_value_string(v: &IdentifierValue) -> Option<String> {
    match v {
        IdentifierValue::String(s) => Some(s.clone()),
        IdentifierValue::Integer(i) => Some(i.to_string()),
        IdentifierValue::Boolean(b) => Some(b.to_string()),
        IdentifierValue::SemanticAddress(address) => Some(format!("{address:?}")),
        IdentifierValue::Strings(values) if values.len() == 1 => Some(values[0].clone()),
        IdentifierValue::SemanticAddresses(values) if values.len() == 1 => {
            Some(format!("{:?}", values[0]))
        }
        _ => None,
    }
}

fn is_temporal_probe_root(a: &SemanticAddress) -> bool {
    matches!(
        a,
        SemanticAddress::Object(_)
            | SemanticAddress::Unit(_)
            | SemanticAddress::TemporalAnchor(_)
            | SemanticAddress::Identifier(_)
    )
}
fn push_unique<T: PartialEq>(v: &mut Vec<T>, x: T) {
    if !v.contains(&x) {
        v.push(x)
    }
}
fn merge<T: Clone + PartialEq>(v: &mut Vec<T>, add: &[T]) {
    for x in add {
        push_unique(v, x.clone());
    }
}
fn with_context(prov: &[ActivationProvenance]) -> Vec<ActivationProvenance> {
    let mut p = prov.to_vec();
    push_unique(
        &mut p,
        ActivationProvenance::ConfiguredDefault {
            configuration_key: "bounded_structural_context".into(),
        },
    );
    p
}

fn insert_bundle(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    direct: &SemanticAddress,
    exposure: &ExposureContext,
    band: &ProjectionActivationProbeBand,
    direct_edge_to_skip: Option<&EdgeTuple>,
) -> Result<bool, ProjectionActivationViolation> {
    let mut required = Vec::new();
    closure_addresses(p, direct, &mut required)?;
    let contextual = ExposureContext {
        probe_id: exposure.probe_id.clone(),
        activation_provenance: with_context(&exposure.activation_provenance),
    };
    let mut no = 0usize;
    let mut nr = 0usize;
    let mut nu = 0usize;
    let mut ni = 0usize;
    let mut noc = 0usize;
    let mut na = 0usize;
    for addr in &required {
        match addr {
            SemanticAddress::Object(_) if !has_object(w, addr) => no += 1,
            SemanticAddress::Region(_) if !has_region(w, addr) => nr += 1,
            SemanticAddress::Unit(_) if !has_unit(w, addr) => nu += 1,
            SemanticAddress::Identifier(_) if !has_assignment(w, addr, p) => ni += 1,
            SemanticAddress::Occurrence(_) if !has_occurrence(w, addr) => noc += 1,
            SemanticAddress::TemporalAnchor(_) if !has_anchor(w, addr) => na += 1,
            _ => {}
        }
    }
    if w.objects
        .len()
        .checked_add(no)
        .ok_or(ProjectionActivationViolation::CountOverflow)?
        > c.maximum_activated_objects as usize
        || w.regions
            .len()
            .checked_add(nr)
            .ok_or(ProjectionActivationViolation::CountOverflow)?
            > c.maximum_activated_regions as usize
        || w.units
            .len()
            .checked_add(nu)
            .ok_or(ProjectionActivationViolation::CountOverflow)?
            > c.maximum_activated_units as usize
        || w.assignments
            .len()
            .checked_add(ni)
            .ok_or(ProjectionActivationViolation::CountOverflow)?
            > c.maximum_activated_identifier_assignments as usize
        || w.occurrences
            .len()
            .checked_add(noc)
            .ok_or(ProjectionActivationViolation::CountOverflow)?
            > c.maximum_activated_occurrences as usize
        || w.anchors
            .len()
            .checked_add(na)
            .ok_or(ProjectionActivationViolation::CountOverflow)?
            > c.maximum_activated_temporal_anchors as usize
    {
        return Ok(false);
    }
    for addr in &required {
        let provenance = if addr == direct {
            exposure
        } else {
            &contextual
        };
        insert_one(p, c, w, addr, provenance, band)?;
    }
    register_context_edges(p, w, &required, &contextual, direct_edge_to_skip);
    add_optional_context(p, c, w, direct, &contextual, band)?;
    Ok(true)
}

fn register_context_edges(
    p: &SemanticSpaceProjection,
    w: &mut Work,
    addresses: &[SemanticAddress],
    exposure: &ExposureContext,
    direct_edge_to_skip: Option<&EdgeTuple>,
) {
    for source in addresses {
        for edge in enumerate_edges(p, source) {
            if addresses.contains(&edge.target)
                && visible(p, w, &edge.source)
                && visible(p, w, &edge.target)
                && direct_edge_to_skip != Some(&edge)
            {
                remember_edge_provenance(w, edge, exposure);
            }
        }
    }
}

fn closure_addresses(
    p: &SemanticSpaceProjection,
    a: &SemanticAddress,
    out: &mut Vec<SemanticAddress>,
) -> Result<(), ProjectionActivationViolation> {
    match a {
        SemanticAddress::Object(_) => push_unique(out, a.clone()),
        SemanticAddress::Region(r) => {
            push_unique(out, SemanticAddress::Object(r.object_id.clone()));
            push_unique(out, a.clone());
        }
        SemanticAddress::Unit(id) => {
            let u = p.units.iter().find(|u| &u.unit_id == id).ok_or_else(|| {
                ProjectionActivationViolation::InvalidActivatedReference {
                    context: "unit missing".into(),
                }
            })?;
            push_unique(out, SemanticAddress::Object(u.parent_object_id.clone()));
            push_unique(
                out,
                SemanticAddress::Region(u.parent_region_address.clone()),
            );
            push_unique(out, a.clone());
        }
        SemanticAddress::Identifier(_) => {
            let assignment = resolve_identifier_assignment(p, a).map_err(|context| {
                ProjectionActivationViolation::InvalidActivatedReference { context }
            })?;
            closure_addresses(p, &assignment.subject, out)?;
            push_unique(out, a.clone());
        }
        SemanticAddress::TemporalAnchor(id) => {
            let anchor = p
                .temporal_anchors
                .iter()
                .find(|x| &x.anchor_id == id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: "temporal anchor missing".into(),
                    },
                )?;
            closure_addresses(p, &anchor.subject, out)?;
            push_unique(out, a.clone());
        }
        SemanticAddress::Occurrence(id) => {
            let occurrence = p
                .occurrences
                .iter()
                .find(|o| &o.occurrence_id == id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: "occurrence missing".into(),
                    },
                )?;
            match &occurrence.source {
                OccurrenceSource::ObjectField { object_id, .. } => {
                    closure_addresses(p, &SemanticAddress::Object(object_id.clone()), out)?
                }
                OccurrenceSource::SemanticUnit { unit_id } => {
                    closure_addresses(p, &SemanticAddress::Unit(unit_id.clone()), out)?
                }
            }
            closure_addresses(p, &occurrence.resolved_target, out)?;
            push_unique(out, a.clone());
        }
        SemanticAddress::RetrievalSurface(_) => {}
    }
    Ok(())
}

fn insert_one(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    a: &SemanticAddress,
    exposure: &ExposureContext,
    band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    match a {
        SemanticAddress::Object(id) => {
            remember_address_exposure(w, a.clone(), exposure);
            if let Some(record) = w.objects.iter_mut().find(|r| &r.object_id == id) {
                merge(
                    &mut record.activation_provenance,
                    &exposure.activation_provenance,
                );
            } else {
                let o = p.objects.iter().find(|o| &o.object_id == id).unwrap();
                w.order.push(a.clone());
                w.objects.push(ActivatedObjectRecord {
                    object_id: id.clone(),
                    title: o.title.clone(),
                    aliases: o.aliases.clone(),
                    object_class: o.object_class.clone(),
                    visible_region_addresses: vec![],
                    visible_unit_ids: vec![],
                    visible_identifier_assignment_ids: vec![],
                    contained_region_count: o.region_addresses.len() as u64,
                    contained_unit_count: o.unit_ids.len() as u64,
                    incoming_occurrence_count: o.incoming_occurrence_ids.len() as u64,
                    outgoing_occurrence_count: (o.object_field_occurrence_ids.len()
                        + o.body_occurrence_ids.len())
                        as u64,
                    available_surface_ids: o.retrieval_surface_ids.clone(),
                    activation_provenance: exposure.activation_provenance.clone(),
                });
            }
            enrich_object(p, c, w, id, exposure, band)?;
        }
        SemanticAddress::Region(id) => {
            remember_address_exposure(w, a.clone(), exposure);
            if let Some(record) = w.regions.iter_mut().find(|r| &r.address == id) {
                merge(
                    &mut record.activation_provenance,
                    &exposure.activation_provenance,
                );
            } else {
                let r = p.regions.iter().find(|r| &r.address == id).unwrap();
                w.order.push(a.clone());
                w.regions.push(ActivatedRegionRecord {
                    address: id.clone(),
                    heading_path: r.heading_path.clone(),
                    heading_identity: r.heading_identity.clone(),
                    visible_identifier_assignment_ids: vec![],
                    visible_unit_ids: vec![],
                    contained_unit_count: r.contained_unit_ids.len() as u64,
                    available_surface_ids: r.retrieval_surface_ids.clone(),
                    activation_provenance: exposure.activation_provenance.clone(),
                });
            }
            enrich_region(p, c, w, id, exposure, band)?;
        }
        SemanticAddress::Unit(id) => {
            remember_address_exposure(w, a.clone(), exposure);
            if let Some(record) = w.units.iter_mut().find(|r| &r.unit_id == id) {
                merge(
                    &mut record.activation_provenance,
                    &exposure.activation_provenance,
                );
                record.text_preview = larger_preview(
                    record.text_preview.clone(),
                    p.units
                        .iter()
                        .find(|u| &u.unit_id == id)
                        .unwrap()
                        .content
                        .clone(),
                    band_config(c, band).text_preview_character_limit,
                );
            } else {
                let u = p.units.iter().find(|u| &u.unit_id == id).unwrap();
                w.order.push(a.clone());
                w.units.push(ActivatedUnitRecord {
                    unit_id: id.clone(),
                    parent_object_id: u.parent_object_id.clone(),
                    parent_region_address: u.parent_region_address.clone(),
                    authored_block_type: u.authored_block_type.clone(),
                    heading_path: u.heading_path.clone(),
                    visible_inherited_identifier_assignment_ids: vec![],
                    visible_unit_local_identifier_assignment_ids: vec![],
                    text_preview: preview(
                        &u.content,
                        band_config(c, band).text_preview_character_limit,
                    ),
                    incoming_occurrence_count: u.incoming_occurrence_ids.len() as u64,
                    outgoing_occurrence_count: u.outgoing_occurrence_ids.len() as u64,
                    temporal_anchor_count: u.temporal_anchor_ids.len() as u64,
                    available_surface_ids: u.retrieval_surface_ids.clone(),
                    activation_provenance: exposure.activation_provenance.clone(),
                });
            }
            enrich_unit_identifiers(p, c, w, id, exposure)?;
        }
        SemanticAddress::Identifier(_) => insert_assignment_address(p, c, w, a, exposure)?,
        SemanticAddress::Occurrence(id) => {
            remember_address_exposure(w, a.clone(), exposure);
            if let Some(record) = w.occurrences.iter_mut().find(|r| &r.occurrence_id == id) {
                merge(
                    &mut record.activation_provenance,
                    &exposure.activation_provenance,
                );
            } else {
                let o = p
                    .occurrences
                    .iter()
                    .find(|o| &o.occurrence_id == id)
                    .unwrap();
                w.order.push(a.clone());
                w.occurrences.push(ActivatedOccurrenceRecord {
                    occurrence_id: id.clone(),
                    source: o.source.clone(),
                    authored_target_text: o.authored_target_text.clone(),
                    display_alias: o.display_alias.clone(),
                    resolved_target: o.resolved_target.clone(),
                    presentation_mode: o.presentation_mode.clone(),
                    direction: o.direction.clone(),
                    source_span: o.source_span.clone(),
                    available_surface_ids: capable_surface_ids(p, AddressKind::Occurrence),
                    activation_provenance: exposure.activation_provenance.clone(),
                });
            }
        }
        SemanticAddress::TemporalAnchor(id) => {
            remember_address_exposure(w, a.clone(), exposure);
            if let Some(record) = w.anchors.iter_mut().find(|r| &r.anchor_id == id) {
                merge(
                    &mut record.activation_provenance,
                    &exposure.activation_provenance,
                );
            } else {
                let t = p
                    .temporal_anchors
                    .iter()
                    .find(|t| &t.anchor_id == id)
                    .unwrap();
                w.order.push(a.clone());
                w.anchors.push(ActivatedTemporalAnchorRecord {
                    anchor_id: id.clone(),
                    subject: t.subject.clone(),
                    value: t.value.clone(),
                    record_provenance: t.provenance.clone(),
                    available_surface_ids: capable_surface_ids(p, AddressKind::TemporalAnchor),
                    activation_provenance: exposure.activation_provenance.clone(),
                });
            }
        }
        SemanticAddress::RetrievalSurface(_) => {}
    }
    Ok(())
}

fn larger_preview(
    existing: ActivatedTextPreview,
    content: SemanticUnitContent,
    limit: u32,
) -> ActivatedTextPreview {
    let candidate = preview(&content, limit);
    match (&existing, &candidate) {
        (
            ActivatedTextPreview::Inline { text: old, .. },
            ActivatedTextPreview::Inline { text: new, .. },
        ) if old.chars().count() >= new.chars().count() => existing,
        _ => candidate,
    }
}

fn preview(content: &SemanticUnitContent, limit: u32) -> ActivatedTextPreview {
    match content {
        SemanticUnitContent::Inline {
            normalized_text, ..
        } => {
            let text: String = normalized_text.chars().take(limit as usize).collect();
            let truncated = normalized_text.chars().count() > limit as usize;
            ActivatedTextPreview::Inline { text, truncated }
        }
        SemanticUnitContent::HydrationAddress { .. } => {
            ActivatedTextPreview::UnavailableWithoutHydration
        }
    }
}

fn capable_surface_ids(p: &SemanticSpaceProjection, kind: AddressKind) -> Vec<String> {
    p.retrieval_surfaces
        .iter()
        .filter(|surface| surface.available && surface.visible_address_kinds.contains(&kind))
        .map(|surface| surface.surface_id.clone())
        .collect()
}

fn insert_assignment_id(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    assignment_id: &str,
    exposure: &ExposureContext,
) -> Result<bool, ProjectionActivationViolation> {
    if let Some(record) = w
        .assignments
        .iter_mut()
        .find(|record| record.assignment_id == assignment_id)
    {
        merge(
            &mut record.activation_provenance,
            &exposure.activation_provenance,
        );
        register_assignment_edge(p, w, assignment_id, exposure)?;
        return Ok(true);
    }
    if w.assignments.len() >= c.maximum_activated_identifier_assignments as usize {
        return Ok(false);
    }
    let assignment = p
        .identifier_assignments
        .iter()
        .find(|a| a.assignment_id == assignment_id)
        .ok_or_else(
            || ProjectionActivationViolation::InvalidActivatedReference {
                context: format!("assignment {assignment_id} missing"),
            },
        )?;
    w.assignments.push(ActivatedIdentifierAssignmentRecord {
        assignment_id: assignment.assignment_id.clone(),
        identifier_name: assignment.identifier_name.clone(),
        subject: assignment.subject.clone(),
        value: assignment.value.clone(),
        record_provenance: assignment.provenance.clone(),
        available_surface_ids: p
            .identifier_descriptors
            .iter()
            .find(|d| d.identifier_name == assignment.identifier_name)
            .map(|d| d.retrieval_surface_ids.clone())
            .unwrap_or_default(),
        activation_provenance: exposure.activation_provenance.clone(),
    });
    register_assignment_edge(p, w, assignment_id, exposure)?;
    Ok(true)
}

fn register_assignment_edge(
    p: &SemanticSpaceProjection,
    w: &mut Work,
    assignment_id: &str,
    exposure: &ExposureContext,
) -> Result<(), ProjectionActivationViolation> {
    let assignment = p
        .identifier_assignments
        .iter()
        .find(|a| a.assignment_id == assignment_id)
        .ok_or_else(
            || ProjectionActivationViolation::InvalidActivatedReference {
                context: format!("assignment {assignment_id} missing"),
            },
        )?;
    let address = SemanticAddress::Identifier(crate::model::IdentifierAddress {
        identifier_name: assignment.identifier_name.clone(),
        represented_value: identifier_value_string(&assignment.value),
    });
    for edge in enumerate_edges(p, &assignment.subject) {
        if edge.target == address && visible(p, w, &edge.source) && visible(p, w, &edge.target) {
            remember_edge_provenance(w, edge, exposure);
        }
    }
    for edge in enumerate_edges(p, &address) {
        if edge.target == assignment.subject
            && visible(p, w, &edge.source)
            && visible(p, w, &edge.target)
        {
            remember_edge_provenance(w, edge, exposure);
        }
    }
    Ok(())
}

fn insert_assignment_address(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    address: &SemanticAddress,
    exposure: &ExposureContext,
) -> Result<(), ProjectionActivationViolation> {
    remember_address_exposure(w, address.clone(), exposure);
    if !w.order.contains(address) {
        w.order.push(address.clone());
    }
    let assignment_id = resolve_identifier_assignment(p, address)
        .map_err(|context| ProjectionActivationViolation::InvalidActivatedReference { context })?
        .assignment_id
        .clone();
    let _ = insert_assignment_id(p, c, w, &assignment_id, exposure)?;
    Ok(())
}

fn add_optional_context(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    direct: &SemanticAddress,
    exposure: &ExposureContext,
    band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    match direct {
        SemanticAddress::Object(id) => materialize_object_children(p, c, w, id, exposure, band)?,
        SemanticAddress::Region(id) => materialize_region_units(p, c, w, id, exposure, band)?,
        _ => {}
    }
    Ok(())
}

fn materialize_object_children(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    id: &crate::model::SemanticObjectId,
    exposure: &ExposureContext,
    band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    let Some(object) = p.objects.iter().find(|o| &o.object_id == id) else {
        return Ok(());
    };
    let limit = band_config(c, band).maximum_structural_neighbors_per_record as usize;
    let mut used = w
        .objects
        .iter()
        .find(|o| &o.object_id == id)
        .map(|o| o.visible_region_addresses.len() + o.visible_unit_ids.len())
        .unwrap_or(0);
    for region in &object.region_addresses {
        if w.objects
            .iter()
            .find(|o| &o.object_id == id)
            .is_some_and(|record| record.visible_region_addresses.contains(region))
        {
            continue;
        }
        if used >= limit {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
        if insert_bundle(
            p,
            c,
            w,
            &SemanticAddress::Region(region.clone()),
            exposure,
            band,
            None,
        )? {
            if let Some(record) = w.objects.iter_mut().find(|o| &o.object_id == id) {
                push_unique(&mut record.visible_region_addresses, region.clone());
            }
            used += 1;
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            return Ok(());
        }
    }
    for unit in &object.unit_ids {
        if w.objects
            .iter()
            .find(|o| &o.object_id == id)
            .is_some_and(|record| record.visible_unit_ids.contains(unit))
        {
            continue;
        }
        if used >= limit {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
        if insert_bundle(
            p,
            c,
            w,
            &SemanticAddress::Unit(unit.clone()),
            exposure,
            band,
            None,
        )? {
            if let Some(record) = w.objects.iter_mut().find(|o| &o.object_id == id) {
                push_unique(&mut record.visible_unit_ids, unit.clone());
            }
            used += 1;
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
    }
    enrich_object(p, c, w, id, exposure, band)
}

fn materialize_region_units(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    id: &crate::model::SemanticRegionAddress,
    exposure: &ExposureContext,
    band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    let Some(region) = p.regions.iter().find(|r| &r.address == id) else {
        return Ok(());
    };
    let limit = band_config(c, band).maximum_visible_units_per_region as usize;
    let mut visible = w
        .regions
        .iter()
        .find(|r| &r.address == id)
        .map(|r| r.visible_unit_ids.len())
        .unwrap_or(0);
    for unit in &region.contained_unit_ids {
        if w.regions
            .iter()
            .find(|r| &r.address == id)
            .is_some_and(|record| record.visible_unit_ids.contains(unit))
        {
            continue;
        }
        if visible >= limit {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
        if insert_bundle(
            p,
            c,
            w,
            &SemanticAddress::Unit(unit.clone()),
            exposure,
            band,
            None,
        )? {
            if let Some(record) = w.regions.iter_mut().find(|r| &r.address == id) {
                push_unique(&mut record.visible_unit_ids, unit.clone());
            }
            visible += 1;
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
    }
    enrich_region(p, c, w, id, exposure, band)
}

fn enrich_object(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    id: &crate::model::SemanticObjectId,
    exposure: &ExposureContext,
    _band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    let Some(object) = p.objects.iter().find(|o| &o.object_id == id) else {
        return Ok(());
    };
    for assignment_id in &object.identifier_assignment_ids {
        if insert_assignment_id(
            p,
            c,
            w,
            assignment_id,
            &ExposureContext {
                probe_id: exposure.probe_id.clone(),
                activation_provenance: with_context(&exposure.activation_provenance),
            },
        )? {
            if let Some(record) = w.objects.iter_mut().find(|o| &o.object_id == id) {
                push_unique(
                    &mut record.visible_identifier_assignment_ids,
                    assignment_id.clone(),
                );
            }
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
    }
    Ok(())
}
fn enrich_region(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    id: &crate::model::SemanticRegionAddress,
    exposure: &ExposureContext,
    _band: &ProjectionActivationProbeBand,
) -> Result<(), ProjectionActivationViolation> {
    let Some(region) = p.regions.iter().find(|r| &r.address == id) else {
        return Ok(());
    };
    for assignment_id in &region.inherited_identifier_assignment_ids {
        if insert_assignment_id(
            p,
            c,
            w,
            assignment_id,
            &ExposureContext {
                probe_id: exposure.probe_id.clone(),
                activation_provenance: with_context(&exposure.activation_provenance),
            },
        )? {
            if let Some(record) = w.regions.iter_mut().find(|r| &r.address == id) {
                push_unique(
                    &mut record.visible_identifier_assignment_ids,
                    assignment_id.clone(),
                );
            }
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
    }
    Ok(())
}
fn enrich_unit_identifiers(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    id: &crate::model::SemanticUnitId,
    exposure: &ExposureContext,
) -> Result<(), ProjectionActivationViolation> {
    let Some(unit) = p.units.iter().find(|u| &u.unit_id == id) else {
        return Ok(());
    };
    for assignment_id in &unit.inherited_identifier_assignment_ids {
        if insert_assignment_id(
            p,
            c,
            w,
            assignment_id,
            &ExposureContext {
                probe_id: exposure.probe_id.clone(),
                activation_provenance: with_context(&exposure.activation_provenance),
            },
        )? {
            if let Some(record) = w.units.iter_mut().find(|u| &u.unit_id == id) {
                push_unique(
                    &mut record.visible_inherited_identifier_assignment_ids,
                    assignment_id.clone(),
                );
            }
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            return Ok(());
        }
    }
    for assignment_id in &unit.unit_local_identifier_assignment_ids {
        if insert_assignment_id(
            p,
            c,
            w,
            assignment_id,
            &ExposureContext {
                probe_id: exposure.probe_id.clone(),
                activation_provenance: with_context(&exposure.activation_provenance),
            },
        )? {
            if let Some(record) = w.units.iter_mut().find(|u| &u.unit_id == id) {
                push_unique(
                    &mut record.visible_unit_local_identifier_assignment_ids,
                    assignment_id.clone(),
                );
            }
        } else {
            w.bounded_probe_ids.insert(exposure.probe_id.clone());
            break;
        }
    }
    Ok(())
}

fn has_object(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a, SemanticAddress::Object(id) if w.objects.iter().any(|r| &r.object_id == id))
}
fn has_region(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a, SemanticAddress::Region(id) if w.regions.iter().any(|r| &r.address == id))
}
fn has_unit(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a, SemanticAddress::Unit(id) if w.units.iter().any(|r| &r.unit_id == id))
}
fn has_occurrence(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a, SemanticAddress::Occurrence(id) if w.occurrences.iter().any(|r| &r.occurrence_id == id))
}
fn has_anchor(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a, SemanticAddress::TemporalAnchor(id) if w.anchors.iter().any(|r| &r.anchor_id == id))
}
fn has_assignment(w: &Work, a: &SemanticAddress, p: &SemanticSpaceProjection) -> bool {
    resolve_identifier_assignment(p, a).is_ok_and(|assignment| {
        w.assignments
            .iter()
            .any(|r| r.assignment_id == assignment.assignment_id)
    })
}
fn visible(p: &SemanticSpaceProjection, w: &Work, a: &SemanticAddress) -> bool {
    match a {
        SemanticAddress::RetrievalSurface(_) => false,
        SemanticAddress::Identifier(_) => has_assignment(w, a, p),
        _ => {
            has_object(w, a)
                || has_region(w, a)
                || has_unit(w, a)
                || has_occurrence(w, a)
                || has_anchor(w, a)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn add_surface_handle(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    probe: &ProjectionActivationProbe,
    res: &ProjectionActivationProbeResult,
    exposure: &ExposureContext,
) -> Result<bool, ProjectionActivationViolation> {
    let Some(cont) = &res.continuation else {
        return Ok(false);
    };
    if c.continuation_page_limit == 0 || w.handles.len() >= c.maximum_continuation_handles as usize
    {
        w.bounded_probe_ids.insert(exposure.probe_id.clone());
        return Ok(false);
    }
    let origin = match &probe.source {
        ProjectionActivationProbeSource::Text { text } => ContinuationOrigin::TextProbe {
            query_text: text.clone(),
            match_mode: probe.match_mode.clone(),
        },
        ProjectionActivationProbeSource::Address { address } => {
            ContinuationOrigin::StructuralNeighbourhood {
                subject: address.clone(),
                transition_id: None,
                direction: None,
            }
        }
        ProjectionActivationProbeSource::Temporal { .. } => ContinuationOrigin::TemporalProbe {
            start: None,
            end: None,
        },
    };
    let handle_id = next_id("activation-continuation", &mut w.continuation_counter)?;
    w.handles.push(ContinuationHandle {
        handle_id,
        projection_snapshot_id: p.projection_snapshot_id.clone(),
        configuration_snapshot_id: c.configuration_snapshot_id.clone(),
        problem_space_thread_id: ps.thread_id.clone(),
        problem_space_version: ps.version,
        newest_utterance_id: u.utterance_id.clone(),
        origin,
        access: ContinuationAccess::RetrievalSurface {
            surface_id: probe.surface_id.clone(),
            surface_kind: probe.surface_kind.clone(),
        },
        filters: vec![],
        ordering: ContinuationOrdering::SurfaceDeclared {
            ordering_key: cont.ordering_key.clone(),
        },
        next_offset: cont.next_offset,
        remaining_count: cont.remaining_count,
        next_page_limit: c.continuation_page_limit.min(probe.candidate_limit),
        activation_provenance: probe.activation_provenance.clone(),
    });
    Ok(true)
}

fn remember_edge_provenance(w: &mut Work, edge: EdgeTuple, exposure: &ExposureContext) {
    if let Some((_, existing)) = w
        .edge_provenance
        .iter_mut()
        .find(|(known, _)| known == &edge)
    {
        push_unique(&mut existing.probe_ids, exposure.probe_id.clone());
        merge(
            &mut existing.activation_provenance,
            &exposure.activation_provenance,
        );
    } else {
        w.edge_provenance.push((
            edge,
            ExposureAggregate {
                probe_ids: vec![exposure.probe_id.clone()],
                activation_provenance: exposure.activation_provenance.clone(),
            },
        ));
    }
}
fn remember_address_exposure(w: &mut Work, address: SemanticAddress, exposure: &ExposureContext) {
    if let Some((_, existing)) = w
        .address_exposure
        .iter_mut()
        .find(|(known, _)| known == &address)
    {
        push_unique(&mut existing.probe_ids, exposure.probe_id.clone());
        merge(
            &mut existing.activation_provenance,
            &exposure.activation_provenance,
        );
    } else {
        w.address_exposure.push((
            address,
            ExposureAggregate {
                probe_ids: vec![exposure.probe_id.clone()],
                activation_provenance: exposure.activation_provenance.clone(),
            },
        ));
    }
}
fn exposure_for_address(w: &Work, address: &SemanticAddress) -> Option<ExposureAggregate> {
    w.address_exposure
        .iter()
        .find(|(known, _)| known == address)
        .map(|(_, exposure)| exposure.clone())
}
fn merge_exposure(into: &mut ExposureAggregate, exposure: ExposureAggregate) {
    for probe_id in exposure.probe_ids {
        push_unique(&mut into.probe_ids, probe_id);
    }
    merge(
        &mut into.activation_provenance,
        &exposure.activation_provenance,
    );
}
fn exposure_for_edge(w: &Work, edge: &EdgeTuple) -> Option<ExposureAggregate> {
    w.edge_provenance
        .iter()
        .find(|(known, _)| known == edge)
        .map(|(_, exposure)| exposure.clone())
}
fn build_visible_edges_and_structure_handles(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    u: &ActivationUtterance,
    c: &ProjectionActivationConfig,
    w: &mut Work,
) -> Result<(), ProjectionActivationViolation> {
    let mut visible_edges = Vec::<EdgeTuple>::new();
    for source in w.order.clone() {
        for edge in enumerate_edges(p, &source) {
            if visible(p, w, &edge.source)
                && visible(p, w, &edge.target)
                && !matches!(edge.target, SemanticAddress::RetrievalSurface(_))
                && !visible_edges.contains(&edge)
            {
                visible_edges.push(edge);
            }
        }
    }
    for edge in &visible_edges {
        let exposure = exposure_for_edge(w, edge).ok_or_else(|| {
            ProjectionActivationViolation::InvalidActivatedReference {
                context: format!(
                    "visible edge has no exposure provenance: source={:?} transition={} direction={:?} target={:?}",
                    edge.source, edge.transition_id, edge.direction, edge.target
                ),
            }
        })?;
        if exposure.activation_provenance.is_empty() {
            return Err(ProjectionActivationViolation::InvalidActivatedReference {
                context: format!(
                    "visible edge has empty exposure provenance: source={:?} transition={} direction={:?} target={:?}",
                    edge.source, edge.transition_id, edge.direction, edge.target
                ),
            });
        }
        if w.edges.len() >= c.maximum_activated_edges as usize {
            for probe_id in exposure.probe_ids {
                w.bounded_probe_ids.insert(probe_id);
            }
            continue;
        }
        let edge_id = next_id("activated-edge", &mut w.edge_counter)?;
        w.edges.push(ActivatedEdge {
            edge_id,
            source: edge.source.clone(),
            transition_id: edge.transition_id.clone(),
            direction: edge.direction.clone(),
            target: edge.target.clone(),
            activation_provenance: exposure.activation_provenance,
        });
    }
    for source in w.order.clone() {
        let all = enumerate_edges(p, &source);
        let degree =
            u64::try_from(all.len()).map_err(|_| ProjectionActivationViolation::CountOverflow)?;
        let mut grouped: Vec<(&String, &Direction, Vec<&EdgeTuple>)> = Vec::new();
        for edge in &all {
            if let Some((_, _, targets)) = grouped
                .iter_mut()
                .find(|(tid, dir, _)| *tid == &edge.transition_id && *dir == &edge.direction)
            {
                targets.push(edge);
            } else {
                grouped.push((&edge.transition_id, &edge.direction, vec![edge]));
            }
        }
        for (transition_id, direction, group) in grouped {
            let visible_count_usize = group
                .iter()
                .filter(|edge| visible(p, w, &edge.target))
                .count();
            if visible_count_usize >= group.len() && degree < c.hub_degree_threshold {
                continue;
            }
            let mut related = ExposureAggregate {
                probe_ids: vec![],
                activation_provenance: vec![],
            };
            if let Some(exposure) = exposure_for_address(w, &source) {
                merge_exposure(&mut related, exposure);
            }
            for edge in &group {
                if let Some(exposure) = exposure_for_edge(w, edge) {
                    merge_exposure(&mut related, exposure);
                }
            }
            let visible_count = u64::try_from(visible_count_usize)
                .map_err(|_| ProjectionActivationViolation::CountOverflow)?;
            let total = u64::try_from(group.len())
                .map_err(|_| ProjectionActivationViolation::CountOverflow)?;
            let remaining = total
                .checked_sub(visible_count)
                .ok_or(ProjectionActivationViolation::CountOverflow)?;
            if remaining == 0 {
                continue;
            }
            if related.activation_provenance.is_empty() {
                return Err(ProjectionActivationViolation::InvalidActivatedReference {
                    context: format!(
                        "structural continuation group has no exposure provenance: subject={:?} transition={} direction={:?}",
                        source, transition_id, direction
                    ),
                });
            }
            let key = if degree >= c.hub_degree_threshold {
                "high_degree_summary"
            } else {
                "bounded_structural_context"
            };
            for probe_id in &related.probe_ids {
                w.bounded_probe_ids.insert(probe_id.clone());
            }
            related
                .activation_provenance
                .retain(|provenance| !is_summary_policy_marker(provenance));
            push_unique(
                &mut related.activation_provenance,
                ActivationProvenance::ConfiguredDefault {
                    configuration_key: key.into(),
                },
            );
            if degree >= c.hub_degree_threshold {
                for telemetry in &mut w.telemetry {
                    if related.probe_ids.contains(&telemetry.probe_id) {
                        push_unique(
                            &mut telemetry.activation_provenance,
                            ActivationProvenance::ConfiguredDefault {
                                configuration_key: "high_degree_summary".into(),
                            },
                        );
                    }
                }
            }
            if c.continuation_page_limit == 0
                || w.handles.len() >= c.maximum_continuation_handles as usize
            {
                continue;
            }
            let handle_id = next_id("activation-continuation", &mut w.continuation_counter)?;
            for telemetry in &mut w.telemetry {
                if related.probe_ids.contains(&telemetry.probe_id) {
                    telemetry.continuation_available = true;
                }
            }
            w.handles.push(ContinuationHandle {
                handle_id,
                projection_snapshot_id: p.projection_snapshot_id.clone(),
                configuration_snapshot_id: c.configuration_snapshot_id.clone(),
                problem_space_thread_id: ps.thread_id.clone(),
                problem_space_version: ps.version,
                newest_utterance_id: u.utterance_id.clone(),
                origin: ContinuationOrigin::StructuralNeighbourhood {
                    subject: source.clone(),
                    transition_id: Some(transition_id.clone()),
                    direction: Some(direction.clone()),
                },
                access: ContinuationAccess::ProjectionStructure,
                filters: vec![ContinuationFilter::Transition {
                    transition_id: transition_id.clone(),
                }],
                ordering: ContinuationOrdering::ProjectionVectorOrder,
                next_offset: visible_count,
                remaining_count: Some(remaining),
                next_page_limit: c.continuation_page_limit,
                activation_provenance: related.activation_provenance.clone(),
            });
        }
    }
    for telemetry in &mut w.telemetry {
        if w.bounded_probe_ids.contains(&telemetry.probe_id) {
            telemetry.truncation_state = TruncationState::Bounded;
        }
    }
    Ok(())
}

fn is_summary_policy_marker(provenance: &ActivationProvenance) -> bool {
    matches!(
        provenance,
        ActivationProvenance::ConfiguredDefault { configuration_key }
            if configuration_key == "bounded_structural_context"
                || configuration_key == "high_degree_summary"
    )
}

fn edge_exists(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    tid: &str,
    dir: &Direction,
    target: &SemanticAddress,
) -> bool {
    enumerate_edges(p, source)
        .iter()
        .any(|e| e.transition_id == tid && &e.direction == dir && &e.target == target)
}
fn enumerate_edges(p: &SemanticSpaceProjection, source: &SemanticAddress) -> Vec<EdgeTuple> {
    let mut out = Vec::new();
    for transition in &p.valid_transitions {
        if transition.from == source.kind() {
            append_transition_edges(p, source, transition, &mut out);
        }
    }
    out
}
fn add_edge(
    out: &mut Vec<EdgeTuple>,
    source: &SemanticAddress,
    transition: &StructuralTransition,
    target: SemanticAddress,
) {
    if source.kind() != transition.from || target.kind() != transition.to {
        return;
    }
    let edge = EdgeTuple {
        source: source.clone(),
        transition_id: transition.transition_id.clone(),
        direction: transition.direction.clone(),
        target,
    };
    if !out.contains(&edge) {
        out.push(edge);
    }
}
fn append_transition_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    transition: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match transition.operation {
        StructuralTransitionOperation::Containment => {
            append_containment_edges(p, source, transition, out)
        }
        StructuralTransitionOperation::Parent => append_parent_edges(p, source, transition, out),
        StructuralTransitionOperation::Occurrence => {
            append_occurrence_edges(p, source, transition, out)
        }
        StructuralTransitionOperation::Identifier => {
            append_identifier_edges(p, source, transition, out)
        }
        StructuralTransitionOperation::TemporalAnchor => {
            append_temporal_edges(p, source, transition, out)
        }
        StructuralTransitionOperation::Hydration => {
            append_hydration_edges(p, source, transition, out)
        }
        StructuralTransitionOperation::RetrievalSurface => {
            append_retrieval_surface_edges(source, transition, out)
        }
    }
}
fn append_containment_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match source {
        SemanticAddress::Object(id) => {
            if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                match t.to {
                    AddressKind::SemanticRegion => {
                        for r in &o.region_addresses {
                            add_edge(out, source, t, SemanticAddress::Region(r.clone()));
                        }
                    }
                    AddressKind::SemanticUnit => {
                        for u in &o.unit_ids {
                            add_edge(out, source, t, SemanticAddress::Unit(u.clone()));
                        }
                    }
                    _ => {}
                }
            }
        }
        SemanticAddress::Region(id) => {
            if let Some(r) = p.regions.iter().find(|r| &r.address == id) {
                match t.to {
                    AddressKind::SemanticRegion => {
                        for r in &r.child_region_addresses {
                            add_edge(out, source, t, SemanticAddress::Region(r.clone()));
                        }
                    }
                    AddressKind::SemanticUnit => {
                        for u in &r.contained_unit_ids {
                            add_edge(out, source, t, SemanticAddress::Unit(u.clone()));
                        }
                    }
                    AddressKind::SemanticObject => add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Object(id.object_id.clone()),
                    ),
                    _ => {}
                }
            }
        }
        SemanticAddress::Unit(id)
            if t.to == AddressKind::SemanticObject || t.to == AddressKind::SemanticRegion =>
        {
            if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                if t.to == AddressKind::SemanticObject {
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Object(u.parent_object_id.clone()),
                    );
                } else {
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Region(u.parent_region_address.clone()),
                    );
                }
            }
        }
        _ => {}
    }
}
fn append_parent_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match source {
        SemanticAddress::Unit(id) => {
            if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                match t.to {
                    AddressKind::SemanticObject => add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Object(u.parent_object_id.clone()),
                    ),
                    AddressKind::SemanticRegion => add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Region(u.parent_region_address.clone()),
                    ),
                    _ => {}
                }
            }
        }
        SemanticAddress::Region(id) if t.to == AddressKind::SemanticObject => add_edge(
            out,
            source,
            t,
            SemanticAddress::Object(id.object_id.clone()),
        ),
        SemanticAddress::Object(id) => {
            if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                match t.to {
                    AddressKind::SemanticRegion => {
                        for r in &o.region_addresses {
                            add_edge(out, source, t, SemanticAddress::Region(r.clone()));
                        }
                    }
                    AddressKind::SemanticUnit => {
                        for u in &o.unit_ids {
                            add_edge(out, source, t, SemanticAddress::Unit(u.clone()));
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
fn append_occurrence_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match (source, &t.direction) {
        (SemanticAddress::Object(id), Direction::Outgoing) => {
            if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                for oid in o
                    .object_field_occurrence_ids
                    .iter()
                    .chain(o.body_occurrence_ids.iter())
                {
                    add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()));
                }
            }
        }
        (SemanticAddress::Object(id), Direction::Incoming) => {
            if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                for oid in &o.incoming_occurrence_ids {
                    add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()));
                }
            }
        }
        (SemanticAddress::Region(id), Direction::Incoming) => {
            if let Some(r) = p.regions.iter().find(|r| &r.address == id) {
                for oid in &r.incoming_occurrence_ids {
                    add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()));
                }
            }
        }
        (SemanticAddress::Unit(id), Direction::Outgoing) => {
            if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                for oid in &u.outgoing_occurrence_ids {
                    add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()));
                }
            }
        }
        (SemanticAddress::Unit(id), Direction::Incoming) => {
            if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                for oid in &u.incoming_occurrence_ids {
                    add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()));
                }
            }
        }
        (SemanticAddress::Occurrence(id), Direction::Outgoing) => {
            if let Some(o) = p.occurrences.iter().find(|o| &o.occurrence_id == id) {
                add_edge(out, source, t, o.resolved_target.clone());
            }
        }
        (SemanticAddress::Occurrence(id), Direction::Incoming) => {
            if let Some(o) = p.occurrences.iter().find(|o| &o.occurrence_id == id) {
                match &o.source {
                    OccurrenceSource::ObjectField { object_id, .. } => {
                        add_edge(out, source, t, SemanticAddress::Object(object_id.clone()))
                    }
                    OccurrenceSource::SemanticUnit { unit_id } => {
                        add_edge(out, source, t, SemanticAddress::Unit(unit_id.clone()))
                    }
                }
            }
        }
        _ => {}
    }
}
fn append_identifier_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match source {
        SemanticAddress::Identifier(_) => {
            if let Ok(assignment) = resolve_identifier_assignment(p, source) {
                add_edge(out, source, t, assignment.subject.clone());
            }
        }
        _ => {
            for assignment in p
                .identifier_assignments
                .iter()
                .filter(|assignment| &assignment.subject == source)
            {
                add_edge(
                    out,
                    source,
                    t,
                    SemanticAddress::Identifier(crate::model::IdentifierAddress {
                        identifier_name: assignment.identifier_name.clone(),
                        represented_value: identifier_value_string(&assignment.value),
                    }),
                );
            }
        }
    }
}
fn append_temporal_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match source {
        SemanticAddress::Object(_) | SemanticAddress::Unit(_) => {
            for anchor in p
                .temporal_anchors
                .iter()
                .filter(|anchor| &anchor.subject == source)
            {
                add_edge(
                    out,
                    source,
                    t,
                    SemanticAddress::TemporalAnchor(anchor.anchor_id.clone()),
                );
            }
        }
        SemanticAddress::TemporalAnchor(id) => {
            if let Some(anchor) = p
                .temporal_anchors
                .iter()
                .find(|anchor| &anchor.anchor_id == id)
            {
                add_edge(out, source, t, anchor.subject.clone());
            }
        }
        _ => {}
    }
}
fn append_hydration_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match source {
        SemanticAddress::Occurrence(id) => {
            if let Some(occurrence) = p
                .occurrences
                .iter()
                .find(|occurrence| &occurrence.occurrence_id == id)
            {
                if matches!(occurrence.resolved_target, SemanticAddress::Unit(_)) {
                    add_edge(out, source, t, occurrence.resolved_target.clone());
                }
            }
        }
        SemanticAddress::Unit(id) => add_edge(out, source, t, SemanticAddress::Unit(id.clone())),
        _ => {}
    }
}
fn append_retrieval_surface_edges(
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    if t.to == AddressKind::RetrievalSurface {
        if let Some(surface_id) = &t.retrieval_surface_id {
            add_edge(
                out,
                source,
                t,
                SemanticAddress::RetrievalSurface(RetrievalSurfaceAddress {
                    surface_id: surface_id.clone(),
                }),
            );
        }
    }
}

impl fmt::Display for ProjectionActivationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "projection activation violation: {self:?}")
    }
}
impl Error for ProjectionActivationViolation {}
