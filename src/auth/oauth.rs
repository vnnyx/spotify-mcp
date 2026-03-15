use anyhow::{Context, Result};
use chrono::Utc;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, RedirectUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl,
};
use std::io::{BufRead, Write};

use super::token::TokenData;

const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const REDIRECT_URI: &str = "http://127.0.0.1:8888/callback";

const SCOPES: &[&str] = &[
    "user-modify-playback-state",
    "user-read-playback-state",
    "user-read-currently-playing",
    "user-read-recently-played",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-library-read",
    "user-library-modify",
];

/// BasicClient with auth and token URLs set.
pub type SpotifyOAuthClient = oauth2::Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

fn http_client() -> oauth2::reqwest::Client {
    oauth2::reqwest::ClientBuilder::new()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build HTTP client")
}

pub fn build_oauth_client(client_id: &str, client_secret: &str) -> Result<SpotifyOAuthClient> {
    let client = oauth2::basic::BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(AuthUrl::new(AUTH_URL.to_string())?)
        .set_token_uri(TokenUrl::new(TOKEN_URL.to_string())?)
        .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string())?);

    Ok(client)
}

pub async fn run_oauth_flow(client: &SpotifyOAuthClient) -> Result<TokenData> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let mut auth_request = client.authorize_url(CsrfToken::new_random);
    for scope in SCOPES {
        auth_request = auth_request.add_scope(Scope::new(scope.to_string()));
    }
    let (auth_url, csrf_state) = auth_request.set_pkce_challenge(pkce_challenge).url();

    eprintln!("Opening browser for Spotify authorization...");
    if open::that(auth_url.as_str()).is_err() {
        eprintln!("Could not open browser. Please visit this URL manually:");
        eprintln!("{}", auth_url);
    }

    let code = receive_callback(csrf_state.secret()).await?;

    let http = http_client();
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http)
        .await
        .context("Failed to exchange authorization code for token")?;

    let expires_in = token_result
        .expires_in()
        .unwrap_or(std::time::Duration::from_secs(3600));

    let token_data = TokenData {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: token_result
            .refresh_token()
            .context("No refresh token received")?
            .secret()
            .clone(),
        expires_at: Utc::now() + chrono::Duration::seconds(expires_in.as_secs() as i64),
    };

    Ok(token_data)
}

async fn receive_callback(expected_state: &str) -> Result<String> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888").await?;
    eprintln!("Waiting for Spotify callback on http://127.0.0.1:8888/callback ...");

    let (stream, _) = listener.accept().await?;
    let std_stream = stream.into_std()?;
    std_stream.set_nonblocking(false)?;
    let mut reader = std::io::BufReader::new(std_stream.try_clone()?);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let url = request_line
        .split_whitespace()
        .nth(1)
        .context("Invalid HTTP request")?;

    let full_url = format!("http://127.0.0.1:8888{}", url);
    let parsed = reqwest::Url::parse(&full_url)?;

    let code = parsed
        .query_pairs()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.to_string())
        .context("No code parameter in callback")?;

    let state = parsed
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.to_string())
        .context("No state parameter in callback")?;

    if state != expected_state {
        anyhow::bail!("CSRF state mismatch");
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authorization successful!</h1><p>You can close this tab.</p></body></html>";
    let mut writer = std_stream;
    writer.write_all(response.as_bytes())?;
    writer.flush()?;

    Ok(code)
}

pub async fn refresh_access_token(
    client: &SpotifyOAuthClient,
    refresh_token: &str,
) -> Result<TokenData> {
    let http = http_client();
    let token_result = client
        .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token.to_string()))
        .request_async(&http)
        .await
        .context("Failed to refresh access token")?;

    let expires_in = token_result
        .expires_in()
        .unwrap_or(std::time::Duration::from_secs(3600));

    let new_refresh = token_result
        .refresh_token()
        .map(|t| t.secret().clone())
        .unwrap_or_else(|| refresh_token.to_string());

    Ok(TokenData {
        access_token: token_result.access_token().secret().clone(),
        refresh_token: new_refresh,
        expires_at: Utc::now() + chrono::Duration::seconds(expires_in.as_secs() as i64),
    })
}
