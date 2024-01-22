use givemeback_internet::{startup, settings};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
use tracing_bunyan_formatter::BunyanFormattingLayer;

#[tokio::main]
async fn main() {
    set_tracing();
    let conf = settings::get_configuration().expect("Failed to read env file.");
    startup::run(conf).await.unwrap();
}

fn set_tracing() {
    let formated = BunyanFormattingLayer::new("Givemeback-internet".into(), std::io::stdout);
    let registry = tracing_subscriber::registry::Registry::default()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(formated);

    tracing::subscriber::set_global_default(registry).expect("Failed to set subscriber.");
}
