fn main() {
    #[cfg(target_os = "windows")]
    windows::build! {
        Windows::Media::{SystemMediaTransportControls, MediaPlaybackStatus, MusicDisplayProperties, SystemMediaTransportControlsDisplayUpdater, MediaPlaybackType},
        Windows::Media::Playback::{MediaPlayer, MediaPlaybackCommandManager},
        Windows::Storage::Streams::RandomAccessStreamReference,
        Windows::Foundation::Uri
    }
}
