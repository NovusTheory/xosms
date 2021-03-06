use neon::event::Channel;
use neon::prelude::*;

pub trait MediaServiceTrait: Send {
    // region Control
    fn is_enabled(&self) -> Result<bool, String>;

    fn set_is_enabled(&self, enabled: bool) -> Result<(), String>;
    // endregion Control

    // region Buttons
    fn is_play_enabled(&self) -> Result<bool, String>;

    fn set_is_play_enabled(&self, enabled: bool) -> Result<(), String>;

    fn is_pause_enabled(&self) -> Result<bool, String>;

    fn set_is_pause_enabled(&self, enabled: bool) -> Result<(), String>;

    fn is_previous_enabled(&self) -> Result<bool, String>;

    fn set_is_previous_enabled(&self, enabled: bool) -> Result<(), String>;

    fn is_next_enabled(&self) -> Result<bool, String>;

    fn set_is_next_enabled(&self, enabled: bool) -> Result<(), String>;
    // endregion Buttons

    // region Media Information
    fn get_media_type(&self) -> Result<i32, String>;

    fn set_media_type(&self, media_type: i32) -> Result<(), String>;

    fn get_playback_status(&self) -> Result<i32, String>;

    fn set_playback_status(&self, status: i32) -> Result<(), String>;

    fn get_artist(&self) -> Result<String, String>;

    fn set_artist(&self, artist: String) -> Result<(), String>;

    fn get_album_artist(&self) -> Result<String, String>;

    fn set_album_artist(&self, album_artist: String) -> Result<(), String>;

    fn get_album_title(&self) -> Result<String, String>;

    fn set_album_title(&self, album_title: String) -> Result<(), String>;

    fn get_title(&self) -> Result<String, String>;

    fn set_title(&self, title: String) -> Result<(), String>;

    fn get_track_id(&self) -> Result<String, String>;

    fn set_track_id(&self, track_id: String) -> Result<(), String>;

    fn set_thumbnail(&self, thumbnail_type: i32, thumbnail: String) -> Result<(), String>;
    // endregion Media Information

    // region Events
    fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String>;

    fn remove_button_pressed_callback(&mut self) -> Result<(), String>;
    // endregion Events
}
