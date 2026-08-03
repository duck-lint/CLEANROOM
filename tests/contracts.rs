#[path = "../examples/schema_support/mod.rs"]
mod schema_support;

use std::{collections::BTreeSet, fs, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};
use static_assertions::assert_type_ne_all;

use semantic_traversal_core::{
    AcceptedBoundaryContribution, ActivatedProjection, AttentionLens, BoundaryContribution,
    BoundaryContributionLog, ConformanceResult, ExecutionLimits, OccurrenceId, OpenTension,
    ProblemRegion, ProblemRelation, ProblemSpaceState, RetrievalResult, SemanticAccessPlan,
    SemanticObjectId, SemanticRegionAddress, SemanticSpaceProjection, SemanticUnitId,
    SynthesisInput, TemporalAnchorId, TransportSegmentId,
    activation::{
        ActivatedEdge, ActivatedObjectRecord, ActivatedRegionRecord, ActivatedUnitRecord,
        ActivationProvenance, CandidateCount, ContinuationHandle, CountByLabel,
        ProjectionTelemetry, TruncationState,
    },
    conformance::{
        StructuralViolation, StructuralViolationCode, StructuralViolationDetail,
        StructuralViolations,
    },
    execution::{
        OperationExecutionRecord, OperationExecutionStatus, RetrievalExecutionStatus,
        RetrievalProvenance, RetrievedIdentifierAssignment, RetrievedSemanticUnit,
        RetrievedTemporalAnchor, TransportSegmentProvenance,
    },
    model::{
        AddressKind, Direction, IdentifierAddress, RecordProvenance, Requirement,
        RetrievalSurfaceKind, SemanticAddress, SourceSpan,
    },
    packet::{AppliedExecutionBound, CoverageFact},
    problem_space::{
        ActivationBand, AttentionOperation, BoundaryOperationKind, ConstraintOperation,
        ContributionHistoryRecord, OpenTensionType, ProblemConstraint,
        ProblemConstraintApplicability, ProblemReferent, ProblemRelationType, RecordLifecycle,
        RegionOperation, RegionPersistenceState, RelationOperation, ReleaseDeclaration,
        ReleaseMode, SourceTurnRange, TensionLifecycle, TensionOperation,
    },
    projection::{
        AuthoredBlockType, BlockTargetMapping, CoverageSemantics, IdentifierAssignment,
        IdentifierAssignmentMode, IdentifierCardinality, IdentifierDescriptor, IdentifierRole,
        IdentifierValue, IdentifierValueShape, OccurrencePresentation, OccurrenceRecord,
        OccurrenceSource, ProjectionValidationStatus, RetrievalSurfaceDescriptor,
        SemanticObjectClassDescriptor, SemanticObjectRecord, SemanticRegionRecord,
        SemanticUnitContent, SemanticUnitRecord, SourceKind, StructuralTransition,
        StructuralTransitionOperation, SurfaceMatchMode, TemporalAffordance, TemporalAnchorRecord,
        TemporalValue, TransportSegmentRecord,
    },
    semantic_access::{
        AddressBinding, ConstraintBinding, CoverageRequirement, CoverageRequirementKind,
        JoinOperation, OpenTensionBinding, OperationConstraints, PlanJoin, PlanOperation,
        PlanOperationType, ProblemRegionBinding, ProblemRelationBinding,
        ProblemSpacePlanProvenance, ProblemSpaceReference, RequestedOutput, RequestedOutputKind,
        SurfaceQuery, TraversalPath,
    },
    synthesis::{ConversationalContinuity, CurrentUtterance},
};

assert_type_ne_all!(
    SemanticObjectId,
    SemanticUnitId,
    SemanticRegionAddress,
    OccurrenceId,
    TemporalAnchorId,
    TransportSegmentId
);

fn object_id() -> SemanticObjectId {
    SemanticObjectId::parse("019fc58d-42aa-7919-95f8-a69b609aadff").expect("valid object UUID")
}

fn unit_id() -> SemanticUnitId {
    SemanticUnitId::parse("unit:capital:chapter-2:1").expect("non-empty unit identity")
}

fn occurrence_id() -> OccurrenceId {
    OccurrenceId::parse("occurrence:journal:capital").expect("non-empty occurrence identity")
}

fn anchor_id() -> TemporalAnchorId {
    TemporalAnchorId::parse("anchor:1867").expect("non-empty anchor identity")
}

fn segment_id() -> TransportSegmentId {
    TransportSegmentId::parse("segment:capital:1").expect("non-empty segment identity")
}

