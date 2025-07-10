use crate::{ADDRESS, Configuration};
use axum::{Router, extract::Query, response::Html, routing::get};
use serde::Deserialize;
use surrealdb::{Surreal, engine::local::Db};

/// URL Query for Musicup
#[derive(Debug, Deserialize)]
struct MusicupRequest {
    extension: Extension,
}

/// File extensions we support
#[derive(Debug, Deserialize)]
enum Extension {
    // FLAC
    #[serde(rename = "flac")]
    Flac,

    // MP3
    #[serde(rename = "mp3")]
    Mp3,
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Extension::Flac => write!(f, "flac"),
            Extension::Mp3 => write!(f, "mp3"),
        }
    }
}

/// Web server
pub async fn server(
    config: &Configuration,
    database: Surreal<Db>,
) -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a single route
    let app = Router::new().route("/", get(root));

    tracing::info!("Running at http://{}...", ADDRESS);
    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Root page
#[tracing::instrument]
async fn root(request: Query<MusicupRequest>) -> Html<String> {
    tracing::info!("Accessing...");
    let message: String = format!(
        "<h1>Hello, World!</h1><p>Selected file extension is: {}</p>",
        request.extension
    );
    Html(message)
}
