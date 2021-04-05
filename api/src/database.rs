use mongodb::{error::Result, options::ClientOptions, Client, Database};

pub async fn init() -> Result<Database> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("app");

    if !collection_exists("todos", &database).await? {
        database.create_collection("todos", None).await?;
    }

    return Ok(database);
}

async fn collection_exists(collection_name: &str, database: &Database) -> Result<bool> {
    let collection_names = database.list_collection_names(None).await?;
    return Ok(collection_names.contains(&collection_name.to_string()));
}