fn region_address() -> SemanticRegionAddress {
    SemanticRegionAddress::parse(object_id(), "Chapter 2").expect("non-empty region address")
}

fn problem_region() -> ProblemRegion {
    ProblemRegion {
        region_id: "region:chronology".into(),
        anchor_referents: vec![ProblemReferent {
            referent_id: "referent:capital".into(),
            expression: "Capital".into(),
            source_contribution_id: "contribution:1".into(),
        }],
        relation_ids: vec!["relation:comparison".into()],
        local_constraint_ids: vec!["constraint:temporal".into()],
        open_tension_ids: vec!["tension:dimension".into()],
        source_contribution_ids: vec!["contribution:1".into()],
        persistence_state: RegionPersistenceState::Unresolved,
        activation_band: ActivationBand::Primary,
        supersedes_region_id: None,
    }
}

fn problem_relation() -> ProblemRelation {
    ProblemRelation {
        relation_id: "relation:comparison".into(),
        source_region_id: "region:chronology".into(),
        relation_type: ProblemRelationType::Comparison,
        target_region_id: None,
        source_contribution_id: "contribution:1".into(),
        lifecycle: RecordLifecycle::Active,
    }
}

fn open_tension() -> OpenTension {
    OpenTension {
        tension_id: "tension:dimension".into(),
        region_id: "region:chronology".into(),
        tension_type: OpenTensionType::MissingComparisonDimension,
        unresolved_expression: Some("before".into()),
        candidate_bindings: vec!["publication chronology".into(), "reading chronology".into()],
        source_turn_id: "turn:1".into(),
        lifecycle: TensionLifecycle::Open,
    }
}

fn attention_lens() -> AttentionLens {
    AttentionLens {
        primary_region_ids: vec!["region:chronology".into()],
        secondary_region_ids: vec![],
        tertiary_region_ids: vec![],
        background_region_ids: vec![],
    }
}

fn boundary_contribution() -> BoundaryContribution {
    BoundaryContribution {
        contribution_id: "contribution:1".into(),
        source_turn_id: "turn:1".into(),
        source_utterance_id: "utterance:1".into(),
        region_operations: vec![RegionOperation::Create {
            region: problem_region(),
        }],
        relation_operations: vec![RelationOperation::Connect {
            relation: problem_relation(),
        }],
        constraint_operations: vec![ConstraintOperation::Add {
            constraint: ProblemConstraint {
                constraint_id: "constraint:temporal".into(),
                expression: "compare temporal anchors".into(),
                applicability: ProblemConstraintApplicability::Regions {
                    region_ids: vec!["region:chronology".into()],
                },
                source_contribution_id: "contribution:1".into(),
                lifecycle: RecordLifecycle::Active,
            },
        }],
        tension_operations: vec![TensionOperation::Open {
            tension: open_tension(),
        }],
        attention_operations: vec![AttentionOperation {
            region_id: "region:chronology".into(),
            band: ActivationBand::Primary,
        }],
        preservation_declarations: vec![],
        release_declarations: vec![ReleaseDeclaration {
            subject: semantic_traversal_core::problem_space::ProblemSpaceSubject::Constraint(
                "constraint:reading".into(),
            ),
            mode: ReleaseMode::Supersede,
            reason: "publication chronology replaces reading chronology".into(),
        }],
    }
}

fn problem_space_state() -> ProblemSpaceState {
    ProblemSpaceState {
        thread_id: "thread:1".into(),
        version: 1,
        regions: vec![problem_region()],
        relations: vec![problem_relation()],
        constraints: vec![ProblemConstraint {
            constraint_id: "constraint:temporal".into(),
            expression: "compare temporal anchors".into(),
            applicability: ProblemConstraintApplicability::Regions {
                region_ids: vec!["region:chronology".into()],
            },
            source_contribution_id: "contribution:1".into(),
            lifecycle: RecordLifecycle::Active,
        }],
        open_tensions: vec![open_tension()],
        contribution_history: vec![ContributionHistoryRecord {
            contribution_id: "contribution:1".into(),
            source_turn_id: "turn:1".into(),
            transformations: vec![
                BoundaryOperationKind::Create,
                BoundaryOperationKind::Tension,
            ],
        }],
        attention_lens: attention_lens(),
        source_turn_range: SourceTurnRange {
            first_turn_id: "turn:1".into(),
            last_turn_id: "turn:1".into(),
        },
    }
}

