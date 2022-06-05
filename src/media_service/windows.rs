use ::windows::core::HSTRING;
use ::windows::Foundation::{EventRegistrationToken, TypedEventHandler, Uri};
use ::windows::Media::Playback::MediaPlayer;
use ::windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
    SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
};
use ::windows::Storage::Streams::RandomAccessStreamReference;
use neon::event::Channel;
use neon::prelude::*;

use crate::media_service::service_trait::MediaServiceTrait;

pub struct MediaService {
    player: MediaPlayer,
    smtc: SystemMediaTransportControls,
    active_buttons: Vec<EventRegistrationToken>,
    active_channels: Vec<Channel>,
}

impl Finalize for MediaService {
    fn finalize<'a, C: Context<'a>>(self, cx: &mut C) {
        for token in self.active_buttons.iter() {
            self.smtc.RemoveButtonPressed(token);
        }

        for channel in self.active_channels.iter() {
            drop(channel);
        }
    }
}

impl MediaService {
    pub fn new(service_name: String, identity: String) -> Self {
        let player = MediaPlayer::new().expect("Failed to create MediaPlayer");
        let smtc = player
            .SystemMediaTransportControls()
            .expect("Failed to get SystemMediaTransportControls");
        player
            .CommandManager()
            .expect("Failed to get CommandManager of MediaPlayer")
            .SetIsEnabled(false)
            .expect("Failed to disable CommandManager");

        Self {
            player,
            smtc,
            active_buttons: Vec::new(),
            active_channels: Vec::new(),
        }
    }
}

impl MediaServiceTrait for MediaService {
    // region Control
    fn is_enabled(&self) -> Result<bool, String> {
        let get = self.smtc.IsEnabled();
        if get.is_err() {
            return Err("Failed to get SystemMediaTransportControls.IsEnabled".to_string());
        }

        Ok(get.unwrap())
    }

    fn set_is_enabled(&self, enabled: bool) -> Result<(), String> {
        let set = self.smtc.SetIsEnabled(enabled);
        if set.is_err() {
            return Err("Failed to set SystemMediaTransportControls.SetIsEnabled".to_string());
        }

        Ok(())
    }
    // endregion Control

    // region Buttons
    fn is_play_enabled(&self) -> Result<bool, String> {
        let get = self.smtc.IsPlayEnabled();
        if get.is_err() {
            return Err("Failed to get SystemMediaTransportControls.IsPlayEnabled".to_string());
        }

        Ok(get.unwrap())
    }

    fn set_is_play_enabled(&self, enabled: bool) -> Result<(), String> {
        let set = self.smtc.SetIsPlayEnabled(enabled);
        if set.is_err() {
            return Err("Failed to set SystemMediaTransportControls.SetIsPlayEnabled".to_string());
        }

        Ok(())
    }

    fn is_pause_enabled(&self) -> Result<bool, String> {
        let get = self.smtc.IsPauseEnabled();
        if get.is_err() {
            return Err("Failed to get SystemMediaTransportControls.IsPauseEnabled".to_string());
        }

        Ok(get.unwrap())
    }

    fn set_is_pause_enabled(&self, enabled: bool) -> Result<(), String> {
        let set = self.smtc.SetIsPauseEnabled(enabled);
        if set.is_err() {
            return Err("Failed to set SystemMediaTransportControls.SetIsPauseEnabled".to_string());
        }

        Ok(())
    }

    fn is_previous_enabled(&self) -> Result<bool, String> {
        let get = self.smtc.IsPreviousEnabled();
        if get.is_err() {
            return Err("Failed to get SystemMediaTransportControls.IsPreviousEnabled".to_string());
        }

        Ok(get.unwrap())
    }

    fn set_is_previous_enabled(&self, enabled: bool) -> Result<(), String> {
        let set = self.smtc.SetIsPreviousEnabled(enabled);
        if set.is_err() {
            return Err(
                "Failed to set SystemMediaTransportControls.SetIsPreviousEnabled".to_string(),
            );
        }

        Ok(())
    }

    fn is_next_enabled(&self) -> Result<bool, String> {
        let get = self.smtc.IsNextEnabled();
        if get.is_err() {
            return Err("Failed to get SystemMediaTransportControls.IsNextEnabled".to_string());
        }

        Ok(get.unwrap())
    }

    fn set_is_next_enabled(&self, enabled: bool) -> Result<(), String> {
        let set = self.smtc.SetIsNextEnabled(enabled);
        if set.is_err() {
            return Err("Failed to set SystemMediaTransportControls.SetIsNextEnabled".to_string());
        }

        Ok(())
    }
    // endregion Buttons

    // region Media Information
    fn get_media_type(&self) -> Result<i32, String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();

        let du_type = du.Type();
        if du_type.is_err() {
            return Err("Failed to get DisplayUpdater.Type".to_string());
        }
        let du_type = du.Type().unwrap();

