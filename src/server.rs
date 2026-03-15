use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::spotify::client::SpotifyClient;
use crate::tools::library::*;
use crate::tools::player::*;
use crate::tools::playlist::*;
use crate::tools::queue::*;
use crate::tools::search::*;

pub struct SpotifyMcpServer {
    client: SpotifyClient,
    tool_router: ToolRouter<Self>,
}

impl SpotifyMcpServer {
    pub fn new(client: SpotifyClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

// Helper for formatting durations
fn format_duration(ms: u64) -> String {
    let secs = ms / 1000;
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{}:{:02}", mins, secs)
}

fn format_artists(artists: &[crate::spotify::models::SimplifiedArtist]) -> String {
    artists
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_track(track: &crate::spotify::models::FullTrack) -> String {
    let artists = format_artists(&track.artists);
    let album = track
        .album
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("Unknown Album");
    format!(
        "\"{}\" by {} (Album: {}) [{}] - {}",
        track.name,
        artists,
        album,
        format_duration(track.duration_ms),
        track.uri,
    )
}

#[tool_router]
impl SpotifyMcpServer {
    /// Start or resume playback. Optionally provide a context URI (album/playlist/artist), specific track URIs, or a target device ID.
    #[tool(name = "play")]
    async fn play(&self, Parameters(params): Parameters<PlayParams>) -> Result<String, String> {
        self.client
            .play(
                params.context_uri.as_deref(),
                params.uris.as_deref(),
                params.device_id.as_deref(),
            )
            .await
            .map(|_| "Playback started/resumed.".to_string())
            .map_err(|e| e.to_string())
    }

    /// Pause playback on the current or specified device.
    #[tool(name = "pause")]
    async fn pause(&self, Parameters(params): Parameters<PauseParams>) -> Result<String, String> {
        self.client
            .pause(params.device_id.as_deref())
            .await
            .map(|_| "Playback paused.".to_string())
            .map_err(|e| e.to_string())
    }

    /// Skip to the next track in the queue.
    #[tool(name = "next_track")]
    async fn next_track(
        &self,
        Parameters(params): Parameters<NextTrackParams>,
    ) -> Result<String, String> {
        self.client
            .next_track(params.device_id.as_deref())
            .await
            .map(|_| "Skipped to next track.".to_string())
            .map_err(|e| e.to_string())
    }

    /// Skip to the previous track.
    #[tool(name = "previous_track")]
    async fn previous_track(
        &self,
        Parameters(params): Parameters<PreviousTrackParams>,
    ) -> Result<String, String> {
        self.client
            .previous_track(params.device_id.as_deref())
            .await
            .map(|_| "Skipped to previous track.".to_string())
            .map_err(|e| e.to_string())
    }

    /// Set the playback volume (0-100).
    #[tool(name = "set_volume")]
    async fn set_volume(
        &self,
        Parameters(params): Parameters<SetVolumeParams>,
    ) -> Result<String, String> {
        self.client
            .set_volume(params.volume_percent, params.device_id.as_deref())
            .await
            .map(|_| format!("Volume set to {}%.", params.volume_percent))
            .map_err(|e| e.to_string())
    }

    /// Enable or disable shuffle mode.
    #[tool(name = "set_shuffle")]
    async fn set_shuffle(
        &self,
        Parameters(params): Parameters<SetShuffleParams>,
    ) -> Result<String, String> {
        self.client
            .set_shuffle(params.state, params.device_id.as_deref())
            .await
            .map(|_| {
                format!(
                    "Shuffle {}.",
                    if params.state { "enabled" } else { "disabled" }
                )
            })
            .map_err(|e| e.to_string())
    }

    /// Set the repeat mode: "track", "context", or "off".
    #[tool(name = "set_repeat")]
    async fn set_repeat(
        &self,
        Parameters(params): Parameters<SetRepeatParams>,
    ) -> Result<String, String> {
        self.client
            .set_repeat(&params.state, params.device_id.as_deref())
            .await
            .map(|_| format!("Repeat mode set to '{}'.", params.state))
            .map_err(|e| e.to_string())
    }

    /// Seek to a position in the currently playing track (in milliseconds).
    #[tool(name = "seek")]
    async fn seek(&self, Parameters(params): Parameters<SeekParams>) -> Result<String, String> {
        self.client
            .seek(params.position_ms, params.device_id.as_deref())
            .await
            .map(|_| format!("Seeked to {}.", format_duration(params.position_ms)))
            .map_err(|e| e.to_string())
    }

    /// Get the current playback state including track, device, and progress.
    #[tool(name = "get_playback_state")]
    async fn get_playback_state(&self) -> Result<String, String> {
        let state = self
            .client
            .get_playback_state()
            .await
            .map_err(|e| e.to_string())?;

        match state {
            None => Ok("No active playback session.".to_string()),
            Some(s) => {
                let mut lines = vec![];
                lines.push(format!(
                    "Device: {} ({})",
                    s.device.name, s.device.device_type
                ));
                lines.push(format!("Playing: {}", s.is_playing));
                lines.push(format!("Shuffle: {}", s.shuffle_state));
                lines.push(format!("Repeat: {}", s.repeat_state));
                if let Some(vol) = s.device.volume_percent {
                    lines.push(format!("Volume: {}%", vol));
                }
                if let Some(ref track) = s.item {
                    lines.push(format!("Track: {}", format_track(track)));
                    if let Some(progress) = s.progress_ms {
                        lines.push(format!(
                            "Progress: {} / {}",
                            format_duration(progress),
                            format_duration(track.duration_ms)
                        ));
                    }
                }
                Ok(lines.join("\n"))
            }
        }
    }

    /// Get the currently playing track.
    #[tool(name = "get_currently_playing")]
    async fn get_currently_playing(&self) -> Result<String, String> {
        let current = self
            .client
            .get_currently_playing()
            .await
            .map_err(|e| e.to_string())?;

        match current {
            None => Ok("Nothing is currently playing.".to_string()),
            Some(c) => {
                let mut lines = vec![];
                lines.push(format!("Playing: {}", c.is_playing));
                if let Some(ref track) = c.item {
                    lines.push(format!("Track: {}", format_track(track)));
                    if let Some(progress) = c.progress_ms {
                        lines.push(format!(
                            "Progress: {} / {}",
                            format_duration(progress),
                            format_duration(track.duration_ms)
                        ));
                    }
                }
                Ok(lines.join("\n"))
            }
        }
    }

    /// Get recently played tracks.
    #[tool(name = "get_recently_played")]
    async fn get_recently_played(
        &self,
        Parameters(params): Parameters<GetRecentlyPlayedParams>,
    ) -> Result<String, String> {
        let recent = self
            .client
            .get_recently_played(params.limit)
            .await
            .map_err(|e| e.to_string())?;

        if recent.items.is_empty() {
            return Ok("No recently played tracks.".to_string());
        }

        let mut lines = vec![format!("Recently played ({} tracks):", recent.items.len())];
        for (i, item) in recent.items.iter().enumerate() {
            lines.push(format!(
                "{}. {} (played at {})",
                i + 1,
                format_track(&item.track),
                item.played_at
            ));
        }
        Ok(lines.join("\n"))
    }

    /// Add a track to the playback queue by its Spotify URI.
    #[tool(name = "add_to_queue")]
    async fn add_to_queue(
        &self,
        Parameters(params): Parameters<AddToQueueParams>,
    ) -> Result<String, String> {
        self.client
            .add_to_queue(&params.uri, params.device_id.as_deref())
            .await
            .map(|_| format!("Added {} to queue.", params.uri))
            .map_err(|e| e.to_string())
    }

    /// Get the current playback queue.
    #[tool(name = "get_queue")]
    async fn get_queue(&self) -> Result<String, String> {
        let queue = self.client.get_queue().await.map_err(|e| e.to_string())?;

        let mut lines = vec![];
        if let Some(ref current) = queue.currently_playing {
            lines.push(format!("Now playing: {}", format_track(current)));
        }
        if queue.queue.is_empty() {
            lines.push("Queue is empty.".to_string());
        } else {
            lines.push(format!("\nUp next ({} tracks):", queue.queue.len()));
            for (i, track) in queue.queue.iter().enumerate() {
                lines.push(format!("{}. {}", i + 1, format_track(track)));
            }
        }
        Ok(lines.join("\n"))
    }

    /// Search for tracks, artists, albums, or playlists on Spotify. The type parameter is a comma-separated list of item types (e.g. "track,artist").
    #[tool(name = "search")]
    async fn search(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<String, String> {
        let result = self
            .client
            .search(&params.query, &params.search_type, params.limit, params.offset)
            .await
            .map_err(|e| e.to_string())?;

        let mut lines = vec![];

        if let Some(ref tracks) = result.tracks {
            lines.push(format!("Tracks ({} total):", tracks.total));
            for (i, track) in tracks.items.iter().enumerate() {
                lines.push(format!("  {}. {}", i + 1, format_track(track)));
            }
        }

        if let Some(ref artists) = result.artists {
            lines.push(format!("\nArtists ({} total):", artists.total));
            for (i, artist) in artists.items.iter().enumerate() {
                let genres = artist
                    .genres
                    .as_ref()
                    .map(|g| g.join(", "))
                    .unwrap_or_default();
                let url = artist.external_urls.spotify.as_deref().unwrap_or("");
                lines.push(format!(
                    "  {}. {} (Genres: {}) - {}",
                    i + 1,
                    artist.name,
                    genres,
                    url
                ));
            }
        }

        if let Some(ref albums) = result.albums {
            lines.push(format!("\nAlbums ({} total):", albums.total));
            for (i, album) in albums.items.iter().enumerate() {
                let artists = format_artists(&album.artists);
                let url = album.external_urls.spotify.as_deref().unwrap_or("");
                lines.push(format!(
                    "  {}. \"{}\" by {} ({}) - {}",
                    i + 1,
                    album.name,
                    artists,
                    album.release_date.as_deref().unwrap_or(""),
                    url
                ));
            }
        }

        if let Some(ref playlists) = result.playlists {
            lines.push(format!("\nPlaylists ({} total):", playlists.total));
            for (i, pl) in playlists.items.iter().enumerate() {
                let owner = pl.owner.display_name.as_deref().unwrap_or(&pl.owner.id);
                let url = pl.external_urls.spotify.as_deref().unwrap_or("");
                lines.push(format!(
                    "  {}. \"{}\" by {} ({} tracks) - {}",
                    i + 1, pl.name, owner, pl.tracks.total, url
                ));
            }
        }

        if lines.is_empty() {
            Ok("No results found.".to_string())
        } else {
            Ok(lines.join("\n"))
        }
    }

    /// List the current user's playlists.
    #[tool(name = "list_playlists")]
    async fn list_playlists(
        &self,
        Parameters(params): Parameters<ListPlaylistsParams>,
    ) -> Result<String, String> {
        let playlists = self
            .client
            .list_playlists(params.limit, params.offset)
            .await
            .map_err(|e| e.to_string())?;

        if playlists.items.is_empty() {
            return Ok("No playlists found.".to_string());
        }

        let mut lines = vec![format!(
            "Your playlists ({} total):",
            playlists.total
        )];
        for (i, pl) in playlists.items.iter().enumerate() {
            let visibility = if pl.public.unwrap_or(false) {
                "public"
            } else {
                "private"
            };
            lines.push(format!(
                "{}. \"{}\" ({}, {} tracks) - ID: {}",
                i + 1,
                pl.name,
                visibility,
                pl.tracks.total,
                pl.id
            ));
        }
        Ok(lines.join("\n"))
    }

    /// Get details and tracks of a playlist by its Spotify ID.
    #[tool(name = "get_playlist")]
    async fn get_playlist(
        &self,
        Parameters(params): Parameters<GetPlaylistParams>,
    ) -> Result<String, String> {
        let playlist = self
            .client
            .get_playlist(&params.playlist_id)
            .await
            .map_err(|e| e.to_string())?;

        let mut lines = vec![];
        lines.push(format!("Playlist: \"{}\"", playlist.name));
        if let Some(ref desc) = playlist.description {
            if !desc.is_empty() {
                lines.push(format!("Description: {}", desc));
            }
        }
        let owner = playlist
            .owner
            .display_name
            .as_deref()
            .unwrap_or(&playlist.owner.id);
        lines.push(format!("Owner: {}", owner));
        let visibility = if playlist.public.unwrap_or(false) {
            "public"
        } else {
            "private"
        };
        lines.push(format!("Visibility: {}", visibility));
        lines.push(format!("Total tracks: {}", playlist.tracks.total));

        if !playlist.tracks.items.is_empty() {
            lines.push("\nTracks:".to_string());
            for (i, item) in playlist.tracks.items.iter().enumerate() {
                if let Some(ref track) = item.track {
                    lines.push(format!("  {}. {}", i + 1, format_track(track)));
                }
            }
        }
        Ok(lines.join("\n"))
    }

    /// Create a new playlist for the current user.
    #[tool(name = "create_playlist")]
    async fn create_playlist(
        &self,
        Parameters(params): Parameters<CreatePlaylistParams>,
    ) -> Result<String, String> {
        let playlist = self
            .client
            .create_playlist(&params.name, params.description.as_deref(), params.public)
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "Created playlist \"{}\" (ID: {})",
            playlist.name, playlist.id
        ))
    }

    /// Add tracks to a playlist by their Spotify URIs.
    #[tool(name = "add_tracks_to_playlist")]
    async fn add_tracks_to_playlist(
        &self,
        Parameters(params): Parameters<AddTracksToPlaylistParams>,
    ) -> Result<String, String> {
        let result = self
            .client
            .add_tracks_to_playlist(&params.playlist_id, &params.uris, params.position)
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "Added {} track(s) to playlist. Snapshot: {}",
            params.uris.len(),
            result.snapshot_id
        ))
    }