fn projection() -> SemanticSpaceProjection {
    let object = object_id();
    let unit = unit_id();
    let region = region_address();
    let occurrence = occurrence_id();
    let anchor = anchor_id();

    SemanticSpaceProjection {
        projection_snapshot_id: "projection:1".into(),
        ingest_identity: "ingest:1".into(),
        schema_version: "v0.1.0".into(),
        logical_hash: "sha256:projection".into(),
        corpus_snapshot_identity: "corpus:1".into(),
        configuration_snapshot_id: "configuration:1".into(),
        validation_status: ProjectionValidationStatus::Validated,
        object_classes: vec![SemanticObjectClassDescriptor {
            class_name: "source_material".into(),
            applicable_identifier_names: vec!["note_type".into(), "title".into()],
            permitted_source_kinds: vec![SourceKind::Markdown],
        }],
        objects: vec![SemanticObjectRecord {
            object_id: object.clone(),
            source_identity: "vault:Marx, Karl — Capital.md".into(),
            source_kind: SourceKind::Markdown,
            canonical_path: "LAYER-2 INTERFACE/READING & RESEARCH/SOURCE MATERIAL".into(),
            filename: "Marx, Karl — Capital.md".into(),
            title: "Capital".into(),
            aliases: vec!["Das Kapital".into()],
            object_class: "source_material".into(),
            region_addresses: vec![region.clone()],
            unit_ids: vec![unit.clone()],
            identifier_assignment_ids: vec!["assignment:note_type".into()],
            object_field_occurrence_ids: vec![],
            body_occurrence_ids: vec![occurrence.clone()],
            incoming_occurrence_ids: vec![],
            temporal_anchor_ids: vec![anchor.clone()],
            retrieval_surface_ids: vec!["surface:exact".into(), "surface:graph".into()],
        }],
        regions: vec![SemanticRegionRecord {
            address: region.clone(),
            heading_path: vec!["Chapter 2".into()],
            heading_identity: "heading:chapter-2".into(),
            source_span: Some(SourceSpan {
                source: "Marx, Karl — Capital.md".into(),
                start_byte: Some(100),
                end_byte: Some(110),
            }),
            child_region_addresses: vec![],
            contained_unit_ids: vec![unit.clone()],
            block_target_mappings: vec![BlockTargetMapping {
                authored_block_id: "capital-block".into(),
                target_unit_id: unit.clone(),
            }],
            incoming_occurrence_ids: vec![occurrence.clone()],
            inherited_identifier_assignment_ids: vec!["assignment:note_type".into()],
            retrieval_surface_ids: vec!["surface:exact".into()],
        }],
        units: vec![SemanticUnitRecord {
            unit_id: unit.clone(),
            parent_object_id: object.clone(),
            parent_region_address: region.clone(),
            authored_block_type: AuthoredBlockType::Paragraph,
            heading_path: vec!["Chapter 2".into()],
            block_ordinal: 1,
            explicit_block_id: Some("capital-block".into()),
            content: SemanticUnitContent::Inline {
                authored_markdown: "The commodity is...".into(),
                normalized_text: "The commodity is...".into(),
            },
            inherited_identifier_assignment_ids: vec!["assignment:note_type".into()],
            unit_local_identifier_assignment_ids: vec![],
            outgoing_occurrence_ids: vec![occurrence.clone()],
            incoming_occurrence_ids: vec![],
            temporal_anchor_ids: vec![anchor.clone()],
            retrieval_surface_ids: vec!["surface:exact".into()],
            source_provenance: RecordProvenance::Materialization {
                rule: "authored_markdown_block".into(),
                sources: vec![SemanticAddress::Region(region.clone())],
            },
            transport_segments: vec![TransportSegmentRecord {
                segment_id: segment_id(),
                parent_unit_id: unit.clone(),
                segment_ordinal: 0,
                source_span: SourceSpan {
                    source: "Marx, Karl — Capital.md".into(),
                    start_byte: Some(111),
                    end_byte: Some(140),
                },
                total_segments: 1,
                reconstruction_group: "reconstruct:capital-unit".into(),
            }],
        }],
        identifier_descriptors: vec![IdentifierDescriptor {
            identifier_name: "note_type".into(),
            semantic_role: IdentifierRole::ObjectClass,
            value_shape: IdentifierValueShape::String,
            cardinality: IdentifierCardinality::Scalar,
            applicable_address_kinds: vec![AddressKind::SemanticObject],
            assignment_mode: IdentifierAssignmentMode::Intrinsic,
            source_surface: "frontmatter.note_type".into(),
            may_contain_canonical_links: false,
            temporal_affordance: TemporalAffordance::None,
            retrieval_surface_ids: vec!["surface:exact".into()],
            enabled_transition_ids: vec!["transition:identifier".into()],
        }],
        identifier_assignments: vec![IdentifierAssignment {
            assignment_id: "assignment:note_type".into(),
            identifier_name: "note_type".into(),
            subject: SemanticAddress::Object(object.clone()),
            value: IdentifierValue::String("source_material".into()),
            provenance: RecordProvenance::ObjectField {
                object_id: object.clone(),
                field_path: "note_type".into(),
            },
        }],
        occurrences: vec![OccurrenceRecord {
            occurrence_id: occurrence.clone(),
            source: OccurrenceSource::SemanticUnit {
                unit_id: unit.clone(),
            },
            authored_target_text: "Marx, Karl — Capital#Chapter 2".into(),
            display_alias: Some("Chapter 2".into()),
            resolved_target: SemanticAddress::Region(region.clone()),
            presentation_mode: OccurrencePresentation::Link,
            direction: Direction::Outgoing,
            source_span: None,
        }],
        temporal_anchors: vec![TemporalAnchorRecord {
            anchor_id: anchor.clone(),
            subject: SemanticAddress::Object(object),
            value: TemporalValue::Year(1867),
            provenance: RecordProvenance::ObjectField {
                object_id: object_id(),
                field_path: "original_year_published".into(),
            },
        }],
        retrieval_surfaces: vec![RetrievalSurfaceDescriptor {
            surface_id: "surface:exact".into(),
            kind: RetrievalSurfaceKind::Exact,
            available: true,
            visible_address_kinds: vec![AddressKind::SemanticObject, AddressKind::SemanticUnit],
            match_modes: vec![SurfaceMatchMode::Literal],
            default_candidate_limit: 25,
            hard_candidate_limit: 100,
            returned_identity: AddressKind::SemanticUnit,
            hydrates_to_semantic_units: true,
            coverage_semantics: CoverageSemantics::Exhaustive,
            exhaustive_total_count_supported: true,
            continuation_supported: false,
            technical_limitations: vec![],
        }],
        valid_transitions: vec![StructuralTransition {
            transition_id: "transition:containment".into(),
            from: AddressKind::SemanticObject,
            operation: StructuralTransitionOperation::Containment,
            direction: Direction::Outgoing,
            to: AddressKind::SemanticUnit,
            retrieval_surface_id: None,
        }],
    }
}

