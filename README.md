# rusty_ytdl (fork)

A fork of rusty_ytdl with custom functionality.

## Playlist MP3 Download

Download all tracks from a YouTube or YouTube Music playlist as MP3 files. Requires FFmpeg installed on your system and the `search` + `ffmpeg` feature flags.

### Programmatic Usage

```rust
use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};

#[tokio::main]
async fn main() {
    // Works with both YouTube and YouTube Music URLs
    let url = "https://music.youtube.com/playlist?list=OLAK5uy_xxxxx";

    let playlist = Playlist::get(url, Some(&PlaylistSearchOptions {
        fetch_all: true,
        ..Default::default()
    })).await.unwrap();

    println!("{} ({} tracks)", playlist.name, playlist.videos.len());

    // Download all tracks as MP3
    let results = playlist.download_audio_as_mp3("./output", None, None).await;

    for result in &results {
        match result {
            Ok(path) => println!("✓ {}", path.display()),
            Err(e) => println!("✗ {}", e),
        }
    }
}
```

### Custom FFmpeg & Video Options

```rust
use rusty_ytdl::{FFmpegArgs, VideoOptions, VideoQuality, VideoSearchOptions};

let results = playlist.download_audio_as_mp3(
    "./output",
    Some(VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAudio,
        ..Default::default()
    }),
    Some(FFmpegArgs {
        format: Some("mp3".to_string()),
        audio_filter: Some("aresample=48000".to_string()),
        video_filter: None,
    }),
).await;
```

### Raw Audio Download (No FFmpeg)

```rust
// Downloads as m4a (no FFmpeg required, only needs "search" feature)
let results = playlist.download_audio("./output", None).await;
```

### CLI Example

```bash
cargo run --example download_playlist_mp3 --features "search,ffmpeg" -- \
  "https://music.youtube.com/playlist?list=YOUR_PLAYLIST_ID" ./my_music
```

### Supported URL Formats

| Format | Example |
|---|---|
| YouTube Music playlist | `https://music.youtube.com/playlist?list=OLAK5uy_xxx` |
| YouTube Music album | `https://music.youtube.com/playlist?list=OLAK5uy_xxx` |
| YouTube playlist | `https://www.youtube.com/playlist?list=PLxxx` |
| Bare playlist ID | `PLxxx` or `OLAK5uy_xxx` |