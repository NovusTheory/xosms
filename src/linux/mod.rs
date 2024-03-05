mod dbus;

use std::{
  sync::{Arc, RwLock},
  time::{Duration, Instant},
};

use ::dbus::{
  arg::{PropMap, Variant},
  blocking::stdintf::org_freedesktop_dbus::{EmitsChangedSignal, PropertiesPropertiesChanged},
  message::SignalArgs,
  MethodErr, Path,
};
use dashmap::DashMap;
use dbus_crossroads::Crossroads;
use float_duration::FloatDuration;
use napi::{
  bindgen_prelude::ObjectFinalize,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  Env, JsFunction, NapiRaw,
};

use self::dbus::{
  mediaplayer2::{register_org_mpris_media_player2, OrgMprisMediaPlayer2},
  mediaplayer2_player::{
    register_org_mpris_media_player2_player, OrgMprisMediaPlayer2Player,
    OrgMprisMediaPlayer2PlayerSeeked,
  },
  session::DBusSession,
};

#[napi]
#[derive(Debug, PartialEq, Eq)]
pub enum MediaPlayerThumbnailType {
  Unknown = -1,
  File = 1,
  Uri = 2,
}

#[napi]
#[derive(Debug, PartialEq, Eq)]
pub enum MediaPlayerMediaType {
  Unknown = -1,
  Music = 1,
}

#[napi]
#[derive(Debug, PartialEq, Eq)]
pub enum MediaPlayerPlaybackStatus {
  Unknown = -1,
  Playing = 1,
  Paused = 2,
  Stopped = 3,
}

#[napi]
struct MediaPlayerThumbnail {
  thumbnail_type: MediaPlayerThumbnailType,
  thumbnail: String,
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
        return Ok(Self {
          thumbnail_type,
          thumbnail: format!("file://{}", thumbnail),
        });
      }
      MediaPlayerThumbnailType::Uri => {
        return Ok(Self {
          thumbnail_type,
          thumbnail,
        });
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
  service_name: String,
  button_pressed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>>>,
  playback_position_changed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>>,
  playback_position_seeked_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>>,
  player_state: Arc<RwLock<MprisPlayerState>>,
  properties_changed: PropertiesPropertiesChanged,
  active: bool,
  dbus_session: DBusSession,
}

#[napi]
impl MediaPlayer {
  #[napi(constructor)]
  #[allow(dead_code)]
  pub fn new(service_name: String, identity: String) -> napi::Result<Self> {
    let button_pressed_listeners: Arc<
      DashMap<usize, ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>>,
    > = Arc::new(DashMap::new());
    let playback_position_changed_listeners: Arc<
      DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>,
    > = Arc::new(DashMap::new());
    let playback_position_seeked_listeners: Arc<
      DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>,
    > = Arc::new(DashMap::new());
    let mpris_player_state = Arc::new(RwLock::new(MprisPlayerState {
      identity,
      can_go_next: false,
      can_go_previous: false,
      can_play: false,
      can_pause: false,
      can_seek: false,
      can_control: true,
      media_type: MediaPlayerMediaType::Unknown,
      playback_status: MediaPlayerPlaybackStatus::Unknown,
      thumbnail: "".to_string(),
      artist: "".to_string(),
      album_title: "".to_string(),
      title: "".to_string(),
      track_id: "".to_string(),
      position: 0.0,
      last_updated_position: Instant::now(),
      duration: 0.0,
      volume: 1.0,
      playback_rate: 1.0,
    }));

    Ok(Self {
      service_name,
      button_pressed_listeners,
      playback_position_changed_listeners,
      playback_position_seeked_listeners,
      player_state: mpris_player_state,
      properties_changed: PropertiesPropertiesChanged {
        interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
        changed_properties: Default::default(),
        invalidated_properties: vec![],
      },
      active: false,
      dbus_session: DBusSession::new(),
    })
  }

  /// Activates the MediaPlayer allowing the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn activate(&mut self) -> napi::Result<()> {
    if self.active {
      return Ok(());
    }

