
use surrealdb::{
    Surreal,
    engine::local::{Db, Mem},
};

use crate::Configuration;

/// Setup SurrealDB
pub async fn setup_db(config: &Configuration) -> Result<Surreal<Db>, Box<dyn std::error::Error>> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("musicup").use_db("musicup").await?;

    Ok(db)
}
