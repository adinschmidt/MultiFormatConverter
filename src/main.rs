use anyhow::Result;
use clap::Parser;
use tracing::debug;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser)]
#[command(author, version, about = "Multi Format Converter - Convert images and videos in bulk to save disk space — losslessly when possible.", long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Console layer (stdout)
    let console_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(false)
        .with_level(true);
    let log_level_string = if cli.debug { "debug" } else { "info" }.to_string();
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .parse_lossy(format!("mfc={0}", &log_level_string).as_str());
    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .try_init()?;
    
    debug!("Debug mode enabled");

    info!("Hello, world!");

    Ok(())
}
