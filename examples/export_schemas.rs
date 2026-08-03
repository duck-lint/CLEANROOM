mod schema_support;

use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas");
    fs::create_dir_all(&schema_directory)?;

    for (filename, schema) in schema_support::generated_schemas() {
        fs::write(schema_directory.join(filename), schema)?;
    }

    Ok(())
}
