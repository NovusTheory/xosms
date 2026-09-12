mod bindings;

use std::sync::{
  Arc,
  atomic::{AtomicUsize, Ordering},
};

use tokio::{
  sync::{RwLock, mpsc::Sender},
  time::Instant,
};
use windows_core::{HSTRING, Interface};
use windows_time::TimeSpan;

use crate::{
  ButtonPressedType, LoopType, MediaPlayerState, MediaPlayerThumbnailType, PlaybackStatus,
  backends::{
    PlatformBackend, PlatformThumbnailBackend,
    common::{MediaPlayerTimelineState, PlatformBackendEvent},
    error::{PlatformBackendError, WindowsPlatformError, XosmsError},
    windows::bindings::{
      MediaPlaybackAutoRepeatMode, MediaPlaybackStatus, MediaPlaybackType,
      MediaPlayer as WindowsMediaPlayer, RandomAccessStreamReference, StorageFile,
      SystemMediaTransportControls, SystemMediaTransportControlsButton,
      SystemMediaTransportControlsTimelineProperties, Uri,
    },
  },
};

#[derive(Default)]
pub struct PlatformData {
  media_player: Option<WindowsMediaPlayer>,
  button_pressed_revoker: Option<i64>,
  playback_position_change_requested_revoker: Option<i64>,
  shuffle_enabled_change_requested_revoker: Option<i64>,
  auto_repeat_mode_change_requested_revoker: Option<i64>,
}

impl PlatformBackend {
  pub fn new(
    service_name: String,
    identity: String,
    callback_tx: Sender<PlatformBackendEvent>,
    state: &MediaPlayerState,
  ) -> Self {
    Self {
      service_name,
      identity,
      state: Arc::new(RwLock::new(state.clone())),
      timeline_state: Arc::new(MediaPlayerTimelineState::default()),
      last_timeline_state_update: Instant::now(),
      callback_tx: Arc::new(callback_tx),

      platform_data: PlatformData::default(),
    }
  }