fn activated_projection() -> ActivatedProjection {
    let provenance = vec![ActivationProvenance::ProblemRegion {
        region_id: "region:chronology".into(),
    }];
    ActivatedProjection {
        projection_snapshot_id: "projection:1".into(),
        configuration_snapshot_id: "configuration:1".into(),
        activated_objects: vec![ActivatedObjectRecord {
            object_id: object_id(),
            visible_identifier_assignment_ids: vec!["assignment:note_type".into()],
            contained_region_count: 1,
            contained_unit_count: 1,
            incoming_occurrence_count: 1,
            outgoing_occurrence_count: 0,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: provenance.clone(),
        }],
        activated_regions: vec![ActivatedRegionRecord {
            address: region_address(),
            visible_unit_ids: vec![unit_id()],
            contained_unit_count: 1,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: provenance.clone(),
        }],
        activated_units: vec![ActivatedUnitRecord {
            unit_id: unit_id(),
            parent_object_id: object_id(),
            parent_region_address: region_address(),
            text_preview: "The commodity is...".into(),
            incoming_occurrence_count: 0,
            outgoing_occurrence_count: 1,
            temporal_anchor_count: 1,
            available_surface_ids: vec!["surface:exact".into()],
            activation_provenance: provenance.clone(),
        }],
        edges: vec![ActivatedEdge {
            edge_id: "edge:containment".into(),
            source: SemanticAddress::Object(object_id()),
            transition_id: "transition:containment".into(),
            direction: Direction::Outgoing,
            target: SemanticAddress::Unit(unit_id()),
            activation_provenance: provenance.clone(),
        }],
        telemetry: vec![ProjectionTelemetry {
            surface_kind: RetrievalSurfaceKind::Exact,
            surface_id: "surface:exact".into(),
            candidate_count: CandidateCount::Exact(1),
            current_depth: 1,
            maximum_depth: 3,
            returned_count: 1,
            remaining_expansion_budget: 9,
            truncation_state: TruncationState::Complete,
            identifier_type_distribution: vec![CountByLabel {
                label: "source_material".into(),
                count: 1,
            }],
            temporal_anchor_count: 1,
            unresolved_target_count: 0,
            continuation_available: false,
            activation_provenance: provenance.clone(),
        }],
        continuation_handles: vec![ContinuationHandle {
            handle_id: "continuation:incoming".into(),
            subject: SemanticAddress::Object(object_id()),
            surface_kind: RetrievalSurfaceKind::Graph,
            direction: Some(Direction::Incoming),
            remaining_count: Some(2),
            next_page_limit: 2,
            activation_provenance: provenance,
        }],
    }
}

