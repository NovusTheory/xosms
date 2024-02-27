use neon::event::Channel;
use neon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::media_service::service_trait::MediaServiceTrait;

static COMMAND_CALLBACK: Mutex<Option<RemoteCommandButtonCallback>> = Mutex::new(None);

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct NowPlayingInfo {
        track_id: String,
        media_type: usize,
        title: String,
        artist: String,
        album_artist: String,
        album_title: String,
        artwork: String,
        artwork_type: usize
    }

    extern "Rust" {
        fn rust_remote_command_handler(command: &str);
    }

    extern "Swift" {
        fn swift_configure_commands();

        fn swift_set_playback_state(state: usize);
        fn swift_set_info(info: NowPlayingInfo);
        fn swift_set_remote_command_enabled(command: &str, enabled: bool);

        fn swift_get_playback_state() -> usize;
        fn swift_get_info() -> NowPlayingInfo;

        fn swift_is_remote_command_enabled(command: &str) -> bool;
    }
}

fn rust_remote_command_handler(command: &str) {
    let command_callback_option = COMMAND_CALLBACK.lock().unwrap();
    match &*command_callback_option {
        Some(command_callback) => {
            let callback = command_callback.callback.clone();
            let channel = command_callback.channel.clone();
            match command {
                "pause" => MediaService::send_button_pressed(callback, channel, "pause"),
                "play" => MediaService::send_button_pressed(callback, channel, "play"),
                "stop" => MediaService::send_button_pressed(callback, channel, "stop"),
                "playpause" => MediaService::send_button_pressed(callback, channel, "playpause"),
                "next" => MediaService::send_button_pressed(callback, channel, "next"),
                "previous" => MediaService::send_button_pressed(callback, channel, "previous"),
                _ => {}
            }
        }
        None => {}
    }
}

pub struct RemoteCommandButtonCallback {
    callback: Arc<Root<JsFunction>>,
    channel: Channel,
}

pub struct MediaService {
    playing_info: ffi::NowPlayingInfo
}

impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        ffi::swift_configure_commands();

        Self {
            playing_info: ffi::NowPlayingInfo { 
                track_id: "".to_string(),
                media_type: 0,
                title: "".to_string(),
                artist: "".to_string(),
                album_artist: "".to_string(),
                album_title: "".to_string(),
                artwork: "".to_string(),
                artwork_type: 0
            }
        }
    }

    fn update_info(&self) {
        // TODO: Make this better
        ffi::swift_set_info(ffi::NowPlayingInfo {
            track_id: self.playing_info.track_id.clone(),
            media_type: self.playing_info.media_type,
            title: self.playing_info.title.clone(),
            artist: self.playing_info.artist.clone(),
            album_artist: self.playing_info.album_artist.clone(),
            album_title: self.playing_info.album_title.clone(),
            artwork: self.playing_info.artwork.clone(),
            artwork_type: self.playing_info.artwork_type
        });
    }

    fn send_button_pressed(
        callback: Arc<Root<JsFunction>>,
        channel: Channel,
        button: &'static str,
    ) {
        let channel = channel.clone();
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let args = [cx.string(button).upcast()];
            let _result = callback.call(&mut cx, this, args);
            Ok(())
        });
    }
}

impl MediaServiceTrait for MediaService {
    fn get_playback_status(&self) -> Result<i32, String> {
        let state = ffi::swift_get_playback_state();
        let corrected_state = match state {
            1 => 3, // Playing
            2 => 4, // Paused
            3 => 2, // Stopped
            _ => -1,
        };

        Ok(corrected_state)
    }

    fn set_playback_status(&mut self, status: i32) -> Result<(), String> {
        let state = match status {
            2 => 3, // Stopped
            3 => 1, // Playing
            4 => 2, // Paused
            _ => 0,
        };
        ffi::swift_set_playback_state(state);

        Ok(())
    }

    fn get_media_type(&self) -> Result<i32, String> {
        let media_type = ffi::swift_get_info().media_type;
        let corrected_media_type = match media_type {
            1 => 1,
            0xff00 => 2,
            _ => 0
        };

        Ok(corrected_media_type)
    }

    fn set_media_type(&mut self, media_type: i32) -> Result<(), String> {
        let state: usize = match media_type {
            1 => 1,
            2 => 0xff00,
            _ => !0,
        };

        self.playing_info.media_type = state;
        self.update_info();

        Ok(())
    }

    fn get_artist(&self) -> Result<String, String> {
        Ok(ffi::swift_get_info().artist)
    }

    fn set_artist(&mut self, artist: String) -> Result<(), String> {
        self.playing_info.artist = artist;
        self.update_info();

        Ok(())
    }

    fn get_album_artist(&self) -> Result<String, String> {
        Ok(ffi::swift_get_info().album_artist)
    }

    fn set_album_artist(&mut self, album_artist: String) -> Result<(), String> {
        self.playing_info.album_artist = album_artist;
        self.update_info();

        Ok(())
    }

