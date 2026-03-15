mod auth;
mod error;
mod server;
mod spotify;
mod tools;

use anyhow::Context;
use rmcp::{ServiceExt, transport::stdio};

use auth::ensure_authenticated;
use server::SpotifyMcpServer;
use spotify::client::SpotifyClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Log to stderr so stdout is reserved for MCP stdio transport
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let client_id =
        std::env::var("SPOTIFY_CLIENT_ID").context("SPOTIFY_CLIENT_ID env var not set")?;
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").context("SPOTIFY_CLIENT_SECRET env var not set")?;

    eprintln!("Authenticating with Spotify...");
    let (oauth_client, token) = ensure_authenticated(&client_id, &client_secret).await?;
    eprintln!("Authenticated successfully.");

    let spotify_client = SpotifyClient::new(oauth_client, token);
    let server = SpotifyMcpServer::new(spotify_client);

    eprintln!("Starting Spotify MCP server on stdio...");
    let running = server.serve(stdio()).await?;
    running.waiting().await?;

    Ok(())
}
