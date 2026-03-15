use anyhow::Result;
use serde_json::json;

use super::client::SpotifyClient;
use super::models::SavedTracks;

impl SpotifyClient {
    pub async fn get_saved_tracks(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SavedTracks> {
        let limit = limit.unwrap_or(20).min(50);
        let offset = offset.unwrap_or(0);
        self.get(&format!("/me/tracks?limit={}&offset={}", limit, offset))
            .await
    }

    pub async fn save_tracks(&self, track_ids: &[String]) -> Result<()> {
        let body = json!({ "ids": track_ids });
        self.put("/me/tracks", Some(body)).await
    }

    pub async fn remove_saved_tracks(&self, track_ids: &[String]) -> Result<()> {
        let body = json!({ "ids": track_ids });
        self.delete("/me/tracks", Some(body)).await
    }
}
