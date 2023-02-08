use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configurations::get_configurations;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configurations().expect("Failed to read configurations.");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configurations.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");

    let address = format!(
        "{}:{}",
        configurations.application.host, configurations.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener, connection_pool)?.await
}
