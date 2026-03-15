use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSavedTracksParams {
    /// Maximum number of tracks to return (max 50)
    pub limit: Option<u32>,
    /// Index of the first track to return
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SaveTracksParams {
    /// List of Spotify track IDs to save (not URIs, just the ID part)
    pub track_ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveSavedTracksParams {
    /// List of Spotify track IDs to remove from Liked Songs
    pub track_ids: Vec<String>,
}
