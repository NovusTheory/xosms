#![deny(clippy::all)]

mod backends;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use napi::{Env, Error, Status, bindgen_prelude::Promise, threadsafe_function::ThreadsafeFunction};
use napi_derive::napi;
use tokio::{sync::{Mutex, RwLock as AsyncRwLock, mpsc::{self, Receiver}}, task::{self, JoinHandle}};

use crate::backends::{PlatformBackend, PlatformThumbnailBackend, common::PlatformBackendEvent};

pub type ButtonPressedCallbackTSFN = ThreadsafeFunction<ButtonPressedType, Option<Promise<()>>, ButtonPressedType, Status, false, true>;
pub type PositionChangedCallbackTSFN = ThreadsafeFunction<f64, Option<Promise<()>>, f64, Status, false, true>;
pub type PositionSeekedCallbackTSFN = ThreadsafeFunction<f64, Option<Promise<()>>, f64, Status, false, true>;
pub type LoopChangedCallbackTSFN = ThreadsafeFunction<LoopType, Option<Promise<()>>, LoopType, Status, false, true>;
pub type RateChangedCallbackTSFN = ThreadsafeFunction<f64, Option<Promise<()>>, f64, Status, false, true>;
pub type ShuffleChangedCallbackTSFN = ThreadsafeFunction<bool, Option<Promise<()>>, bool, Status, false, true>;
pub type VolumeChangedCallbackTSFN = ThreadsafeFunction<f64, Option<Promise<()>>, f64, Status, false, true>;

// Checks if the service name follows the required format for https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
fn is_valid_service_name(s: &str) -> bool {
  if s.is_empty() {
    return false;
  }

  if s.starts_with(|c: char| c.is_ascii_digit()) {
    return false;
  }

  s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[derive(Default, Clone)]
pub struct MediaPlayerState {
  play_button_enabled: bool, 
  pause_button_enabled: bool,
  stop_button_enabled: bool,
  previous_button_enabled: bool,
  next_button_enabled: bool,
  seek_enabled: bool,
  fullscreen: bool,
  playback_rate: f64,
  minimum_playback_rate: f64,
  maximum_playback_rate: f64,
  shuffle: bool,
  r#loop: LoopType,
  volume: f64,
  playback_status: PlaybackStatus,
  // media_type
  title: String,
  artist: Vec<String>,
  album_title: String,
  track_id: String,

  thumbnail: Option<Arc<PlatformThumbnailBackend>>
}

#[derive(Default)]
pub struct TSFNCallbacks {
  button_pressed: Option<ButtonPressedCallbackTSFN>,
  position_changed: Option<PositionChangedCallbackTSFN>,
  position_seeked: Option<PositionSeekedCallbackTSFN>,
  loop_changed: Option<LoopChangedCallbackTSFN>,
  rate_changed: Option<RateChangedCallbackTSFN>,
  shuffle_changed: Option<ShuffleChangedCallbackTSFN>,
  volume_changed: Option<VolumeChangedCallbackTSFN>
}

#[napi(string_enum = "lowercase")]
pub enum ButtonPressedType {
  Play,
  Pause,
  PlayPause,
  Stop,
  Next,
  Previous
}

#[derive(Default, Clone, PartialEq)]
#[napi(string_enum = "lowercase")]
pub enum LoopType {
  #[default]
  None,
  Track,
  Playlist
}

#[derive(Default, Clone, PartialEq)]
#[napi(string_enum = "lowercase")]
pub enum PlaybackStatus {
  Playing,
  Paused,
  #[default]
  Stopped
}

#[napi]
pub struct MediaPlayer {
  // We use a Mutex for the PlatformBackend as accessing the PlatformBackend mutably is within an async context and is unsafe
  backend: Mutex<Option<PlatformBackend>>,
  // We use a sync RwLock on these to absorb the complexity of accessing the state if JS tries to borrow the class again
  state: RwLock<MediaPlayerState>,
  callbacks: Arc<AsyncRwLock<TSFNCallbacks>>,
  callbacks_rx: Mutex<Option<Receiver<PlatformBackendEvent>>>,
  callbacks_task_handle: Mutex<Option<JoinHandle<()>>>
}

fn acquire_clear_poison_read<'lock, T>(lock: &'lock RwLock<T>) -> RwLockReadGuard<'lock, T> {
  lock.read().unwrap_or_else(|poison_error| {
    poison_error.into_inner()
  })
}

