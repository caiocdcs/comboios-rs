use comboios_server::{
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "comboios-server".into(),
        format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);
    tracing::info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    run(listener).await?;
    Ok(())
}
