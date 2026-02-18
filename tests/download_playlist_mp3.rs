#[cfg(test)]
mod tests {
    use rusty_ytdl::search::Playlist;

    #[test]
    fn test_youtube_music_playlist_url_detection() {
        // YouTube Music playlist URL
        assert!(Playlist::is_playlist(
            "https://music.youtube.com/playlist?list=OLAK5uy_l1m0thk3g31NmIIz_vMIbQtyy7h4lLsE4"
        ));

        // YouTube Music album URL
        assert!(Playlist::is_playlist(
            "https://music.youtube.com/playlist?list=OLAK5uy_lx1m0thk3g31NmIIz_vMIbQtyy7h4lLsE"
        ));

        // Regular YouTube playlist URL
        assert!(Playlist::is_playlist(
            "https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD"
        ));

        // Bare playlist ID
        assert!(Playlist::is_playlist("PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD"));

        // Not a playlist
        assert!(!Playlist::is_playlist(
            "https://www.youtube.com/watch?v=FZ8BxMU3BYc"
        ));
    }

    #[test]
    fn test_youtube_music_get_playlist_url() {
        // YouTube Music playlist URL should be normalized to www.youtube.com
        let url = Playlist::get_playlist_url(
            "https://music.youtube.com/playlist?list=OLAK5uy_l1m0thk3g31NmIIz_vMIbQtyy7h4lLsE4",
        );
        assert!(url.is_some());
        let url = url.unwrap();
        assert!(url.starts_with("https://www.youtube.com/playlist?list="));
        assert!(url.contains("OLAK5uy_"));

        // Regular YouTube playlist URL
        let url = Playlist::get_playlist_url(
            "https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD",
        );
        assert!(url.is_some());
        assert!(url.unwrap().contains("PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD"));

        // Bare playlist ID
        let url = Playlist::get_playlist_url("PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD");
        assert!(url.is_some());
        assert!(url.unwrap().contains("PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD"));

        // Not a playlist
        let url = Playlist::get_playlist_url("https://www.youtube.com/watch?v=FZ8BxMU3BYc");
        assert!(url.is_none());
    }

    #[tokio::test]
    async fn test_fetch_youtube_playlist() {
        let playlist = Playlist::get(
            "https://www.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD",
            None,
        )
        .await;

        assert!(
            playlist.is_ok(),
            "Failed to fetch playlist: {:?}",
            playlist.err()
        );
        let playlist = playlist.unwrap();
        assert!(
            !playlist.name.is_empty(),
            "Playlist name should not be empty"
        );
        assert!(
            !playlist.videos.is_empty(),
            "Playlist should have at least one video"
        );

        // Each video should have an ID and title
        for video in &playlist.videos {
            assert!(!video.id.is_empty(), "Video ID should not be empty");
            assert!(!video.title.is_empty(), "Video title should not be empty");
        }
    }

    #[tokio::test]
    async fn test_fetch_youtube_music_playlist_url() {
        // Test that a music.youtube.com URL is properly handled
        // Using a regular YouTube playlist URL rewritten to music.youtube.com format
        let result = Playlist::get(
            "https://music.youtube.com/playlist?list=PLwMEL7UNT4o9iMzrvNBXZqXbNPFfT6rVD",
            None,
        )
        .await;

        // The playlist should either succeed or fail with a network error,
        // not an "is not a playlist" error
        match &result {
            Err(rusty_ytdl::VideoError::IsNotPlaylist(_)) => {
                panic!("music.youtube.com URL should be recognized as a playlist");
            }
            _ => {
                // Success or other network-related error is acceptable
            }
        }
    }
}
