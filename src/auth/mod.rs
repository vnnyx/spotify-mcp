pub mod oauth;
pub mod token;

use anyhow::Result;

use self::oauth::{build_oauth_client, refresh_access_token, run_oauth_flow, SpotifyOAuthClient};
use self::token::{load_token, save_token, TokenData};

pub async fn ensure_authenticated(
    client_id: &str,
    client_secret: &str,
) -> Result<(SpotifyOAuthClient, TokenData)> {
    let oauth_client = build_oauth_client(client_id, client_secret)?;

    let token = if let Some(mut existing) = load_token()? {
        if existing.is_expired() {
            eprintln!("Token expired, refreshing...");
            existing = refresh_access_token(&oauth_client, &existing.refresh_token).await?;
            save_token(&existing)?;
            eprintln!("Token refreshed successfully.");
        } else {
            eprintln!("Using existing token.");
        }
        existing
    } else {
        eprintln!("No token found, starting OAuth flow...");
        let new_token = run_oauth_flow(&oauth_client).await?;
        save_token(&new_token)?;
        eprintln!("Token saved successfully.");
        new_token
    };

    Ok((oauth_client, token))
}
