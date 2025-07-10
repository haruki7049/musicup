//! Web server

use crate::{Configuration, archive::create_archive};
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeFile;

/// Web server
pub async fn server(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let zip: PathBuf = create_archive(config)?;

    // Generate address with port number
    let address: String = format!("{}:{}", config.address, config.port);

    // build our application with a single route
    let app = Router::new().route_service("/", ServeFile::new(zip));

    tracing::info!("Running at http://{}...", address);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
