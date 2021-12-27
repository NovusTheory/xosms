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

pub struct MPRemoteEventCallback {
    callback: Arc<Root<JsFunction>>,
    channel: Channel,
}

impl MPRemoteEventCallback {
    pub fn new(callback: Arc<Root<JsFunction>>, channel: Channel) -> Self {
        Self {
            callback,
            channel
        } 
    }

    unsafe fn sendEventToJs(callback: Arc<Root<JsFunction>>, channel: Channel, s: &'static str) {
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let js_button = cx.string(s);
            callback.call(&mut cx, this, vec![js_button]);
            Ok(())
        });
    }

    unsafe fn handleEvent(callback: Arc<Root<JsFunction>>, channel: Channel, e: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        let command: MPRemoteCommand = msg_send!(e, command);
        let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();

        if command.0 == remote_command_center.playCommand().0 {
            println!("Command123 playCommand");
            MPRemoteEventCallback::sendEventToJs(callback.clone(), channel.clone(), "play");
            return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
        }
        if command.0 == remote_command_center.pauseCommand().0 {
            println!("Command123 pauseCommand");
            MPRemoteEventCallback::sendEventToJs(callback.clone(), channel.clone(),"pause");
            return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
        }
        if command.0 == remote_command_center.nextTrackCommand().0 {
            println!("Command123 nextTrackCommand");
            MPRemoteEventCallback::sendEventToJs(callback.clone(), channel.clone(),"next");
            return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
        }
        if command.0 == remote_command_center.previousTrackCommand().0 {
            MPRemoteEventCallback::sendEventToJs(callback.clone(), channel.clone(),"previous");
            println!("Command123 previousTrackCommand");
            return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
        }
        println!("MPRemoteCommand unknown");
        return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusCommandFailed;
    }

    pub fn getCommandHandler(&self) -> impl Fn(MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus + '_ {
        unsafe {
            return move |e: MPRemoteCommandEvent| -> MPRemoteCommandHandlerStatus { MPRemoteEventCallback::handleEvent(self.callback.clone(), self.channel.clone(), e) };
        }
    }
}

pub struct MediaService {
    info_center: MPNowPlayingInfoCenter,
    playing_info_dict: NSMutableDictionary,
    event_callback: Option<Arc<MPRemoteEventCallback>>
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

            info_center.setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying);
            info_center.setNowPlayingInfo_(NSDictionary(playing_info_dict.0));
        }
        Self {
            info_center,
            playing_info_dict,
            event_callback: None
        }
    }

    unsafe fn set_metadata(&self, key: MPMediaItemProperty, value: NSString) //TODO: make it work with NSObject
    {
        let key_o;

        match key {
            MPMediaItemProperty::Artist => key_o = MPMediaItemPropertyArtist.0,
            MPMediaItemProperty::Title => key_o = MPMediaItemPropertyTitle.0,
            MPMediaItemProperty::AlbumArtist => key_o = MPMediaItemPropertyAlbumArtist.0,
            MPMediaItemProperty::AlbumTitle => key_o = MPMediaItemPropertyAlbumTitle.0,
            MPMediaItemProperty::TrackID => key_o = MPMediaItemPropertyPersistentID.0,
        }
        
        let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : value forKey : key_o);
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

    pub fn set_playback_status(&self, _status: i32) {

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
    pub fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> i64 {
        // self.event_callback = Some(, channel)));
        let event_callback = Arc::new(MPRemoteEventCallback::new(Arc::new(callback), channel));
        unsafe {
            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            // let command_handler;
            // if let Some(ref mut c) = self.event_callback {

            //     command_handler = ConcreteBlock::new(c.getCommandHandler());
            // }
            let command_handler = ConcreteBlock::new(event_callback.getCommandHandler());
            let command_handler = command_handler.copy();
            remote_command_center.playCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.pauseCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.nextTrackCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.previousTrackCommand().addTargetWithHandler_(&*command_handler);
        }
        return -1;
    }

    pub fn remove_button_presed_callback(&mut self) {}
    // endregion Events
}
