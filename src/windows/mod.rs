use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use napi::{
  bindgen_prelude::ObjectFinalize,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  Env, JsFunction, NapiRaw,
};
use windows::{
  core::HSTRING,
  Foundation::{EventRegistrationToken, TimeSpan, TypedEventHandler, Uri},
  Media::{
    MediaPlaybackStatus, MediaPlaybackType, Playback::MediaPlayer as WindowsMediaPlayer,
    PlaybackPositionChangeRequestedEventArgs, SystemMediaTransportControls,
    SystemMediaTransportControlsButton, SystemMediaTransportControlsButtonPressedEventArgs,
    SystemMediaTransportControlsTimelineProperties,
  },
  Storage::{StorageFile, Streams::RandomAccessStreamReference},
};

#[napi]
#[derive(Debug)]
pub enum MediaPlayerThumbnailType {
  Unknown = -1,
  File = 1,
  Uri = 2,
}

#[napi]
#[derive(Debug)]
pub enum MediaPlayerMediaType {
  Unknown = -1,
  Music = 1,
}

#[napi]
#[derive(Debug)]
pub enum MediaPlayerPlaybackStatus {
  Unknown = -1,
  Playing = 1,
  Paused = 2,
  Stopped = 3,
}

#[napi]
struct MediaPlayerThumbnail {
  thumbnail_type: MediaPlayerThumbnailType,
  stream_ref: RandomAccessStreamReference,
}

#[napi]
impl MediaPlayerThumbnail {
  #[napi(factory)]
  #[allow(dead_code)]
  pub async fn create(
    thumbnail_type: MediaPlayerThumbnailType,
    thumbnail: String,
  ) -> napi::Result<Self> {
    match thumbnail_type {
      MediaPlayerThumbnailType::File => {
        let file_async_operation = StorageFile::GetFileFromPathAsync(&HSTRING::from(thumbnail));
        if let Ok(file_async_operation) = file_async_operation {
          let file_async_operation_result = file_async_operation.await;
          if let Ok(file) = file_async_operation_result {
            let stream_ref_result = RandomAccessStreamReference::CreateFromFile(&file);
            if let Ok(stream_ref) = stream_ref_result {
              return Ok(Self {
                thumbnail_type,
                stream_ref,
              });
            } else {
              return Err(napi::Error::from_reason(
                stream_ref_result.unwrap_err().message(),
              ));
            }
          } else {
            return Err(napi::Error::from_reason(
              file_async_operation_result.unwrap_err().message(),
            ));
          }
        } else {
          return Err(napi::Error::from_reason(
            file_async_operation.unwrap_err().message(),
          ));
        }
      }
      MediaPlayerThumbnailType::Uri => {
        let uri_result = Uri::CreateUri(&HSTRING::from(thumbnail));
        if let Ok(uri) = uri_result {
          let stream_ref_result = RandomAccessStreamReference::CreateFromUri(&uri);
          if let Ok(stream_ref) = stream_ref_result {
            return Ok(Self {
              thumbnail_type,
              stream_ref,
            });
          } else {
            return Err(napi::Error::from_reason(
              stream_ref_result.unwrap_err().message(),
            ));
          }
        } else {
          return Err(napi::Error::from_reason(uri_result.unwrap_err().message()));
        }
      }
      _ => Err(napi::Error::from_reason(format!(
        "{:?} is not a valid MediaPlayerThumbnailType to create",
        thumbnail_type
      ))),
    }
  }

  #[napi(getter, js_name = "type")]
  #[allow(dead_code)]
  pub fn thumbnail_type(&self) -> MediaPlayerThumbnailType {
    self.thumbnail_type
  }
}

#[napi(custom_finalize)]
struct MediaPlayer {
  player: WindowsMediaPlayer,
  smtc_button_pressed_registration: EventRegistrationToken,
  smtc_playback_position_changed_registration: EventRegistrationToken,
  button_pressed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>>>,
  playback_position_changed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>>,
}

