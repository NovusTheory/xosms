mod bindings;

use block::ConcreteBlock;

// use std::any::type_name_of_val; //Debug

use bindings::*;
use neon::event::Channel;
use neon::prelude::*;
use std::sync::Arc;

use fruity::foundation::NSString;

enum MPMediaItemProperty {
    Artist,
    Title,
    AlbumArtist,
    AlbumTitle,
    TrackID
}

pub struct MediaService {
    info_center: MPNowPlayingInfoCenter,
    playing_info_dict: NSMutableDictionary,
}

unsafe impl Send for MediaService {} //TODO: Research deletion of that
impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        let playing_info_dict: NSMutableDictionary;
        let info_center: MPNowPlayingInfoCenter;

        unsafe {
            info_center = MPNowPlayingInfoCenter::defaultCenter();
            info_center.setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped);

            playing_info_dict = NSMutableDictionary(bindings::INSMutableDictionary::<id, id>::init(
                &NSMutableDictionary::alloc(),
            ));
            info_center.setNowPlayingInfo_(NSDictionary(playing_info_dict.0));
        }
        Self {
            info_center,
            playing_info_dict
        }
    }

    unsafe fn set_metadata(&self, key: MPMediaItemProperty, value: NSString) //TODO: make it work with NSObject
    {
        let key = match key {
            MPMediaItemProperty::Artist => MPMediaItemPropertyArtist.0,
            MPMediaItemProperty::Title => MPMediaItemPropertyTitle.0,
            MPMediaItemProperty::AlbumArtist => MPMediaItemPropertyAlbumArtist.0,
            MPMediaItemProperty::AlbumTitle => MPMediaItemPropertyAlbumTitle.0,
            MPMediaItemProperty::TrackID => MPMediaItemPropertyPersistentID.0,
        };
        
        let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : value forKey : key);
        self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
    }

    fn set_metadata_str(&self, key: MPMediaItemProperty, value: String)
    {
        unsafe {
            self.set_metadata(key, NSString::from(value.as_str()))
        }
    }

    // region Control
    pub fn is_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_enabled(&self, _enabled: bool) {}
    // endregion Control

    // region Buttons
    pub fn is_play_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_play_enabled(&self, _enabled: bool) {}

    pub fn is_pause_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_pause_enabled(&self, _enabled: bool) {}

    pub fn is_previous_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_previous_enabled(&self, _enabled: bool) {}

    pub fn is_next_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_next_enabled(&self, _enabled: bool) {}
    // endregion Buttons

    // region Media Information
    pub fn get_media_type(&self) -> i32 {
        return -1;
    }

    pub fn set_media_type(&self, _media_type: i32) {}

    pub fn get_playback_status(&self) -> i32 {
        return -1;
    }

    pub fn set_playback_status(&self, status: i32) {
        let state = match status{
            1 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying,
            2 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped,
            3 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying,
            4 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePaused,
            _ => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateUnknown,
        };
        
        unsafe {
            self.info_center.setPlaybackState_(state);
        }
    }

    pub fn get_artist(&self) -> String {
        println!("Get artist111");
        let artist: NSString;
        unsafe {
            let info = self.info_center.nowPlayingInfo().0;
            println!("NowPlaying info {:p}", info);
            artist = msg_send!(info, objectForKey: MPMediaItemPropertyArtist.0);
            println!("Artists {}", artist)
        }

        return artist.to_string();
    }

    pub fn set_artist(&self, artist: String) {
        self.set_metadata_str(MPMediaItemProperty::Artist, artist)
    }

    pub fn get_album_artist(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_artist(&self, album_artist: String) {
        self.set_metadata_str(MPMediaItemProperty::AlbumArtist, album_artist)
    }

    pub fn get_album_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_title(&self, album_title: String) {
        self.set_metadata_str(MPMediaItemProperty::AlbumTitle, album_title)
    }

    pub fn get_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_title(&self, title: String) {
        self.set_metadata_str(MPMediaItemProperty::Title, title)
    }

    pub fn get_track_id(&self) -> String {
        return "".to_string();
    }

    pub fn set_track_id(&self, track_id: String) {
        self.set_metadata_str(MPMediaItemProperty::TrackID, track_id)
    }

    pub fn set_thumbnail(&self, _thumbnail_type: i32, _thumbnail: String) {}
    // endregion Media Information

    // region Events
    fn send_button_pressed(callback: Arc<Root<JsFunction>>, channel: Channel, button: &'static str) {
        let channel = channel.clone();
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let js_button = cx.string(button);
            let _ = callback.call(&mut cx, this, vec![js_button]);
            Ok(())
        });
    }

    pub fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> i64 {
        unsafe {
            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            let callback = std::sync::Arc::new(callback);

            let command_handler = ConcreteBlock::new(move |e: MPRemoteCommandEvent| -> MPRemoteCommandHandlerStatus { 
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
                println!("MPRemoteCommand unknown");
                return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusCommandFailed;
            });
            let command_handler = command_handler.copy();
            remote_command_center.playCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.pauseCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.togglePlayPauseCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.stopCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.nextTrackCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.previousTrackCommand().addTargetWithHandler_(&*command_handler);
        }
        return -1;
    }

    pub fn remove_button_presed_callback(&mut self) {}
    // endregion Events
}
