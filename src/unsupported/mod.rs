use napi::{Env, JsFunction};

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
}

#[napi]
impl MediaPlayerThumbnail {
  #[napi(factory)]
  #[allow(dead_code)]
  pub async fn create(
    thumbnail_type: MediaPlayerThumbnailType,
    _thumbnail: String,
  ) -> napi::Result<Self> {
    match thumbnail_type {
      MediaPlayerThumbnailType::File => {
        return Ok(Self { thumbnail_type });
      }
      MediaPlayerThumbnailType::Uri => {
        return Ok(Self { thumbnail_type });
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

#[napi]
struct MediaPlayer {}

#[napi]
impl MediaPlayer {
  #[napi(constructor)]
  #[allow(dead_code)]
  pub fn new(_service_name: String, _identity: String) -> napi::Result<Self> {
    Ok(Self {})
  }

  /// Activates the MediaPlayer allowing the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn activate(&self) -> napi::Result<()> {
    Ok(())
  }

  /// Deactivates the MediaPlayer denying the operating system to see and use it
  #[napi]
  #[allow(dead_code)]
  pub fn deactivate(&self) -> napi::Result<()> {
    Ok(())
  }

  /// Adds an event listener to the MediaPlayer
  #[napi]
  #[allow(dead_code)]
  pub fn add_event_listener(
    &mut self,
    _env: Env,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] _event_name: String,
    _callback: JsFunction,
  ) -> napi::Result<()> {
    Ok(())
  }

  /// Removes an event listener from the MediaPlayer
  #[napi]
  #[allow(dead_code)]
  pub fn remove_event_listener(
    &mut self,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] _event_name: String,
    _callback: JsFunction,
  ) -> napi::Result<()> {
    Ok(())
  }

  /// Adds an event listener to the MediaPlayer
  ///
  /// Alias for addEventListener
  #[napi]
  #[allow(dead_code)]
  pub fn on(
    &mut self,
    _env: Env,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] _event_name: String,
    _callback: JsFunction,
  ) -> napi::Result<()> {
    Ok(())
  }

  /// Removes an event listener from the MediaPlayer
  ///
  /// Alias for removeEventListener
  #[napi]
  #[allow(dead_code)]
  pub fn off(
    &mut self,
    #[napi(ts_arg_type = "'buttonpressed' | 'positionchanged'")] _event_name: String,
    _callback: JsFunction,
  ) -> napi::Result<()> {
    Ok(())
  }

  /// Instructs the media service to update its media information being displayed
  #[napi]
  #[allow(dead_code)]
  pub fn update(&self) -> napi::Result<()> {
    Ok(())
  }

  /// Sets the thumbnail
  #[napi]
  #[allow(dead_code)]
  pub fn set_thumbnail(&mut self, _thumbnail: &MediaPlayerThumbnail) -> napi::Result<()> {
    Ok(())
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

    Ok(())
  }

  /// Gets the play button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_play_button_enabled(&self) -> napi::Result<bool> {
    Ok(false)
  }

  /// Sets the play button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_play_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_pause_button_enabled(&self) -> napi::Result<bool> {
    Ok(false)
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_pause_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the paused button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_stop_button_enabled(&self) -> napi::Result<bool> {
    Ok(false)
  }

  /// Sets the paused button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_stop_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the previous button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_previous_button_enabled(&self) -> napi::Result<bool> {
    Ok(false)
  }

  /// Sets the previous button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_previous_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the next button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_next_button_enabled(&self) -> napi::Result<bool> {
    Ok(false)
  }

  /// Sets the next button enbled state
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_next_button_enabled(&mut self, _enabled: bool) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the next button enbled state
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_rate(&self) -> napi::Result<f64> {
    Ok(1.0)
  }

  /// Sets the playback rate
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_playback_rate(&mut self, _playback_rate: f64) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the playback status
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_playback_status(&self) -> napi::Result<MediaPlayerPlaybackStatus> {
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

    Ok(())
  }

  /// Gets the media type
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_media_type(&self) -> napi::Result<MediaPlayerMediaType> {
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

    Ok(())
  }

  /// Gets the media title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_title(&self) -> napi::Result<String> {
    Ok("".to_string())
  }

  /// Sets the media title
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_title(&mut self, _title: String) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the media artist
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_artist(&self) -> napi::Result<String> {
    Ok("".to_string())
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_artist(&mut self, _artist: String) -> napi::Result<()> {
    Ok(())
  }

  /// Gets the media album title
  #[napi(getter)]
  #[allow(dead_code)]
  pub fn get_album_title(&self) -> napi::Result<String> {
    Ok("".to_string())
  }

  /// Sets the media artist
  #[napi(setter)]
  #[allow(dead_code)]
  pub fn set_album_title(&mut self, _album_title: String) -> napi::Result<()> {
    Ok(())
  }
}
