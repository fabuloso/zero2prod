use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration, email_client::EmailClient, get_subscriber, init_subscriber,
    startup::run,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed reading configuration file");

    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres!");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let email_sender = configuration
        .email_client
        .sender()
        .expect("Failed to create email sender!");
    let email_client = EmailClient::new(email_sender, configuration.email_client.base_url);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await
}