    /// Remove tracks from a playlist by their Spotify URIs.
    #[tool(name = "remove_tracks_from_playlist")]
    async fn remove_tracks_from_playlist(
        &self,
        Parameters(params): Parameters<RemoveTracksFromPlaylistParams>,
    ) -> Result<String, String> {
        self.client
            .remove_tracks_from_playlist(&params.playlist_id, &params.uris)
            .await
            .map(|_| {
                format!(
                    "Removed {} track(s) from playlist.",
                    params.uris.len()
                )
            })
            .map_err(|e| e.to_string())
    }

    /// Update a playlist's name, description, or visibility.
    #[tool(name = "update_playlist")]
    async fn update_playlist(
        &self,
        Parameters(params): Parameters<UpdatePlaylistParams>,
    ) -> Result<String, String> {
        self.client
            .update_playlist(
                &params.playlist_id,
                params.name.as_deref(),
                params.description.as_deref(),
                params.public,
            )
            .await
            .map(|_| "Playlist updated.".to_string())
            .map_err(|e| e.to_string())
    }

    /// Reorder tracks in a playlist. Moves a range of tracks from one position to another.
    #[tool(name = "reorder_playlist_tracks")]
    async fn reorder_playlist_tracks(
        &self,
        Parameters(params): Parameters<ReorderPlaylistTracksParams>,
    ) -> Result<String, String> {
        let result = self
            .client
            .reorder_playlist_tracks(
                &params.playlist_id,
                params.range_start,
                params.insert_before,
                params.range_length,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "Reordered tracks in playlist. Snapshot: {}",
            result.snapshot_id
        ))
    }

    /// Replace all tracks in a playlist with the given track URIs. This can be used to reorder an entire playlist.
    #[tool(name = "replace_playlist_tracks")]
    async fn replace_playlist_tracks(
        &self,
        Parameters(params): Parameters<ReplacePlaylistTracksParams>,
    ) -> Result<String, String> {
        let result = self
            .client
            .replace_playlist_tracks(&params.playlist_id, &params.uris)
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "Replaced all tracks in playlist ({} tracks). Snapshot: {}",
            params.uris.len(),
            result.snapshot_id
        ))
    }

    /// Get the current user's Liked Songs (saved tracks).
    #[tool(name = "get_saved_tracks")]
    async fn get_saved_tracks(
        &self,
        Parameters(params): Parameters<GetSavedTracksParams>,
    ) -> Result<String, String> {
        let saved = self
            .client
            .get_saved_tracks(params.limit, params.offset)
            .await
            .map_err(|e| e.to_string())?;

        if saved.items.is_empty() {
            return Ok("No saved tracks found.".to_string());
        }

        let mut lines = vec![format!(
            "Liked Songs ({} total, showing {}-{}):",
            saved.total,
            saved.offset + 1,
            saved.offset + saved.items.len() as u32
        )];
        for (i, item) in saved.items.iter().enumerate() {
            lines.push(format!(
                "{}. {}",
                saved.offset as usize + i + 1,
                format_track(&item.track)
            ));
        }
        Ok(lines.join("\n"))
    }

    /// Save tracks to the current user's Liked Songs library.
    #[tool(name = "save_tracks")]
    async fn save_tracks(
        &self,
        Parameters(params): Parameters<SaveTracksParams>,
    ) -> Result<String, String> {
        self.client
            .save_tracks(&params.track_ids)
            .await
            .map(|_| format!("Saved {} track(s) to Liked Songs.", params.track_ids.len()))
            .map_err(|e| e.to_string())
    }

    /// Remove tracks from the current user's Liked Songs library.
    #[tool(name = "remove_saved_tracks")]
    async fn remove_saved_tracks(
        &self,
        Parameters(params): Parameters<RemoveSavedTracksParams>,
    ) -> Result<String, String> {
        self.client
            .remove_saved_tracks(&params.track_ids)
            .await
            .map(|_| {
                format!(
                    "Removed {} track(s) from Liked Songs.",
                    params.track_ids.len()
                )
            })
            .map_err(|e| e.to_string())
    }
}

#[tool_handler]
impl ServerHandler for SpotifyMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("spotify-mcp", "0.1.0"))
    }
}
