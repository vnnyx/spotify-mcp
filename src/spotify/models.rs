use serde::Deserialize;

// Common types

#[derive(Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: Option<String>,
}

// Artist

#[derive(Debug, Deserialize)]
pub struct SimplifiedArtist {
    pub id: String,
    pub name: String,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Deserialize)]
pub struct FullArtist {
    pub id: String,
    pub name: String,
    pub genres: Option<Vec<String>>,
    pub popularity: Option<u32>,
    pub external_urls: ExternalUrls,
    pub images: Option<Vec<Image>>,
}

// Album

#[derive(Debug, Deserialize)]
pub struct SimplifiedAlbum {
    pub id: String,
    pub name: String,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    pub release_date: Option<String>,
    pub total_tracks: Option<u32>,
    pub external_urls: ExternalUrls,
    pub images: Option<Vec<Image>>,
}

// Track

#[derive(Debug, Deserialize)]
pub struct FullTrack {
    pub id: Option<String>,
    pub name: String,
    pub artists: Vec<SimplifiedArtist>,
    pub album: Option<SimplifiedAlbum>,
    pub duration_ms: u64,
    pub track_number: Option<u32>,
    pub popularity: Option<u32>,
    pub uri: String,
    pub external_urls: ExternalUrls,
    pub is_local: Option<bool>,
}

// Playback state

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub volume_percent: Option<u32>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub uri: String,
    #[serde(rename = "type")]
    pub context_type: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaybackState {
    pub device: Device,
    pub shuffle_state: bool,
    pub repeat_state: String,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
    pub progress_ms: Option<u64>,
    pub context: Option<Context>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentlyPlaying {
    pub is_playing: bool,
    pub item: Option<FullTrack>,
    pub progress_ms: Option<u64>,
    pub context: Option<Context>,
}

// Recently played

#[derive(Debug, Deserialize)]
pub struct PlayHistory {
    pub track: FullTrack,
    pub played_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RecentlyPlayed {
    pub items: Vec<PlayHistory>,
}

// Queue

#[derive(Debug, Deserialize)]
pub struct Queue {
    pub currently_playing: Option<FullTrack>,
    pub queue: Vec<FullTrack>,
}

// Search

#[derive(Debug, Deserialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub tracks: Option<Paginated<FullTrack>>,
    pub artists: Option<Paginated<FullArtist>>,
    pub albums: Option<Paginated<SimplifiedAlbum>>,
    pub playlists: Option<Paginated<SimplifiedPlaylist>>,
}

// Playlist

#[derive(Debug, Deserialize)]
pub struct PlaylistOwner {
    pub id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SimplifiedPlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub owner: PlaylistOwner,
    pub tracks: PlaylistTracksRef,
    pub external_urls: ExternalUrls,
    pub images: Option<Vec<Image>>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTracksRef {
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct FullPlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub owner: PlaylistOwner,
    pub tracks: Paginated<PlaylistTrack>,
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub track: Option<FullTrack>,
}

#[derive(Debug, Deserialize)]
pub struct UserPlaylists {
    pub items: Vec<SimplifiedPlaylist>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub display_name: Option<String>,
}

// Saved tracks (Liked Songs)

#[derive(Debug, Deserialize)]
pub struct SavedTrack {
    pub added_at: Option<String>,
    pub track: FullTrack,
}

#[derive(Debug, Deserialize)]
pub struct SavedTracks {
    pub items: Vec<SavedTrack>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

// Snapshot response for playlist modifications
#[derive(Debug, Deserialize)]
pub struct SnapshotResponse {
    pub snapshot_id: String,
}
