mod inference_handler;

use std::error::Error;
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// Import the async inference function from your inference_handler module
use inference_handler::inference_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Initialize SurrealDB with an ephemeral local storage engine:
    //    Using () as the endpoint creates an in-memory DB (no file).
    let db = Surreal::new::<Mem>(()).await?;

    // Optional sign-in (depends on your SurrealDB config)
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // Select namespace and database
    db.use_ns("test").use_db("test").await?;

    // 2. Simulate a string input from Whisper
    let whisper_input = "Open the door quietly";

    // 3. Run inference and get the JSON-like response
    let result_json = inference_handler(&db, whisper_input).await?;

    // 4. Output the resulting JSON to stdout (to be piped to Unreal Engine)
    println!("{}", result_json);

    Ok(())
}
