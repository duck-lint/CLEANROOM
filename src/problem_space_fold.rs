//! Pure, bounded application and replay of declared problem-space operations.
//!
//! This module implements `P_t = U(P_{t-1}, B_t)`. It validates and applies
//! declarations; it has no authority to infer, normalize, or repair semantics.

use crate::problem_space::*;
use std::{collections::HashSet, error::Error, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemSpaceFoldLimits {
    pub max_total_declarations_per_contribution: usize,
    pub max_operational_regions: usize,
    pub max_active_relations: usize,
    pub max_open_tensions: usize,
    pub max_background_regions: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemSpaceFoldOutput {
    pub state: ProblemSpaceState,
    pub accepted_log: BoundaryContributionLog,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProblemSpaceRecordKind {
    Region,
    Relation,
    Constraint,
    Tension,
    Referent,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcceptedIdentityKind {
    Contribution,
    SourceTurn,
    SourceUtterance,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProblemSpaceFoldBoundKind {
    TotalDeclarations,
    OperationalRegions,
    ActiveRelations,
    OpenTensions,
    BackgroundRegions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProblemSpaceFoldViolation {
    EmptyRequiredIdentity {
        context: &'static str,
    },
    InvalidFreshStateLogCombination,
    ThreadMismatch,
    StateVersionLogLengthMismatch,
    ContributionHistoryLogMismatch,
    SourceTurnRangeLogMismatch,
    AcceptedEntrySequenceMismatch {
        index: usize,
    },
    AcceptedEntryPriorVersionMismatch {
        index: usize,
    },
    DuplicateAcceptedIdentity {
        kind: AcceptedIdentityKind,
        identity: String,
    },
    ContributionDeclarationCountExcess,
    DuplicateRecordIdentity {
        kind: ProblemSpaceRecordKind,
        identity: String,
    },
    RecordIdentityAlreadyPresent {
        kind: ProblemSpaceRecordKind,
        identity: String,
    },
    MissingReferencedRecord {
        kind: ProblemSpaceRecordKind,
        identity: String,
    },
    InvalidRecordLifecycle {
        kind: ProblemSpaceRecordKind,
        identity: String,
    },
    InvalidMergeShape,
    InvalidSplitShape,
    InvalidSupersession,
    ContradictoryTerminalOperations {
        kind: ProblemSpaceRecordKind,
        identity: String,
    },
    InvalidConstraintApplicability,
    InvalidAttentionAssignment,
    InvalidPreservationDeclaration,
    InvalidReleaseDeclaration,
    UnsupportedSubjectReleaseModeCombination,
    FinalReferentialIntegrityFailure {
        context: &'static str,
    },
    ConfiguredFinalStateBoundExcess {
        kind: ProblemSpaceFoldBoundKind,
        actual: usize,
        maximum: usize,
    },
    StateVersionOverflow,
}
impl fmt::Display for ProblemSpaceFoldViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "problem-space fold violation: {self:?}")
    }
}
impl Error for ProblemSpaceFoldViolation {}

type V = ProblemSpaceFoldViolation;
fn operational(p: &RegionPersistenceState) -> bool {
    matches!(
        p,
        RegionPersistenceState::Active
            | RegionPersistenceState::Background
            | RegionPersistenceState::Unresolved
    )
}
fn add_once(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|v| v == value) {
        values.push(value.to_owned());
    }
}
fn region_pos(s: &ProblemSpaceState, id: &str) -> Result<usize, V> {
    s.regions
        .iter()
        .position(|x| x.region_id == id)
        .ok_or_else(|| V::MissingReferencedRecord {
            kind: ProblemSpaceRecordKind::Region,
            identity: id.into(),
        })
}
fn relation_pos(s: &ProblemSpaceState, id: &str) -> Result<usize, V> {
    s.relations
        .iter()
        .position(|x| x.relation_id == id)
        .ok_or_else(|| V::MissingReferencedRecord {
            kind: ProblemSpaceRecordKind::Relation,
            identity: id.into(),
        })
}
fn constraint_pos(s: &ProblemSpaceState, id: &str) -> Result<usize, V> {
    s.constraints
        .iter()
        .position(|x| x.constraint_id == id)
        .ok_or_else(|| V::MissingReferencedRecord {
            kind: ProblemSpaceRecordKind::Constraint,
            identity: id.into(),
        })
}
fn tension_pos(s: &ProblemSpaceState, id: &str) -> Result<usize, V> {
    s.open_tensions
        .iter()
        .position(|x| x.tension_id == id)
        .ok_or_else(|| V::MissingReferencedRecord {
            kind: ProblemSpaceRecordKind::Tension,
            identity: id.into(),
        })
}
fn nonempty(id: &str, context: &'static str) -> Result<(), V> {
    if id.is_empty() {
        Err(V::EmptyRequiredIdentity { context })
    } else {
        Ok(())
    }
}

fn transformations(c: &BoundaryContribution) -> Vec<BoundaryOperationKind> {
    let mut flags = [false; 12];
    for op in &c.region_operations {
        match op {
            RegionOperation::Create { .. } => flags[0] = true,
            RegionOperation::Preserve { .. } => flags[1] = true,
            RegionOperation::Reinforce { .. } => flags[2] = true,
            RegionOperation::Extend { .. } => flags[3] = true,
            RegionOperation::Merge { .. } => {
                flags[4] = true;
                flags[10] = true
            }
            RegionOperation::Split { .. } => {
                flags[5] = true;
                flags[10] = true
            }
            RegionOperation::Supersede { .. } => flags[10] = true,
            RegionOperation::Retire { .. } => flags[11] = true,
        }
    }
    if !c.relation_operations.is_empty() {
        flags[6] = true
    }
    if c.relation_operations
        .iter()
        .any(|x| matches!(x, RelationOperation::Disconnect { .. }))
    {
        flags[11] = true
    }
    if !c.constraint_operations.is_empty() {
        flags[7] = true
    }
    if c.constraint_operations
        .iter()
        .any(|x| matches!(x, ConstraintOperation::Replace { .. }))
    {
        flags[10] = true
    }
    if c.constraint_operations
        .iter()
        .any(|x| matches!(x, ConstraintOperation::Retire { .. }))
    {
        flags[11] = true
    }
    if !c.tension_operations.is_empty() {
        flags[8] = true
    }
    if c.tension_operations
        .iter()
        .any(|x| matches!(x, TensionOperation::Supersede { .. }))
    {
        flags[10] = true
    }
    if c.tension_operations
        .iter()
        .any(|x| matches!(x, TensionOperation::Abandon { .. }))
    {
        flags[11] = true
    }
    if !c.attention_operations.is_empty() {
        flags[9] = true
    }
    if !c.preservation_declarations.is_empty() {
        flags[1] = true
    }
    for r in &c.release_declarations {
        match r.mode {
            ReleaseMode::Supersede => flags[10] = true,
            ReleaseMode::Retire | ReleaseMode::Abandon => flags[11] = true,
        }
    }
    let all = [
        BoundaryOperationKind::Create,
        BoundaryOperationKind::Preserve,
        BoundaryOperationKind::Reinforce,
        BoundaryOperationKind::Extend,
        BoundaryOperationKind::Merge,
        BoundaryOperationKind::Split,
        BoundaryOperationKind::Relate,
        BoundaryOperationKind::Constrain,
        BoundaryOperationKind::Tension,
        BoundaryOperationKind::RedirectAttention,
        BoundaryOperationKind::Supersede,
        BoundaryOperationKind::Retire,
    ];
    all.into_iter()
        .enumerate()
        .filter_map(|(i, x)| flags[i].then_some(x))
        .collect()
}

fn validate_log(log: &BoundaryContributionLog) -> Result<(), V> {
    nonempty(&log.thread_id, "log thread_id")?;
    let mut c = HashSet::new();
    let mut t = HashSet::new();
    let mut u = HashSet::new();
    for (i, e) in log.entries.iter().enumerate() {
        let seq = u64::try_from(i + 1).map_err(|_| V::StateVersionOverflow)?;
        let prior = u64::try_from(i).map_err(|_| V::StateVersionOverflow)?;
        if e.sequence != seq {
            return Err(V::AcceptedEntrySequenceMismatch { index: i });
        }
        if e.prior_state_version != prior {
            return Err(V::AcceptedEntryPriorVersionMismatch { index: i });
        }
        for (id, set, kind) in [
            (
                &e.contribution.contribution_id,
                &mut c,
                AcceptedIdentityKind::Contribution,
            ),
            (
                &e.contribution.source_turn_id,
                &mut t,
                AcceptedIdentityKind::SourceTurn,
            ),
            (
                &e.contribution.source_utterance_id,
                &mut u,
                AcceptedIdentityKind::SourceUtterance,
            ),
        ] {
            nonempty(id, "accepted identity")?;
            if !set.insert(id) {
                return Err(V::DuplicateAcceptedIdentity {
                    kind,
                    identity: id.clone(),
                });
            }
        }
    }
    Ok(())
}
fn validate_pair(
    prior: Option<&ProblemSpaceState>,
    log: &BoundaryContributionLog,
) -> Result<(), V> {
    validate_log(log)?;
    match (prior, log.entries.is_empty()) {
        (None, true) => Ok(()),
        (Some(s), false) => {
            if s.thread_id != log.thread_id {
                return Err(V::ThreadMismatch);
            }
            if s.version != u64::try_from(log.entries.len()).map_err(|_| V::StateVersionOverflow)? {
                return Err(V::StateVersionLogLengthMismatch);
            }
            if s.contribution_history.len() != log.entries.len() {
                return Err(V::ContributionHistoryLogMismatch);
            }
            for (h, e) in s.contribution_history.iter().zip(&log.entries) {
                if h.contribution_id != e.contribution.contribution_id
                    || h.source_turn_id != e.contribution.source_turn_id
                    || h.transformations != transformations(&e.contribution)
                {
                    return Err(V::ContributionHistoryLogMismatch);
                }
            }
            if s.source_turn_range.first_turn_id != log.entries[0].contribution.source_turn_id
                || s.source_turn_range.last_turn_id
                    != log.entries.last().unwrap().contribution.source_turn_id
            {
                return Err(V::SourceTurnRangeLogMismatch);
            }
            Ok(())
        }
        _ => Err(V::InvalidFreshStateLogCombination),
    }
}
fn check_app(s: &ProblemSpaceState, a: &ProblemConstraintApplicability) -> Result<(), V> {
    if let ProblemConstraintApplicability::Regions { region_ids } = a {
        if region_ids.is_empty() {
            return Err(V::InvalidConstraintApplicability);
        }
        let mut seen = HashSet::new();
        for id in region_ids {
            if id.is_empty() || !seen.insert(id) {
                return Err(V::InvalidConstraintApplicability);
            }
            let p = region_pos(s, id)?;
            if !operational(&s.regions[p].persistence_state) {
                return Err(V::InvalidConstraintApplicability);
            }
        }
    }
    Ok(())
}
fn record_new(kind: ProblemSpaceRecordKind, id: &str, present: bool) -> Result<(), V> {
    nonempty(id, "record identity")?;
    if present {
        Err(V::RecordIdentityAlreadyPresent {
            kind,
            identity: id.into(),
        })
    } else {
        Ok(())
    }
}

pub fn fold_boundary_contribution(
    prior_state: Option<&ProblemSpaceState>,
    accepted_log: &BoundaryContributionLog,
    contribution: &BoundaryContribution,
    limits: &ProblemSpaceFoldLimits,
) -> Result<ProblemSpaceFoldOutput, V> {
    validate_pair(prior_state, accepted_log)?;
    nonempty(&contribution.contribution_id, "contribution_id")?;
    nonempty(&contribution.source_turn_id, "source_turn_id")?;
    nonempty(&contribution.source_utterance_id, "source_utterance_id")?;
    for e in &accepted_log.entries {
        for (a, b, k) in [
            (
                &e.contribution.contribution_id,
                &contribution.contribution_id,
                AcceptedIdentityKind::Contribution,
            ),
            (
                &e.contribution.source_turn_id,
                &contribution.source_turn_id,
                AcceptedIdentityKind::SourceTurn,
            ),
            (
                &e.contribution.source_utterance_id,
                &contribution.source_utterance_id,
                AcceptedIdentityKind::SourceUtterance,
            ),
        ] {
            if a == b {
                return Err(V::DuplicateAcceptedIdentity {
                    kind: k,
                    identity: b.clone(),
                });
            }
        }
    }
    let counts = [
        contribution.region_operations.len(),
        contribution.relation_operations.len(),
        contribution.constraint_operations.len(),
        contribution.tension_operations.len(),
        contribution.attention_operations.len(),
        contribution.preservation_declarations.len(),
        contribution.release_declarations.len(),
    ];
    let total = counts
        .into_iter()
        .try_fold(0usize, usize::checked_add)
        .ok_or(V::ContributionDeclarationCountExcess)?;
    if total > limits.max_total_declarations_per_contribution {
        return Err(V::ContributionDeclarationCountExcess);
    }
    let mut s = prior_state.cloned().unwrap_or_else(|| ProblemSpaceState {
        thread_id: accepted_log.thread_id.clone(),
        version: 0,
        regions: vec![],
        relations: vec![],
        constraints: vec![],
        open_tensions: vec![],
        contribution_history: vec![],
        attention_lens: AttentionLens {
            primary_region_ids: vec![],
            secondary_region_ids: vec![],
            tertiary_region_ids: vec![],
            background_region_ids: vec![],
        },
        source_turn_range: SourceTurnRange {
            first_turn_id: contribution.source_turn_id.clone(),
            last_turn_id: contribution.source_turn_id.clone(),
        },
    });
    let prior_snapshot = s.clone();
    let cid = &contribution.contribution_id;
    let mut region_terminal = HashSet::new();
    for op in &contribution.region_operations {
        match op {
            RegionOperation::Create { region } => {
                record_new(
                    ProblemSpaceRecordKind::Region,
                    &region.region_id,
                    s.regions.iter().any(|x| x.region_id == region.region_id),
                )?;
                if !operational(&region.persistence_state) || region.supersedes_region_id.is_some()
                {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region.region_id.clone(),
                    });
                }
                let mut x = region.clone();
                add_once(&mut x.source_contribution_ids, cid);
                s.regions.push(x)
            }
            RegionOperation::Preserve { region_id, .. } => {
                let p = prior_snapshot
                    .regions
                    .iter()
                    .position(|x| x.region_id == *region_id)
                    .ok_or_else(|| V::MissingReferencedRecord {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    })?;
                if !operational(&prior_snapshot.regions[p].persistence_state) {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    });
                }
                let q = region_pos(&s, region_id)?;
                if !operational(&s.regions[q].persistence_state) {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    });
                }
                add_once(&mut s.regions[q].source_contribution_ids, cid)
            }
            RegionOperation::Reinforce { region_id, .. } => {
                let p = region_pos(&s, region_id)?;
                if !operational(&s.regions[p].persistence_state) {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    });
                }
                add_once(&mut s.regions[p].source_contribution_ids, cid)
            }
            RegionOperation::Extend {
                region_id,
                referent,
            } => {
                nonempty(&referent.referent_id, "referent_id")?;
                if referent.source_contribution_id != *cid {
                    return Err(V::FinalReferentialIntegrityFailure {
                        context: "referent source contribution",
                    });
                }
                if s.regions
                    .iter()
                    .flat_map(|r| &r.anchor_referents)
                    .any(|x| x.referent_id == referent.referent_id)
                {
                    return Err(V::RecordIdentityAlreadyPresent {
                        kind: ProblemSpaceRecordKind::Referent,
                        identity: referent.referent_id.clone(),
                    });
                }
                let p = region_pos(&s, region_id)?;
                if !operational(&s.regions[p].persistence_state) {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    });
                }
                s.regions[p].anchor_referents.push(referent.clone());
                add_once(&mut s.regions[p].source_contribution_ids, cid)
            }
            RegionOperation::Merge {
                source_region_ids,
                resulting_region,
                ..
            } => {
                let unique: HashSet<_> = source_region_ids.iter().collect();
                if source_region_ids.len() < 2
                    || unique.len() != source_region_ids.len()
                    || !operational(&resulting_region.persistence_state)
                    || resulting_region.supersedes_region_id.is_some()
                {
                    return Err(V::InvalidMergeShape);
                }
                record_new(
                    ProblemSpaceRecordKind::Region,
                    &resulting_region.region_id,
                    s.regions
                        .iter()
                        .any(|x| x.region_id == resulting_region.region_id),
                )?;
                for id in source_region_ids {
                    let p = region_pos(&s, id)?;
                    if !operational(&s.regions[p].persistence_state)
                        || !region_terminal.insert(id.clone())
                    {
                        return Err(V::ContradictoryTerminalOperations {
                            kind: ProblemSpaceRecordKind::Region,
                            identity: id.clone(),
                        });
                    }
                    s.regions[p].persistence_state = RegionPersistenceState::Superseded;
                    add_once(&mut s.regions[p].source_contribution_ids, cid)
                }
                let mut x = resulting_region.clone();
                add_once(&mut x.source_contribution_ids, cid);
                s.regions.push(x)
            }
            RegionOperation::Split {
                source_region_id,
                resulting_regions,
                ..
            } => {
                if resulting_regions.len() < 2 {
                    return Err(V::InvalidSplitShape);
                }
                let mut ids = HashSet::new();
                for x in resulting_regions {
                    if !ids.insert(&x.region_id)
                        || !operational(&x.persistence_state)
                        || x.supersedes_region_id.as_deref() != Some(source_region_id)
                        || s.regions.iter().any(|r| r.region_id == x.region_id)
                    {
                        return Err(V::InvalidSplitShape);
                    }
                }
                let p = region_pos(&s, source_region_id)?;
                if !operational(&s.regions[p].persistence_state)
                    || !region_terminal.insert(source_region_id.clone())
                {
                    return Err(V::InvalidSplitShape);
                }
                s.regions[p].persistence_state = RegionPersistenceState::Superseded;
                add_once(&mut s.regions[p].source_contribution_ids, cid);
                for x in resulting_regions {
                    let mut x = x.clone();
                    add_once(&mut x.source_contribution_ids, cid);
                    s.regions.push(x)
                }
            }
            RegionOperation::Supersede {
                region_id,
                superseded_by_region_id,
                ..
            } => {
                if region_id == superseded_by_region_id {
                    return Err(V::InvalidSupersession);
                }
                let a = region_pos(&s, region_id)?;
                let b = region_pos(&s, superseded_by_region_id)?;
                let replacement_link_valid = s.regions[b]
                    .supersedes_region_id
                    .as_deref()
                    .is_none_or(|id| id == region_id);
                if !operational(&s.regions[a].persistence_state)
                    || !operational(&s.regions[b].persistence_state)
                    || !region_terminal.insert(region_id.clone())
                    || !replacement_link_valid
                {
                    return Err(V::InvalidSupersession);
                }
                s.regions[a].persistence_state = RegionPersistenceState::Superseded;
                s.regions[b].supersedes_region_id = Some(region_id.clone());
                add_once(&mut s.regions[a].source_contribution_ids, cid);
                add_once(&mut s.regions[b].source_contribution_ids, cid)
            }
            RegionOperation::Retire { region_id, .. } => {
                let p = region_pos(&s, region_id)?;
                if !operational(&s.regions[p].persistence_state)
                    || !region_terminal.insert(region_id.clone())
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: region_id.clone(),
                    });
                }
                s.regions[p].persistence_state = RegionPersistenceState::Retired;
                add_once(&mut s.regions[p].source_contribution_ids, cid)
            }
        }
    }
    let mut terminals = HashSet::new();
    for op in &contribution.relation_operations {
        match op {
            RelationOperation::Connect { relation } => {
                record_new(
                    ProblemSpaceRecordKind::Relation,
                    &relation.relation_id,
                    s.relations
                        .iter()
                        .any(|x| x.relation_id == relation.relation_id),
                )?;
                if relation.lifecycle != RecordLifecycle::Active
                    || relation.source_contribution_id != *cid
                {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Relation,
                        identity: relation.relation_id.clone(),
                    });
                }
                let p = region_pos(&s, &relation.source_region_id)?;
                if !operational(&s.regions[p].persistence_state) {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Region,
                        identity: relation.source_region_id.clone(),
                    });
                }
                if let Some(id) = &relation.target_region_id {
                    let p = region_pos(&s, id)?;
                    if !operational(&s.regions[p].persistence_state) {
                        return Err(V::InvalidRecordLifecycle {
                            kind: ProblemSpaceRecordKind::Region,
                            identity: id.clone(),
                        });
                    }
                }
                s.relations.push(relation.clone())
            }
            RelationOperation::Disconnect { relation_id, .. } => {
                let p = relation_pos(&s, relation_id)?;
                if s.relations[p].lifecycle != RecordLifecycle::Active
                    || !terminals.insert(relation_id)
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Relation,
                        identity: relation_id.clone(),
                    });
                }
                s.relations[p].lifecycle = RecordLifecycle::Retired
            }
        }
    }
    terminals.clear();
    for op in &contribution.constraint_operations {
        match op {
            ConstraintOperation::Add { constraint } => {
                record_new(
                    ProblemSpaceRecordKind::Constraint,
                    &constraint.constraint_id,
                    s.constraints
                        .iter()
                        .any(|x| x.constraint_id == constraint.constraint_id),
                )?;
                if constraint.lifecycle != RecordLifecycle::Active
                    || constraint.source_contribution_id != *cid
                {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Constraint,
                        identity: constraint.constraint_id.clone(),
                    });
                }
                check_app(&s, &constraint.applicability)?;
                s.constraints.push(constraint.clone())
            }
            ConstraintOperation::Replace {
                prior_constraint_id,
                replacement,
                ..
            } => {
                let p = constraint_pos(&s, prior_constraint_id)?;
                if s.constraints[p].lifecycle != RecordLifecycle::Active
                    || !terminals.insert(prior_constraint_id)
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Constraint,
                        identity: prior_constraint_id.clone(),
                    });
                }
                record_new(
                    ProblemSpaceRecordKind::Constraint,
                    &replacement.constraint_id,
                    s.constraints
                        .iter()
                        .any(|x| x.constraint_id == replacement.constraint_id),
                )?;
                if replacement.lifecycle != RecordLifecycle::Active
                    || replacement.source_contribution_id != *cid
                {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Constraint,
                        identity: replacement.constraint_id.clone(),
                    });
                }
                check_app(&s, &replacement.applicability)?;
                s.constraints[p].lifecycle = RecordLifecycle::Superseded;
                s.constraints.push(replacement.clone())
            }
            ConstraintOperation::Retire { constraint_id, .. } => {
                let p = constraint_pos(&s, constraint_id)?;
                if s.constraints[p].lifecycle != RecordLifecycle::Active
                    || !terminals.insert(constraint_id)
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Constraint,
                        identity: constraint_id.clone(),
                    });
                }
                s.constraints[p].lifecycle = RecordLifecycle::Retired
            }
        }
    }
    terminals.clear();
    for op in &contribution.tension_operations {
        match op {
            TensionOperation::Open { tension } => {
                record_new(
                    ProblemSpaceRecordKind::Tension,
                    &tension.tension_id,
                    s.open_tensions
                        .iter()
                        .any(|x| x.tension_id == tension.tension_id),
                )?;
                let p = region_pos(&s, &tension.region_id)?;
                if tension.lifecycle != TensionLifecycle::Open
                    || !operational(&s.regions[p].persistence_state)
                    || tension.source_turn_id != contribution.source_turn_id
                {
                    return Err(V::InvalidRecordLifecycle {
                        kind: ProblemSpaceRecordKind::Tension,
                        identity: tension.tension_id.clone(),
                    });
                }
                s.open_tensions.push(tension.clone())
            }
            TensionOperation::Resolve { tension_id, .. } => {
                let p = tension_pos(&s, tension_id)?;
                if s.open_tensions[p].lifecycle != TensionLifecycle::Open
                    || !terminals.insert(tension_id)
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Tension,
                        identity: tension_id.clone(),
                    });
                }
                s.open_tensions[p].lifecycle = TensionLifecycle::Resolved
            }
            TensionOperation::Supersede {
                tension_id,
                superseded_by_tension_id,
                ..
            } => {
                let p = tension_pos(&s, tension_id)?;
                let q = tension_pos(&s, superseded_by_tension_id)?;
                if tension_id == superseded_by_tension_id
                    || s.open_tensions[p].lifecycle != TensionLifecycle::Open
                    || s.open_tensions[q].lifecycle != TensionLifecycle::Open
                    || !terminals.insert(tension_id)
                {
                    return Err(V::InvalidSupersession);
                }
                s.open_tensions[p].lifecycle = TensionLifecycle::Superseded
            }
            TensionOperation::Abandon { tension_id, .. } => {
                let p = tension_pos(&s, tension_id)?;
                if s.open_tensions[p].lifecycle != TensionLifecycle::Open
                    || !terminals.insert(tension_id)
                {
                    return Err(V::ContradictoryTerminalOperations {
                        kind: ProblemSpaceRecordKind::Tension,
                        identity: tension_id.clone(),
                    });
                }
                s.open_tensions[p].lifecycle = TensionLifecycle::Abandoned
            }
        }
    }
    let mut attention = HashSet::new();
    for op in &contribution.attention_operations {
        nonempty(&op.region_id, "attention region_id")?;
        if !attention.insert(&op.region_id) {
            return Err(V::InvalidAttentionAssignment);
        }
        let p = region_pos(&s, &op.region_id)?;
        if !operational(&s.regions[p].persistence_state) {
            return Err(V::InvalidAttentionAssignment);
        }
        s.regions[p].activation_band = op.band.clone();
        add_once(&mut s.regions[p].source_contribution_ids, cid)
    }
    validate_declarations(&prior_snapshot, &s, contribution)?;
    rebuild(&mut s);
    closure(&s, accepted_log, contribution)?;
    bounds(&s, limits)?;
    let prior_version = s.version;
    s.version = s.version.checked_add(1).ok_or(V::StateVersionOverflow)?;
    s.source_turn_range.last_turn_id = contribution.source_turn_id.clone();
    s.contribution_history.push(ContributionHistoryRecord {
        contribution_id: cid.clone(),
        source_turn_id: contribution.source_turn_id.clone(),
        transformations: transformations(contribution),
    });
    let mut log = accepted_log.clone();
    log.entries.push(AcceptedBoundaryContribution {
        sequence: u64::try_from(log.entries.len() + 1).map_err(|_| V::StateVersionOverflow)?,
        prior_state_version: prior_version,
        contribution: contribution.clone(),
    });
    Ok(ProblemSpaceFoldOutput {
        state: s,
        accepted_log: log,
    })
}

