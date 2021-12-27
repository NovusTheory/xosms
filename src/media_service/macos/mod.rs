mod bindings;

use std::convert::TryInto;
use std::ffi::CStr;
use std::ops::Deref;
use block::ConcreteBlock;

// use std::any::type_name_of_val; //Debug

use std::os::raw::c_char;

use bindings::*;
use neon::event::Channel;
use neon::prelude::*;

use fruity::foundation::NSString;

const UTF8_ENCODING: NSUInteger = 4;

pub struct MediaService {
    nowPlayingInfoCenter: MPNowPlayingInfoCenter,
    remoteCommandCenter: MPRemoteCommandCenter,
    playingInfoDict: NSMutableDictionary
}

unsafe impl Send for MediaService {} //TODO: Research deletion of that
impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        let playingInfoDict: NSMutableDictionary;
        let nowPlayingInfoCenter: MPNowPlayingInfoCenter;
        let remoteCommandCenter: MPRemoteCommandCenter;

        unsafe {
            nowPlayingInfoCenter = MPNowPlayingInfoCenter::defaultCenter();
            remoteCommandCenter = MPRemoteCommandCenter::sharedCommandCenter();
            println!("Fruity set author 1234!");
            
            let commandHandler = ConcreteBlock::new(|e: MPRemoteCommandEvent| -> MPRemoteCommandHandlerStatus { 
                // println!("commandHelper: {}", type_name_of_val(&e));
                println!("Callback handler executed");
                return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
            });
            let commandHandler = commandHandler.copy();
            
            println!("Debug");
            remoteCommandCenter.playCommand().addTargetWithHandler_(&*commandHandler);
            remoteCommandCenter.pauseCommand().addTargetWithHandler_(&*commandHandler);

            println!("Debug 1");
            nowPlayingInfoCenter.setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped);

            playingInfoDict = NSMutableDictionary(bindings::INSMutableDictionary::<id, id>::init(
                &NSMutableDictionary::alloc(),
            ));

            println!("Debug before fruity usage");

            let _result: objc::runtime::Object = msg_send!(playingInfoDict, setObject : fruity::nsstring!("My Title") forKey : MPMediaItemPropertyTitle.0);
            let _result: objc::runtime::Object = msg_send!(playingInfoDict, setObject : fruity::nsstring!("My Artist") forKey : MPMediaItemPropertyArtist.0);
            let _result: objc::runtime::Object = msg_send!(playingInfoDict, setObject : fruity::nsstring!("My Album") forKey : MPMediaItemPropertyAlbumTitle.0);

            nowPlayingInfoCenter
                .setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying);
                nowPlayingInfoCenter.setNowPlayingInfo_(NSDictionary(playingInfoDict.0));
        }
        Self {
            nowPlayingInfoCenter,
            remoteCommandCenter,
            playingInfoDict
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

    pub fn set_playback_status(&self, _status: i32) {}

    pub fn get_artist(&self) -> String {
        println!("Get artist111");
        let artist: NSString;
        unsafe {
            let info = self.nowPlayingInfoCenter.nowPlayingInfo().0;
            println!("NowPlaying info {:p}", info);
            artist = msg_send!(info, objectForKey: MPMediaItemPropertyArtist.0);
            println!("Artists {}", artist)
        }

        return artist.to_string();
    }

    pub fn set_artist(&self, artist: String) {
        println!("Sent artist called !");
        println!("Read old: {}", self.get_artist());
        unsafe {
            let _result: objc::runtime::Object = msg_send!(self.playingInfoDict, setObject : NSString::from(artist.as_str()) forKey : MPMediaItemPropertyTitle.0);
            println!("Sent artist moddle");
            self.nowPlayingInfoCenter.setNowPlayingInfo_(NSDictionary(self.playingInfoDict.0));
        }
        // let nowPlayingInfo;
        // unsafe {
        //     nowPlayingInfo = MPNowPlayingInfoCenter::defaultCenter().nowPlayingInfo();
        // }
        // std::string artist = std::string([nowPlayingInfo.objectForKey_(MPMediaItemPropertyArtist) UTF8String]);
        // println!("Artist set old value: {}", artist);
    }

    pub fn get_album_artist(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_artist(&self, _album_artist: String) {}

    pub fn get_album_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_title(&self, _album_title: String) {}

    pub fn get_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_title(&self, _title: String) {}

    pub fn get_track_id(&self) -> String {
        return "".to_string();
    }

    pub fn set_track_id(&self, _track_id: String) {}

    pub fn set_thumbnail(&self, _thumbnail_type: i32, _thumbnail: String) {}
    // endregion Media Information

    // region Events
    pub fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> i64 {
        return -1;
    }

    pub fn remove_button_presed_callback(&mut self) {}
    // endregion Events
}