fn semantic_access_plan() -> SemanticAccessPlan {
    SemanticAccessPlan {
        plan_id: "plan:1".into(),
        projection_snapshot_id: "projection:1".into(),
        problem_space_version: 1,
        focus_utterance_id: "utterance:1".into(),
        configuration_snapshot_id: "configuration:1".into(),
        problem_space_provenance: ProblemSpacePlanProvenance {
            thread_id: "thread:1".into(),
            contribution_ids: vec!["contribution:1".into()],
        },
        problem_region_bindings: vec![ProblemRegionBinding {
            problem_region_id: "region:chronology".into(),
            address_binding_ids: vec!["binding:capital".into()],
            rationale: "bind the named source-material object".into(),
        }],
        relation_bindings: vec![ProblemRelationBinding {
            problem_relation_id: "relation:comparison".into(),
            traversal_path_ids: vec!["path:capital".into()],
            transition_ids: vec!["transition:containment".into()],
        }],
        constraint_bindings: vec![ConstraintBinding {
            constraint_id: "constraint:temporal".into(),
            operation_ids: vec!["operation:exact".into()],
            requirement: Requirement::Required,
        }],
        open_tension_bindings: vec![OpenTensionBinding {
            tension_id: "tension:dimension".into(),
            candidate_binding_ids: vec!["binding:capital".into()],
            requested_output_ids: vec!["output:units".into()],
        }],
        address_bindings: vec![AddressBinding {
            binding_id: "binding:capital".into(),
            address: SemanticAddress::Object(object_id()),
            problem_space_provenance: vec![ProblemSpaceReference::Region(
                "region:chronology".into(),
            )],
        }],
        traversal_paths: vec![TraversalPath {
            path_id: "path:capital".into(),
            start_binding_ids: vec!["binding:capital".into()],
            operations: vec![PlanOperation {
                operation_id: "operation:exact".into(),
                requirement: Requirement::Required,
                input_bindings: vec!["binding:capital".into()],
                operation: PlanOperationType::SearchSurface {
                    surface_id: "surface:exact".into(),
                    surface_kind: RetrievalSurfaceKind::Exact,
                    match_mode: SurfaceMatchMode::Literal,
                    query: SurfaceQuery::Literal {
                        value: "commodity".into(),
                    },
                },
                constraints: OperationConstraints {
                    maximum_depth: None,
                    maximum_candidates: Some(100),
                    eligible_scope_binding_ids: vec!["binding:capital".into()],
                },
                output_binding: "binding:exact-results".into(),
            }],
            output_binding: "binding:capital-units".into(),
            problem_space_provenance: vec![ProblemSpaceReference::Constraint(
                "constraint:temporal".into(),
            )],
        }],
        joins: vec![PlanJoin {
            join_id: "join:group".into(),
            input_bindings: vec!["binding:capital-units".into()],
            operation: JoinOperation::Group,
            output_binding: "binding:grouped".into(),
        }],
        requested_outputs: vec![RequestedOutput {
            output_id: "output:units".into(),
            requirement: Requirement::Required,
            kind: RequestedOutputKind::SemanticUnits,
            source_binding: "binding:grouped".into(),
        }],
        coverage_requirements: vec![CoverageRequirement {
            coverage_requirement_id: "coverage:exact".into(),
            requirement: Requirement::Required,
            kind: CoverageRequirementKind::ExhaustiveExact {
                surface_id: "surface:exact".into(),
                eligible_scope_binding_ids: vec!["binding:capital".into()],
            },
        }],
    }
}

fn conformance_result() -> ConformanceResult {
    ConformanceResult::Invalid {
        plan_id: "plan:1".into(),
        projection_snapshot_id: "projection:1".into(),
        violations: StructuralViolations {
            first: StructuralViolation {
                operation_id: Some("operation:invalid".into()),
                address: Some(SemanticAddress::Identifier(IdentifierAddress {
                    identifier_name: "journal_entry_date".into(),
                    represented_value: None,
                })),
                code: StructuralViolationCode::IdentifierNotApplicable,
                detail: StructuralViolationDetail::IdentifierApplicability {
                    identifier_name: "journal_entry_date".into(),
                    proposed_address_kind: AddressKind::SemanticObject,
                    applicable_address_kinds: vec![AddressKind::SemanticUnit],
                },
                requirement: Some(Requirement::Required),
            },
            additional: vec![],
        },
    }
}

