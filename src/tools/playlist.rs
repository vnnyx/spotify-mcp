use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListPlaylistsParams {
    /// Maximum number of playlists to return (max 50)
    pub limit: Option<u32>,
    /// Index of the first playlist to return
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetPlaylistParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreatePlaylistParams {
    /// Name for the new playlist
    pub name: String,
    /// Description for the new playlist
    pub description: Option<String>,
    /// Whether the playlist should be public
    pub public: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddTracksToPlaylistParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
    /// List of Spotify track URIs to add
    pub uris: Vec<String>,
    /// Position to insert the tracks (0-based)
    pub position: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveTracksFromPlaylistParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
    /// List of Spotify track URIs to remove
    pub uris: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdatePlaylistParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
    /// New name for the playlist
    pub name: Option<String>,
    /// New description for the playlist
    pub description: Option<String>,
    /// Whether the playlist should be public
    pub public: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReorderPlaylistTracksParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
    /// The position of the first track to be reordered (0-based index)
    pub range_start: u32,
    /// The position where the tracks should be inserted (0-based index)
    pub insert_before: u32,
    /// The number of tracks to be reordered (default: 1)
    pub range_length: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReplacePlaylistTracksParams {
    /// The Spotify ID of the playlist
    pub playlist_id: String,
    /// List of Spotify track URIs that will replace all tracks in the playlist
    pub uris: Vec<String>,
}