fn subject_operational(s: &ProblemSpaceState, x: &ProblemSpaceSubject) -> bool {
    match x {
        ProblemSpaceSubject::Region(id) => s
            .regions
            .iter()
            .any(|x| x.region_id == *id && operational(&x.persistence_state)),
        ProblemSpaceSubject::Relation(id) => s
            .relations
            .iter()
            .any(|x| x.relation_id == *id && x.lifecycle == RecordLifecycle::Active),
        ProblemSpaceSubject::Constraint(id) => s
            .constraints
            .iter()
            .any(|x| x.constraint_id == *id && x.lifecycle == RecordLifecycle::Active),
        ProblemSpaceSubject::OpenTension(id) => s
            .open_tensions
            .iter()
            .any(|x| x.tension_id == *id && x.lifecycle == TensionLifecycle::Open),
        ProblemSpaceSubject::Referent(id) => s.regions.iter().any(|r| {
            operational(&r.persistence_state)
                && r.anchor_referents.iter().any(|x| x.referent_id == *id)
        }),
    }
}
fn validate_declarations(
    prior: &ProblemSpaceState,
    finals: &ProblemSpaceState,
    c: &BoundaryContribution,
) -> Result<(), V> {
    for (i, p) in c.preservation_declarations.iter().enumerate() {
        if c.preservation_declarations[..i]
            .iter()
            .any(|x| x.subject == p.subject)
            || c.release_declarations
                .iter()
                .any(|x| x.subject == p.subject)
            || !subject_operational(prior, &p.subject)
            || !subject_operational(finals, &p.subject)
        {
            return Err(V::InvalidPreservationDeclaration);
        }
    }
    for (i, r) in c.release_declarations.iter().enumerate() {
        if c.release_declarations[..i]
            .iter()
            .any(|x| x.subject == r.subject)
            || !subject_operational(prior, &r.subject)
        {
            return Err(V::InvalidReleaseDeclaration);
        }
        let valid = match (&r.subject, &r.mode) {
            (ProblemSpaceSubject::Region(id), ReleaseMode::Supersede) => {
                finals.regions.iter().any(|x| {
                    x.region_id == *id && x.persistence_state == RegionPersistenceState::Superseded
                })
            }
            (ProblemSpaceSubject::Region(id), ReleaseMode::Retire) => {
                finals.regions.iter().any(|x| {
                    x.region_id == *id && x.persistence_state == RegionPersistenceState::Retired
                })
            }
            (ProblemSpaceSubject::Relation(id), ReleaseMode::Supersede) => finals
                .relations
                .iter()
                .any(|x| x.relation_id == *id && x.lifecycle == RecordLifecycle::Superseded),
            (ProblemSpaceSubject::Relation(id), ReleaseMode::Retire) => finals
                .relations
                .iter()
                .any(|x| x.relation_id == *id && x.lifecycle == RecordLifecycle::Retired),
            (ProblemSpaceSubject::Constraint(id), ReleaseMode::Supersede) => finals
                .constraints
                .iter()
                .any(|x| x.constraint_id == *id && x.lifecycle == RecordLifecycle::Superseded),
            (ProblemSpaceSubject::Constraint(id), ReleaseMode::Retire) => finals
                .constraints
                .iter()
                .any(|x| x.constraint_id == *id && x.lifecycle == RecordLifecycle::Retired),
            (ProblemSpaceSubject::OpenTension(id), ReleaseMode::Supersede) => finals
                .open_tensions
                .iter()
                .any(|x| x.tension_id == *id && x.lifecycle == TensionLifecycle::Superseded),
            (ProblemSpaceSubject::OpenTension(id), ReleaseMode::Abandon) => finals
                .open_tensions
                .iter()
                .any(|x| x.tension_id == *id && x.lifecycle == TensionLifecycle::Abandoned),
            (ProblemSpaceSubject::Referent(id), _) => !finals.regions.iter().any(|x| {
                operational(&x.persistence_state)
                    && x.anchor_referents.iter().any(|q| q.referent_id == *id)
            }),
            _ => return Err(V::UnsupportedSubjectReleaseModeCombination),
        };
        if !valid {
            return Err(V::InvalidReleaseDeclaration);
        }
    }
    Ok(())
}
fn rebuild(s: &mut ProblemSpaceState) {
    for r in &mut s.regions {
        r.relation_ids.clear();
        r.local_constraint_ids.clear();
        r.open_tension_ids.clear()
    }
    for x in &s.relations {
        if x.lifecycle == RecordLifecycle::Active {
            for r in &mut s.regions {
                if operational(&r.persistence_state)
                    && (r.region_id == x.source_region_id
                        || x.target_region_id.as_ref() == Some(&r.region_id))
                {
                    r.relation_ids.push(x.relation_id.clone())
                }
            }
        }
    }
    for x in &s.constraints {
        if x.lifecycle == RecordLifecycle::Active
            && let ProblemConstraintApplicability::Regions { region_ids } = &x.applicability
        {
            for r in &mut s.regions {
                if operational(&r.persistence_state) && region_ids.contains(&r.region_id) {
                    r.local_constraint_ids.push(x.constraint_id.clone())
                }
            }
        }
    }
    for x in &s.open_tensions {
        if x.lifecycle == TensionLifecycle::Open
            && let Some(r) = s
                .regions
                .iter_mut()
                .find(|r| r.region_id == x.region_id && operational(&r.persistence_state))
        {
            r.open_tension_ids.push(x.tension_id.clone())
        }
    }
    s.attention_lens = AttentionLens {
        primary_region_ids: vec![],
        secondary_region_ids: vec![],
        tertiary_region_ids: vec![],
        background_region_ids: vec![],
    };
    for r in &s.regions {
        if operational(&r.persistence_state) {
            match r.activation_band {
                ActivationBand::Primary => &mut s.attention_lens.primary_region_ids,
                ActivationBand::Secondary => &mut s.attention_lens.secondary_region_ids,
                ActivationBand::Tertiary => &mut s.attention_lens.tertiary_region_ids,
                ActivationBand::Background => &mut s.attention_lens.background_region_ids,
            }
            .push(r.region_id.clone())
        }
    }
}
fn closure(
    s: &ProblemSpaceState,
    log: &BoundaryContributionLog,
    c: &BoundaryContribution,
) -> Result<(), V> {
    let fail = |context| V::FinalReferentialIntegrityFailure { context };
    let mut ids = HashSet::new();
    for r in &s.regions {
        nonempty(&r.region_id, "region_id")?;
        if !ids.insert(&r.region_id) {
            return Err(V::DuplicateRecordIdentity {
                kind: ProblemSpaceRecordKind::Region,
                identity: r.region_id.clone(),
            });
        }
    }
    ids.clear();
    for x in &s.relations {
        nonempty(&x.relation_id, "relation_id")?;
        if !ids.insert(&x.relation_id) {
            return Err(V::DuplicateRecordIdentity {
                kind: ProblemSpaceRecordKind::Relation,
                identity: x.relation_id.clone(),
            });
        }
    }
    ids.clear();
    for x in &s.constraints {
        nonempty(&x.constraint_id, "constraint_id")?;
        if !ids.insert(&x.constraint_id) {
            return Err(V::DuplicateRecordIdentity {
                kind: ProblemSpaceRecordKind::Constraint,
                identity: x.constraint_id.clone(),
            });
        }
    }
    ids.clear();
    for x in &s.open_tensions {
        nonempty(&x.tension_id, "tension_id")?;
        if !ids.insert(&x.tension_id) {
            return Err(V::DuplicateRecordIdentity {
                kind: ProblemSpaceRecordKind::Tension,
                identity: x.tension_id.clone(),
            });
        }
    }
    ids.clear();
    for r in &s.regions {
        for x in &r.anchor_referents {
            nonempty(&x.referent_id, "referent_id")?;
            if !ids.insert(&x.referent_id) {
                return Err(V::DuplicateRecordIdentity {
                    kind: ProblemSpaceRecordKind::Referent,
                    identity: x.referent_id.clone(),
                });
            }
        }
    }
    let contributions: HashSet<&str> = log
        .entries
        .iter()
        .map(|e| e.contribution.contribution_id.as_str())
        .chain(std::iter::once(c.contribution_id.as_str()))
        .collect();
    let turns: HashSet<&str> = log
        .entries
        .iter()
        .map(|e| e.contribution.source_turn_id.as_str())
        .chain(std::iter::once(c.source_turn_id.as_str()))
        .collect();
    for r in &s.regions {
        if let Some(id) = &r.supersedes_region_id {
            let q = s
                .regions
                .iter()
                .find(|x| x.region_id == *id)
                .ok_or_else(|| fail("region supersession target missing"))?;
            if q.persistence_state != RegionPersistenceState::Superseded {
                return Err(fail("region supersession target is not superseded"));
            }
        }
        if r.source_contribution_ids
            .iter()
            .any(|x| !contributions.contains(x.as_str()))
            || r.anchor_referents
                .iter()
                .any(|x| !contributions.contains(x.source_contribution_id.as_str()))
        {
            return Err(fail("region provenance"));
        }
    }
    for x in &s.relations {
        let a = s
            .regions
            .iter()
            .find(|r| r.region_id == x.source_region_id)
            .ok_or_else(|| fail("relation source missing"))?;
        let b = x
            .target_region_id
            .as_ref()
            .map(|id| {
                s.regions
                    .iter()
                    .find(|r| r.region_id == *id)
                    .ok_or_else(|| fail("relation target missing"))
            })
            .transpose()?;
        if x.lifecycle == RecordLifecycle::Active
            && (!operational(&a.persistence_state)
                || b.is_some_and(|r| !operational(&r.persistence_state)))
        {
            return Err(fail("active relation endpoint"));
        }
        if !contributions.contains(x.source_contribution_id.as_str()) {
            return Err(fail("relation provenance"));
        }
    }
    for x in &s.constraints {
        if let ProblemConstraintApplicability::Regions { region_ids } = &x.applicability {
            for id in region_ids {
                let r = s
                    .regions
                    .iter()
                    .find(|r| r.region_id == *id)
                    .ok_or_else(|| fail("constraint target missing"))?;
                if x.lifecycle == RecordLifecycle::Active && !operational(&r.persistence_state) {
                    return Err(fail("active constraint target"));
                }
            }
        }
        if !contributions.contains(x.source_contribution_id.as_str()) {
            return Err(fail("constraint provenance"));
        }
    }
    for x in &s.open_tensions {
        let r = s
            .regions
            .iter()
            .find(|r| r.region_id == x.region_id)
            .ok_or_else(|| fail("tension region missing"))?;
        if x.lifecycle == TensionLifecycle::Open && !operational(&r.persistence_state) {
            return Err(fail("open tension region"));
        }
        if !turns.contains(x.source_turn_id.as_str()) {
            return Err(fail("tension source turn"));
        }
    }
    let mut rebuilt = s.clone();
    rebuild(&mut rebuilt);
    if rebuilt.attention_lens != s.attention_lens
        || rebuilt.regions.iter().zip(&s.regions).any(|(a, b)| {
            a.relation_ids != b.relation_ids
                || a.local_constraint_ids != b.local_constraint_ids
                || a.open_tension_ids != b.open_tension_ids
        })
    {
        return Err(fail("derived active view"));
    }
    Ok(())
}
fn bounds(s: &ProblemSpaceState, l: &ProblemSpaceFoldLimits) -> Result<(), V> {
    for (kind, actual, max) in [
        (
            ProblemSpaceFoldBoundKind::OperationalRegions,
            s.regions
                .iter()
                .filter(|x| operational(&x.persistence_state))
                .count(),
            l.max_operational_regions,
        ),
        (
            ProblemSpaceFoldBoundKind::ActiveRelations,
            s.relations
                .iter()
                .filter(|x| x.lifecycle == RecordLifecycle::Active)
                .count(),
            l.max_active_relations,
        ),
        (
            ProblemSpaceFoldBoundKind::OpenTensions,
            s.open_tensions
                .iter()
                .filter(|x| x.lifecycle == TensionLifecycle::Open)
                .count(),
            l.max_open_tensions,
        ),
        (
            ProblemSpaceFoldBoundKind::BackgroundRegions,
            s.regions
                .iter()
                .filter(|x| x.persistence_state == RegionPersistenceState::Background)
                .count(),
            l.max_background_regions,
        ),
    ] {
        if actual > max {
            return Err(V::ConfiguredFinalStateBoundExcess {
                kind,
                actual,
                maximum: max,
            });
        }
    }
    Ok(())
}

pub fn replay_boundary_contribution_log(
    log: &BoundaryContributionLog,
    limits: &ProblemSpaceFoldLimits,
) -> Result<Option<ProblemSpaceState>, V> {
    validate_log(log)?;
    let mut state = None;
    let mut rebuilt = BoundaryContributionLog {
        thread_id: log.thread_id.clone(),
        entries: vec![],
    };
    for expected in &log.entries {
        let out =
            fold_boundary_contribution(state.as_ref(), &rebuilt, &expected.contribution, limits)?;
        let actual = out.accepted_log.entries.last().expect("fold appends entry");
        if actual.sequence != expected.sequence {
            return Err(V::AcceptedEntrySequenceMismatch {
                index: rebuilt.entries.len(),
            });
        }
        if actual.prior_state_version != expected.prior_state_version {
            return Err(V::AcceptedEntryPriorVersionMismatch {
                index: rebuilt.entries.len(),
            });
        }
        state = Some(out.state);
        rebuilt = out.accepted_log
    }
    Ok(state)
}
