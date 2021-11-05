mod bindings;

use std::convert::TryInto;
use std::ffi::CStr;
use std::ops::Deref;

use bindings::*;
use neon::event::Channel;
use neon::prelude::*;

const UTF8_ENCODING: NSUInteger = 4;

pub struct MediaService {}

impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        unsafe {
            println!("Generating NowPlaying");
            let now_playing: MPNowPlayingInfoCenter = MPNowPlayingInfoCenter::defaultCenter();

            let remote: MPRemoteCommandCenter = MPRemoteCommandCenter::sharedCommandCenter();
            remote.playCommand().setEnabled_(true as i8);
            remote.pauseCommand().setEnabled_(true as i8);
            remote.togglePlayPauseCommand().setEnabled_(true as i8);
            remote
                .changePlaybackPositionCommand()
                .setEnabled_(false as i8);
            remote.changePlaybackRateCommand().setEnabled_(false as i8);
            remote.changeRepeatModeCommand().setEnabled_(false as i8);
            remote.changeShuffleModeCommand().setEnabled_(false as i8);
            remote.nextTrackCommand().setEnabled_(true as i8);
            remote.previousTrackCommand().setEnabled_(true as i8);

            let dictionary = NSMutableDictionary(bindings::INSMutableDictionary::<id, id>::init(
                &NSMutableDictionary::alloc(),
            ));

            let song_title_str = "Title";
            let song_title = NSString::alloc().initWithBytes_length_encoding_(
                song_title_str.as_ptr() as *mut std::ffi::c_void,
                song_title_str.len().try_into().unwrap(),
                UTF8_ENCODING,
            );

            let song_artist_str = "Artist";
            let song_artist = NSString::alloc().initWithBytes_length_encoding_(
                song_artist_str.as_ptr() as *mut std::ffi::c_void,
                song_artist_str.len().try_into().unwrap(),
                UTF8_ENCODING,
            );

            let song_album_title_str = "Album Title";
            let song_album_title = NSString::alloc().initWithBytes_length_encoding_(
                song_album_title_str.as_ptr() as *mut std::ffi::c_void,
                song_album_title_str.len().try_into().unwrap(),
                UTF8_ENCODING,
            );

            let _result: objc::runtime::Object = msg_send!(dictionary.0 , setObject : song_title forKey : MPMediaItemPropertyTitle.0);
            let _result: objc::runtime::Object = msg_send!(dictionary.0 , setObject : song_artist forKey : MPMediaItemPropertyArtist.0);
            let _result: objc::runtime::Object = msg_send!(dictionary.0 , setObject : song_album_title forKey : MPMediaItemPropertyAlbumTitle.0);

            /*dictionary.setObject_forKey_(song_title, MPMediaItemPropertyTitle.0 as *mut u64);
            dictionary.setObject_forKey_(song_artist, MPMediaItemPropertyArtist.0 as *mut u64);
            dictionary.setObject_forKey_(
                song_album_title,
                MPMediaItemPropertyAlbumTitle.0 as *mut u64,
            );*/

            now_playing
                .setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying);
            now_playing.setNowPlayingInfo_(NSDictionary(dictionary.0));
            println!("Generated NowPlaying {:p}", now_playing.0);

            let info = now_playing.nowPlayingInfo();
            println!("NowPlaying info {:p}", info.0);
            let _title: NSString = msg_send!(info.0, objectForKey: MPMediaItemPropertyTitle.0);
            let _artist: NSString = msg_send!(info.0, objectForKey: MPMediaItemPropertyArtist.0);
            let _album_title: NSString =
                msg_send!(info.0, objectForKey: MPMediaItemPropertyAlbumTitle.0);
            println!("Title {:?}", CStr::from_ptr(_title.cString()));
            println!("Artist {:?}", CStr::from_ptr(_artist.cString()));
            println!("Album Title {:?}", CStr::from_ptr(_album_title.cString()));

            //now_playing.dealloc();
        }
        Self {}
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
        return "".to_string();
    }

    pub fn set_artist(&self, _artist: String) {}

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