#[napi]
impl MediaPlayer {
  #[napi(constructor)]
  #[allow(dead_code)]
  pub fn new(service_name: String, _identity: String) -> napi::Result<Self> {
    let button_pressed_listeners: Arc<
      DashMap<usize, ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>>,
    > = Arc::new(DashMap::new());
    let playback_position_changed_listeners: Arc<
      DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>,
    > = Arc::new(DashMap::new());
    let player_result = WindowsMediaPlayer::new();
    return match player_result {
      Ok(player) => {
        let smtc_result = player.SystemMediaTransportControls();
        if let Ok(smtc) = smtc_result {
          let smtc_button_pressed_listeners = button_pressed_listeners.clone();
          let handler = TypedEventHandler::<
            SystemMediaTransportControls,
            SystemMediaTransportControlsButtonPressedEventArgs,
          >::new(move |_sender, args| {
            if let Some(args) = args {
              let smtc_button_result = args.Button();
              if let Ok(smtc_button) = smtc_button_result {
                match smtc_button {
                  SystemMediaTransportControlsButton::Play => {
                    for listener in smtc_button_pressed_listeners.iter() {
                      listener.call(
                        Ok("play".to_string()),
                        ThreadsafeFunctionCallMode::NonBlocking,
                      );
                    }
                  }
                  SystemMediaTransportControlsButton::Pause => {
                    for listener in smtc_button_pressed_listeners.iter() {
                      listener.call(
                        Ok("pause".to_string()),
                        ThreadsafeFunctionCallMode::NonBlocking,
                      );
                    }
                  }
                  SystemMediaTransportControlsButton::Stop => {
                    for listener in smtc_button_pressed_listeners.iter() {
                      listener.call(
                        Ok("stop".to_string()),
                        ThreadsafeFunctionCallMode::NonBlocking,
                      );
                    }
                  }
                  SystemMediaTransportControlsButton::Next => {
                    for listener in smtc_button_pressed_listeners.iter() {
                      listener.call(
                        Ok("next".to_string()),
                        ThreadsafeFunctionCallMode::NonBlocking,
                      );
                    }
                  }
                  SystemMediaTransportControlsButton::Previous => {
                    for listener in smtc_button_pressed_listeners.iter() {
                      listener.call(
                        Ok("previous".to_string()),
                        ThreadsafeFunctionCallMode::NonBlocking,
                      );
                    }
                  }
                  _ => {}
                };
              }
            }

            Ok(())
          });
          let button_pressed_registration_result = smtc.ButtonPressed(&handler);
          if let Ok(button_pressed_registration) = button_pressed_registration_result {
            let smtc_playback_position_changed_listeners =
              playback_position_changed_listeners.clone();
            let handler = TypedEventHandler::<
              SystemMediaTransportControls,
              PlaybackPositionChangeRequestedEventArgs,
            >::new(move |_sender, args| {
              if let Some(args) = args {
                let smtc_requested_playback_position_result = args.RequestedPlaybackPosition();
                if let Ok(requested_playback_position) = smtc_requested_playback_position_result {
                  for listener in smtc_playback_position_changed_listeners.iter() {
                    listener.call(
                      Ok(Duration::from(requested_playback_position).as_secs_f64()),
                      ThreadsafeFunctionCallMode::NonBlocking,
                    );
                  }
                }
              }

              Ok(())
            });

            let playback_position_changed_registration_result =
              smtc.PlaybackPositionChangeRequested(&handler);
            if let Ok(playback_position_changed_registration) =
              playback_position_changed_registration_result
            {
              let du_result = smtc.DisplayUpdater();
              if let Ok(du) = du_result {
                let set_app_media_id_result = du.SetAppMediaId(&HSTRING::from(service_name));
                if set_app_media_id_result.is_ok() {
                  return Ok(Self {
                    player,
                    button_pressed_listeners,
                    playback_position_changed_listeners,
                    smtc_button_pressed_registration: button_pressed_registration,
                    smtc_playback_position_changed_registration:
                      playback_position_changed_registration,
                  });
                } else {
                  return Err(napi::Error::from_reason(
                    set_app_media_id_result.unwrap_err().message(),
                  ));
                }
              } else {
                return Err(napi::Error::from_reason(du_result.unwrap_err().message()));
              }
            } else {
              return Err(napi::Error::from_reason(
                playback_position_changed_registration_result
                  .unwrap_err()
                  .message(),
              ));
            }
          } else {
            return Err(napi::Error::from_reason(
              button_pressed_registration_result.unwrap_err().message(),
            ));
          }
        }

        Err(napi::Error::from_reason(smtc_result.unwrap_err().message()))
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    };
  }

  /// Activates the MediaPlayer allowing the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn activate(&self) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_enabled_result = smtc.SetIsEnabled(true);
        match set_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => return Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Deactivates the MediaPlayer denying the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn deactivate(&self) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_enabled_result = smtc.SetIsEnabled(false);
        match set_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => return Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Adds an event listener to the MediaPlayer
  #[napi]
  #[allow(dead_code)]
  pub fn add_event_listener(
    &mut self,
    env: Env,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] event_name: String,
    callback: JsFunction,
  ) -> napi::Result<()> {
    let callback_ptr = unsafe { callback.raw() as usize };

    match event_name.as_str() {
      "buttonpressed" => {
        if !self.button_pressed_listeners.contains_key(&callback_ptr) {
          let mut threadsafe_callback = callback.create_threadsafe_function(0, |ctx| {
            ctx.env.create_string_from_std(ctx.value).map(|v| vec![v])
          })?;
          let _ = threadsafe_callback.unref(&env)?;
          self
            .button_pressed_listeners
            .insert(callback_ptr, threadsafe_callback);
        }
      }
      "positionchanged" => {
        if !self
          .playback_position_changed_listeners
          .contains_key(&callback_ptr)
        {
          let mut threadsafe_callback = callback.create_threadsafe_function(0, |ctx| {
            ctx.env.create_double(ctx.value).map(|v| vec![v])
          })?;
          let _ = threadsafe_callback.unref(&env)?;
          self
            .playback_position_changed_listeners
            .insert(callback_ptr, threadsafe_callback);
        }
      }
      _ => {}
    };

    Ok(())
  }

