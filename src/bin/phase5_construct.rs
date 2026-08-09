use semantic_traversal_core::construction::construct;
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let observation = args.next().map(PathBuf::from);
    let projection = args.next().map(PathBuf::from);
    let Some((observation, projection)) = observation.zip(projection) else {
        eprintln!("usage: phase5_construct <accepted-observation.json> <private-projection.json>");
        return ExitCode::from(2);
    };
    match construct(&observation, &projection) {
        Ok(report) => {
            println!("{}", serde_json::to_string_pretty(&report).unwrap());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("phase 5 construction blocked: {error}");
            ExitCode::from(1)
        }
    }
}
