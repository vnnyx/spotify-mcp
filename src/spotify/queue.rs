use anyhow::Result;

use super::client::SpotifyClient;
use super::models::Queue;

impl SpotifyClient {
    pub async fn add_to_queue(&self, uri: &str, device_id: Option<&str>) -> Result<()> {
        let mut path = format!(
            "/me/player/queue?uri={}",
            urlencoding::encode(uri)
        );
        if let Some(id) = device_id {
            path = format!("{}&device_id={}", path, id);
        }
        self.post_no_response(&path, None).await
    }

    pub async fn get_queue(&self) -> Result<Queue> {
        self.get("/me/player/queue").await
    }
}