  /// Removes an event listener from the MediaPlayer
  #[napi]
  #[allow(dead_code)]
  pub fn remove_event_listener(
    &mut self,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] event_name: String,
    callback: JsFunction,
  ) -> napi::Result<()> {
    let callback_ptr = unsafe { callback.raw() as usize };

    match event_name.as_str() {
      "buttonpressed" => {
        if self.button_pressed_listeners.contains_key(&callback_ptr) {
          self.button_pressed_listeners.remove(&callback_ptr);
        }
      }
      "positionchanged" => {
        if self
          .playback_position_changed_listeners
          .contains_key(&callback_ptr)
        {
          self
            .playback_position_changed_listeners
            .remove(&callback_ptr);
        }
      }
      _ => {}
    };

    Ok(())
  }

  /// Adds an event listener to the MediaPlayer
  ///
  /// Alias for addEventListener
  #[napi]
  #[allow(dead_code)]
  pub fn on(
    &mut self,
    env: Env,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] event_name: String,
    callback: JsFunction,
  ) -> napi::Result<()> {
    self.add_event_listener(env, event_name, callback)
  }

  /// Removes an event listener from the MediaPlayer
  ///
  /// Alias for removeEventListener
  #[napi]
  #[allow(dead_code)]
  pub fn off(
    &mut self,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] event_name: String,
    callback: JsFunction,
  ) -> napi::Result<()> {
    self.remove_event_listener(event_name, callback)
  }

  /// Instructs the media service to update its media information being displayed
  #[napi]
  #[allow(dead_code)]
  pub fn update(&self) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let update_result = du.Update();
          match update_result {
            Err(error) => return Err(napi::Error::from_reason(error.message())),
            Ok(()) => Ok(()),
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the thumbnail
  #[napi]
  #[allow(dead_code)]
  pub fn set_thumbnail(&mut self, thumbnail: &MediaPlayerThumbnail) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let set_thumbnail_result = du.SetThumbnail(&thumbnail.stream_ref);
          match set_thumbnail_result {
            Err(error) => return Err(napi::Error::from_reason(error.message())),
            Ok(()) => Ok(()),
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the timeline data
  #[napi]
  #[allow(dead_code)]
  pub fn set_timeline(&mut self, duration: f64, position: f64) -> napi::Result<()> {
    if duration < 0.0 {
      return Err(napi::Error::from_reason("Duration cannot be less than 0"));
    }
    if position < 0.0 {
      return Err(napi::Error::from_reason("Position cannot be less than 0"));
    }
    if position > duration {
      return Err(napi::Error::from_reason(
        "Position cannot be greather than provided duration",
      ));
    }

    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let timeline_props = SystemMediaTransportControlsTimelineProperties::new().unwrap();
        let set_start_time_result =
          timeline_props.SetStartTime(TimeSpan::from(Duration::from_secs_f64(0.0)));
        let set_end_time_result =
          timeline_props.SetEndTime(TimeSpan::from(Duration::from_secs_f64(duration)));
        let set_position_result =
          timeline_props.SetPosition(TimeSpan::from(Duration::from_secs_f64(position)));
        let set_min_seek_time_result =
          timeline_props.SetMinSeekTime(TimeSpan::from(Duration::from_secs_f64(0.0)));
        let set_max_seek_time_result =
          timeline_props.SetMaxSeekTime(TimeSpan::from(Duration::from_secs_f64(duration)));
        match set_start_time_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => {}
        };
        match set_end_time_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => {}
        };
        match set_position_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => {}
        };
        match set_min_seek_time_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => {}
        };
        match set_max_seek_time_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => {}
        };

        let update_timeline_properties_result = smtc.UpdateTimelineProperties(&timeline_props);
        match update_timeline_properties_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the play button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_play_button_enabled(&self) -> napi::Result<bool> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_is_play_enabled_result = smtc.IsPlayEnabled();
        match get_is_play_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(is_play_enabled) => Ok(is_play_enabled),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the play button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_play_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_is_play_enabled_result = smtc.SetIsPlayEnabled(enabled);
        match set_is_play_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_pause_button_enabled(&self) -> napi::Result<bool> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_is_pause_enabled_result = smtc.IsPauseEnabled();
        match get_is_pause_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(is_pause_enabled) => Ok(is_pause_enabled),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_pause_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_is_pause_enabled_result = smtc.SetIsPauseEnabled(enabled);
        match set_is_pause_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_stop_button_enabled(&self) -> napi::Result<bool> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_is_stop_enabled_result = smtc.IsStopEnabled();
        match get_is_stop_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(is_stop_enabled) => Ok(is_stop_enabled),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_stop_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_is_stop_enabled_result = smtc.SetIsStopEnabled(enabled);
        match set_is_stop_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the previous button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_previous_button_enabled(&self) -> napi::Result<bool> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_is_previous_enabled_result = smtc.IsPreviousEnabled();
        match get_is_previous_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(is_previous_enabled) => Ok(is_previous_enabled),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the previous button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_previous_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_is_previous_enabled_result = smtc.SetIsPreviousEnabled(enabled);
        match set_is_previous_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the next button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_next_button_enabled(&self) -> napi::Result<bool> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_is_next_enabled_result = smtc.IsNextEnabled();
        match get_is_next_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(is_next_enabled) => Ok(is_next_enabled),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the next button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_next_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_is_next_enabled_result = smtc.SetIsNextEnabled(enabled);
        match set_is_next_enabled_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the next button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_rate(&self) -> napi::Result<f64> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_playback_rate_result = smtc.PlaybackRate();
        match get_playback_rate_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(playback_rate) => Ok(playback_rate),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the playback rate
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_playback_rate(&mut self, playback_rate: f64) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_playback_rate_result = smtc.SetPlaybackRate(playback_rate);
        match set_playback_rate_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the playback status
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_status(&self) -> napi::Result<MediaPlayerPlaybackStatus> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let get_playback_status_result = smtc.PlaybackStatus();
        match get_playback_status_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(playback_status) => Ok(match playback_status {
            MediaPlaybackStatus::Playing => MediaPlayerPlaybackStatus::Playing,
            MediaPlaybackStatus::Paused => MediaPlayerPlaybackStatus::Paused,
            MediaPlaybackStatus::Stopped => MediaPlayerPlaybackStatus::Stopped,
            _ => MediaPlayerPlaybackStatus::Unknown,
          }),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the playback status
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_playback_status(
    &mut self,
    playback_status: MediaPlayerPlaybackStatus,
  ) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let set_playback_status_result = smtc.SetPlaybackStatus(match playback_status {
          MediaPlayerPlaybackStatus::Playing => MediaPlaybackStatus::Playing,
          MediaPlayerPlaybackStatus::Paused => MediaPlaybackStatus::Paused,
          MediaPlayerPlaybackStatus::Stopped => MediaPlaybackStatus::Stopped,
          _ => {
            return Err(napi::Error::from_reason(format!(
              "{:?} is not a valid MediaPlayerPlaybackStatus to set",
              playback_status
            )))
          }
        });
        match set_playback_status_result {
          Err(error) => return Err(napi::Error::from_reason(error.message())),
          Ok(()) => Ok(()),
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the media type
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_media_type(&self) -> napi::Result<MediaPlayerMediaType> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let get_type_result = du.Type();
          match get_type_result {
            Err(error) => return Err(napi::Error::from_reason(error.message())),
            Ok(media_type) => Ok(match media_type {
              MediaPlaybackType::Music => MediaPlayerMediaType::Music,
              _ => MediaPlayerMediaType::Unknown,
            }),
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the media type
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_media_type(&mut self, media_type: MediaPlayerMediaType) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let set_type_result = du.SetType(match media_type {
            MediaPlayerMediaType::Music => MediaPlaybackType::Music,
            _ => {
              return Err(napi::Error::from_reason(format!(
                "{:?} is not a valid MediaPlayerMediaType to set",
                media_type
              )))
            }
          });
          match set_type_result {
            Err(error) => return Err(napi::Error::from_reason(error.message())),
            Ok(()) => Ok(()),
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the media title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_title(&self) -> napi::Result<String> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let get_title_result = mp.Title();
            match get_title_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(title) => Ok(title.to_string()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the media title
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_title(&mut self, title: String) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let set_title_result = mp.SetTitle(&HSTRING::from(title));
            match set_title_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(()) => Ok(()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the media artist
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_artist(&self) -> napi::Result<String> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let get_artist_result = mp.Artist();
            match get_artist_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(artist) => Ok(artist.to_string()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_artist(&mut self, artist: String) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let set_artist_result = mp.SetArtist(&HSTRING::from(artist));
            match set_artist_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(()) => Ok(()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Gets the media album title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_album_title(&self) -> napi::Result<String> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let get_album_title_result = mp.AlbumTitle();
            match get_album_title_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(album_title) => Ok(album_title.to_string()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_album_title(&mut self, album_title: String) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    match smtc_result {
      Ok(smtc) => {
        let du_result = smtc.DisplayUpdater();
        if let Ok(du) = du_result {
          let mp_result = du.MusicProperties();
          if let Ok(mp) = mp_result {
            let set_album_title_result = mp.SetAlbumTitle(&HSTRING::from(album_title));
            match set_album_title_result {
              Err(error) => return Err(napi::Error::from_reason(error.message())),
              Ok(()) => Ok(()),
            }
          } else {
            Err(napi::Error::from_reason(mp_result.unwrap_err().message()))
          }
        } else {
          Err(napi::Error::from_reason(du_result.unwrap_err().message()))
        }
      }
      Err(error) => Err(napi::Error::from_reason(error.message())),
    }
  }
}

impl ObjectFinalize for MediaPlayer {
  fn finalize(self, _env: napi::Env) -> napi::Result<()> {
    let smtc_result = self.player.SystemMediaTransportControls();
    if let Ok(smtc) = smtc_result {
      let remove_button_pressed_result =
        smtc.RemoveButtonPressed(self.smtc_button_pressed_registration);
      if let Err(error) = remove_button_pressed_result {
        return Err(napi::Error::from_reason(error.message()));
      }

      let remove_playback_position_changed_result = smtc
        .RemovePlaybackPositionChangeRequested(self.smtc_playback_position_changed_registration);
      if let Err(error) = remove_playback_position_changed_result {
        return Err(napi::Error::from_reason(error.message()));
      }
    } else {
      return Err(napi::Error::from_reason(smtc_result.unwrap_err().message()));
    }
    self.button_pressed_listeners.clear();
    self.playback_position_changed_listeners.clear();

    let close_result = self.player.Close();
    if let Err(error) = close_result {
      return Err(napi::Error::from_reason(error.message()));
    }

    Ok(())
  }
}