fn acquire_clear_poison_write<'lock, T>(lock: &'lock RwLock<T>) -> RwLockWriteGuard<'lock, T> {
  lock.write().unwrap_or_else(|poison_error| {
    poison_error.into_inner()
  })
}

/// Indicates whether this platform shares all instances of MediaPlayer. Independent MediaPlayer's override the same information to the platform's media service.
#[napi]
pub fn platform_shares_media_players() -> bool {
  if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
    return true;
  }

  return false;
}

#[napi]
impl MediaPlayer {
  /// Creates a MediaPlayer which provides an independent source of controls and information on the platform backend
  #[napi(constructor)]
  pub fn new(service_name: String, identity: String) -> napi::Result<Self> {
    if !is_valid_service_name(&service_name) {
      return Err(Error::new(Status::InvalidArg, "serviceName must only contain the ASCII characters '[A-Z][a-z][0-9]_-'"));
    }

    let (callback_tx, callback_rx) = mpsc::channel::<PlatformBackendEvent>(128);

    let mut state = MediaPlayerState::default();
    // Replace with https://github.com/rust-lang/rust/issues/132162 when available
    state.playback_rate = 1.0;
    state.minimum_playback_rate = 1.0;
    state.maximum_playback_rate = 1.0;
    Ok(Self {
      backend: Mutex::new(Some(PlatformBackend::new(service_name, identity, callback_tx, &state))),
      state: RwLock::new(state),
      callbacks: Arc::new(AsyncRwLock::new(TSFNCallbacks::default())),
      callbacks_rx: Mutex::new(Some(callback_rx)),
      callbacks_task_handle: Mutex::new(None)
    })
  }

