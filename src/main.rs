use clap::Parser;
use musicup::{Configuration, webserver::server};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt().init();

    // Get Command-Line arguments
    let args: CLIArgs = CLIArgs::parse();

    // Get configuration
    let config: Configuration = confy::load_path(args.config)?;

    // Run the web application.
    server(&config).await?;

    Ok(())
}

#[derive(Parser)]
#[clap(version, about, author)]
struct CLIArgs {
    #[arg(short, long, default_value = confy::get_configuration_file_path("musicup", "musicup").expect("Failed to get default configuration path by confy crate").into_os_string())]
    config: PathBuf,
}
