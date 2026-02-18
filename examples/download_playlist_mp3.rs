use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};

#[tokio::main]
async fn main() {
    // Works with both YouTube and YouTube Music playlist URLs:
    //   - https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD
    //   - https://music.youtube.com/playlist?list=OLAK5uy_example
    let playlist_url = std::env::args()
        .nth(1)
        .expect("Usage: download_playlist_mp3 <playlist_url> [output_dir]");

    let output_dir = std::env::args().nth(2).unwrap_or("./downloads".to_string());

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    println!("Fetching playlist...");
    let playlist = Playlist::get(
        &playlist_url,
        Some(&PlaylistSearchOptions {
            fetch_all: true,
            ..Default::default()
        }),
    )
    .await
    .expect("Failed to fetch playlist");

    println!(
        "Playlist: {} ({} tracks)",
        playlist.name,
        playlist.videos.len()
    );
    println!("Downloading to: {}\n", output_dir);

    // Download all tracks as MP3 using FFmpeg
    // Uses audio-only stream with highest audio quality by default
    let results = playlist
        .download_audio_as_mp3(&output_dir, None, None)
        .await;

    // Print results
    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(path) => {
                success_count += 1;
                println!("[{}/{}] ✓ {}", i + 1, results.len(), path.display());
            }
            Err(e) => {
                fail_count += 1;
                println!("[{}/{}] ✗ {}", i + 1, results.len(), e);
            }
        }
    }

    println!(
        "\nDone! {} succeeded, {} failed out of {} tracks.",
        success_count,
        fail_count,
        results.len()
    );
}
