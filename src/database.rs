//! Database

pub mod archive;

use crate::Configuration;
use surrealdb::{
    Surreal,
    engine::local::{Db, Mem},
};

/// Setup SurrealDB
pub async fn setup_db(config: &Configuration) -> Result<Surreal<Db>, Box<dyn std::error::Error>> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("musicup").use_db("musicup").await?;

    archive::create_archive(config)?;

    Ok(db)
}
