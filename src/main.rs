use zero2prod::configurations::get_configurations;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configurations().expect("Failed to read configurations.");
    let application = Application::build(configurations).await?;
    application.run_until_stopped().await?;
    Ok(())
}