    let mut crossroads = Crossroads::new();

    let mpris_iface_token = register_org_mpris_media_player2(&mut crossroads);
    let mpris_player_iface_token = register_org_mpris_media_player2_player(&mut crossroads);

    crossroads.insert(
      "/org/mpris/MediaPlayer2",
      &[mpris_iface_token, mpris_player_iface_token],
      MprisPlayer {
        button_pressed_listeners: self.button_pressed_listeners.clone(),
        playback_position_changed_listeners: self.playback_position_changed_listeners.clone(),
        playback_position_seeked_listeners: self.playback_position_seeked_listeners.clone(),
        state: self.player_state.clone(),
      },
    );

    if !self.dbus_session.register(&self.service_name, crossroads) {
      return Err(napi::Error::from_reason(
        "Could not obtain service name on D-Bus",
      ));
    }

    self.active = true;
    Ok(())
  }

  /// Deactivates the MediaPlayer denying the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn deactivate(&mut self) -> napi::Result<()> {
    if self.active {
      self.active = false;
      self.dbus_session.unregister(&self.service_name);
    }
    Ok(())
  }

  /// Adds an event listener to the MediaPlayer
  ///
  /// 'buttonpressed' - Emitted when a media services button is pressed
  /// 'positionchanged' - Emitted when the media service requests a position change
  /// 'positionseeked' - Emitted when the media service requests a forward or backward position seek from current position
  #[napi]
  #[allow(dead_code)]
  pub fn add_event_listener(
    &mut self,
    env: Env,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged' | 'positionseeked'")]
    event_name: String,
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
      "positionseeked" => {
        if !self
          .playback_position_seeked_listeners
          .contains_key(&callback_ptr)
        {
          let mut threadsafe_callback = callback.create_threadsafe_function(0, |ctx| {
            ctx.env.create_double(ctx.value).map(|v| vec![v])
          })?;
          let _ = threadsafe_callback.unref(&env)?;
          self
            .playback_position_seeked_listeners
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
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged' | 'positionseeked'")]
    event_name: String,
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
      "positionseeked" => {
        if self
          .playback_position_seeked_listeners
          .contains_key(&callback_ptr)
        {
          self
            .playback_position_seeked_listeners
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
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged' | 'positionseeked'")]
    event_name: String,
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
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged' | 'positionseeked'")]
    event_name: String,
    callback: JsFunction,
  ) -> napi::Result<()> {
    self.remove_event_listener(event_name, callback)
  }

  /// Instructs the media service to update its media information being displayed
  #[napi]
  #[allow(dead_code)]
  pub fn update(&mut self) -> napi::Result<()> {
    self.dbus_session.emit_message(
      self
        .properties_changed
        .to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()),
    );
    self.properties_changed.changed_properties.clear();
    self.properties_changed.invalidated_properties.clear();

    Ok(())
  }

  /// Sets the thumbnail
  #[napi]
  #[allow(dead_code)]
  pub fn set_thumbnail(&mut self, thumbnail: &MediaPlayerThumbnail) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.thumbnail = thumbnail.thumbnail.to_owned();
      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  /// Sets the timeline data
  ///
  /// You MUST call this function everytime the position changes in the song. The media service will become out of sync if this is not called enough or cause seeked signals to be emitted to the media service unnecessarily.
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

    if let Ok(mut player_state) = self.player_state.write() {
      // If the position moved more than 1 second within 1 second of time then a seeked signal needs to be emitted
      if position - player_state.position > player_state.playback_rate
        && player_state.last_updated_position.elapsed().as_secs() < 1
      {
        let seeked = OrgMprisMediaPlayer2PlayerSeeked {
          position: FloatDuration::seconds(position)
            .as_microseconds()
            .max(i64::MIN as f64)
            .min(i64::MAX as f64)
            .round() as i64,
        };
        self
          .dbus_session
          .emit_message(seeked.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));
      }

      player_state.duration = duration;
      player_state.position = position;
      player_state.last_updated_position = Instant::now();

      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  /// Gets the play button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_play_button_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.can_play);
    }

    Ok(false)
  }

  /// Sets the play button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_play_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.can_play = enabled;
      drop(player_state);

      self
        .properties_changed
        .add_prop("CanPlay", EmitsChangedSignal::True, || Box::new(enabled));
    }

    Ok(())
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_pause_button_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.can_pause);
    }

    Ok(false)
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_pause_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.can_pause = enabled;
      drop(player_state);

      self
        .properties_changed
        .add_prop("CanPause", EmitsChangedSignal::True, || Box::new(enabled));
    }

    Ok(())
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_stop_button_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      // Stop button for MPRIS is tied to CanControl
      return Ok(player_state.can_control);
    }

    Ok(false)
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_stop_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    // Stop button for MPRIS is tied to CanControl
    Ok(())
  }

  /// Gets the previous button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_previous_button_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.can_go_previous);
    }

    Ok(false)
  }

  /// Sets the previous button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_previous_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.can_go_previous = enabled;
      drop(player_state);

      self
        .properties_changed
        .add_prop("CanGoPrevious", EmitsChangedSignal::True, || {
          Box::new(enabled)
        });
    }

    Ok(())
  }

  /// Gets the next button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_next_button_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.can_go_next);
    }

    Ok(false)
  }

  /// Sets the next button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_next_button_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.can_go_next = enabled;
      drop(player_state);

      self
        .properties_changed
        .add_prop("CanGoNext", EmitsChangedSignal::True, || Box::new(enabled));
    }

    Ok(())
  }

  /// Gets the seek enabled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_seek_enabled(&self) -> napi::Result<bool> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.can_seek);
    }

    Ok(false)
  }

  /// Sets the seek enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_seek_enabled(&mut self, enabled: bool) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.can_seek = enabled;
      drop(player_state);

      self
        .properties_changed
        .add_prop("CanSeek", EmitsChangedSignal::True, || Box::new(enabled));
    }

    Ok(())
  }

  /// Gets the playback rate
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_rate(&self) -> napi::Result<f64> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.playback_rate);
    }

    Ok(1.0)
  }

  /// Sets the playback rate
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_playback_rate(&mut self, playback_rate: f64) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.playback_rate = playback_rate;
      drop(player_state);

      self
        .properties_changed
        .add_prop("Rate", EmitsChangedSignal::True, || Box::new(playback_rate));
    }

    Ok(())
  }

  /// Gets the playback status
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_status(&self) -> napi::Result<MediaPlayerPlaybackStatus> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.playback_status);
    }

    Ok(MediaPlayerPlaybackStatus::Unknown)
  }

  /// Sets the playback status
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_playback_status(
    &mut self,
    playback_status: MediaPlayerPlaybackStatus,
  ) -> napi::Result<()> {
    if playback_status == MediaPlayerPlaybackStatus::Unknown {
      return Err(napi::Error::from_reason(format!(
        "{:?} is not a valid MediaPlayerPlaybackStatus to set",
        playback_status
      )));
    }

    if let Ok(mut player_state) = self.player_state.write() {
      player_state.playback_status = playback_status;
      drop(player_state);

      self
        .properties_changed
        .add_prop(
          "PlaybackStatus",
          EmitsChangedSignal::True,
          || match playback_status {
            MediaPlayerPlaybackStatus::Playing => Box::new("Playing".to_string()),
            MediaPlayerPlaybackStatus::Paused => Box::new("Paused".to_string()),
            MediaPlayerPlaybackStatus::Stopped => Box::new("Stopped".to_string()),
            _ => Box::new("Stopped".to_string()),
          },
        );
    }

    Ok(())
  }

  /// Gets the media type
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_media_type(&self) -> napi::Result<MediaPlayerMediaType> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.media_type);
    }

    Ok(MediaPlayerMediaType::Unknown)
  }

  /// Sets the media type
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_media_type(&mut self, media_type: MediaPlayerMediaType) -> napi::Result<()> {
    if media_type == MediaPlayerMediaType::Unknown {
      return Err(napi::Error::from_reason(format!(
        "{:?} is not a valid MediaPlayerMediaType to set",
        media_type
      )));
    }

    if let Ok(mut player_state) = self.player_state.write() {
      player_state.media_type = media_type;
      drop(player_state);
    }

    Ok(())
  }

  /// Gets the media title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_title(&self) -> napi::Result<String> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.title.to_owned());
    }

    Ok("".to_string())
  }

  /// Sets the media title
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_title(&mut self, title: String) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.title = title;
      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  /// Gets the media artist
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_artist(&self) -> napi::Result<String> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.artist.to_owned());
    }

    Ok("".to_string())
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_artist(&mut self, artist: String) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.artist = artist;
      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  /// Gets the media album title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_album_title(&self) -> napi::Result<String> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.album_title.to_owned());
    }

    Ok("".to_string())
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_album_title(&mut self, album_title: String) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.album_title = album_title;
      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  /// Gets the track id
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_track_id(&self) -> napi::Result<String> {
    if let Ok(player_state) = self.player_state.read() {
      return Ok(player_state.track_id.to_owned());
    }

    Ok("".to_string())
  }

  /// Sets the track id
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_track_id(&mut self, track_id: String) -> napi::Result<()> {
    if let Ok(mut player_state) = self.player_state.write() {
      player_state.track_id = track_id;
      drop(player_state);

      let metadata = self.construct_metadata();
      self
        .properties_changed
        .add_prop("Metadata", EmitsChangedSignal::True, || metadata);
    }

    Ok(())
  }

  fn construct_metadata(&self) -> Box<PropMap> {
    if let Ok(state) = self.player_state.read() {
      let mut metadata = Box::new(PropMap::new());
      metadata.insert(
        "mpris:trackid".to_string(),
        Variant(Box::new(format!("/xosms/trackid/{}", state.track_id))),
      );
      metadata.insert(
        "mpris:length".to_string(),
        Variant(Box::new(
          FloatDuration::seconds(state.position)
            .as_microseconds()
            .max(i64::MIN as f64)
            .min(i64::MAX as f64)
            .round() as i64,
        )),
      );
      metadata.insert(
        "mpris:artUrl".to_string(),
        Variant(Box::new(state.thumbnail.to_owned())),
      );
      metadata.insert(
        "xesam:title".to_string(),
        Variant(Box::new(state.title.to_owned())),
      );
      metadata.insert(
        "xesam:album".to_string(),
        Variant(Box::new(state.album_title.to_owned())),
      );
      metadata.insert(
        "xesam:artist".to_string(),
        Variant(Box::new(state.artist.to_owned())),
      );
      return metadata;
    }

    Box::new(PropMap::new())
  }
}

