use semantic_traversal_core::validation::{ValidationError, validate};
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let observation = args.next().map(PathBuf::from);
    let phase5_projection = args.next().map(PathBuf::from);
    let phase6_projection = args.next().map(PathBuf::from);
    let Some((observation, phase5_projection, phase6_projection)) = observation
        .zip(phase5_projection)
        .zip(phase6_projection)
        .map(|((a, b), c)| (a, b, c))
    else {
        eprintln!(
            "usage: phase6_validate <vault-observation-v3.json> <phase5-projection.json> <phase6-validated-projection.json>"
        );
        return ExitCode::from(2);
    };
    match validate(&observation, &phase5_projection, &phase6_projection) {
        Ok(summary) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&summary).expect("summary serializes")
            );
            ExitCode::SUCCESS
        }
        Err(ValidationError::Violations(summary)) => {
            eprintln!(
                "phase 6 validation violations: {}",
                serde_json::to_string(&summary).expect("summary serializes")
            );
            ExitCode::from(1)
        }
        Err(error) => {
            eprintln!("phase 6 validation blocked: {error}");
            ExitCode::from(1)
        }
    }
}
