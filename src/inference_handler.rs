use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealml_core::storage::surml_file::SurMlFile;
use surrealml_core::execution::compute::ModelComputation;

use ndarray::{Array, ArrayD};
use serde_json::json;
use std::error::Error;

// Pseudo-code for fetching environment data from SurrealDB
async fn fetch_scene_context(_db: &Surreal<Db>) -> Result<String, Box<dyn Error>> {
    // Example:
    // let query_result = db.query("SELECT * FROM environment;").await?;
    // let environment_data = query_result.take::<Vec<EnvironmentRecord>>(0)?;
    //
    // For demonstration, just return a static JSON:
    let dummy_json = json!({
        "meshes": ["Wall", "Floor"],
        "objects": ["Key", "Door"]
    });
    Ok(dummy_json.to_string())
}

/// An async handler that:
/// 1. Fetches scene context from SurrealDB.
/// 2. Loads and runs a SurML model.
/// 3. Returns the output as a JSON string.
pub async fn inference_handler(
    db: &Surreal<Db>,
    instruction: &str
) -> Result<String, Box<dyn Error>> {
    // 1) Query SurrealDB for scene context (async).
    let scene_context = fetch_scene_context(db).await?;

    // 2) Combine user instruction + scene context into one string.
    let combined_input = format!("Instruction: {}\nScene: {}", instruction, scene_context);

    // 3) Load the SurML file.
    let mut file = SurMlFile::from_file("./test.surml")
        .map_err(|e| format!("Failed to load SurML file: {}", e))?;

    // 4) Create a compute unit for the model.
    let compute_unit = ModelComputation {
        surml_file: &mut file,
    };

    // 5) Convert the combined input into a raw ndarray<float>.
    //    For demonstration, map each byte to an f32.
    let input_vector: Vec<f32> = combined_input
        .bytes()
        .map(|b| b as f32)
        .collect();
    let data: ArrayD<f32> = Array::from(input_vector).into_dyn();

    // 6) Execute with `raw_compute` (headerless .surml).
    //    If your .surml has a header, you can use `buffered_compute` with named inputs.
    let output = compute_unit
        .raw_compute(data, None)
        .map_err(|e| format!("Failed to run raw_compute: {}", e))?;

    // 7) Serialize the model output to JSON.
    let output_json = serde_json::to_string(&output)
        .map_err(|e| format!("Failed to serialize model output: {}", e))?;

    // Return the final JSON.
    Ok(output_json)
}