impl ObjectFinalize for MediaPlayer {
  fn finalize(self, _env: napi::Env) -> napi::Result<()> {
    self.dbus_session.unregister(&self.service_name);
    Ok(())
  }
}

pub struct MprisPlayerState {
  identity: String,
  can_go_next: bool,
  can_go_previous: bool,
  can_play: bool,
  can_pause: bool,
  can_seek: bool,
  can_control: bool,
  media_type: MediaPlayerMediaType,
  playback_status: MediaPlayerPlaybackStatus,
  thumbnail: String,
  artist: String,
  album_title: String,
  title: String,
  track_id: String,
  position: f64,
  last_updated_position: Instant,
  duration: f64,
  volume: f64,
  playback_rate: f64,
}

struct MprisPlayer {
  button_pressed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>>>,
  playback_position_changed_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>>,
  playback_position_seeked_listeners:
    Arc<DashMap<usize, ThreadsafeFunction<f64, ErrorStrategy::CalleeHandled>>>,
  state: Arc<RwLock<MprisPlayerState>>,
}

impl OrgMprisMediaPlayer2 for MprisPlayer {
  fn raise(&mut self) -> Result<(), ::dbus::MethodErr> {
    Ok(())
  }

  fn quit(&mut self) -> Result<(), ::dbus::MethodErr> {
    Ok(())
  }

