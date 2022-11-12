#![feature(error_iter)]
use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