  pub async fn activate(&mut self) -> Result<(), XosmsError> {
    let media_player = WindowsMediaPlayer::new()?;
    let smtc = media_player.SystemMediaTransportControls()?;

    let callback_tx = self.callback_tx.clone();
    let button_pressed_revoker = smtc.ButtonPressed(move |_sender, args| {
      if let Some(args) = &*args {
        if let Ok(button) = args.Button() {
          match button {
            SystemMediaTransportControlsButton::ChannelDown => {}
            SystemMediaTransportControlsButton::ChannelUp => {}
            SystemMediaTransportControlsButton::FastForward => {}
            SystemMediaTransportControlsButton::Next => {
              let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed {
                button: ButtonPressedType::Next,
              });
            }
            SystemMediaTransportControlsButton::Pause => {
              let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed {
                button: ButtonPressedType::Pause,
              });
            }
            SystemMediaTransportControlsButton::Play => {
              let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed {
                button: ButtonPressedType::Play,
              });
            }
            SystemMediaTransportControlsButton::Previous => {
              let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed {
                button: ButtonPressedType::Previous,
              });
            }
            SystemMediaTransportControlsButton::Record => {}
            SystemMediaTransportControlsButton::Rewind => {}
            SystemMediaTransportControlsButton::Stop => {
              let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed {
                button: ButtonPressedType::Stop,
              });
            }
            _ => {} // Windows can send any i32 as the button but those aren't defined
          }
        }
      }
    })?;
    self.platform_data.button_pressed_revoker = Some(button_pressed_revoker.into_token());

    let callback_tx = self.callback_tx.clone();
    let playback_position_change_requested_revoker =
      smtc.PlaybackPositionChangeRequested(move |_sender, args| {
        if let Some(args) = &*args {
          if let Ok(requested_playback_position) = args.RequestedPlaybackPosition() {
            let position = requested_playback_position.duration as f64 / 10_000_000.0;
            let _ = callback_tx.try_send(PlatformBackendEvent::PositionChanged { position });
          }
        }
      })?;
    self
      .platform_data
      .playback_position_change_requested_revoker =
      Some(playback_position_change_requested_revoker.into_token());

    let callback_tx = self.callback_tx.clone();
    let shuffle_enabled_change_requested_revoker =
      smtc.ShuffleEnabledChangeRequested(move |_sender, args| {
        if let Some(args) = &*args {
          if let Ok(requested_shuffle_enabled) = args.RequestedShuffleEnabled() {
            let _ = callback_tx.try_send(PlatformBackendEvent::ShuffleChanged {
              shuffle: requested_shuffle_enabled,
            });
          }
        }
      })?;
    self.platform_data.shuffle_enabled_change_requested_revoker =
      Some(shuffle_enabled_change_requested_revoker.into_token());

    let callback_tx = self.callback_tx.clone();
    let auto_repeat_mode_change_requested_revoker =
      smtc.AutoRepeatModeChangeRequested(move |_sender, args| {
        if let Some(args) = &*args {
          if let Ok(requested_auto_repeat_mode) = args.RequestedAutoRepeatMode() {
            let _ = callback_tx.try_send(PlatformBackendEvent::LoopChanged {
              loop_type: match requested_auto_repeat_mode {
                MediaPlaybackAutoRepeatMode::None => LoopType::None,
                MediaPlaybackAutoRepeatMode::Track => LoopType::Track,
                MediaPlaybackAutoRepeatMode::List => LoopType::Playlist,
                _ => LoopType::None,
              },
            });
          }
        }
      })?;
    self.platform_data.auto_repeat_mode_change_requested_revoker =
      Some(auto_repeat_mode_change_requested_revoker.into_token());

    let du = smtc.DisplayUpdater()?;
    du.SetAppMediaId(&HSTRING::from(self.service_name.clone()))?;
    du.SetType(MediaPlaybackType::Music)?;

    smtc.SetIsEnabled(true)?;

    self.platform_data.media_player = Some(media_player);

    Ok(())
  }

  pub async fn deactivate(&mut self) -> Result<(), XosmsError> {
    if let Some(media_player) = self.platform_data.media_player.take() {
      let smtc = media_player.SystemMediaTransportControls()?;
      self.unregister_windows_handlers(smtc)?;
    }

    Ok(())
  }

  pub async fn update(&self, state: MediaPlayerState) -> Result<(), XosmsError> {
    if let Some(media_player) = self.platform_data.media_player.as_ref() {
      let smtc = media_player.SystemMediaTransportControls()?;
      smtc.SetIsPlayEnabled(state.play_button_enabled)?;
      smtc.SetIsPauseEnabled(state.pause_button_enabled)?;
      smtc.SetIsStopEnabled(state.stop_button_enabled)?;
      smtc.SetIsPreviousEnabled(state.previous_button_enabled)?;
      smtc.SetIsNextEnabled(state.next_button_enabled)?;
      smtc.SetShuffleEnabled(state.shuffle)?;
      smtc.SetAutoRepeatMode(match state.r#loop {
        LoopType::None => MediaPlaybackAutoRepeatMode::None,
        LoopType::Track => MediaPlaybackAutoRepeatMode::Track,
        LoopType::Playlist => MediaPlaybackAutoRepeatMode::List,
      })?;
      smtc.SetPlaybackRate(state.playback_rate)?;
      smtc.SetPlaybackStatus(match state.playback_status {
        PlaybackStatus::Playing => MediaPlaybackStatus::Playing,
        PlaybackStatus::Paused => MediaPlaybackStatus::Paused,
        PlaybackStatus::Stopped => MediaPlaybackStatus::Stopped,
      })?;

      let du = smtc.DisplayUpdater()?;
      if let Some(thumbnail) = state.thumbnail.as_ref() {
        du.SetThumbnail(&thumbnail.platform_data.thumbnail)?;
      }

      let mp = du.MusicProperties()?;
      mp.SetTitle(&HSTRING::from(state.title))?;
      mp.SetArtist(&HSTRING::from(state.artist.join(", ")))?;
      mp.SetAlbumTitle(&HSTRING::from(state.album_title))?;

      du.Update()?;
    }

    Ok(())
  }

  pub async fn set_timeline(&mut self, duration: f64, position: f64) -> Result<(), XosmsError> {
    if let Some(media_player) = self.platform_data.media_player.as_ref() {
      let smtc = media_player.SystemMediaTransportControls()?;
      let timeline_props = SystemMediaTransportControlsTimelineProperties::new()?;
      timeline_props.SetStartTime(TimeSpan::ZERO)?;
      timeline_props.SetEndTime(TimeSpan::from_micros(
        (duration * 1_000_000.0).round() as i64
      ))?;
      timeline_props.SetPosition(TimeSpan::from_micros(
        (position * 1_000_000.0).round() as i64
      ))?;
      timeline_props.SetMinSeekTime(TimeSpan::ZERO)?;
      timeline_props.SetMaxSeekTime(TimeSpan::from_micros(
        (duration * 1_000_000.0).round() as i64
      ))?;
      smtc.UpdateTimelineProperties(&timeline_props)?;
    }

    Ok(())
  }

  fn unregister_windows_handlers(
    &mut self,
    smtc: SystemMediaTransportControls,
  ) -> Result<(), XosmsError> {
    // windows-bindgen doesn't generate a `Remove` method for the events so we need to manually call it from the vtable
    if let Some(token) = self.platform_data.button_pressed_revoker.take() {
      unsafe {
        let result =
          (windows_core::Interface::vtable(&smtc).RemoveButtonPressed)(smtc.as_raw(), token);
        if result.is_err() {
          return Err(XosmsError::PlatformBackend(PlatformBackendError::Windows(
            WindowsPlatformError::HResult(result),
          )));
        }
      }
    }
    if let Some(token) = self
      .platform_data
      .playback_position_change_requested_revoker
      .take()
    {
      let this = &windows_core::Interface::cast::<bindings::ISystemMediaTransportControls2>(&smtc)?;
      unsafe {
        let result = (windows_core::Interface::vtable(this).RemovePlaybackPositionChangeRequested)(
          this.as_raw(),
          token,
        );
        if result.is_err() {
          return Err(XosmsError::PlatformBackend(PlatformBackendError::Windows(
            WindowsPlatformError::HResult(result),
          )));
        }
      }
    }
    if let Some(token) = self
      .platform_data
      .shuffle_enabled_change_requested_revoker
      .take()
    {
      let this = &windows_core::Interface::cast::<bindings::ISystemMediaTransportControls2>(&smtc)?;
      unsafe {
        let result = (windows_core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested)(
          this.as_raw(),
          token,
        );
        if result.is_err() {
          return Err(XosmsError::PlatformBackend(PlatformBackendError::Windows(
            WindowsPlatformError::HResult(result),
          )));
        }
      }
    }
    if let Some(token) = self
      .platform_data
      .auto_repeat_mode_change_requested_revoker
      .take()
    {
      let this = &windows_core::Interface::cast::<bindings::ISystemMediaTransportControls2>(&smtc)?;
      unsafe {
        let result = (windows_core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested)(
          this.as_raw(),
          token,
        );
        if result.is_err() {
          return Err(XosmsError::PlatformBackend(PlatformBackendError::Windows(
            WindowsPlatformError::HResult(result),
          )));
        }
      }
    }

    Ok(())
  }
}