fn retrieval_result() -> RetrievalResult {
    RetrievalResult {
        plan_id: "plan:1".into(),
        projection_snapshot_id: "projection:1".into(),
        execution_status: RetrievalExecutionStatus::Complete,
        returned_units: vec![RetrievedSemanticUnit {
            unit_id: unit_id(),
            parent_object_id: object_id(),
            parent_region_address: region_address(),
            authored_content: "The commodity is...".into(),
            identifier_assignments: vec![RetrievedIdentifierAssignment {
                assignment_id: "assignment:note_type".into(),
                identifier_name: "note_type".into(),
                value: IdentifierValue::String("source_material".into()),
                provenance: RecordProvenance::ObjectField {
                    object_id: object_id(),
                    field_path: "note_type".into(),
                },
            }],
            outgoing_occurrence_ids: vec![occurrence_id()],
            incoming_occurrence_ids: vec![],
            temporal_anchors: vec![RetrievedTemporalAnchor {
                anchor_id: anchor_id(),
                value: TemporalValue::Year(1867),
                provenance: RecordProvenance::ObjectField {
                    object_id: object_id(),
                    field_path: "original_year_published".into(),
                },
            }],
            retrieval_provenance: vec![RetrievalProvenance {
                surface_id: "surface:exact".into(),
                path_id: "path:capital".into(),
                operation_id: "operation:exact".into(),
                traversed_addresses: vec![
                    SemanticAddress::Object(object_id()),
                    SemanticAddress::Unit(unit_id()),
                ],
                occurrence_ids: vec![occurrence_id()],
                temporal_anchor_ids: vec![anchor_id()],
            }],
            transport_segment_provenance: vec![TransportSegmentProvenance {
                segment_id: segment_id(),
                segment_ordinal: 0,
                total_segments: 1,
            }],
        }],
        operation_results: vec![OperationExecutionRecord {
            operation_id: "operation:exact".into(),
            requirement: Requirement::Required,
            status: OperationExecutionStatus::Completed,
            returned_unit_ids: vec![unit_id()],
            inspected_candidate_count: 1,
        }],
    }
}

fn execution_limits() -> ExecutionLimits {
    ExecutionLimits {
        plan_id: "plan:1".into(),
        requested_operation_count: 1,
        completed_operation_count: 1,
        failed_required_operation_ids: vec![],
        failed_optional_operation_ids: vec![],
        coverage_facts: vec![CoverageFact::ExhaustiveExactCompleted {
            surface_id: "surface:exact".into(),
            eligible_scope: "binding:capital".into(),
            total_matches: 1,
        }],
        applied_bounds: vec![AppliedExecutionBound {
            bound_name: "candidate_limit".into(),
            configured_limit: 100,
            observed_value: 1,
            truncated: false,
        }],
    }
}

fn synthesis_input() -> SynthesisInput {
    SynthesisInput {
        current_problem_space: problem_space_state(),
        newest_utterance: CurrentUtterance {
            utterance_id: "utterance:2".into(),
            text: "When did that change?".into(),
        },
        previous_turn: Some(ConversationalContinuity {
            turn_id: "turn:1".into(),
            user_utterance: "What did the calf eat?".into(),
            assistant_response: "It first drank milk and later ate hay.".into(),
        }),
        semantic_access_plan: semantic_access_plan(),
        retrieval_result: retrieval_result(),
        execution_limits: execution_limits(),
    }
}

fn assert_round_trip<T>(value: T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string_pretty(&value).expect("contract must serialize");
    let decoded = serde_json::from_str(&json).expect("contract must deserialize");
    assert_eq!(value, decoded);
}

macro_rules! round_trip_test {
    ($name:ident, $fixture:expr) => {
        #[test]
        fn $name() {
            assert_round_trip($fixture);
        }
    };
}