        Ok(du_type.0)
    }

    fn set_media_type(&self, media_type: i32) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();

        du.SetType(MediaPlaybackType(media_type));
        du.Update();

        Ok(())
    }

    fn get_playback_status(&self) -> Result<i32, String> {
        let playback_status = self.smtc.PlaybackStatus();
        if playback_status.is_err() {
            return Err("Failed to get SystemMediaTransportControls.PlaybackStatus".to_string());
        }
        let playback_status = playback_status.unwrap();

        Ok(playback_status.0)
    }

    fn set_playback_status(&self, status: i32) -> Result<(), String> {
        let set_result = self.smtc.SetPlaybackStatus(MediaPlaybackStatus(status));
        if set_result.is_err() {
            return Err("Failed to set SystemMediaTransportControls.SetPlaybackStatus".to_string());
        }
        Ok(())
    }

    fn get_artist(&self) -> Result<String, String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        let artist = mp.Artist();
        if artist.is_err() {
            return Err("Failed to get MusicProperties.Artist".to_string());
        }
        let artist = artist.unwrap();

        Ok(artist.to_string())
    }

    fn set_artist(&self, artist: String) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        mp.SetArtist(HSTRING::from(artist));
        du.Update();

        Ok(())
    }

    fn get_album_artist(&self) -> Result<String, String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        let album_artist = mp.AlbumArtist();
        if album_artist.is_err() {
            return Err("Failed to get MusicProperties.Artist".to_string());
        }
        let album_artist = album_artist.unwrap();

        Ok(album_artist.to_string())
    }

    fn set_album_artist(&self, album_artist: String) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        mp.SetAlbumArtist(HSTRING::from(album_artist));
        du.Update();

        Ok(())
    }

    fn get_album_title(&self) -> Result<String, String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        let album_title = mp.AlbumTitle();
        if album_title.is_err() {
            return Err("Failed to get MusicProperties.Artist".to_string());
        }
        let album_title = album_title.unwrap();

        Ok(album_title.to_string())
    }

    fn set_album_title(&self, album_title: String) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        mp.SetAlbumTitle(HSTRING::from(album_title));
        du.Update();

        Ok(())
    }

    fn get_title(&self) -> Result<String, String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        let title = mp.Title();
        if title.is_err() {
            return Err("Failed to get MusicProperties.Title".to_string());
        }
        let title = title.unwrap();

        Ok(title.to_string())
    }

    fn set_title(&self, title: String) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();
        let mp = du.MusicProperties();
        if mp.is_err() {
            return Err("Failed to get DisplayUpdater.MusicProperties".to_string());
        }
        let mp = mp.unwrap();

        mp.SetTitle(HSTRING::from(title));
        du.Update();

        Ok(())
    }

    fn get_track_id(&self) -> Result<String, String> {
        return Ok("".to_string());
    }

    fn set_track_id(&self, title: String) -> Result<(), String> {
        Ok(())
    }

    fn set_thumbnail(&self, thumbnail_type: i32, thumbnail: String) -> Result<(), String> {
        let du = self.smtc.DisplayUpdater();
        if du.is_err() {
            return Err("Failed to get SystemMediaTransportControls.DisplayUpdater".to_string());
        }
        let du = du.unwrap();

        let stream: RandomAccessStreamReference = match thumbnail_type {
            2 => {
                let uri = Uri::CreateUri(HSTRING::from(thumbnail));
                if uri.is_err() {
                    return Err("Failed to create Foundation.Uri from thumbnail".to_string());
                }
                let uri = uri.unwrap();

                let stream_ref = RandomAccessStreamReference::CreateFromUri(uri);
                if stream_ref.is_err() {
                    return Err(
                        "Failed to create Streams.RandomAccessStreamReference from thumbnail"
                            .to_string(),
                    );
                }

                stream_ref.unwrap()
            }
            _ => {
                return Err(format!(
                    "Thumbnail type is not supported on this operating system: {}",
                    thumbnail_type
                ))
            }
        };
        du.SetThumbnail(stream);
        du.Update();

        Ok(())
    }
    // endregion Media Information

    // region Events
    fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        let callback_arc = std::sync::Arc::new(callback);
        let callback_eh_clone = callback_arc.clone();
        let channel_clone = channel.clone();
        let token = self
            .smtc
            .ButtonPressed(TypedEventHandler::<
                SystemMediaTransportControls,
                SystemMediaTransportControlsButtonPressedEventArgs,
            >::new(move |_sender, args| {
                let callback_js_channel_clone = callback_eh_clone.clone();
                if let Some(args) = args {
                    let smtc_button = args.Button().expect("Failed to get button from native TypedEventHandler SystemMediaTransportControlsButtonPressedEventArgs");
                    channel_clone.send(move |mut cx| {
                        let callback = callback_js_channel_clone.to_inner(&mut cx);
                        let this = cx.undefined();
                        let button = match smtc_button {
                            SystemMediaTransportControlsButton::Play => "play",
                            SystemMediaTransportControlsButton::Pause => "pause",
                            SystemMediaTransportControlsButton::Stop => "stop",
                            SystemMediaTransportControlsButton::Record => "record",
                            SystemMediaTransportControlsButton::FastForward => "fastforward",
                            SystemMediaTransportControlsButton::Rewind => "rewind",
                            SystemMediaTransportControlsButton::Next => "next",
                            SystemMediaTransportControlsButton::Previous => "previous",
                            SystemMediaTransportControlsButton::ChannelUp => "channelup",
                            SystemMediaTransportControlsButton::ChannelDown => "channeldown",
                            _ => panic!()
                        };
                        let js_button = cx.string(button);
                        callback.call(&mut cx, this, vec![js_button]);

                        Ok(())
                    });
                }
                Ok(())
            }));

        if token.is_err() {
            return Err("Failed to bind native ButtonPressed callback".to_string());
        }

        let token = token.unwrap();
        self.active_buttons.push(token);
        self.active_channels.push(channel);

        Ok(token.Value)
    }

    fn remove_button_pressed_callback(&mut self) -> Result<(), String> {
        for token in self.active_buttons.iter() {
            self.smtc.RemoveButtonPressed(token);
        }

        for channel in self.active_channels.iter() {
            drop(channel);
        }

        Ok(())
    }
    // endregion Events
}
