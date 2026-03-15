use anyhow::Result;
use serde_json::json;

use super::client::SpotifyClient;
use super::models::{FullPlaylist, SnapshotResponse, UserPlaylists, UserProfile};

impl SpotifyClient {
    pub async fn get_current_user(&self) -> Result<UserProfile> {
        self.get("/me").await
    }

    pub async fn list_playlists(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<UserPlaylists> {
        let limit = limit.unwrap_or(20).min(50);
        let offset = offset.unwrap_or(0);
        self.get(&format!(
            "/me/playlists?limit={}&offset={}",
            limit, offset
        ))
        .await
    }

    pub async fn get_playlist(&self, playlist_id: &str) -> Result<FullPlaylist> {
        self.get(&format!("/playlists/{}", playlist_id)).await
    }

    pub async fn create_playlist(
        &self,
        name: &str,
        description: Option<&str>,
        public: Option<bool>,
    ) -> Result<FullPlaylist> {
        let user = self.get_current_user().await?;
        let mut body = json!({ "name": name });
        if let Some(desc) = description {
            body["description"] = json!(desc);
        }
        if let Some(p) = public {
            body["public"] = json!(p);
        }
        self.post(&format!("/users/{}/playlists", user.id), Some(body))
            .await
    }

    pub async fn add_tracks_to_playlist(
        &self,
        playlist_id: &str,
        uris: &[String],
        position: Option<u32>,
    ) -> Result<SnapshotResponse> {
        let mut body = json!({ "uris": uris });
        if let Some(pos) = position {
            body["position"] = json!(pos);
        }
        self.post(
            &format!("/playlists/{}/tracks", playlist_id),
            Some(body),
        )
        .await
    }

    pub async fn remove_tracks_from_playlist(
        &self,
        playlist_id: &str,
        uris: &[String],
    ) -> Result<()> {
        let tracks: Vec<serde_json::Value> = uris
            .iter()
            .map(|uri| json!({ "uri": uri }))
            .collect();
        let body = json!({ "tracks": tracks });
        self.delete(
            &format!("/playlists/{}/tracks", playlist_id),
            Some(body),
        )
        .await
    }

    pub async fn reorder_playlist_tracks(
        &self,
        playlist_id: &str,
        range_start: u32,
        insert_before: u32,
        range_length: Option<u32>,
    ) -> Result<SnapshotResponse> {
        let mut body = json!({
            "range_start": range_start,
            "insert_before": insert_before,
        });
        if let Some(len) = range_length {
            body["range_length"] = json!(len);
        }
        self.put_with_response(&format!("/playlists/{}/tracks", playlist_id), Some(body))
            .await
    }

    pub async fn replace_playlist_tracks(
        &self,
        playlist_id: &str,
        uris: &[String],
    ) -> Result<SnapshotResponse> {
        let body = json!({ "uris": uris });
        self.put_with_response(&format!("/playlists/{}/tracks", playlist_id), Some(body))
            .await
    }

    pub async fn update_playlist(
        &self,
        playlist_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        public: Option<bool>,
    ) -> Result<()> {
        let mut body = serde_json::Map::new();
        if let Some(n) = name {
            body.insert("name".to_string(), json!(n));
        }
        if let Some(d) = description {
            body.insert("description".to_string(), json!(d));
        }
        if let Some(p) = public {
            body.insert("public".to_string(), json!(p));
        }
        self.put(
            &format!("/playlists/{}", playlist_id),
            Some(serde_json::Value::Object(body)),
        )
        .await
    }
}
