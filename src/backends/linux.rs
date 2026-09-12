
use std::{collections::HashMap, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use tokio::{sync::{RwLock, mpsc::Sender}, time::Instant};
use zbus::{Connection, connection, interface, object_server::SignalEmitter, zvariant::{ObjectPath, Value}};

use crate::{ButtonPressedType, LoopType, MediaPlayerState, MediaPlayerThumbnailType, PlaybackStatus, backends::{PlatformBackend, PlatformThumbnailBackend, common::{MediaPlayerTimelineState, PlatformBackendEvent}, error::XosmsError}};

#[derive(Default)]
pub struct PlatformData {
    connection: Option<Connection>,
}

enum MediaPlayerStatePropertyChange {
    PlayButtonEnabled,
    PauseButtonEnabled,
    StopButtonEnabled,
    PreviousButtonEnabled,
    NextButtonEnabled,
    SeekEnabled,
    Fullscreen,
    PlaybackRate,
    MinimumPlaybackRate,
    MaximumPlaybackRate,
    Shuffle,
    Loop,
    Volume,
    PlaybackStatus,
    // MediaType
    Metadata // Indicates one of title, album, artist, etc has changed
}

impl PlatformBackend {
    pub fn new(service_name: String, identity: String, callback_tx: Sender<PlatformBackendEvent>, state: &MediaPlayerState) -> Self {
        Self {
            service_name,
            identity,
            state: Arc::new(RwLock::new(state.clone())),
            timeline_state: Arc::new(MediaPlayerTimelineState::default()),
            last_timeline_state_update: Instant::now(),
            callback_tx: Arc::new(callback_tx),

            platform_data: PlatformData::default()
        }
    }

    pub async fn activate(&mut self) -> Result<(), XosmsError> {
        self.platform_data.connection = Some(connection::Builder::session()?
            .name(format!("org.mpris.MediaPlayer2.{}", self.service_name))?
            .allow_name_replacements(false)
            .replace_existing_names(false)
            .serve_at("/org/mpris/MediaPlayer2", MPRISMediaPlayer2 {
                state: self.state.clone(),
                identity: self.identity.clone()
            })?
            .serve_at("/org/mpris/MediaPlayer2", MPRISMediaPlayer2Player {
                state: self.state.clone(),
                timeline_state: self.timeline_state.clone(),
                callback_tx: self.callback_tx.clone(),
            })?
            .build()
            .await?);

        Ok(())
    }

    pub async fn deactivate(&mut self) -> Result<(), XosmsError> {
        if let Some(connection) = self.platform_data.connection.take() {
            connection.close().await?;
        }

        Ok(())
    }

    pub async fn update(&self, state: MediaPlayerState) -> Result<(), XosmsError> {
        let mut changes = Vec::new();
        {
            let mut stale_state = self.state.write().await;
            
            if stale_state.play_button_enabled != state.play_button_enabled { changes.push(MediaPlayerStatePropertyChange::PlayButtonEnabled); }
            if stale_state.pause_button_enabled != state.pause_button_enabled { changes.push(MediaPlayerStatePropertyChange::PauseButtonEnabled); }
            if stale_state.stop_button_enabled != state.stop_button_enabled { changes.push(MediaPlayerStatePropertyChange::StopButtonEnabled); }
            if stale_state.previous_button_enabled != state.previous_button_enabled { changes.push(MediaPlayerStatePropertyChange::PreviousButtonEnabled); }
            if stale_state.next_button_enabled != state.next_button_enabled { changes.push(MediaPlayerStatePropertyChange::NextButtonEnabled); }
            if stale_state.seek_enabled != state.seek_enabled { changes.push(MediaPlayerStatePropertyChange::SeekEnabled); }
            if stale_state.fullscreen != state.fullscreen { changes.push(MediaPlayerStatePropertyChange::Fullscreen); }
            if stale_state.playback_rate != state.playback_rate { changes.push(MediaPlayerStatePropertyChange::PlaybackRate); }
            if stale_state.minimum_playback_rate != state.minimum_playback_rate { changes.push(MediaPlayerStatePropertyChange::MinimumPlaybackRate); }
            if stale_state.maximum_playback_rate != state.maximum_playback_rate { changes.push(MediaPlayerStatePropertyChange::MaximumPlaybackRate); }
            if stale_state.shuffle != state.shuffle { changes.push(MediaPlayerStatePropertyChange::Shuffle); }
            if stale_state.r#loop != state.r#loop { changes.push(MediaPlayerStatePropertyChange::Loop); }
            if stale_state.volume != state.volume { changes.push(MediaPlayerStatePropertyChange::Volume); }
            if stale_state.playback_status != state.playback_status { changes.push(MediaPlayerStatePropertyChange::PlaybackStatus); }
            let mut metadata_changed = false;
            if stale_state.title != state.title { metadata_changed = true; }
            if stale_state.artist != state.artist { metadata_changed = true; }
            if stale_state.album_title != state.album_title { metadata_changed = true; }
            if stale_state.track_id != state.track_id { metadata_changed = true; }
            if (stale_state.thumbnail.is_none() && state.thumbnail.is_some()) || (stale_state.thumbnail.is_some() && state.thumbnail.is_none()) { metadata_changed = true; }
            if stale_state.thumbnail.is_some() && state.thumbnail.is_some() && let Some(stale_thumbnail) = stale_state.thumbnail.as_ref() && let Some(thumbnail) = state.thumbnail.as_ref() {
                if stale_thumbnail.id != thumbnail.id { metadata_changed = true; }
            }
            if metadata_changed { changes.push(MediaPlayerStatePropertyChange::Metadata); }

            *stale_state = state;
        }

        if let Some(connection) = &self.platform_data.connection {
            let mediaplayer2_iface = connection.object_server().interface::<_, MPRISMediaPlayer2>("/org/mpris/MediaPlayer2").await?;
            let mediaplayer2_player_iface = connection.object_server().interface::<_, MPRISMediaPlayer2Player>("/org/mpris/MediaPlayer2").await?;

            let mediaplayer2_iface_emitter = mediaplayer2_iface.signal_emitter();
            let mediaplayer2_player_iface_emitter = mediaplayer2_player_iface.signal_emitter();

            let mediaplayer2 = mediaplayer2_iface.get().await;
            let mediaplayer2_player = mediaplayer2_player_iface.get().await;

            for change in changes {
                match change {
                    MediaPlayerStatePropertyChange::PlayButtonEnabled => mediaplayer2_player.can_play_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::PauseButtonEnabled => mediaplayer2_player.can_pause_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::StopButtonEnabled => {}, // Tied to CanControl which is always true. Ignored on this platform
                    MediaPlayerStatePropertyChange::PreviousButtonEnabled => mediaplayer2_player.can_go_previous_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::NextButtonEnabled => mediaplayer2_player.can_go_next_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::SeekEnabled => mediaplayer2_player.can_seek_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::Fullscreen => mediaplayer2.fullscreen_changed(mediaplayer2_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::PlaybackRate => mediaplayer2_player.rate_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::MinimumPlaybackRate => mediaplayer2_player.minimum_rate_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::MaximumPlaybackRate => mediaplayer2_player.maximum_rate_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::Shuffle => mediaplayer2_player.shuffle_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::Loop => mediaplayer2_player.loop_status_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::Volume => mediaplayer2_player.volume_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::PlaybackStatus => mediaplayer2_player.playback_status_changed(mediaplayer2_player_iface_emitter).await?,
                    MediaPlayerStatePropertyChange::Metadata => mediaplayer2_player.metadata_changed(mediaplayer2_player_iface_emitter).await?
                }
            }
        }

        Ok(())
    }

    pub async fn set_timeline(&mut self, duration: f64, position: f64) -> Result<(), XosmsError> {
        let duration = (duration * 1_000_000.0).round() as i64;
        let position = (position * 1_000_000.0).round() as i64;

        let state = self.state.read().await;

        let stale_duration = self.timeline_state.duration.swap(duration, Ordering::Relaxed);
        let stale_position = self.timeline_state.position.swap(position, Ordering::Relaxed);

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_timeline_state_update).as_micros() as f64;
        let delta = position - stale_position;
        let expected_delta = (elapsed * state.playback_rate).round() as i64;
        let drift = (delta - expected_delta).abs();

        self.last_timeline_state_update = now;
        
        // Detect whether the timeline has drifted enough from the playback rate and emit a seeked event
        if drift >= 50_000 && state.playback_status == PlaybackStatus::Playing {
            if let Some(connection) = &self.platform_data.connection {
                let mediaplayer2_player_iface = connection.object_server().interface::<_, MPRISMediaPlayer2Player>("/org/mpris/MediaPlayer2").await?;
                mediaplayer2_player_iface.seeked(position).await?;
            }
        }

        // If the duration changes we need to emit a PropertiesChanged signal for Metadata as that exposes mpris:length 
        if stale_duration != duration {
            if let Some(connection) = &self.platform_data.connection {
                let mediaplayer2_player_iface = connection.object_server().interface::<_, MPRISMediaPlayer2Player>("/org/mpris/MediaPlayer2").await?;
                let mediaplayer2_player_iface_emitter = mediaplayer2_player_iface.signal_emitter();
                let mediaplayer2_player = mediaplayer2_player_iface.get().await;

                mediaplayer2_player.metadata_changed(mediaplayer2_player_iface_emitter).await?;
            }
        }
        
        Ok(())
    }
}

struct MPRISMediaPlayer2 {
    state: Arc<RwLock<MediaPlayerState>>,
    identity: String
}

// https://specifications.freedesktop.org/mpris/latest/Media_Player.html
#[interface(name = "org.mpris.MediaPlayer2")]
impl MPRISMediaPlayer2 {
    async fn raise(&self) {

    }

    async fn quit(&self) {

    }

    #[zbus(property)]
    async fn can_quit(&self) -> bool {
        false
    }

    #[zbus(property)]
    async fn fullscreen(&self) -> bool {
        let state = self.state.read().await;
        state.fullscreen
    }

    #[zbus(property)]
    async fn can_set_fullscreen(&self) -> bool {
        false
    }

    #[zbus(property)]
    async fn can_raise(&self) -> bool {
        false
    }

    #[zbus(property)]
    async fn has_track_list(&self) -> bool {
        false
    }

    #[zbus(property)]
    async fn identity(&self) -> &String {
        &self.identity
    }

    // TODO: Implement this
    /*
    #[zbus(property)]
    async fn desktop_entry(&self) -> String {
        "".to_owned()
    }

    #[zbus(property)]
    async fn supported_uri_schemes(&self) -> Vec<String> {
        vec![]
    }

    #[zbus(property)]
    async fn supported_mime_types(&self) -> Vec<String> {
        vec![]
    }
    */
}

struct MPRISMediaPlayer2Player {
    state: Arc<RwLock<MediaPlayerState>>,
    timeline_state: Arc<MediaPlayerTimelineState>,
    callback_tx: Arc<Sender<PlatformBackendEvent>>
}

// https://specifications.freedesktop.org/mpris/latest/Player_Interface.html
#[interface(name = "org.mpris.MediaPlayer2.Player")]
impl MPRISMediaPlayer2Player {
    async fn next(&self) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.next_button_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Next });
        }
    }

    async fn previous(&self) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.previous_button_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Previous });
        }
    }

    async fn pause(&self) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.pause_button_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Pause });
        }
    }

    async fn play_pause(&self) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.pause_button_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::PlayPause });
        }
    }

    async fn stop(&self) {
        let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Stop });
    }

    async fn play(&self) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.play_button_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Play });
        }
    }

    async fn seek(&self, offset: i64) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.seek_enabled {
            let _ = self.callback_tx.try_send(PlatformBackendEvent::PositionSeeked { offset: offset  as f64 / 1_000_000.0 });
        }
    }

    async fn set_position(&self, track_id: ObjectPath<'_>, position: i64) {
        let state: tokio::sync::RwLockReadGuard<'_, MediaPlayerState> = self.state.read().await;
        if state.seek_enabled {
            let state_track_id = "/org/xosms/MediaPlayer2/Track/".to_owned() + &state.track_id;
            if state_track_id != track_id.as_str() {
                return;
            }

            let _ = self.callback_tx.try_send(PlatformBackendEvent::PositionChanged { position: position as f64 / 1_000_000.0 });
        }
    }

    // TODO: Implement this
    /*async fn open_uri(&self) {
        println!("Received open_uri");
    }*/

    #[zbus(signal)]
    async fn seeked(emitter: &SignalEmitter<'_>, position: i64) -> zbus::Result<()>;

    #[zbus(property)]
    async fn playback_status(&self) -> &str {
        let state = self.state.read().await;
        match state.playback_status {
            PlaybackStatus::Playing => "Playing",
            PlaybackStatus::Paused => "Paused",
            PlaybackStatus::Stopped => "Stopped"
        }
    }

    #[zbus(property)]
    async fn loop_status(&self) -> &str {
        let state = self.state.read().await;
        match state.r#loop {
            LoopType::None => "None",
            LoopType::Track => "Track",
            LoopType::Playlist => "Playlist"
        }
    }

    #[zbus(property)]
    async fn set_loop_status(&self, value: String) {
        let loop_type = match value.as_str() {
            "Track" => LoopType::Track,
            "Playlist" => LoopType::Playlist,
            _ => LoopType::None,
        };

        let _ = self.callback_tx.try_send(PlatformBackendEvent::LoopChanged { loop_type });
    }

    #[zbus(property)]
    async fn rate(&self) -> f64 {
        let state = self.state.read().await;
        state.playback_rate
    }

    #[zbus(property)]
    async fn set_rate(&self, value: f64) {
        let _ = self.callback_tx.try_send(PlatformBackendEvent::RateChanged { rate: value });
    }

    #[zbus(property)]
    async fn shuffle(&self) -> bool {
        let state = self.state.read().await;
        state.shuffle
    }

    #[zbus(property)]
    async fn set_shuffle(&self, value: bool) {
        let _ = self.callback_tx.try_send(PlatformBackendEvent::ShuffleChanged { shuffle: value });
    }

    #[zbus(property)]
    async fn metadata(&self) -> HashMap<String, Value<'_>> {
        let state = self.state.read().await;

        let track_id = if state.track_id.trim().is_empty() { "/org/mpris/MediaPlayer2/TrackList/NoTrack".to_string() } else { "/org/xosms/MediaPlayer2/Track/".to_owned() + &state.track_id };

        let mut metadata = HashMap::new();
        metadata.insert("mpris:trackid".to_owned(), Value::from(ObjectPath::try_from(track_id).unwrap()));
        metadata.insert("mpris:length".to_owned(), Value::from(self.timeline_state.duration.load(Ordering::Relaxed)));
        if let Some(thumbnail) = state.thumbnail.as_ref() {
            metadata.insert("mpris:artUrl".to_owned(), Value::from(thumbnail.platform_data.thumbnail.clone()));
        }
        metadata.insert("xesam:album".to_owned(), Value::from(state.album_title.clone()));
        metadata.insert("xesam:artist".to_owned(), Value::from(state.artist.clone()));
        metadata.insert("xesam:title".to_owned(), Value::from(state.title.clone()));

        metadata
    }

    #[zbus(property)]
    async fn volume(&self) -> f64 {
        let state = self.state.read().await;
        state.volume
    }

    #[zbus(property)]
    async fn set_volume(&self, value: f64) {
        let _ = self.callback_tx.try_send(PlatformBackendEvent::VolumeChanged { volume: value });
    }

    #[zbus(property)]
    async fn position(&self) -> i64 {
        self.timeline_state.position.load(Ordering::Relaxed)
    }

    #[zbus(property)]
    async fn minimum_rate(&self) -> f64 {
        let state = self.state.read().await;
        state.minimum_playback_rate
    }

    #[zbus(property)]
    async fn maximum_rate(&self) -> f64 {
        let state = self.state.read().await;
        state.maximum_playback_rate
    }

    #[zbus(property)]
    async fn can_go_next(&self) -> bool {
        let state = self.state.read().await;
        state.next_button_enabled
    }

    #[zbus(property)]
    async fn can_go_previous(&self) -> bool {
        let state = self.state.read().await;
        state.previous_button_enabled
    }

    #[zbus(property)]
    async fn can_play(&self) -> bool {
        let state = self.state.read().await;
        state.play_button_enabled
    }

    #[zbus(property)]
    async fn can_pause(&self) -> bool {
        let state = self.state.read().await;
        state.pause_button_enabled
    }

    #[zbus(property)]
    async fn can_seek(&self) -> bool {
        let state = self.state.read().await;
        state.seek_enabled
    }

    #[zbus(property)]
    async fn can_control(&self) -> bool {
        true
    }
}

pub struct PlatformThumbnailData {
    thumbnail: String
}

static PLATFORM_THUMBNAIL_NEXT_ID: AtomicUsize = AtomicUsize::new(1);
impl PlatformThumbnailBackend {
    pub async fn new(r#type: MediaPlayerThumbnailType, thumbnail: String) -> Result<Self, XosmsError> {
        let thumbnail = match r#type {
            MediaPlayerThumbnailType::File => {
                let mut thumbnail = thumbnail;
                if !thumbnail.starts_with("file://") {
                    thumbnail = format!("file://{thumbnail}");
                }

                thumbnail
            },
            _ => thumbnail
        };

        Ok(Self {
            id: PLATFORM_THUMBNAIL_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            r#type,
            platform_data: PlatformThumbnailData { thumbnail }
        })
    }
}