  /// Activates this MediaPlayer by starting the platform backend
  #[napi]
  pub async fn activate(&self) -> Result<(), Error> {
    let mut backend = self.backend.lock().await;
    if let Some(backend) = backend.as_mut() {
      let mut callbacks_rx = self.callbacks_rx.lock().await;
      if let Some(mut callback_rx) = callbacks_rx.take() {
        let mut callbacks_task_handle = self.callbacks_task_handle.lock().await;
        let callbacks = self.callbacks.clone();
        let join_handle = tokio::spawn(async move {
          while let Some(event) = callback_rx.recv().await {
            let callbacks = callbacks.read().await;
            match event {
                PlatformBackendEvent::ButtonPressed { button } => {
                  if let Some(callback) = callbacks.button_pressed.as_ref() {
                    let result = callback.call_async(button).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::PositionChanged { position } => {
                  if let Some(callback) = callbacks.position_changed.as_ref() {
                    let result = callback.call_async(position).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::PositionSeeked { offset } => {
                  if let Some(callback) = callbacks.position_seeked.as_ref() {
                    let result = callback.call_async(offset).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::LoopChanged { loop_type } => {
                  if let Some(callback) = callbacks.loop_changed.as_ref() {
                    let result = callback.call_async(loop_type).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::RateChanged { rate } => {
                  if let Some(callback) = callbacks.rate_changed.as_ref() {
                    let result = callback.call_async(rate).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::ShuffleChanged { shuffle } => {
                  if let Some(callback) = callbacks.shuffle_changed.as_ref() {
                    let result = callback.call_async(shuffle).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
                PlatformBackendEvent::VolumeChanged { volume } => {
                  if let Some(callback) = callbacks.volume_changed.as_ref() {
                    let result = callback.call_async(volume).await;
                    if let Ok(promise_result) = result {
                      if let Some(promise) = promise_result {
                        let _ = promise.await;
                      }
                    }
                  }
                },
            }
          }
        });
        *callbacks_task_handle = Some(join_handle);
      }

      backend.activate().await.map_err(|err: backends::error::XosmsError| {
        Error::new(Status::GenericFailure, format!("Failed to activate backend {:?}", err))
      })?;
    } else {
      return Err(Error::new(Status::GenericFailure, "This MediaPlayer has been disposed and cannot be used"));
    }

    Ok(())
  }

  /// Deactivates this MediaPlayer by stopping the platform backend
  #[napi]
  pub async fn deactivate(&self) -> Result<(), Error> {
    let mut callbacks_task = self.callbacks_task_handle.lock().await ;
    if let Some(callbacks_task_handle) = callbacks_task.take() {
      callbacks_task_handle.abort();
    }

    let mut backend = self.backend.lock().await;
    if let Some(backend) = backend.as_mut() {
      backend.deactivate().await.map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to deactivate backend {:?}", err))
      })?;
    } else {
      return Err(Error::new(Status::GenericFailure, "This MediaPlayer has been disposed and cannot be used"));
    }

    Ok(())
  }

  /// Updates the platform backend with the latest state
  #[napi]
  pub async fn update(&self) -> Result<(), Error> {
    let backend = self.backend.lock().await;
    let state_clone = {
      let state = self.state.read().map_err(|_err| {
        Error::new(Status::GenericFailure, "Failed to read state")
      })?;
      state.clone()
    };
    if let Some(backend) = backend.as_ref() {
      backend.update(state_clone).await.map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to update backend {:?}", err))
      })?;
    } else {
      return Err(Error::new(Status::GenericFailure, "This MediaPlayer has been disposed and cannot be used"));
    }

    Ok(())
  }

  /// Sets the timeline data
  /// 
  /// This function does not require `MediaPlayer.update()` to be called and immediately sends changes to the platform backend
  #[napi]
  pub async fn set_timeline(&self, duration: f64, position: f64) -> Result<(), Error> {
    let mut backend = self.backend.lock().await;
    if let Some(backend) = backend.as_mut() {
      backend.set_timeline(duration, position).await.map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to call set_timeline {:?}", err))
      })?;
    } else {
      return Err(Error::new(Status::GenericFailure, "This MediaPlayer has been disposed and cannot be used"));
    }

    Ok(())
  }

  // napi-rs doesn't know how to unwrap the custom defined types
  #[napi(ts_args_type = "callback: (button: ButtonPressedType) => Promise<void> | void")]
  pub fn set_button_pressed_callback(&self, callback: ButtonPressedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().button_pressed = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (position: number) => Promise<void> | void")]
  pub fn set_position_changed_callback(&self, callback: PositionChangedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().position_changed = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (offset: number) => Promise<void> | void")]
  pub fn set_position_seeked_callback(&self, callback: PositionSeekedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().position_seeked = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (loop: LoopType) => Promise<void> | void")]
  pub fn set_loop_changed_callback(&self, callback: LoopChangedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().loop_changed = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (rate: number) => Promise<void> | void")]
  pub fn set_rate_changed_callback(&self, callback: RateChangedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().rate_changed = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (shuffle: boolean) => Promise<void> | void")]
  pub fn set_shuffle_changed_callback(&self, callback: ShuffleChangedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().shuffle_changed = Some(callback);

    Ok(())
  }

  #[napi(ts_args_type = "callback: (volume: number) => Promise<void> | void")]
  pub fn set_volume_changed_callback(&self, callback: VolumeChangedCallbackTSFN) -> Result<(), Error> {
    self.callbacks.blocking_write().volume_changed = Some(callback);

    Ok(())
  }


  #[napi(getter)]
  pub fn get_play_button_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).play_button_enabled
  }

  #[napi(setter)]
  pub fn set_play_button_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).play_button_enabled = enabled;
  }


  #[napi(getter)]
  pub fn get_pause_button_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).pause_button_enabled
  }

  #[napi(setter)]
  pub fn set_pause_button_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).pause_button_enabled = enabled;
  }


  #[napi(getter)]
  pub fn get_stop_button_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).stop_button_enabled
  }

  #[napi(setter)]
  pub fn set_stop_button_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).stop_button_enabled = enabled;
  }

  
  #[napi(getter)]
  pub fn get_previous_button_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).previous_button_enabled
  }

  #[napi(setter)]
  pub fn set_previous_button_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).previous_button_enabled = enabled;
  }


  #[napi(getter)]
  pub fn get_next_button_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).next_button_enabled
  }

  #[napi(setter)]
  pub fn set_next_button_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).next_button_enabled = enabled;
  }



  #[napi(getter)]
  pub fn get_seek_enabled(&self) -> bool {
    acquire_clear_poison_read(&self.state).seek_enabled
  }

  #[napi(setter)]
  pub fn set_seek_enabled(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).seek_enabled = enabled;
  }

  
  #[napi(getter)]
  pub fn get_fullscreen(&self) -> bool {
    acquire_clear_poison_read(&self.state).fullscreen
  }

  #[napi(setter)]
  pub fn set_fullscreen(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).fullscreen = enabled;
  }


  #[napi(getter)]
  pub fn get_playback_rate(&self) -> f64 {
    acquire_clear_poison_read(&self.state).playback_rate
  }

  #[napi(setter)]
  pub fn set_playback_rate(&self, rate: f64) {
    acquire_clear_poison_write(&self.state).playback_rate = rate;
  }


  #[napi(getter)]
  pub fn get_minimum_playback_rate(&self) -> f64 {
    acquire_clear_poison_read(&self.state).minimum_playback_rate
  }

  #[napi(setter)]
  pub fn set_minimum_playback_rate(&self, rate: f64) {
    acquire_clear_poison_write(&self.state).minimum_playback_rate = rate;
  }


  #[napi(getter)]
  pub fn get_maximum_playback_rate(&self) -> f64 {
    acquire_clear_poison_read(&self.state).maximum_playback_rate
  }

  #[napi(setter)]
  pub fn set_maximum_playback_rate(&self, rate: f64) {
    acquire_clear_poison_write(&self.state).maximum_playback_rate = rate;
  }

  
  #[napi(getter)]
  pub fn get_shuffle(&self) -> bool {
    acquire_clear_poison_read(&self.state).shuffle
  }

  #[napi(setter)]
  pub fn set_shuffle(&self, enabled: bool) {
    acquire_clear_poison_write(&self.state).shuffle = enabled;
  }


  #[napi(getter)]
  pub fn get_loop(&self) -> LoopType {
    acquire_clear_poison_read(&self.state).r#loop.clone()
  }

  #[napi(setter, ts_args_type = "loop: LoopType")]
  pub fn set_loop(&self, r#loop: LoopType) {
    acquire_clear_poison_write(&self.state).r#loop = r#loop;
  }


  #[napi(getter)]
  pub fn get_volume(&self) -> f64 {
    acquire_clear_poison_read(&self.state).volume
  }

  #[napi(setter)]
  pub fn set_volume(&self, value: f64) {
    acquire_clear_poison_write(&self.state).volume = value;
  }


  #[napi(getter)]
  pub fn get_playback_status(&self) -> PlaybackStatus {
    acquire_clear_poison_read(&self.state).playback_status.clone()
  }

  #[napi(setter)]
  pub fn set_playback_status(&self, status: PlaybackStatus) {
    acquire_clear_poison_write(&self.state).playback_status = status;
  }


  #[napi(getter)]
  pub fn get_title(&self) -> String {
    acquire_clear_poison_read(&self.state).title.clone()
  }

  #[napi(setter)]
  pub fn set_title(&self, value: String) {
    acquire_clear_poison_write(&self.state).title = value;
  }


  #[napi(getter)]
  pub fn get_artist(&self) -> Vec<String> {
    acquire_clear_poison_read(&self.state).artist.clone()
  }

  #[napi(setter)]
  pub fn set_artist(&self, value: Vec<String>) {
    acquire_clear_poison_write(&self.state).artist = value;
  }


  #[napi(getter)]
  pub fn get_album_title(&self) -> String {
    acquire_clear_poison_read(&self.state).album_title.clone()
  }

  #[napi(setter)]
  pub fn set_album_title(&self, value: String) {
    acquire_clear_poison_write(&self.state).album_title = value;
  }


  #[napi(getter)]
  pub fn get_track_id(&self) -> String {
    acquire_clear_poison_read(&self.state).track_id.clone()
  }

  #[napi(setter)]
  pub fn set_track_id(&self, value: String) {
    acquire_clear_poison_write(&self.state).track_id = value;
  }

  
  #[napi(getter)]
  pub fn get_thumbnail(&self) -> Option<MediaPlayerThumbnail> {
    if let Some(platform_thumbnail) = acquire_clear_poison_read(&self.state).thumbnail.as_ref() {
      return Some(MediaPlayerThumbnail {
        thumbnail_type: platform_thumbnail.r#type,
        backend: platform_thumbnail.clone()
      });
    }

    None
  }

  #[napi(setter)]
  pub fn set_thumbnail(&self, value: Option<&MediaPlayerThumbnail>) {
    if let Some(thumbnail) = value {
      acquire_clear_poison_write(&self.state).thumbnail = Some(thumbnail.backend.clone());
    } else {
      acquire_clear_poison_write(&self.state).thumbnail = None;
    }
  }


  /// Disposes this MediaPlayer releasing all native resources. This MediaPlayer can no longer be used after calling this
  #[napi]
  pub fn dispose(&self) {
    drop(self.backend.blocking_lock().take())
  }
}

