use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Surreal;

// For an in memory database
use surrealdb::engine::local::Mem;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: RecordId,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create database connection in memory
    let db = Surreal::new::<Mem>(()).await?;
    
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(())
}