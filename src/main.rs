use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, get_subscriber, init_subscriber, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed reading configuration file");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres!");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