round_trip_test!(problem_space_state_round_trip, problem_space_state());
round_trip_test!(problem_region_round_trip, problem_region());
round_trip_test!(problem_relation_round_trip, problem_relation());
round_trip_test!(open_tension_round_trip, open_tension());
round_trip_test!(attention_lens_round_trip, attention_lens());
round_trip_test!(boundary_contribution_round_trip, boundary_contribution());
round_trip_test!(
    whole_problem_space_applicability_round_trip,
    ProblemConstraintApplicability::WholeProblemSpace {}
);
round_trip_test!(
    one_region_applicability_round_trip,
    ProblemConstraintApplicability::Regions {
        region_ids: vec!["region:one".into()]
    }
);
round_trip_test!(
    shared_regional_applicability_round_trip,
    ProblemConstraintApplicability::Regions {
        region_ids: vec!["region:one".into(), "region:two".into()]
    }
);
round_trip_test!(semantic_space_projection_round_trip, projection());
round_trip_test!(activated_projection_round_trip, activated_projection());
round_trip_test!(semantic_access_plan_round_trip, semantic_access_plan());
round_trip_test!(conformance_result_round_trip, conformance_result());
round_trip_test!(retrieval_result_round_trip, retrieval_result());
round_trip_test!(execution_limits_round_trip, execution_limits());
round_trip_test!(synthesis_input_round_trip, synthesis_input());

#[test]
fn accepted_contribution_log_round_trip() {
    assert_round_trip(BoundaryContributionLog {
        thread_id: "thread:1".into(),
        entries: vec![AcceptedBoundaryContribution {
            sequence: 1,
            prior_state_version: 0,
            contribution: boundary_contribution(),
        }],
    });
}

#[test]
fn boundary_operation_vector_order_survives_round_trip() {
    let mut contribution = boundary_contribution();
    contribution.attention_operations = vec![
        AttentionOperation {
            region_id: "region:first".into(),
            band: ActivationBand::Secondary,
        },
        AttentionOperation {
            region_id: "region:second".into(),
            band: ActivationBand::Background,
        },
    ];

    let json = serde_json::to_string(&contribution).expect("contribution must serialize");
    let decoded: BoundaryContribution =
        serde_json::from_str(&json).expect("contribution must deserialize");
    assert_eq!(
        decoded.attention_operations,
        contribution.attention_operations
    );
}

#[test]
fn attention_is_orthogonal_to_region_persistence() {
    let mut active_background = problem_region();
    active_background.persistence_state = RegionPersistenceState::Active;
    active_background.activation_band = ActivationBand::Background;
    assert_round_trip(active_background);

    let mut unresolved_primary = problem_region();
    unresolved_primary.persistence_state = RegionPersistenceState::Unresolved;
    unresolved_primary.activation_band = ActivationBand::Primary;
    assert_round_trip(unresolved_primary);
}

#[test]
fn canonical_applicability_and_derived_regional_incidence_round_trip() {
    let mut state = problem_space_state();
    let mut second_region = problem_region();
    second_region.region_id = "region:second".into();
    second_region.local_constraint_ids = vec!["constraint:shared".into()];
    state.regions[0].local_constraint_ids = vec!["constraint:shared".into()];
    state.regions.push(second_region);
    state.constraints = vec![
        ProblemConstraint {
            constraint_id: "constraint:whole".into(),
            expression: "applies throughout".into(),
            applicability: ProblemConstraintApplicability::WholeProblemSpace {},
            source_contribution_id: "contribution:1".into(),
            lifecycle: RecordLifecycle::Active,
        },
        ProblemConstraint {
            constraint_id: "constraint:shared".into(),
            expression: "applies to both regions".into(),
            applicability: ProblemConstraintApplicability::Regions {
                region_ids: vec!["region:chronology".into(), "region:second".into()],
            },
            source_contribution_id: "contribution:1".into(),
            lifecycle: RecordLifecycle::Active,
        },
    ];

    assert!(state.regions.iter().all(|region| {
        !region
            .local_constraint_ids
            .contains(&"constraint:whole".to_owned())
    }));
    assert!(state.regions.iter().all(|region| {
        region
            .local_constraint_ids
            .contains(&"constraint:shared".to_owned())
    }));
    assert_round_trip(state);
}

#[test]
fn rejects_invalid_constraint_applicability_shapes() {
    let mut missing = serde_json::to_value(problem_space_state().constraints.remove(0))
        .expect("constraint must serialize");
    missing
        .as_object_mut()
        .expect("constraint is an object")
        .remove("applicability");
    assert!(serde_json::from_value::<ProblemConstraint>(missing).is_err());

    for invalid in [
        r#"{"kind":"unknown"}"#,
        r#"{"kind":"whole_problem_space","extra":true}"#,
        r#"{"kind":"regions","region_ids":["region:one"],"extra":true}"#,
        r#"{"kind":"regions"}"#,
        r#"{"kind":"whole_problem_space","region_ids":[]}"#,
    ] {
        assert!(
            serde_json::from_str::<ProblemConstraintApplicability>(invalid).is_err(),
            "accepted invalid applicability shape: {invalid}"
        );
    }
}

