//! Query the LSP test database to verify metadata storage

use parseltongue_core::storage::CozoDbStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Querying LSP test database...\n");

    // Connect to database
    let db = CozoDbStorage::new("rocksdb:./lsp-test.db").await?;

    // Query for entities with LSP metadata
    let query = r#"
        ?[isgl1_key, current_code, lsp_metadata] :=
            *CodeGraph{
                isgl1_key,
                current_code,
                lsp_metadata
            }
        :limit 5
    "#;

    let result = db.query(query).await?;

    println!("Query result:");
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
