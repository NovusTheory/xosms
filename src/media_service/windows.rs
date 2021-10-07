windows::include_bindings!();

use neon::prelude::*;
use Windows::Foundation::Uri;
use Windows::Media::Playback::MediaPlayer;
use Windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, MusicDisplayProperties, SystemMediaTransportControls,
    SystemMediaTransportControlsDisplayUpdater,
};
use Windows::Storage::Streams::RandomAccessStreamReference;

pub struct MediaService {
    player: MediaPlayer,
    smtc: SystemMediaTransportControls,
}

impl Finalize for MediaService {}

impl MediaService {
    pub fn new() -> Self {
        let player = MediaPlayer::new().expect("Failed to create MediaPlayer");
        let smtc = player
            .SystemMediaTransportControls()
            .expect("Failed to get SystemMediaTransportControls");
        player
            .CommandManager()
            .expect("Failed to get CommandManager of MediaPlayer")
            .SetIsEnabled(false)
            .expect("Failed to disable CommandManager");

        Self { player, smtc }
    }

    // region Control
    pub fn is_enabled(&self) -> bool {
        return self
            .smtc
            .IsEnabled()
            .expect("Failed to set SystemMediaTransportControls.IsEnabled");
    }

    pub fn set_is_enabled(&self, enabled: bool) {
        self.smtc
            .SetIsEnabled(enabled)
            .expect("Failed to set SystemMediaTransportControls.SetIsEnabled");
    }
    // endregion Control

    // region Buttons
    pub fn is_play_enabled(&self) -> bool {
        return self
            .smtc
            .IsPlayEnabled()
            .expect("Failed to get SystemMediaTransportControls.IsPlayEnabled");
    }

    pub fn set_is_play_enabled(&self, enabled: bool) {
        self.smtc
            .SetIsPlayEnabled(enabled)
            .expect("Failed to set SystemMediaTransportControls.SetIsPlayEnabled");
    }

    pub fn is_pause_enabled(&self) -> bool {
        return self
            .smtc
            .IsPauseEnabled()
            .expect("Failed to get SystemMediaTransportControls.IsPauseEnabled");
    }

    pub fn set_is_pause_enabled(&self, enabled: bool) {
        self.smtc
            .SetIsPauseEnabled(enabled)
            .expect("Failed to set SystemMediaTransportControls.SetIsPauseEnabled");
    }

    pub fn is_previous_enabled(&self) -> bool {
        return self
            .smtc
            .IsPreviousEnabled()
            .expect("Failed to get SystemMediaTransportControls.IsPreviousEnabled");
    }

    pub fn set_is_previous_enabled(&self, enabled: bool) {
        self.smtc
            .SetIsPreviousEnabled(enabled)
            .expect("Failed to set SystemMediaTransportControls.SetIsPreviousEnabled");
    }

    pub fn is_next_enabled(&self) -> bool {
        return self
            .smtc
            .IsNextEnabled()
            .expect("Failed to get SystemMediaTransportControls.IsNextEnabled");
    }

    pub fn set_is_next_enabled(&self, enabled: bool) {
        self.smtc
            .SetIsNextEnabled(enabled)
            .expect("Failed to set SystemMediaTransportControls.SetIsNextEnabled");
    }
    // endregion Buttons

    // region Media Information
    pub fn get_media_type(&self) -> i32 {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        return du.Type().expect("Failed to get DisplayUpdater.Type").0;
    }

    pub fn set_media_type(&self, media_type: i32) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");

        du.SetType(MediaPlaybackType::from(media_type))
            .expect("Failed to get DisplayUpdater.Type");
        du.Update();
    }

    pub fn get_playback_status(&self) -> i32 {
        return self
            .smtc
            .PlaybackStatus()
            .expect("Failed to get SystemMediaTransportControls.PlaybackStatus")
            .0;
    }

    pub fn set_playback_status(&self, status: i32) {
        self.smtc
            .SetPlaybackStatus(MediaPlaybackStatus::from(status))
            .expect("Failed to set SystemMediaTransportControls.SetPlaybackStatus");
    }

    pub fn get_artist(&self) -> String {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        return mp
            .Artist()
            .expect("Failed to get MusicProperties.Artist")
            .to_string();
    }

    pub fn set_artist(&self, artist: String) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        mp.SetArtist(artist);
        du.Update();
    }

    pub fn get_album_artist(&self) -> String {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        return mp
            .AlbumArtist()
            .expect("Failed to get MusicProperties.Artist")
            .to_string();
    }

    pub fn set_album_artist(&self, album_artist: String) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        mp.SetAlbumArtist(album_artist);
        du.Update();
    }

    pub fn get_album_title(&self) -> String {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        return mp
            .AlbumTitle()
            .expect("Failed to get MusicProperties.Artist")
            .to_string();
    }

    pub fn set_album_title(&self, album_title: String) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        mp.SetAlbumTitle(album_title);
        du.Update();
    }

    pub fn get_title(&self) -> String {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        return mp
            .Title()
            .expect("Failed to get MusicProperties.Title")
            .to_string();
    }

    pub fn set_title(&self, title: String) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let mp = du
            .MusicProperties()
            .expect("Failed to get DisplayUpdater.MusicProperties");

        mp.SetTitle(title);
        du.Update();
    }

    pub fn set_thumbnail(&self, thumbnail_type: i32, thumbnail: String) {
        let du = self
            .smtc
            .DisplayUpdater()
            .expect("Failed to get SystemMediaTransportControls.DisplayUpdater");
        let stream = match thumbnail_type {
            2 => RandomAccessStreamReference::CreateFromUri(
                Uri::CreateUri(thumbnail).expect("Failed to create Foundation.Uri from thumbnail"),
            ),
            _ => panic!(),
        };
        du.SetThumbnail(
            stream.expect("Failed to create Streams.RandomAccessStreamReference from thumbnail"),
        );
        du.Update();
    }
    // endregion Media Information
}
