use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PlayParams {
    /// Spotify URI of the context to play (album, artist, or playlist URI)
    pub context_uri: Option<String>,
    /// List of Spotify track URIs to play
    pub uris: Option<Vec<String>>,
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PauseParams {
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NextTrackParams {
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PreviousTrackParams {
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetVolumeParams {
    /// Volume level from 0 to 100
    pub volume_percent: u32,
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetShuffleParams {
    /// Whether to enable shuffle
    pub state: bool,
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetRepeatParams {
    /// Repeat mode: "track", "context", or "off"
    pub state: String,
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SeekParams {
    /// Position in milliseconds to seek to
    pub position_ms: u64,
    /// The ID of the device to target
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetRecentlyPlayedParams {
    /// Number of items to return (max 50)
    pub limit: Option<u32>,
}