#[derive(Clone, Copy)]
#[napi(string_enum = "lowercase")]
pub enum MediaPlayerThumbnailType {
  File,
  Uri
}

#[napi]
pub struct MediaPlayerThumbnail {
  thumbnail_type: MediaPlayerThumbnailType,
  backend: Arc<PlatformThumbnailBackend>
}

#[napi]
impl MediaPlayerThumbnail {
  #[napi(factory)]
  pub async fn create(thumbnail_type: MediaPlayerThumbnailType, thumbnail: String) -> Result<Self, Error> {
    Ok(Self {
      thumbnail_type,
      backend: Arc::new(PlatformThumbnailBackend::new(thumbnail_type, thumbnail).await.map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to create thumbnail backend {:?}", err))
      })?)
    })
  }

  #[napi(getter)]
  pub fn get_thumbnail_type(&self) -> MediaPlayerThumbnailType {
    self.thumbnail_type
  }
}

#[cfg(target_os = "macos")]
#[repr(C)]
struct RunLoopPump {
    handle: libuv_sys2::uv_check_t,
}

#[cfg(target_os = "macos")]
unsafe extern "C" fn run_event_loop(handle: *mut libuv_sys2::uv_check_t) {
  use objc2_core_foundation::{CFRunLoop, kCFRunLoopDefaultMode};

  CFRunLoop::run_in_mode(unsafe { kCFRunLoopDefaultMode }, 0.0, true);
}

