use axum::{
    routing::get,
    Router,
    response::Html,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    let tracing_format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact();
    tracing_subscriber::fmt()
        .event_format(tracing_format)
        .init();

    // build our application with a single route
    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    tracing::info!("Running at 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tracing::instrument]
async fn root() -> Html<&'static str> {
    tracing::info!("Accessed at root");
    Html("<h1>Hello, World!</h1>")
}