    fn get_album_title(&self) -> Result<String, String> {
        Ok(ffi::swift_get_info().album_title)
    }

    fn set_album_title(&mut self, album_title: String) -> Result<(), String> {
        self.playing_info.album_title = album_title;
        self.update_info();

        Ok(())
    }

    fn get_title(&self) -> Result<String, String> {
        Ok(ffi::swift_get_info().title)
    }

    fn set_title(&mut self, title: String) -> Result<(), String> {
        self.playing_info.title = title;
        self.update_info();

        Ok(())
    }

    fn get_track_id(&self) -> Result<String, String> {
        Ok("".to_string())
    }

    fn set_track_id(&mut self, track_id: String) -> Result<(), String> {
        self.playing_info.track_id = track_id;
        self.update_info();

        Ok(())
    }

    fn set_thumbnail(&mut self, thumbnail_type: i32, thumbnail: String) -> Result<(), String> {
        match thumbnail_type {
            1 => {
                self.playing_info.artwork_type = 1;
            },
            2 => {
                self.playing_info.artwork_type = 2;
            },
            _ => {
                return Err(format!(
                    "Thumbnail type is not supported on this operating system: {}",
                    thumbnail_type
                ));
            }
        }
        self.playing_info.artwork = thumbnail;

        Ok(())
    }

    fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        let mut command_callback = COMMAND_CALLBACK.lock().unwrap();
        *command_callback = Some(RemoteCommandButtonCallback { callback: Arc::new(callback), channel });

        /*unsafe {
            let callback = std::sync::Arc::new(callback);

            let command_handler = ConcreteBlock::new(
                move |e: MPRemoteCommandEvent| -> MPRemoteCommandHandlerStatus {
                    let command: MPRemoteCommand = msg_send!(e, command);
                    let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
                    let callback = callback.clone();
                    let channel = channel.clone();

                    if command.0 == remote_command_center.playCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "play");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    if command.0 == remote_command_center.pauseCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "pause");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    if command.0 == remote_command_center.togglePlayPauseCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "playpause");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    if command.0 == remote_command_center.stopCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "stop");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    if command.0 == remote_command_center.nextTrackCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "next");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    if command.0 == remote_command_center.previousTrackCommand().0 {
                        MediaService::send_button_pressed(callback, channel, "previous");
                        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                    }
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusCommandFailed;
                },
            );
            let command_handler = command_handler.copy();
            self.remote_command_center
                .playCommand()
                .addTargetWithHandler_(&*command_handler);
            self.remote_command_center
                .pauseCommand()
                .addTargetWithHandler_(&*command_handler);
            self.remote_command_center
                .togglePlayPauseCommand()
                .addTargetWithHandler_(&*command_handler);
            self.remote_command_center
                .stopCommand()
                .addTargetWithHandler_(&*command_handler);
            self.remote_command_center
                .nextTrackCommand()
                .addTargetWithHandler_(&*command_handler);
            self.remote_command_center
                .previousTrackCommand()
                .addTargetWithHandler_(&*command_handler);
        }*/

        Ok(-1)
    }

    fn remove_button_pressed_callback(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_play_enabled(&self) -> Result<bool, String> {
        Ok(ffi::swift_is_remote_command_enabled("play"))
    }

    fn set_is_play_enabled(&mut self, enabled: bool) -> Result<(), String> {
        ffi::swift_set_remote_command_enabled("play", enabled);

        Ok(())
    }

    fn is_pause_enabled(&self) -> Result<bool, String> {
        Ok(ffi::swift_is_remote_command_enabled("pause"))
    }

    fn set_is_pause_enabled(&mut self, enabled: bool) -> Result<(), String> {
        ffi::swift_set_remote_command_enabled("pause", enabled);

        Ok(())
    }

    fn is_previous_enabled(&self) -> Result<bool, String> {
        Ok(ffi::swift_is_remote_command_enabled("previous"))
    }

    fn set_is_previous_enabled(&mut self, enabled: bool) -> Result<(), String> {
        ffi::swift_set_remote_command_enabled("previous", enabled);

        Ok(())
    }

    fn is_next_enabled(&self) -> Result<bool, String> {
        Ok(ffi::swift_is_remote_command_enabled("next"))
    }

    fn set_is_next_enabled(&mut self, enabled: bool) -> Result<(), String> {
        ffi::swift_set_remote_command_enabled("next", enabled);

        Ok(())
    }

    fn is_enabled(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn set_is_enabled(&mut self, _enabled: bool) -> Result<(), String> {
        Ok(())
    }

    fn set_timeline(&mut self, start_time: u64, end_time: u64, position: u64, min_seek_time: u64, max_seek_time: u64) -> Result<(), String> {
        Ok(())
    }

    fn set_playback_position_change_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        Ok(-1);
    }

    fn remove_playback_position_change_callback(&mut self) -> Result<(), String> {
        Ok(())
    }
}
