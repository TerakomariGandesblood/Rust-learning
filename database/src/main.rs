use std::path::PathBuf;

use anyhow::Result;
use sea_orm::Database;

async fn connect() -> Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test.db");

    let database_url = format!("sqlite:{}?mode=rwc", path.display());

    let _db = Database::connect(database_url).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    connect().await?;
    Ok(())
}