#[cfg(target_os = "macos")]
#[napi(module_exports)]
pub fn init(env: Env) -> napi::bindgen_prelude::Result<()> {
  use objc2::MainThreadMarker;
  use objc2_app_kit::NSApplication;

  if let Some(mtm) = MainThreadMarker::new() {
    let app = NSApplication::sharedApplication(mtm);
    // Xosms is being initialized in an already running NSApplication environment e.g. Electron. We don't need to setup an event loop pump
    if app.isRunning() {
      return Ok(())
    }

    if let Ok(uv_loop_ptr) = env.get_uv_event_loop() {
      let uv_loop_ptr: *mut libuv_sys2::uv_loop_s = uv_loop_ptr as *mut libuv_sys2::uv_loop_s;

      let pump = Box::new(RunLoopPump {
        handle: unsafe { std::mem::zeroed() }
      });
      let pump_ptr = Box::into_raw(pump);
      let check_handle_ptr = unsafe { std::ptr::addr_of_mut!((*pump_ptr).handle) };

      unsafe {
        let init_status = libuv_sys2::uv_check_init(uv_loop_ptr, check_handle_ptr);
        if init_status != 0 {
          let _ = Box::from_raw(pump_ptr);
          return Err(Error::new(
              Status::GenericFailure,
              format!("Failed to initialize uv_check: {}", init_status),
          ));
        }

        (*check_handle_ptr).data = pump_ptr as *mut std::ffi::c_void;

        let start_status = libuv_sys2::uv_check_start(check_handle_ptr, Some(run_event_loop));
        if start_status != 0 {
          return Err(Error::new(
              Status::GenericFailure,
              format!("Failed to start uv_check: {}", start_status),
          ));
        }

        libuv_sys2::uv_unref(check_handle_ptr as *mut libuv_sys2::uv_handle_t);
      }

      return Ok(());
    }
    
    return Err(Error::new(Status::GenericFailure, "Could not setup platform event loop"));
  } else {
    return Err(Error::new(Status::GenericFailure, "This platform requires usage from the main thread!"));
  }
}
