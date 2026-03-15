use anyhow::Result;

use super::client::SpotifyClient;
use super::models::SearchResult;

impl SpotifyClient {
    pub async fn search(
        &self,
        query: &str,
        search_type: &str,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SearchResult> {
        let limit = limit.unwrap_or(20).min(50);
        let offset = offset.unwrap_or(0);
        let path = format!(
            "/search?q={}&type={}&limit={}&offset={}",
            urlencoding::encode(query),
            urlencoding::encode(search_type),
            limit,
            offset,
        );
        self.get(&path).await
    }
}
