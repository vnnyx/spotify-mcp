use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query string
    pub query: String,
    /// Comma-separated list of item types to search: "track", "artist", "album", "playlist"
    #[serde(rename = "type")]
    pub search_type: String,
    /// Maximum number of results to return (max 50)
    pub limit: Option<u32>,
    /// Index of the first result to return
    pub offset: Option<u32>,
}