impl Drop for PlatformBackend {
  fn drop(&mut self) {
    if let Some(media_player) = self.platform_data.media_player.take() {
      if let Ok(smtc) = media_player.SystemMediaTransportControls() {
        let result = self.unregister_windows_handlers(smtc);
        if result.is_ok() {
          return;
        }
      }

      eprintln!(
        "[xosms warning] Native memory leak: Failed to free event handlers for PlatformBackend {:p}",
        self
      );
    }
  }
}

pub struct PlatformThumbnailData {
  thumbnail: RandomAccessStreamReference,
}

static PLATFORM_THUMBNAIL_NEXT_ID: AtomicUsize = AtomicUsize::new(1);
impl PlatformThumbnailBackend {
  pub async fn new(
    r#type: MediaPlayerThumbnailType,
    thumbnail: String,
  ) -> Result<Self, XosmsError> {
    let stream_ref = match r#type {
      MediaPlayerThumbnailType::File => {
        let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(thumbnail))?.await?;
        RandomAccessStreamReference::CreateFromFile(&file)?
      }
      MediaPlayerThumbnailType::Uri => {
        let uri = Uri::CreateUri(&HSTRING::from(thumbnail))?;
        RandomAccessStreamReference::CreateFromUri(&uri)?
      }
    };

    Ok(Self {
      id: PLATFORM_THUMBNAIL_NEXT_ID.fetch_add(1, Ordering::SeqCst),
      r#type,
      platform_data: PlatformThumbnailData {
        thumbnail: stream_ref,
      },
    })
  }
}
