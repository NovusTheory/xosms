mod bindings;

use block::ConcreteBlock;

use bindings::*;
use neon::event::Channel;
use neon::prelude::*;
use std::sync::Arc;

use crate::media_service::service_trait::MediaServiceTrait;

use fruity::foundation::NSString;

enum MPMediaItemProperty {
    Artist,
    Title,
    AlbumArtist,
    AlbumTitle,
    TrackID,
}

unsafe fn mpmedia_item_property_to_key(key: MPMediaItemProperty) -> id {
    return match key {
        MPMediaItemProperty::Artist => MPMediaItemPropertyArtist.0,
        MPMediaItemProperty::Title => MPMediaItemPropertyTitle.0,
        MPMediaItemProperty::AlbumArtist => MPMediaItemPropertyAlbumArtist.0,
        MPMediaItemProperty::AlbumTitle => MPMediaItemPropertyAlbumTitle.0,
        MPMediaItemProperty::TrackID => MPMediaItemPropertyPersistentID.0,
    };
}

pub struct MediaService {
    info_center: MPNowPlayingInfoCenter,
    playing_info_dict: NSMutableDictionary,
    remote_command_center: MPRemoteCommandCenter,
}

unsafe impl Send for MediaService {} //TODO: Research deletion of that
impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        let playing_info_dict: NSMutableDictionary;
        let info_center: MPNowPlayingInfoCenter;
        let remote_command_center: MPRemoteCommandCenter;

        unsafe {
            info_center = MPNowPlayingInfoCenter::defaultCenter();
            remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            info_center
                .setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped);

            playing_info_dict = NSMutableDictionary(
                bindings::INSMutableDictionary::<id, id>::init(&NSMutableDictionary::alloc()),
            );
            info_center.setNowPlayingInfo_(NSDictionary(playing_info_dict.0));
        }
        Self {
            info_center,
            playing_info_dict,
            remote_command_center,
        }
    }

    fn set_metadata(&self, key: MPMediaItemProperty, value: String) {
        unsafe {
            let key = mpmedia_item_property_to_key(key);
            let str = NSString::from(value.as_str());
            let _result: objc::runtime::Object =
                msg_send!(self.playing_info_dict, setObject : str forKey : key);
            self.info_center
                .setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        }
    }

    fn get_metadata(&self, key: MPMediaItemProperty) -> String {
        let value: Option<NSString>;
        unsafe {
            let key = mpmedia_item_property_to_key(key);
            value = msg_send!(self.info_center.nowPlayingInfo().0, objectForKey: key);
        }

        match value {
            Some(n) => return n.to_string(),
            None => return "".to_string(),
        }
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
        let value: Option<MPNowPlayingPlaybackState>;
        unsafe {
            value = msg_send!(self.info_center.nowPlayingInfo().0, objectForKey: "playbackState");
        }

        let return_value = match value {
            None => -1,
            Some(n) => match n {
                MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying => 1,
                MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped => 2,
                MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePaused => 4,
                _ => -1,
            },
        };

        Ok(return_value)
    }

    fn set_playback_status(&self, status: i32) -> Result<(), String> {
        let state = match status {
            1 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying,
            2 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped,
            3 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateUnknown, // There's no Changing status in MacOS so we maps this to Unknown
            4 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePaused,
            _ => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateUnknown,
        };

        unsafe {
            self.info_center.setPlaybackState_(state);
        }

        Ok(())
    }

    fn get_media_type(&self) -> Result<i32, String> {
        let value: Option<MPNowPlayingPlaybackState>;
        unsafe {
            value = msg_send!(self.info_center.nowPlayingInfo().0, objectForKey: "playbackState");
        }

        let return_value = match value {
            None => 0, // Return Unknown
            Some(n) => {
                match n {
                    MPMediaType_MPMediaTypeMusic => 1,
                    MPMediaType_MPMediaTypeAnyVideo => 2,
                    _ => 0, // Return Unknown
                }
            }
        };

        Ok(return_value)
    }

    fn set_media_type(&self, media_type: i32) -> Result<(), String> {
        let state = match media_type {
            1 => MPMediaType_MPMediaTypeMusic,
            2 => MPMediaType_MPMediaTypeAnyVideo,
            3 => MPMediaType_MPMediaTypeAny, // There's no separate type for Image in MacOS, so we maps it also to Any
            _ => MPMediaType_MPMediaTypeAny,
        };

        unsafe {
            self.info_center.setPlaybackState_(state);
        }

        Ok(())
    }

    fn get_artist(&self) -> Result<String, String> {
        Ok(self.get_metadata(MPMediaItemProperty::Artist))
    }

    fn set_artist(&self, artist: String) -> Result<(), String> {
        self.set_metadata(MPMediaItemProperty::Artist, artist);

        Ok(())
    }

    fn get_album_artist(&self) -> Result<String, String> {
        Ok(self.get_metadata(MPMediaItemProperty::AlbumArtist))
    }

    fn set_album_artist(&self, album_artist: String) -> Result<(), String> {
        self.set_metadata(MPMediaItemProperty::AlbumArtist, album_artist);

        Ok(())
    }

    fn get_album_title(&self) -> Result<String, String> {
        Ok(self.get_metadata(MPMediaItemProperty::AlbumTitle))
    }

    fn set_album_title(&self, album_title: String) -> Result<(), String> {
        self.set_metadata(MPMediaItemProperty::AlbumTitle, album_title);

        Ok(())
    }

    fn get_title(&self) -> Result<String, String> {
        Ok(self.get_metadata(MPMediaItemProperty::Title))
    }

    fn set_title(&self, title: String) -> Result<(), String> {
        self.set_metadata(MPMediaItemProperty::Title, title);

        Ok(())
    }

    fn get_track_id(&self) -> Result<String, String> {
        Ok(self.get_metadata(MPMediaItemProperty::TrackID))
    }

    fn set_track_id(&self, track_id: String) -> Result<(), String> {
        self.set_metadata(MPMediaItemProperty::TrackID, track_id);

        Ok(())
    }

    fn set_thumbnail(&self, thumbnail_type: i32, thumbnail: String) -> Result<(), String> {
        unsafe {
            let path: id =
                msg_send![class!(NSURL), URLWithString: NSString::from(thumbnail.as_str())];
            let img: NSImage;
            match thumbnail_type {
                1 => {
                    img = msg_send!(bindings::NSImage::alloc(), initWithContentsOfFile: path);
                }
                2 => {
                    img = msg_send!(bindings::NSImage::alloc(), initWithContentsOfURL: path);
                }
                _ => {
                    return Err(format!(
                        "Thumbnail type is not supported on this operating system: {}",
                        thumbnail_type
                    ));
                }
            }
            let size: bindings::NSSize = msg_send!(img, size);
            let h = ConcreteBlock::new(move |_: CGSize| -> NSImage {
                return img.clone();
            });
            let artwork: MPMediaItemArtwork = msg_send!(bindings::MPMediaItemArtwork::alloc(), initWithBoundsSize : size requestHandler : &*h);
            let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : artwork forKey : MPMediaItemPropertyArtwork.0);
            self.info_center
                .setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));

            Ok(())
        }
    }

    fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        unsafe {
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
        }

        Ok(-1)
    }

    fn remove_button_pressed_callback(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_play_enabled(&self) -> Result<bool, String> {
        unsafe { Ok(self.remote_command_center.playCommand().isEnabled() != 0) }
    }

    fn set_is_play_enabled(&self, enabled: bool) -> Result<(), String> {
        unsafe {
            self.remote_command_center
                .playCommand()
                .setEnabled_(enabled as i8);
        }

        Ok(())
    }

    fn is_pause_enabled(&self) -> Result<bool, String> {
        unsafe { Ok(self.remote_command_center.pauseCommand().isEnabled() != 0) }
    }

    fn set_is_pause_enabled(&self, enabled: bool) -> Result<(), String> {
        unsafe {
            self.remote_command_center
                .pauseCommand()
                .setEnabled_(enabled as i8);
        }

        Ok(())
    }

    fn is_previous_enabled(&self) -> Result<bool, String> {
        unsafe {
            Ok(self
                .remote_command_center
                .previousTrackCommand()
                .isEnabled()
                != 0)
        }
    }

    fn set_is_previous_enabled(&self, enabled: bool) -> Result<(), String> {
        unsafe {
            self.remote_command_center
                .previousTrackCommand()
                .setEnabled_(enabled as i8);
        }

        Ok(())
    }

    fn is_next_enabled(&self) -> Result<bool, String> {
        unsafe { Ok(self.remote_command_center.nextTrackCommand().isEnabled() != 0) }
    }

    fn set_is_next_enabled(&self, enabled: bool) -> Result<(), String> {
        unsafe {
            self.remote_command_center
                .nextTrackCommand()
                .setEnabled_(enabled as i8);
        }

        Ok(())
    }

    fn is_enabled(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn set_is_enabled(&self, _enabled: bool) -> Result<(), String> {
        Ok(())
    }
}
