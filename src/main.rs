mod inference_handler;
mod model_loader;
mod db_handler;

use std::error::Error;

use db_handler::init_db;
use model_loader::load_model;
// Import the async inference function from the inference_handler module
use inference_handler::inference_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the model by calling the function from the model_loader module
    load_model().expect("Failed to load model");
    
    // Initialize the database
    init_db().await.expect("Failed to initialize database");

    // Simulate a string input from Whisper
    let whisper_input = "Open the door";

    // Run inference and get the JSON-like response
    let result_json = inference_handler(whisper_input).await?;

    // Output the resulting JSON to stdout (to be piped to Unreal Engine)
    println!("{:?}", result_json);

    Ok(())
}
