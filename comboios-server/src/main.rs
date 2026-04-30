use comboios_server::{
    configuration::Settings,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::from_env();

    let subscriber = get_subscriber(
        "comboios-server".into(),
        settings.log_filter.clone(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let addr = settings.bind_address();
    tracing::info!("Starting server on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    run(listener, settings).await?;
    Ok(())
}
