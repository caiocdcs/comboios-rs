// basic handler that responds with a static string
pub async fn health_check() -> &'static str {
    "PONG"
}