#[test]
fn rejects_invalid_accepted_contribution_log_shapes() {
    let contribution =
        serde_json::to_value(boundary_contribution()).expect("contribution fixture must serialize");
    let valid_entry = serde_json::json!({
        "sequence": 1,
        "prior_state_version": 0,
        "contribution": contribution,
    });

    assert!(
        serde_json::from_value::<BoundaryContributionLog>(serde_json::json!({
            "entries": []
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<BoundaryContributionLog>(serde_json::json!({
            "thread_id": "thread:1", "entries": [], "extra": true
        }))
        .is_err()
    );

    for missing_field in ["sequence", "prior_state_version", "contribution"] {
        let mut entry = valid_entry.clone();
        entry
            .as_object_mut()
            .expect("entry is an object")
            .remove(missing_field);
        assert!(serde_json::from_value::<AcceptedBoundaryContribution>(entry).is_err());
    }

    let mut unknown = valid_entry;
    unknown
        .as_object_mut()
        .expect("entry is an object")
        .insert("extra".into(), serde_json::json!(true));
    assert!(serde_json::from_value::<AcceptedBoundaryContribution>(unknown).is_err());
}

#[test]
fn rejects_malformed_semantic_object_id() {
    assert!(serde_json::from_str::<SemanticObjectId>(r#""not-a-uuid""#).is_err());
}

#[test]
fn rejects_empty_opaque_identities() {
    assert!(serde_json::from_str::<SemanticUnitId>(r#""""#).is_err());
    assert!(serde_json::from_str::<OccurrenceId>(r#""   ""#).is_err());
    assert!(serde_json::from_str::<TemporalAnchorId>(r#""""#).is_err());
    assert!(serde_json::from_str::<TransportSegmentId>(r#""\n""#).is_err());
}

#[test]
fn rejects_empty_semantic_region_structural_address() {
    let json = format!(
        r#"{{"object_id":"{}","authored_structural_address":" "}}"#,
        object_id()
    );
    assert!(serde_json::from_str::<SemanticRegionAddress>(&json).is_err());
}

#[test]
fn rejects_missing_required_fields() {
    assert!(serde_json::from_str::<AttentionLens>(r#"{"primary_region_ids":[]}"#).is_err());
}

#[test]
fn rejects_unknown_fields_on_strict_records() {
    let mut value = serde_json::to_value(problem_region()).expect("fixture must serialize");
    value
        .as_object_mut()
        .expect("problem region is an object")
        .insert("confidence".into(), serde_json::json!(0.9));
    assert!(serde_json::from_value::<ProblemRegion>(value).is_err());
}

#[test]
fn rejects_unknown_enum_variants() {
    assert!(serde_json::from_str::<ActivationBand>(r#""foreground""#).is_err());
}

#[test]
fn rejects_invalid_tagged_union_shapes() {
    let json = r#"{"kind":"object","value":{"not":"a uuid"}}"#;
    assert!(serde_json::from_str::<SemanticAddress>(json).is_err());

    let unknown_field = format!(
        r#"{{"kind":"object","value":"{}","extra":true}}"#,
        object_id()
    );
    assert!(serde_json::from_str::<SemanticAddress>(&unknown_field).is_err());
}

#[test]
fn required_identities_are_distinct_at_compile_time() {
    let object = object_id();
    let unit = unit_id();
    let segment = segment_id();
    let region = region_address();

    assert_ne!(object.to_string(), unit.to_string());
    assert_ne!(unit.to_string(), segment.to_string());
    assert_ne!(region.authored_structural_address, unit.to_string());
}

#[test]
fn committed_schemas_are_current() {
    let schema_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas");

    let committed_names: BTreeSet<_> = fs::read_dir(&schema_directory)
        .expect("schema directory must exist")
        .map(|entry| {
            entry
                .expect("schema entry must be readable")
                .file_name()
                .into_string()
                .expect("schema filename must be UTF-8")
        })
        .collect();
    let generated_names: BTreeSet<_> = schema_support::generated_schemas()
        .into_iter()
        .map(|(filename, _)| filename.to_owned())
        .collect();
    assert_eq!(committed_names, generated_names, "schema inventory differs");

    for (filename, generated) in schema_support::generated_schemas() {
        let committed = fs::read_to_string(schema_directory.join(filename))
            .unwrap_or_else(|error| panic!("missing committed schema {filename}: {error}"));
        assert_eq!(committed, generated, "schema is stale: {filename}");
    }
}
