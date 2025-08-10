use backend::{create_app, Config, Database};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().map_err(|e| format!("Failed to load configuration: {e}"))?;

    let database = Database::new(&config.database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {e}"))?;

    tracing::info!("Connected to database successfully");

    let app = create_app(config.clone(), database).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("API documentation available at http://{}/api/v1", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
