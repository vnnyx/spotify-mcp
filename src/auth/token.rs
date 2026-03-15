use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl TokenData {
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at - chrono::Duration::seconds(60)
    }
}

pub fn token_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not find config directory")?;
    let dir = config_dir.join("spotify-mcp");
    Ok(dir.join("token.json"))
}

pub fn load_token() -> Result<Option<TokenData>> {
    let path = token_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path)?;
    let token: TokenData = serde_json::from_str(&data)?;
    Ok(Some(token))
}

pub fn save_token(token: &TokenData) -> Result<()> {
    let path = token_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(token)?;
    std::fs::write(&path, data)?;
    Ok(())
}
