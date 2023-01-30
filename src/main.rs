use zero2prod::startup::run;
use zero2prod::configurations::get_configurations;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configurations().expect("Failed to read configurations.");

    let connection_pool = PgPool::connect(
            &configurations.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port.");
    run(listener, connection_pool)?.await
}
