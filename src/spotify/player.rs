use anyhow::Result;

use super::client::SpotifyClient;
use super::models::{CurrentlyPlaying, PlaybackState, RecentlyPlayed};

impl SpotifyClient {
    pub async fn play(
        &self,
        context_uri: Option<&str>,
        uris: Option<&[String]>,
        device_id: Option<&str>,
    ) -> Result<()> {
        let mut path = "/me/player/play".to_string();
        if let Some(id) = device_id {
            path = format!("{}?device_id={}", path, id);
        }

        let body = if context_uri.is_some() || uris.is_some() {
            let mut map = serde_json::Map::new();
            if let Some(ctx) = context_uri {
                map.insert(
                    "context_uri".to_string(),
                    serde_json::Value::String(ctx.to_string()),
                );
            }
            if let Some(u) = uris {
                map.insert(
                    "uris".to_string(),
                    serde_json::Value::Array(
                        u.iter()
                            .map(|s| serde_json::Value::String(s.clone()))
                            .collect(),
                    ),
                );
            }
            Some(serde_json::Value::Object(map))
        } else {
            None
        };

        self.put(&path, body).await
    }

    pub async fn pause(&self, device_id: Option<&str>) -> Result<()> {
        let mut path = "/me/player/pause".to_string();
        if let Some(id) = device_id {
            path = format!("{}?device_id={}", path, id);
        }
        self.put(&path, None).await
    }

    pub async fn next_track(&self, device_id: Option<&str>) -> Result<()> {
        let mut path = "/me/player/next".to_string();
        if let Some(id) = device_id {
            path = format!("{}?device_id={}", path, id);
        }
        self.post_no_response(&path, None).await
    }

    pub async fn previous_track(&self, device_id: Option<&str>) -> Result<()> {
        let mut path = "/me/player/previous".to_string();
        if let Some(id) = device_id {
            path = format!("{}?device_id={}", path, id);
        }
        self.post_no_response(&path, None).await
    }

    pub async fn set_volume(&self, volume_percent: u32, device_id: Option<&str>) -> Result<()> {
        let mut path = format!("/me/player/volume?volume_percent={}", volume_percent);
        if let Some(id) = device_id {
            path = format!("{}&device_id={}", path, id);
        }
        self.put(&path, None).await
    }

    pub async fn set_shuffle(&self, state: bool, device_id: Option<&str>) -> Result<()> {
        let mut path = format!("/me/player/shuffle?state={}", state);
        if let Some(id) = device_id {
            path = format!("{}&device_id={}", path, id);
        }
        self.put(&path, None).await
    }

    pub async fn set_repeat(&self, state: &str, device_id: Option<&str>) -> Result<()> {
        let mut path = format!("/me/player/repeat?state={}", state);
        if let Some(id) = device_id {
            path = format!("{}&device_id={}", path, id);
        }
        self.put(&path, None).await
    }

    pub async fn seek(&self, position_ms: u64, device_id: Option<&str>) -> Result<()> {
        let mut path = format!("/me/player/seek?position_ms={}", position_ms);
        if let Some(id) = device_id {
            path = format!("{}&device_id={}", path, id);
        }
        self.put(&path, None).await
    }

    pub async fn get_playback_state(&self) -> Result<Option<PlaybackState>> {
        self.get_optional("/me/player").await
    }

    pub async fn get_currently_playing(&self) -> Result<Option<CurrentlyPlaying>> {
        self.get_optional("/me/player/currently-playing").await
    }

    pub async fn get_recently_played(&self, limit: Option<u32>) -> Result<RecentlyPlayed> {
        let limit = limit.unwrap_or(20).min(50);
        self.get(&format!("/me/player/recently-played?limit={}", limit))
            .await
    }
}