  fn can_quit(&self) -> Result<bool, ::dbus::MethodErr> {
    Ok(false)
  }

  fn fullscreen(&self) -> Result<bool, ::dbus::MethodErr> {
    Ok(false)
  }

  fn set_fullscreen(&self, _value: bool) -> Result<(), ::dbus::MethodErr> {
    Err(MethodErr::failed("Fullscreen cannot be set"))
  }

  fn can_set_fullscreen(&self) -> Result<bool, ::dbus::MethodErr> {
    Ok(false)
  }

  fn can_raise(&self) -> Result<bool, ::dbus::MethodErr> {
    Ok(false)
  }

  fn has_track_list(&self) -> Result<bool, ::dbus::MethodErr> {
    Ok(false)
  }

  fn identity(&self) -> Result<String, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.identity.to_owned());
    }

    Err(MethodErr::failed("An error occurred while reading Volume"))
  }

  fn desktop_entry(&self) -> Result<String, ::dbus::MethodErr> {
    Err(MethodErr::no_property("DesktopEntry is not implemented"))
  }

  fn supported_uri_schemes(&self) -> Result<Vec<String>, ::dbus::MethodErr> {
    Ok(vec![])
  }

  fn supported_mime_types(&self) -> Result<Vec<String>, ::dbus::MethodErr> {
    Ok(vec![])
  }
}

