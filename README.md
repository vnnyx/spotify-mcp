# Spotify MCP Server

A Model Context Protocol (MCP) server that connects Claude to Spotify, enabling natural language control of playback, search, queue, playlists, and your music library.

## Features

### Playback Control (11 tools)
- **play** - Start/resume playback with optional context (album, playlist, artist) or specific tracks
- **pause** - Pause playback
- **next_track** / **previous_track** - Skip forward or back
- **set_volume** - Set volume (0-100)
- **set_shuffle** - Toggle shuffle on/off
- **set_repeat** - Set repeat mode (track, context, off)
- **seek** - Seek to position in current track
- **get_playback_state** - Get full playback state (track, device, progress, shuffle, repeat)
- **get_currently_playing** - Get the currently playing track
- **get_recently_played** - Get recently played tracks

### Queue Management (2 tools)
- **add_to_queue** - Add a track to the playback queue
- **get_queue** - View the current queue

### Search (1 tool)
- **search** - Search Spotify for tracks, artists, albums, and playlists

### Playlist Management (8 tools)
- **list_playlists** - List your playlists
- **get_playlist** - Get playlist details and tracks
- **create_playlist** - Create a new playlist
- **add_tracks_to_playlist** - Add tracks to a playlist
- **remove_tracks_from_playlist** - Remove tracks from a playlist
- **update_playlist** - Update playlist name, description, or visibility
- **reorder_playlist_tracks** - Move tracks within a playlist
- **replace_playlist_tracks** - Replace all tracks in a playlist (full reorder)

### Library (3 tools)
- **get_saved_tracks** - Get your Liked Songs
- **save_tracks** - Save tracks to Liked Songs
- **remove_saved_tracks** - Remove tracks from Liked Songs

## Prerequisites

- Rust toolchain (1.85+)
- A Spotify account (Premium required for playback control)
- A Spotify Developer application (for Client ID and Secret)

## Setup

### 1. Create a Spotify App

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new app
3. Set the redirect URI to `http://127.0.0.1:8888/callback`
4. Note your **Client ID** and **Client Secret**

### 2. Configure Environment

Create a `.env` file in the project root:

```
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
```

### 3. Build

```bash
cargo build --release
```

### 4. Authenticate

Run the binary once to complete the OAuth flow:

```bash
cargo run --release
```

This opens your browser for Spotify authorization. After approving, the token is saved to `~/.config/spotify-mcp/token.json` and refreshes automatically.

### 5. Configure Claude Desktop

Add the following to your Claude Desktop config file:

- **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **Linux**: `~/.config/Claude/claude_desktop_config.json`

#### Direct (Linux/macOS)

```json
{
  "mcpServers": {
    "spotify": {
      "command": "/path/to/spotify-mcp/target/release/spotify-mcp",
      "env": {
        "SPOTIFY_CLIENT_ID": "your_client_id",
        "SPOTIFY_CLIENT_SECRET": "your_client_secret"
      }
    }
  }
}
```

#### WSL (Windows with Linux backend)

```json
{
  "mcpServers": {
    "spotify": {
      "command": "wsl.exe",
      "args": [
        "--distribution", "YourDistroName",
        "--", "bash", "-c",
        "cd /path/to/spotify-mcp && ./target/release/spotify-mcp"
      ]
    }
  }
}
```

When using WSL, ensure the `.env` file is in the project directory so it gets loaded automatically.

Restart Claude Desktop after updating the config.

## Usage Examples

Once connected, you can ask Claude things like:

- "Play some jazz music"
- "What's currently playing?"
- "Add this song to my queue"
- "Search for songs by Radiohead"
- "Create a playlist called 'Road Trip' and add some upbeat songs"
- "Show me my Liked Songs"
- "Skip to the next track"
- "Set the volume to 50%"
- "Reorder my playlist by vibe"

## Notes

- **Spotify Premium** is required for playback control (play, pause, skip, volume, etc.). Search, playlists, and library tools work with free accounts.
- An **active Spotify session** (open Spotify on any device) is required for playback and queue commands.
- Tokens refresh automatically. If you encounter auth errors, delete `~/.config/spotify-mcp/token.json` and restart to re-authenticate.
