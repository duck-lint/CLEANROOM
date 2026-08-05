//! Deterministic initial projection activation runtime.
#![allow(clippy::collapsible_if, clippy::too_many_arguments)]
//!
//! This module executes the frozen activation contracts against one validated
//! [`SemanticSpaceProjection`]. It deliberately contains no retrieval provider:
//! callers supply a synchronous, read-only [`ProjectionActivationAccess`] seam
//! that receives typed probes and returns only mechanical projected identities.

use std::{collections::HashSet, error::Error, fmt};

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
    model::{Direction, RetrievalSurfaceKind, SemanticAddress},
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
    let mut probe_counter = 0u64;
    let mut root_index = 0usize;
    for seed in seeds {
        let mut root_seen_incidence = Vec::<SemanticAddress>::new();
        let mut root_seen_temporal = Vec::<SemanticAddress>::new();
        dispatch_source(
            projection,
            problem_space,
            config,
            access,
            &mut work,
            &mut probe_counter,
            &seed,
            ProjectionActivationProbeSource::Text {
                text: seed.text.clone(),
            },
            0,
            &mut root_seen_incidence,
            &mut root_seen_temporal,
        )?;
        root_index += 1;
        if root_index == usize::MAX {
            return Err(ProjectionActivationViolation::CountOverflow);
        }
    }
    build_visible_edges_and_structure_handles(projection, problem_space, config, &mut work)?;
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
                add_tension(t, id, &pb, &band, out, &mut used, max);
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
) {
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
                    candidate_index: i as u32,
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

fn dispatch_source<A: ProjectionActivationAccess + ?Sized>(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    c: &ProjectionActivationConfig,
    access: &A,
    w: &mut Work,
    probe_counter: &mut u64,
    seed: &Seed,
    source: ProjectionActivationProbeSource,
    depth: u32,
    seen_i: &mut Vec<SemanticAddress>,
    seen_t: &mut Vec<SemanticAddress>,
) -> Result<(), ProjectionActivationViolation> {
    for s in p.retrieval_surfaces.iter().filter(|s| s.available) {
        for mode in &s.match_modes {
            if !compatible(access, &s.surface_id, mode, &source)? {
                continue;
            }
            if !capable(p, s, &source) {
                continue;
            }
            let probe_id = next_id("activation-probe", probe_counter)?;
            let mut prov = seed.provenance.clone();
            push_unique(
                &mut prov,
                ActivationProvenance::ConfiguredDefault {
                    configuration_key: "automatic_surface_fan_out".into(),
                },
            );
            let probe = ProjectionActivationProbe {
                probe_id: probe_id.clone(),
                band: seed.band.clone(),
                surface_id: s.surface_id.clone(),
                surface_kind: s.kind.clone(),
                match_mode: mode.clone(),
                source: source.clone(),
                candidate_limit: surface_limit(c, &s.surface_id, &seed.band),
                current_depth: depth,
                activation_provenance: prov.clone(),
            };
            if w.telemetry.len() as u32 >= c.maximum_telemetry_records {
                return Err(ProjectionActivationViolation::ActivatedViewBoundExceeded {
                    kind: ActivatedRecordKind::Telemetry,
                    actual: w.telemetry.len() as u64 + 1,
                    maximum: c.maximum_telemetry_records,
                });
            }
            let res = access.execute_probe(p, &probe).map_err(|e| {
                ProjectionActivationViolation::SurfaceAccessFailed {
                    surface_id: s.surface_id.clone(),
                    probe_id: probe_id.clone(),
                    context: e.context,
                }
            })?;
            validate_result(p, s, &probe, &res)?;
            let before = count_records(w);
            let mut bounded = false;
            let returned = res.candidates.len() as u64;
            for cand in &res.candidates {
                let mut cp = prov.clone();
                if let Some(reg) = &seed.region_id {
                    for rel in ps.relations.iter().filter(|r| {
                        r.lifecycle == RecordLifecycle::Active
                            && (r.source_region_id == *reg
                                || r.target_region_id.as_ref() == Some(reg))
                    }) {
                        if matches!(source, ProjectionActivationProbeSource::Address { .. }) {
                            push_unique(
                                &mut cp,
                                ActivationProvenance::ProblemRelation {
                                    relation_id: rel.relation_id.clone(),
                                },
                            );
                        }
                    }
                }
                if !insert_bundle(p, c, w, &cand.address, &cp)? {
                    bounded = true;
                    continue;
                }
                if matches!(source, ProjectionActivationProbeSource::Text { .. })
                    || matches!(source, ProjectionActivationProbeSource::Address { .. })
                {
                    if depth < c.maximum_initial_relation_depth && !seen_i.contains(&cand.address) {
                        seen_i.push(cand.address.clone());
                        dispatch_source(
                            p,
                            ps,
                            c,
                            access,
                            w,
                            probe_counter,
                            seed,
                            ProjectionActivationProbeSource::Address {
                                address: cand.address.clone(),
                            },
                            depth + 1,
                            seen_i,
                            seen_t,
                        )?;
                    }
                    if is_temporal_probe_root(&cand.address) && !seen_t.contains(&cand.address) {
                        seen_t.push(cand.address.clone());
                        dispatch_source(
                            p,
                            ps,
                            c,
                            access,
                            w,
                            probe_counter,
                            seed,
                            ProjectionActivationProbeSource::Temporal {
                                address: cand.address.clone(),
                            },
                            depth,
                            seen_i,
                            seen_t,
                        )?;
                    }
                }
            }
            let handle_emitted = add_surface_handle(p, ps, c, w, &probe, &res)?;
            if res.continuation.is_some() && !handle_emitted {
                bounded = true;
            }
            if count_records(w) == before && returned > 0 {
                bounded = true;
            }
            let state = if bounded || w.bounded_probe_ids.contains(&probe_id) {
                TruncationState::Bounded
            } else {
                TruncationState::Complete
            };
            let tid = format!("activation-telemetry:{}", w.telemetry.len());
            w.telemetry.push(ProjectionTelemetry {
                telemetry_id: tid,
                probe_id,
                match_mode: probe.match_mode,
                surface_kind: probe.surface_kind,
                surface_id: probe.surface_id,
                candidate_count: res.candidate_count,
                current_depth: depth,
                maximum_depth: c.maximum_initial_relation_depth,
                returned_count: returned,
                remaining_expansion_budget: c.maximum_expansion_budget,
                truncation_state: state,
                identifier_type_distribution: res.identifier_type_distribution,
                temporal_anchor_count: res.temporal_anchor_count,
                unresolved_target_count: res.unresolved_target_count,
                continuation_available: handle_emitted,
                activation_provenance: probe.activation_provenance,
            });
        }
    }
    Ok(())
}
fn count_records(w: &Work) -> usize {
    w.objects.len()
        + w.regions.len()
        + w.units.len()
        + w.assignments.len()
        + w.occurrences.len()
        + w.anchors.len()
}
fn compatible<A: ProjectionActivationAccess + ?Sized>(
    access: &A,
    sid: &str,
    m: &SurfaceMatchMode,
    src: &ProjectionActivationProbeSource,
) -> Result<bool, ProjectionActivationViolation> {
    let k = match src {
        ProjectionActivationProbeSource::Text { .. } => ProjectionActivationProbeSourceKind::Text,
        ProjectionActivationProbeSource::Address { .. } => {
            ProjectionActivationProbeSourceKind::Address
        }
        ProjectionActivationProbeSource::Temporal { .. } => {
            ProjectionActivationProbeSourceKind::Temporal
        }
    };
    Ok(match (m, &k) {
        (
            SurfaceMatchMode::Literal
            | SurfaceMatchMode::Terms
            | SurfaceMatchMode::NearestNeighbours,
            ProjectionActivationProbeSourceKind::Text,
        ) => true,
        (SurfaceMatchMode::Incidence, ProjectionActivationProbeSourceKind::Address) => true,
        (SurfaceMatchMode::Temporal, ProjectionActivationProbeSourceKind::Temporal) => true,
        (SurfaceMatchMode::Declared { name }, _) => access
            .declared_mode_source(sid, name)
            .is_some_and(|dk| dk == k),
        _ => false,
    })
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
    if res.candidates.len() as u32 > probe.candidate_limit {
        return Err(fail("returned candidate count exceeds probe limit".into()));
    }
    if probe.candidate_limit == 0 && !res.candidates.is_empty() {
        return Err(fail("zero-limit probe returned candidates".into()));
    }
    if candidate_count_value(&res.candidate_count) < res.candidates.len() as u64 {
        return Err(fail(
            "candidate count smaller than returned candidates".into(),
        ));
    }
    let mut seen = HashSet::new();
    for cand in &res.candidates {
        if !seen.insert(format!("{:?}", cand.address)) {
            return Err(fail("duplicate candidate address".into()));
        }
        if cand.address.kind() != s.returned_identity {
            return Err(fail("returned address kind mismatch".into()));
        }
        if !address_exists(p, &cand.address) {
            return Err(fail(
                "candidate address does not exist in projection".into(),
            ));
        }
        let inc = matches!(probe.match_mode, SurfaceMatchMode::Incidence);
        if inc != cand.transition.is_some() {
            return Err(fail("incidence transition presence mismatch".into()));
        }
        if let Some(tr) = &cand.transition {
            if !edge_exists(
                p,
                &match &probe.source {
                    ProjectionActivationProbeSource::Address { address } => address.clone(),
                    _ => cand.address.clone(),
                },
                &tr.transition_id,
                &tr.direction,
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
        if cont.next_offset != res.candidates.len() as u64 {
            return Err(fail("initial continuation next_offset mismatch".into()));
        }
        if let CandidateCount::Exact(total) = res.candidate_count {
            if total < cont.next_offset + cont.remaining_count.unwrap_or(0) {
                return Err(fail("exact total incompatible with continuation".into()));
            }
        }
    }
    Ok(())
}
fn address_exists(p: &SemanticSpaceProjection, a: &SemanticAddress) -> bool {
    match a {
        SemanticAddress::Object(id) => p.objects.iter().any(|r| &r.object_id == id),
        SemanticAddress::Region(id) => p.regions.iter().any(|r| &r.address == id),
        SemanticAddress::Unit(id) => p.units.iter().any(|r| &r.unit_id == id),
        SemanticAddress::Occurrence(id) => p.occurrences.iter().any(|r| &r.occurrence_id == id),
        SemanticAddress::TemporalAnchor(id) => {
            p.temporal_anchors.iter().any(|r| &r.anchor_id == id)
        }
        SemanticAddress::Identifier(ia) => {
            p.identifier_assignments
                .iter()
                .filter(|x| {
                    x.identifier_name == ia.identifier_name
                        && ia
                            .represented_value
                            .as_ref()
                            .is_none_or(|v| identifier_value_string(&x.value).as_ref() == Some(v))
                })
                .count()
                == 1
        }
        SemanticAddress::RetrievalSurface(_) => false,
    }
}
fn identifier_value_string(v: &IdentifierValue) -> Option<String> {
    match v {
        IdentifierValue::String(s) => Some(s.clone()),
        IdentifierValue::Integer(i) => Some(i.to_string()),
        IdentifierValue::Boolean(b) => Some(b.to_string()),
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

fn insert_bundle(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    a: &SemanticAddress,
    prov: &[ActivationProvenance],
) -> Result<bool, ProjectionActivationViolation> {
    let mut need = Vec::new();
    closure_addresses(p, a, &mut need)?;
    let no = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::Object(_)) && !has_object(w, x))
        .count();
    let nr = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::Region(_)) && !has_region(w, x))
        .count();
    let nu = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::Unit(_)) && !has_unit(w, x))
        .count();
    let ni = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::Identifier(_)) && !has_assignment(w, x, p))
        .count();
    let noc = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::Occurrence(_)) && !has_occurrence(w, x))
        .count();
    let na = need
        .iter()
        .filter(|x| matches!(x, SemanticAddress::TemporalAnchor(_)) && !has_anchor(w, x))
        .count();
    if w.objects.len() + no > c.maximum_activated_objects as usize
        || w.regions.len() + nr > c.maximum_activated_regions as usize
        || w.units.len() + nu > c.maximum_activated_units as usize
        || w.assignments.len() + ni > c.maximum_activated_identifier_assignments as usize
        || w.occurrences.len() + noc > c.maximum_activated_occurrences as usize
        || w.anchors.len() + na > c.maximum_activated_temporal_anchors as usize
    {
        return Ok(false);
    }
    for addr in need {
        insert_one(p, c, w, &addr, prov)?;
    }
    Ok(true)
}
fn closure_addresses(
    p: &SemanticSpaceProjection,
    a: &SemanticAddress,
    out: &mut Vec<SemanticAddress>,
) -> Result<(), ProjectionActivationViolation> {
    push_unique(out, a.clone());
    match a {
        SemanticAddress::Region(r) => {
            push_unique(out, SemanticAddress::Object(r.object_id.clone()))
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
        }
        SemanticAddress::Identifier(ia) => {
            let ass =
                p.identifier_assignments
                    .iter()
                    .find(|x| {
                        x.identifier_name == ia.identifier_name
                            && ia.represented_value.as_ref().is_none_or(|v| {
                                identifier_value_string(&x.value).as_ref() == Some(v)
                            })
                    })
                    .ok_or_else(
                        || ProjectionActivationViolation::InvalidActivatedReference {
                            context: "identifier assignment missing".into(),
                        },
                    )?;
            closure_addresses(p, &ass.subject.clone(), out)?;
        }
        SemanticAddress::TemporalAnchor(id) => {
            let ta = p
                .temporal_anchors
                .iter()
                .find(|x| &x.anchor_id == id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: "temporal anchor missing".into(),
                    },
                )?;
            closure_addresses(p, &ta.subject.clone(), out)?;
        }
        SemanticAddress::Occurrence(id) => {
            let o = p
                .occurrences
                .iter()
                .find(|o| &o.occurrence_id == id)
                .ok_or_else(
                    || ProjectionActivationViolation::InvalidActivatedReference {
                        context: "occurrence missing".into(),
                    },
                )?;
            match &o.source {
                OccurrenceSource::ObjectField { object_id, .. } => {
                    push_unique(out, SemanticAddress::Object(object_id.clone()))
                }
                OccurrenceSource::SemanticUnit { unit_id } => {
                    closure_addresses(p, &SemanticAddress::Unit(unit_id.clone()), out)?
                }
            };
            closure_addresses(p, &o.resolved_target.clone(), out)?;
        }
        _ => {}
    }
    Ok(())
}
fn insert_one(
    p: &SemanticSpaceProjection,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    a: &SemanticAddress,
    prov: &[ActivationProvenance],
) -> Result<(), ProjectionActivationViolation> {
    match a {
        SemanticAddress::Object(id) => {
            if let Some(r) = w.objects.iter_mut().find(|r| &r.object_id == id) {
                merge(&mut r.activation_provenance, prov);
                enrich_object(p, w, id, c, prov)?;
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
                    activation_provenance: prov.to_vec(),
                });
                enrich_object(p, w, id, c, prov)?;
            }
        }
        SemanticAddress::Region(id) => {
            if let Some(r) = w.regions.iter_mut().find(|r| &r.address == id) {
                merge(&mut r.activation_provenance, prov);
                enrich_region(p, w, id, c, prov)?;
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
                    activation_provenance: prov.to_vec(),
                });
                enrich_region(p, w, id, c, prov)?;
            }
        }
        SemanticAddress::Unit(id) => {
            if let Some(r) = w.units.iter_mut().find(|r| &r.unit_id == id) {
                merge(&mut r.activation_provenance, prov);
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
                        band_config(c, &ProjectionActivationProbeBand::Unbanded)
                            .text_preview_character_limit,
                    ),
                    incoming_occurrence_count: u.incoming_occurrence_ids.len() as u64,
                    outgoing_occurrence_count: u.outgoing_occurrence_ids.len() as u64,
                    temporal_anchor_count: u.temporal_anchor_ids.len() as u64,
                    available_surface_ids: u.retrieval_surface_ids.clone(),
                    activation_provenance: prov.to_vec(),
                });
            }
        }
        SemanticAddress::Identifier(ia) => {
            if !has_assignment(w, a, p) {
                let ass = p
                    .identifier_assignments
                    .iter()
                    .find(|x| {
                        x.identifier_name == ia.identifier_name
                            && ia.represented_value.as_ref().is_none_or(|v| {
                                identifier_value_string(&x.value).as_ref() == Some(v)
                            })
                    })
                    .unwrap();
                w.order.push(a.clone());
                w.assignments.push(ActivatedIdentifierAssignmentRecord {
                    assignment_id: ass.assignment_id.clone(),
                    identifier_name: ass.identifier_name.clone(),
                    subject: ass.subject.clone(),
                    value: ass.value.clone(),
                    record_provenance: ass.provenance.clone(),
                    available_surface_ids: record_surfaces(p, a).cloned().unwrap_or_default(),
                    activation_provenance: prov.to_vec(),
                });
            }
        }
        SemanticAddress::Occurrence(id) => {
            if let Some(r) = w.occurrences.iter_mut().find(|r| &r.occurrence_id == id) {
                merge(&mut r.activation_provenance, prov);
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
                    available_surface_ids: vec![],
                    activation_provenance: prov.to_vec(),
                });
            }
        }
        SemanticAddress::TemporalAnchor(id) => {
            if let Some(r) = w.anchors.iter_mut().find(|r| &r.anchor_id == id) {
                merge(&mut r.activation_provenance, prov);
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
                    available_surface_ids: vec![],
                    activation_provenance: prov.to_vec(),
                });
            }
        }
        SemanticAddress::RetrievalSurface(_) => {}
    }
    Ok(())
}
fn merge<T: Clone + PartialEq>(v: &mut Vec<T>, add: &[T]) {
    for x in add {
        push_unique(v, x.clone());
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
fn has_object(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a,SemanticAddress::Object(id) if w.objects.iter().any(|r|&r.object_id==id))
}
fn has_region(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a,SemanticAddress::Region(id) if w.regions.iter().any(|r|&r.address==id))
}
fn has_unit(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a,SemanticAddress::Unit(id) if w.units.iter().any(|r|&r.unit_id==id))
}
fn has_occurrence(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a,SemanticAddress::Occurrence(id) if w.occurrences.iter().any(|r|&r.occurrence_id==id))
}
fn has_anchor(w: &Work, a: &SemanticAddress) -> bool {
    matches!(a,SemanticAddress::TemporalAnchor(id) if w.anchors.iter().any(|r|&r.anchor_id==id))
}
fn has_assignment(w: &Work, a: &SemanticAddress, p: &SemanticSpaceProjection) -> bool {
    if let SemanticAddress::Identifier(ia) = a {
        if let Some(ass) = p.identifier_assignments.iter().find(|x| {
            x.identifier_name == ia.identifier_name
                && ia
                    .represented_value
                    .as_ref()
                    .is_none_or(|v| identifier_value_string(&x.value).as_ref() == Some(v))
        }) {
            return w
                .assignments
                .iter()
                .any(|r| r.assignment_id == ass.assignment_id);
        }
    }
    false
}
fn enrich_object(
    p: &SemanticSpaceProjection,
    w: &mut Work,
    id: &crate::model::SemanticObjectId,
    c: &ProjectionActivationConfig,
    prov: &[ActivationProvenance],
) -> Result<(), ProjectionActivationViolation> {
    let o = p.objects.iter().find(|o| &o.object_id == id).unwrap();
    let limit = c.unbanded.maximum_structural_neighbors_per_record as usize;
    let mut visible_regions = Vec::new();
    let mut visible_units = Vec::new();
    let mut used = 0;
    for r in &o.region_addresses {
        if used >= limit {
            break;
        }
        if w.regions.iter().any(|x| &x.address == r) {
            visible_regions.push(r.clone());
            used += 1;
        }
    }
    for u in &o.unit_ids {
        if used >= limit {
            break;
        }
        if w.units.iter().any(|x| &x.unit_id == u) {
            visible_units.push(u.clone());
            used += 1;
        }
    }
    if let Some(rec) = w.objects.iter_mut().find(|r| &r.object_id == id) {
        for r in visible_regions {
            push_unique(&mut rec.visible_region_addresses, r)
        }
        for u in visible_units {
            push_unique(&mut rec.visible_unit_ids, u)
        }
    }
    for aid in &o.identifier_assignment_ids {
        if w.assignments.len() >= c.maximum_activated_identifier_assignments as usize {
            break;
        }
        if let Some(ass) = p
            .identifier_assignments
            .iter()
            .find(|a| &a.assignment_id == aid)
        {
            insert_one(
                p,
                c,
                w,
                &SemanticAddress::Identifier(crate::model::IdentifierAddress {
                    identifier_name: ass.identifier_name.clone(),
                    represented_value: identifier_value_string(&ass.value),
                }),
                &with_context(prov),
            )?;
            if let Some(rec) = w.objects.iter_mut().find(|r| &r.object_id == id) {
                push_unique(&mut rec.visible_identifier_assignment_ids, aid.clone())
            }
        }
    }
    Ok(())
}
fn enrich_region(
    p: &SemanticSpaceProjection,
    w: &mut Work,
    id: &crate::model::SemanticRegionAddress,
    c: &ProjectionActivationConfig,
    prov: &[ActivationProvenance],
) -> Result<(), ProjectionActivationViolation> {
    let r = p.regions.iter().find(|r| &r.address == id).unwrap();
    let mut ids = Vec::new();
    for uid in r
        .contained_unit_ids
        .iter()
        .take(c.unbanded.maximum_visible_units_per_region as usize)
    {
        if w.units.iter().any(|u| &u.unit_id == uid) {
            ids.push(uid.clone())
        }
    }
    if let Some(rec) = w.regions.iter_mut().find(|x| &x.address == id) {
        for uid in ids {
            push_unique(&mut rec.visible_unit_ids, uid)
        }
    }
    for aid in &r.inherited_identifier_assignment_ids {
        if w.assignments.len() >= c.maximum_activated_identifier_assignments as usize {
            break;
        }
        if let Some(ass) = p
            .identifier_assignments
            .iter()
            .find(|a| &a.assignment_id == aid)
        {
            insert_one(
                p,
                c,
                w,
                &SemanticAddress::Identifier(crate::model::IdentifierAddress {
                    identifier_name: ass.identifier_name.clone(),
                    represented_value: identifier_value_string(&ass.value),
                }),
                &with_context(prov),
            )?;
            if let Some(rec) = w.regions.iter_mut().find(|x| &x.address == id) {
                push_unique(&mut rec.visible_identifier_assignment_ids, aid.clone())
            }
        }
    }
    Ok(())
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

fn add_surface_handle(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    c: &ProjectionActivationConfig,
    w: &mut Work,
    probe: &ProjectionActivationProbe,
    res: &ProjectionActivationProbeResult,
) -> Result<bool, ProjectionActivationViolation> {
    let Some(cont) = &res.continuation else {
        return Ok(false);
    };
    if c.continuation_page_limit == 0 {
        return Ok(false);
    };
    if w.handles.len() >= c.maximum_continuation_handles as usize {
        return Ok(false);
    };
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
    let handle_id = format!("activation-continuation:{}", w.handles.len());
    w.handles.push(ContinuationHandle {
        handle_id,
        projection_snapshot_id: p.projection_snapshot_id.clone(),
        configuration_snapshot_id: c.configuration_snapshot_id.clone(),
        problem_space_thread_id: ps.thread_id.clone(),
        problem_space_version: ps.version,
        newest_utterance_id: probe
            .activation_provenance
            .iter()
            .find_map(|x| {
                if let ActivationProvenance::NewestUtterance { utterance_id } = x {
                    Some(utterance_id.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default(),
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

fn build_visible_edges_and_structure_handles(
    p: &SemanticSpaceProjection,
    ps: &ProblemSpaceState,
    c: &ProjectionActivationConfig,
    w: &mut Work,
) -> Result<(), ProjectionActivationViolation> {
    let order = w.order.clone();
    let mut tuples = Vec::new();
    for source in order {
        for e in enumerate_edges(p, &source) {
            if visible(w, &e.source) && visible(w, &e.target) && !tuples.contains(&e) {
                tuples.push(e)
            }
        }
    }
    for e in tuples {
        if w.edges.len() >= c.maximum_activated_edges as usize {
            break;
        }
        let id = format!("activated-edge:{}", w.edges.len());
        w.edges.push(ActivatedEdge {
            edge_id: id,
            source: e.source,
            transition_id: e.transition_id,
            direction: e.direction,
            target: e.target,
            activation_provenance: vec![ActivationProvenance::ConfiguredDefault {
                configuration_key: "bounded_structural_context".into(),
            }],
        });
    }
    if c.continuation_page_limit > 0 {
        for source in w.order.clone() {
            let all = enumerate_edges(p, &source);
            let visible_count = all
                .iter()
                .filter(|e| visible(w, &e.source) && visible(w, &e.target))
                .count() as u64;
            if all.len() as u64 > visible_count
                && w.handles.len() < c.maximum_continuation_handles as usize
            {
                if let Some(e) = all.first() {
                    let degree = all.len() as u64;
                    let key = if degree >= c.hub_degree_threshold {
                        "high_degree_summary"
                    } else {
                        "bounded_structural_context"
                    };
                    let id = format!("activation-continuation:{}", w.handles.len());
                    w.handles.push(ContinuationHandle {
                        handle_id: id,
                        projection_snapshot_id: p.projection_snapshot_id.clone(),
                        configuration_snapshot_id: c.configuration_snapshot_id.clone(),
                        problem_space_thread_id: ps.thread_id.clone(),
                        problem_space_version: ps.version,
                        newest_utterance_id: String::new(),
                        origin: ContinuationOrigin::StructuralNeighbourhood {
                            subject: source.clone(),
                            transition_id: Some(e.transition_id.clone()),
                            direction: Some(e.direction.clone()),
                        },
                        access: ContinuationAccess::ProjectionStructure,
                        filters: vec![ContinuationFilter::Transition {
                            transition_id: e.transition_id.clone(),
                        }],
                        ordering: ContinuationOrdering::ProjectionVectorOrder,
                        next_offset: visible_count,
                        remaining_count: Some(all.len() as u64 - visible_count),
                        next_page_limit: c.continuation_page_limit,
                        activation_provenance: vec![ActivationProvenance::ConfiguredDefault {
                            configuration_key: key.into(),
                        }],
                    });
                }
            }
        }
    }
    Ok(())
}
fn visible(w: &Work, a: &SemanticAddress) -> bool {
    has_object(w, a)
        || has_region(w, a)
        || has_unit(w, a)
        || has_assignment(
            w,
            a,
            &SemanticSpaceProjection {
                projection_snapshot_id: String::new(),
                ingest_identity: String::new(),
                schema_version: String::new(),
                logical_hash: String::new(),
                corpus_snapshot_identity: String::new(),
                configuration_snapshot_id: String::new(),
                validation_status: ProjectionValidationStatus::Validated,
                object_classes: vec![],
                objects: vec![],
                regions: vec![],
                units: vec![],
                identifier_descriptors: vec![],
                identifier_assignments: vec![],
                occurrences: vec![],
                temporal_anchors: vec![],
                retrieval_surfaces: vec![],
                valid_transitions: vec![],
            },
        )
        || has_occurrence(w, a)
        || has_anchor(w, a)
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
    for t in &p.valid_transitions {
        if t.from != source.kind() {
            continue;
        }
        append_transition_edges(p, source, t, &mut out);
    }
    out
}
fn add_edge(
    out: &mut Vec<EdgeTuple>,
    source: &SemanticAddress,
    t: &StructuralTransition,
    target: SemanticAddress,
) {
    let e = EdgeTuple {
        source: source.clone(),
        transition_id: t.transition_id.clone(),
        direction: t.direction.clone(),
        target,
    };
    if !out.contains(&e) {
        out.push(e)
    }
}
fn append_transition_edges(
    p: &SemanticSpaceProjection,
    source: &SemanticAddress,
    t: &StructuralTransition,
    out: &mut Vec<EdgeTuple>,
) {
    match t.operation {
        StructuralTransitionOperation::Containment => match source {
            SemanticAddress::Object(id) => {
                if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                    for r in &o.region_addresses {
                        add_edge(out, source, t, SemanticAddress::Region(r.clone()))
                    }
                    for u in &o.unit_ids {
                        add_edge(out, source, t, SemanticAddress::Unit(u.clone()))
                    }
                }
            }
            SemanticAddress::Region(id) => {
                if let Some(r) = p.regions.iter().find(|r| &r.address == id) {
                    for cr in &r.child_region_addresses {
                        add_edge(out, source, t, SemanticAddress::Region(cr.clone()))
                    }
                    for u in &r.contained_unit_ids {
                        add_edge(out, source, t, SemanticAddress::Unit(u.clone()))
                    }
                }
            }
            _ => {}
        },
        StructuralTransitionOperation::Parent => match source {
            SemanticAddress::Unit(id) => {
                if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Object(u.parent_object_id.clone()),
                    );
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Region(u.parent_region_address.clone()),
                    );
                }
            }
            SemanticAddress::Region(id) => add_edge(
                out,
                source,
                t,
                SemanticAddress::Object(id.object_id.clone()),
            ),
            _ => {}
        },
        StructuralTransitionOperation::Occurrence => match source {
            SemanticAddress::Object(id) => {
                if let Some(o) = p.objects.iter().find(|o| &o.object_id == id) {
                    for oid in o
                        .object_field_occurrence_ids
                        .iter()
                        .chain(o.body_occurrence_ids.iter())
                        .chain(o.incoming_occurrence_ids.iter())
                    {
                        add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()))
                    }
                }
            }
            SemanticAddress::Unit(id) => {
                if let Some(u) = p.units.iter().find(|u| &u.unit_id == id) {
                    for oid in u
                        .outgoing_occurrence_ids
                        .iter()
                        .chain(u.incoming_occurrence_ids.iter())
                    {
                        add_edge(out, source, t, SemanticAddress::Occurrence(oid.clone()))
                    }
                }
            }
            SemanticAddress::Occurrence(id) => {
                if let Some(o) = p.occurrences.iter().find(|o| &o.occurrence_id == id) {
                    add_edge(out, source, t, o.resolved_target.clone());
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
        },
        StructuralTransitionOperation::Identifier => {
            for ass in &p.identifier_assignments {
                if &ass.subject == source {
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::Identifier(crate::model::IdentifierAddress {
                            identifier_name: ass.identifier_name.clone(),
                            represented_value: identifier_value_string(&ass.value),
                        }),
                    );
                }
            }
        }
        StructuralTransitionOperation::TemporalAnchor => match source {
            SemanticAddress::Object(_) | SemanticAddress::Unit(_) => {
                for a in p.temporal_anchors.iter().filter(|a| &a.subject == source) {
                    add_edge(
                        out,
                        source,
                        t,
                        SemanticAddress::TemporalAnchor(a.anchor_id.clone()),
                    )
                }
            }
            SemanticAddress::TemporalAnchor(id) => {
                if let Some(a) = p.temporal_anchors.iter().find(|a| &a.anchor_id == id) {
                    add_edge(out, source, t, a.subject.clone())
                }
            }
            _ => {}
        },
        StructuralTransitionOperation::Hydration => {}
        StructuralTransitionOperation::RetrievalSurface => {}
    }
}

impl fmt::Display for ProjectionActivationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "projection activation violation: {self:?}")
    }
}
impl Error for ProjectionActivationViolation {}
