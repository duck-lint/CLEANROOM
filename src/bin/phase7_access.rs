use semantic_traversal_core::{
    access::{
        AccessError, AccessOperand, OllamaEmbeddingProvider, ProjectionAccessProbe,
        TemporalPrecision, TemporalQuery, build_projection_access_artifacts,
    },
    model::{Direction, RetrievalSurfaceKind, SemanticAddress},
    projection::{SemanticSpaceProjection, SurfaceMatchMode, TemporalValue},
};
use serde_json::Value;
use std::{env, fs, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1).map(PathBuf::from);
    let Some(projection_path) = args.next() else {
        eprintln!(
            "usage: phase7_access <phase6-projection.json> <observation.json> <access-artifacts.json>"
        );
        return ExitCode::from(2);
    };
    let Some(observation_path) = args.next() else {
        eprintln!(
            "usage: phase7_access <phase6-projection.json> <observation.json> <access-artifacts.json>"
        );
        return ExitCode::from(2);
    };
    let Some(output_path) = args.next() else {
        eprintln!(
            "usage: phase7_access <phase6-projection.json> <observation.json> <access-artifacts.json>"
        );
        return ExitCode::from(2);
    };
    if let Err(error) = run(&projection_path, &observation_path, &output_path) {
        eprintln!("phase 7 access failed: {error}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn run(
    projection_path: &PathBuf,
    observation_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let projection: SemanticSpaceProjection = serde_json::from_slice(&fs::read(projection_path)?)?;
    let observation: Value = serde_json::from_slice(&fs::read(observation_path)?)?;
    let provider = OllamaEmbeddingProvider::default();
    let artifacts =
        build_projection_access_artifacts(&projection, Some(&observation), Some(&provider))?;
    artifacts.validate_against(&projection)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, serde_json::to_vec_pretty(&artifacts)?)?;

    let first_unit = projection.units.first().ok_or("projection has no units")?;
    let first_object = projection
        .objects
        .first()
        .ok_or("projection has no objects")?;
    let first_anchor = projection
        .temporal_anchors
        .first()
        .ok_or("projection has no temporal anchors")?;
    let (precision, value) = temporal_value(&first_anchor.value);
    let vector_query = artifacts
        .vector
        .segments
        .first()
        .ok_or("provider-backed vector index contains no segments")?
        .embedding
        .clone();
    let probes = vec![
        ProjectionAccessProbe {
            probe_id: "phase7-real-exact".into(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: "surface:exact".into(),
            surface_kind: RetrievalSurfaceKind::Exact,
            match_mode: SurfaceMatchMode::Literal,
            operand: AccessOperand::ExactLiteral(first_unit.unit_id.to_string()),
            page_size: 5,
            cursor: None,
        },
        ProjectionAccessProbe {
            probe_id: "phase7-real-lexical".into(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: "surface:lexical".into(),
            surface_kind: RetrievalSurfaceKind::Lexical,
            match_mode: SurfaceMatchMode::Terms,
            operand: AccessOperand::LexicalTerms(vec!["the".into()]),
            page_size: 5,
            cursor: None,
        },
        ProjectionAccessProbe {
            probe_id: "phase7-real-vector".into(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: "surface:vector".into(),
            surface_kind: RetrievalSurfaceKind::Vector,
            match_mode: SurfaceMatchMode::NearestNeighbours,
            // A zero-norm operand is not a defined cosine nearest-neighbour
            // query.  Use a corpus-derived provider embedding for the real
            // probe; the invalid zero-vector case is checked explicitly below.
            operand: AccessOperand::Vector(vector_query),
            page_size: 5,
            cursor: None,
        },
        ProjectionAccessProbe {
            probe_id: "phase7-real-graph".into(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: "surface:graph".into(),
            surface_kind: RetrievalSurfaceKind::Graph,
            match_mode: SurfaceMatchMode::Incidence,
            operand: AccessOperand::Graph {
                seed: SemanticAddress::Object(first_object.object_id.clone()),
                direction: Direction::Outgoing,
                transition_ids: Vec::new(),
            },
            page_size: 5,
            cursor: None,
        },
        ProjectionAccessProbe {
            probe_id: "phase7-real-temporal".into(),
            projection_snapshot_id: projection.projection_snapshot_id.clone(),
            surface_id: "surface:temporal".into(),
            surface_kind: RetrievalSurfaceKind::Temporal,
            match_mode: SurfaceMatchMode::Temporal,
            operand: AccessOperand::Temporal(TemporalQuery::Exact { precision, value }),
            page_size: 5,
            cursor: None,
        },
    ];
    println!("artifact_identity={}", artifacts.artifact_identity);
    println!(
        "projection_snapshot_id={}",
        artifacts.manifest.projection_snapshot_id
    );
    println!(
        "projection_logical_hash={}",
        artifacts.manifest.projection_logical_hash
    );
    println!("exact_records={}", artifacts.exact.entries.len());
    println!("lexical_postings={}", artifacts.lexical.postings.len());
    println!("graph_edges={}", artifacts.graph.edges.len());
    println!("temporal_records={}", artifacts.temporal.entries.len());
    println!("vector_segments={}", artifacts.vector.segments.len());
    let zero_vector_probe = ProjectionAccessProbe {
        probe_id: "phase7-zero-vector".into(),
        projection_snapshot_id: projection.projection_snapshot_id.clone(),
        surface_id: "surface:vector".into(),
        surface_kind: RetrievalSurfaceKind::Vector,
        match_mode: SurfaceMatchMode::NearestNeighbours,
        operand: AccessOperand::Vector(vec![0.0; 1024]),
        page_size: 5,
        cursor: None,
    };
    match artifacts.probe(&projection, &zero_vector_probe) {
        Err(AccessError::Probe(message)) => {
            println!("zero_vector_probe=invalid_operand:{message}");
        }
        Err(error) => return Err(format!("unexpected zero-vector failure: {error}").into()),
        Ok(_) => return Err("zero vector unexpectedly executed".into()),
    }
    println!("probes=");
    for probe in probes {
        println!(
            "{}",
            serde_json::to_string(&artifacts.probe(&projection, &probe)?)?
        );
    }
    Ok(())
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
