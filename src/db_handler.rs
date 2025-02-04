use surrealdb::engine::local::Mem;
// use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealml_core::errors::error::SurrealError;

pub async fn init_db() -> Result<Mem, SurrealError>{
    
    // nitialize SurrealDB with an ephemeral local storage engine:
    // Using () as the endpoint creates an in-memory DB (no file).
    let db = Surreal::new::<Mem>(()).await.expect("Failed to initialize SurrealDB");
    
    // Select namespace and database
    db.use_ns("test").use_db("test").await.expect( "Failed to select namespace and database");
    Ok(Mem)
}