impl OrgMprisMediaPlayer2Player for MprisPlayer {
  fn next(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_go_next {
        return Ok(());
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("next".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn previous(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_go_previous {
        return Ok(());
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("previous".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn pause(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_pause {
        return Ok(());
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("pause".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn play_pause(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_pause {
        return Err(MethodErr::failed("This media player cannot be paused"));
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("playpause".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn stop(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_control {
        return Err(MethodErr::failed("This media player cannot be controlled"));
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("stop".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn play(&mut self) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_play {
        return Ok(());
      }
    }

    for listener in self.button_pressed_listeners.iter() {
      listener.call(
        Ok("play".to_string()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn seek(&mut self, offset: i64) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_seek {
        return Ok(());
      }
    }

    for listener in self.playback_position_seeked_listeners.iter() {
      listener.call(
        Ok(FloatDuration::microseconds(offset as f64).as_seconds()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn set_position(
    &mut self,
    track_id: ::dbus::Path<'static>,
    position: i64,
  ) -> Result<(), ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      if !state.can_seek {
        return Ok(());
      }
      if position < 0 {
        return Ok(());
      }
      if Duration::from_micros(position as u64).as_secs_f64() > state.position {
        return Ok(());
      }
      // The track id being different signifies that this may have been called too late and should be ignored
      if track_id.to_string() != format!("/xosms/trackid/{}", state.track_id) {
        return Ok(());
      }
    }

    for listener in self.playback_position_changed_listeners.iter() {
      listener.call(
        Ok(Duration::from_micros(position as u64).as_secs_f64()),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }

    Ok(())
  }

  fn open_uri(&mut self, _uri: String) -> Result<(), ::dbus::MethodErr> {
    Err(MethodErr::failed("OpenUri is not supported"))
  }

  fn playback_status(&self) -> Result<String, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return match state.playback_status {
        MediaPlayerPlaybackStatus::Playing => Ok("Playing".to_string()),
        MediaPlayerPlaybackStatus::Paused => Ok("Paused".to_string()),
        MediaPlayerPlaybackStatus::Stopped => Ok("Stopped".to_string()),
        _ => Ok("Stopped".to_string()),
      };
    }

    Err(MethodErr::failed(
      "An error occurred while reading PlaybackStatus",
    ))
  }

  fn loop_status(&self) -> Result<String, ::dbus::MethodErr> {
    Err(MethodErr::no_property("LoopStatus is not implemented"))
  }

  fn set_loop_status(&self, _value: String) -> Result<(), ::dbus::MethodErr> {
    Err(MethodErr::no_property("LoopStatus is not implemented"))
  }

  fn rate(&self) -> Result<f64, ::dbus::MethodErr> {
    Ok(1.0)
  }

  fn set_rate(&self, _value: f64) -> Result<(), ::dbus::MethodErr> {
    // Clients should not be calling this with 0.0 but D-Bus states rate being set to 0.0 is equivalent to pausing
    // TODO: Make 0.0 call pause

    Ok(())
  }

  fn shuffle(&self) -> Result<bool, ::dbus::MethodErr> {
    Err(MethodErr::no_property("Shuffle is not implemented"))
  }

  fn set_shuffle(&self, _value: bool) -> Result<(), ::dbus::MethodErr> {
    Err(MethodErr::no_property("Shuffle is not implemented"))
  }

  fn metadata(&self) -> Result<::dbus::arg::PropMap, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      let mut metadata = PropMap::new();
      metadata.insert(
        "mpris:trackid".to_string(),
        Variant(Box::new(format!("/xosms/trackid/{}", state.track_id))),
      );
      metadata.insert(
        "mpris:length".to_string(),
        Variant(Box::new(
          FloatDuration::seconds(state.duration)
            .as_microseconds()
            .max(i64::MIN as f64)
            .min(i64::MAX as f64)
            .round() as i64,
        )),
      );
      metadata.insert(
        "mpris:artUrl".to_string(),
        Variant(Box::new(state.thumbnail.to_owned())),
      );
      metadata.insert(
        "xesam:title".to_string(),
        Variant(Box::new(state.title.to_owned())),
      );
      metadata.insert(
        "xesam:album".to_string(),
        Variant(Box::new(state.album_title.to_owned())),
      );
      metadata.insert(
        "xesam:artist".to_string(),
        Variant(Box::new(state.artist.to_owned())),
      );
      return Ok(metadata);
    }

    Err(MethodErr::failed(
      "An error occurred while reading Metadata",
    ))
  }

  fn volume(&self) -> Result<f64, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.volume);
    }

    Err(MethodErr::failed("An error occurred while reading Volume"))
  }

  fn set_volume(&self, value: f64) -> Result<(), ::dbus::MethodErr> {
    if let Ok(mut state) = self.state.write() {
      state.volume = value;
      return Ok(());
    }

    Err(MethodErr::failed("An error occurred while writing Volume"))
  }

  fn position(&self) -> Result<i64, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(
        FloatDuration::seconds(state.position)
          .as_microseconds()
          .max(i64::MIN as f64)
          .min(i64::MAX as f64)
          .round() as i64,
      );
    }

    Err(MethodErr::failed("An error occurred while reading Volume"))
  }

  fn minimum_rate(&self) -> Result<f64, ::dbus::MethodErr> {
    Ok(1.0)
  }

  fn maximum_rate(&self) -> Result<f64, ::dbus::MethodErr> {
    Ok(1.0)
  }

  fn can_go_next(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_go_next);
    }

    Err(MethodErr::failed(
      "An error occurred while reading CanGoNext",
    ))
  }

  fn can_go_previous(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_go_previous);
    }

    Err(MethodErr::failed(
      "An error occurred while reading CanGoPrevious",
    ))
  }

  fn can_play(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_play);
    }

    Err(MethodErr::failed("An error occurred while reading CanPlay"))
  }

  fn can_pause(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_pause);
    }

    Err(MethodErr::failed(
      "An error occurred while reading CanPause",
    ))
  }

  fn can_seek(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_seek);
    }

    Err(MethodErr::failed("An error occurred while reading CanSeek"))
  }

  fn can_control(&self) -> Result<bool, ::dbus::MethodErr> {
    if let Ok(state) = self.state.read() {
      return Ok(state.can_control);
    }

    Err(MethodErr::failed(
      "An error occurred while reading CanControl",
    ))
  }
}
