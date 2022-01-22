mod bindings;

use block::ConcreteBlock;
use std::ffi::CStr;
// use std::convert::TryInto;

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
    TrackID,
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

    fn set_metadata(&self, key: MPMediaItemProperty, value: String)
    {
        unsafe{
            let key = match key {
                MPMediaItemProperty::Artist => MPMediaItemPropertyArtist.0,
                MPMediaItemProperty::Title => MPMediaItemPropertyTitle.0,
                MPMediaItemProperty::AlbumArtist => MPMediaItemPropertyAlbumArtist.0,
                MPMediaItemProperty::AlbumTitle => MPMediaItemPropertyAlbumTitle.0,
                MPMediaItemProperty::TrackID => MPMediaItemPropertyPersistentID.0,
            };
            let str = NSString::from(value.as_str());
            let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
            self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
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
        self.set_metadata(MPMediaItemProperty::Artist, artist);
    }

    pub fn get_album_artist(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_artist(&self, album_artist: String) {
        self.set_metadata(MPMediaItemProperty::AlbumArtist, album_artist);
    }

    pub fn get_album_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_title(&self, album_title: String) {
        self.set_metadata(MPMediaItemProperty::AlbumTitle, album_title);
    }

    pub fn get_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_title(&self, title: String) {
        self.set_metadata(MPMediaItemProperty::Title, title);
    }

    pub fn get_track_id(&self) -> String {
        return "".to_string();
    }

    pub fn set_track_id(&self, track_id: String) {
        self.set_metadata(MPMediaItemProperty::TrackID, track_id);
    }

    pub fn set_thumbnail(&self, _thumbnail_type: i32, _thumbnail: String) {
        // match thumbnail_type {
        //     2 => {
        //         let str = NSString::from(value.as_str());
        //         let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
        //         self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        //     },
        //     _ => println!("Unsupported thumbnail type: {}", thumbnail_type),
        // }
        unsafe {
            //TODO: Try 
            //https://github.com/tauri-apps/wry/blob/8f72c85d7c1135c38c18ac65870aba7e77f4f1ae/src/webview/wkwebview/mod.rs#L437
            //https://omz-software.com/pythonista/docs/ios/objc_util.html

            let str = "https://i.ytimg.com/vi/7zjPXE-clcU/hq720.jpg?sqp=-oaymwEXCNUGEOADIAQqCwjVARCqCBh4INgESFo&rs=AMzJL3mFsc3BpLft72R0kb8OIkalJddfQA";

            println!("STR");
            let url: id = msg_send![class!(NSURL), URLWithString: NSString::from(str)];
            println!("DEBUG URL");
            let debug: bindings::NSString = msg_send!(url, absoluteString);
            println!("URL: {:?}", CStr::from_ptr(debug.cString()));

            println!("IMG1");
            // let img: id = msg_send!(bindings::NSImage::alloc(), initWithContentsOfURL: url);
            let img: NSImage = msg_send!(bindings::NSImage::alloc(), initWithContentsOfURL: url);

            println!("DEBUG IMAGE");
            let size: bindings::NSSize = msg_send!(img, size);
            println!("Image size: {:?}", size);
            
            println!("ARTWORK ConcreteBlock");
            let x = ConcreteBlock::new(move |_: CGSize| -> NSImage {
                println!("X executed");
                return img.clone();
            });
            println!("ARTWORK NEW");
            // let id: id = msg_send!(class!(MPMediaItemArtwork), new);
            let artwork: MPMediaItemArtwork = msg_send!(bindings::MPMediaItemArtwork::alloc(), initWithBoundsSize : size requestHandler : &*x);
            println!("SETTING PLAYBACK INFO");
            // let artwork: MPMediaItemArtwork = msg_send!(class!(MPMediaItemArtwork), new);
            // artwork.initWithBoundsSize_requestHandler_(size, &*x);
    //         pub type _bindgen_ty_id_231239 =
    // *const ::block::Block<(MPRemoteCommandEvent,), MPRemoteCommandHandlerStatus>;

    // pub type _bindgen_ty_id_231220 = *const ::block::Block<(CGSize,), NSImage>;
            // let command_handler = command_handler.copy();
            // remote_command_center.playCommand().addTargetWithHandler_(&*command_handler);

            // let artwork: MPMediaItemArtwork = <MPMediaItemArtwork as bindings::IMPMediaItemArtwork>::new();
            // artwork.0.
            
            println!("DICT");
            let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : artwork forKey : MPMediaItemPropertyArtwork.0);
            self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
            // let _result: objc::runtime::Object = msg_send!(img, initWithContentsOfURL: );
            // let str = ;
            // let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
            // self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        }
    }
    // endregion Media Information

    // region Events
    fn send_button_pressed(callback: Arc<Root<JsFunction>>, channel: Channel, button: &'static str) {
        let channel = channel.clone();
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let js_button = cx.string(button);
            let _result = callback.call(&mut cx, this, vec![js_button]);
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


// NSString helpers
// const UTF8_ENCODING: usize = 4;

// struct NSString(Id<Object>);

// impl bindings::NSString {
//   fn new(s: &str) -> Self {
//     // Safety: objc runtime calls are unsafe
//     NSString(unsafe {
//       let nsstring: id = msg_send![class!(NSString), alloc];
//       bindings::id::from_ptr(
//         msg_send![nsstring, initWithBytes:s.as_ptr() length:s.len() encoding:UTF8_ENCODING],
//       )
//     })
//   }

//   fn to_str(&self) -> &str {
//     unsafe {
//       let bytes: *const c_char = msg_send![self.0, UTF8String];
//       let len = msg_send![self.0, lengthOfBytesUsingEncoding: UTF8_ENCODING];
//       let bytes = slice::from_raw_parts(bytes as *const u8, len);
//       str::from_utf8_unchecked(bytes)
//     }
//   }
// }
