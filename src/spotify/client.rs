use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::{Context, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::auth::oauth::{refresh_access_token, SpotifyOAuthClient};
use crate::auth::token::{save_token, TokenData};

const BASE_URL: &str = "https://api.spotify.com/v1";

#[derive(Clone)]
pub struct SpotifyClient {
    http: Client,
    token: Arc<RwLock<TokenData>>,
    oauth_client: SpotifyOAuthClient,
}

impl SpotifyClient {
    pub fn new(oauth_client: SpotifyOAuthClient, token: TokenData) -> Self {
        Self {
            http: Client::new(),
            token: Arc::new(RwLock::new(token)),
            oauth_client,
        }
    }

    async fn ensure_valid_token(&self) -> Result<String> {
        {
            let token = self.token.read().await;
            if !token.is_expired() {
                return Ok(token.access_token.clone());
            }
        }

        let mut token = self.token.write().await;
        // Double-check after acquiring write lock
        if !token.is_expired() {
            return Ok(token.access_token.clone());
        }

        eprintln!("Token expired, refreshing...");
        let new_token = refresh_access_token(&self.oauth_client, &token.refresh_token).await?;
        save_token(&new_token)?;
        *token = new_token;
        eprintln!("Token refreshed.");
        Ok(token.access_token.clone())
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let token = self.ensure_valid_token().await?;
        let resp = self
            .http
            .get(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        resp.json().await.context("Failed to parse response")
    }

    pub async fn get_optional<T: DeserializeOwned>(&self, path: &str) -> Result<Option<T>> {
        let token = self.ensure_valid_token().await?;
        let resp = self
            .http
            .get(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token)
            .send()
            .await?;

        let status = resp.status();
        if status.as_u16() == 204 {
            return Ok(None);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        Ok(Some(resp.json().await.context("Failed to parse response")?))
    }

    pub async fn put_with_response<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let token = self.ensure_valid_token().await?;
        let mut req = self
            .http
            .put(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token);

        match body {
            Some(b) => req = req.json(&b),
            None => req = req.header("Content-Length", "0"),
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        resp.json().await.context("Failed to parse response")
    }

    pub async fn put(&self, path: &str, body: Option<serde_json::Value>) -> Result<()> {
        let token = self.ensure_valid_token().await?;
        let mut req = self
            .http
            .put(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token);

        match body {
            Some(b) => req = req.json(&b),
            None => req = req.header("Content-Length", "0"),
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        Ok(())
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let token = self.ensure_valid_token().await?;
        let mut req = self
            .http
            .post(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token);

        match body {
            Some(b) => req = req.json(&b),
            None => req = req.header("Content-Length", "0"),
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        resp.json().await.context("Failed to parse response")
    }

    pub async fn post_no_response(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<()> {
        let token = self.ensure_valid_token().await?;
        let mut req = self
            .http
            .post(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token);

        match body {
            Some(b) => req = req.json(&b),
            None => req = req.header("Content-Length", "0"),
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        Ok(())
    }

    pub async fn delete(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<()> {
        let token = self.ensure_valid_token().await?;
        let mut req = self
            .http
            .delete(format!("{}{}", BASE_URL, path))
            .bearer_auth(&token);

        match body {
            Some(b) => req = req.json(&b),
            None => req = req.header("Content-Length", "0"),
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Spotify API error ({}): {}", status, body);
        }

        Ok(())
    }
}
