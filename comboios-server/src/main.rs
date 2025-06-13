use comboios_server::{
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "comboios-server".into(),
        format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    run(listener).await?;
    Ok(())
}
