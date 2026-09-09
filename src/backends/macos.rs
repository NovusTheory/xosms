use std::{ptr::NonNull, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use block2::RcBlock;
use objc2::{AnyThread, rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::NSImage;
use objc2_foundation::{NSMutableCopying, NSMutableDictionary, NSNumber, NSSize, NSString, NSURL};
use objc2_media_player::{MPChangePlaybackPositionCommandEvent, MPChangePlaybackRateCommandEvent, MPChangeRepeatModeCommandEvent, MPChangeShuffleModeCommandEvent, MPMediaItemArtwork, MPMediaItemPropertyAlbumTitle, MPMediaItemPropertyArtist, MPMediaItemPropertyArtwork, MPMediaItemPropertyPlaybackDuration, MPMediaItemPropertyTitle, MPNowPlayingInfoCenter, MPNowPlayingInfoPropertyElapsedPlaybackTime, MPNowPlayingInfoPropertyPlaybackRate, MPNowPlayingPlaybackState, MPRemoteCommandCenter, MPRemoteCommandEvent, MPRemoteCommandHandlerStatus, MPRepeatType, MPShuffleType};
use tokio::{sync::{RwLock, mpsc::Sender}, time::Instant};

use crate::{ButtonPressedType, LoopType, MediaPlayerState, MediaPlayerThumbnailType, PlaybackStatus, backends::{PlatformBackend, PlatformThumbnailBackend, common::{MediaPlayerTimelineState, PlatformBackendEvent}, error::XosmsError}};

#[derive(Default)]
pub struct PlatformData;

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
        unsafe {
            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            let callback_tx = self.callback_tx.clone();
            let play_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Play });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.playCommand().addTargetWithHandler(&play_handler));
            
            let callback_tx = self.callback_tx.clone();
            let pause_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Pause });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.pauseCommand().addTargetWithHandler(&pause_handler));

            let callback_tx = self.callback_tx.clone();
            let play_pause_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::PlayPause });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.togglePlayPauseCommand().addTargetWithHandler(&play_pause_handler));
            
            let callback_tx = self.callback_tx.clone();
            let stop_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Stop });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.stopCommand().addTargetWithHandler(&stop_handler));
            
            let callback_tx = self.callback_tx.clone();
            let next_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Next });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.nextTrackCommand().addTargetWithHandler(&next_handler));
            
            let callback_tx = self.callback_tx.clone();
            let previous_handler = RcBlock::new(move |_event: NonNull<MPRemoteCommandEvent>| {
                let _ = callback_tx.try_send(PlatformBackendEvent::ButtonPressed { button: ButtonPressedType::Previous });
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.previousTrackCommand().addTargetWithHandler(&previous_handler));

            let callback_tx = self.callback_tx.clone();
            let playback_position_handler = RcBlock::new(move |event: NonNull<MPRemoteCommandEvent>| {
                if let Some(event) = event.as_ref().downcast_ref::<MPChangePlaybackPositionCommandEvent>() {
                    let _ = callback_tx.try_send(PlatformBackendEvent::PositionChanged { position: event.positionTime() });
                }
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.changePlaybackPositionCommand().addTargetWithHandler(&playback_position_handler));

            let callback_tx = self.callback_tx.clone();
            let playback_rate_handler = RcBlock::new(move |event: NonNull<MPRemoteCommandEvent>| {
                if let Some(event) = event.as_ref().downcast_ref::<MPChangePlaybackRateCommandEvent>() {
                    let _ = callback_tx.try_send(PlatformBackendEvent::RateChanged { rate: event.playbackRate() as f64 });
                }
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.changePlaybackRateCommand().addTargetWithHandler(&playback_rate_handler));

            let callback_tx = self.callback_tx.clone();
            let repeat_mode_handler = RcBlock::new(move |event: NonNull<MPRemoteCommandEvent>| {
                if let Some(event) = event.as_ref().downcast_ref::<MPChangeRepeatModeCommandEvent>() {
                    let _ = callback_tx.try_send(PlatformBackendEvent::LoopChanged { loop_type: match event.repeatType() {
                        MPRepeatType::Off => LoopType::None,
                        MPRepeatType::All => LoopType::Playlist,
                        MPRepeatType::One => LoopType::Track,
                        _ => LoopType::None
                    } });
                }
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.changeRepeatModeCommand().addTargetWithHandler(&repeat_mode_handler));

            let callback_tx = self.callback_tx.clone();
            let shuffle_mode_handler = RcBlock::new(move |event: NonNull<MPRemoteCommandEvent>| {
                if let Some(event) = event.as_ref().downcast_ref::<MPChangeShuffleModeCommandEvent>() {
                    let _ = callback_tx.try_send(PlatformBackendEvent::ShuffleChanged { shuffle: match event.shuffleType() {
                        MPShuffleType::Off => false,
                        MPShuffleType::Collections => true,
                        MPShuffleType::Items => false,
                        _ => false
                    } });
                }
                MPRemoteCommandHandlerStatus::Success
            });
            Some(remote_command_center.changeShuffleModeCommand().addTargetWithHandler(&shuffle_mode_handler));
        }

        Ok(())
    }

    pub async fn deactivate(&mut self) -> Result<(), XosmsError> {
        unsafe {
            let now_playing: Retained<MPNowPlayingInfoCenter> = MPNowPlayingInfoCenter::defaultCenter();
            now_playing.setNowPlayingInfo(None);

            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            remote_command_center.playCommand().removeTarget(None);
            remote_command_center.pauseCommand().removeTarget(None);
            remote_command_center.togglePlayPauseCommand().removeTarget(None);
            remote_command_center.stopCommand().removeTarget(None);
            remote_command_center.nextTrackCommand().removeTarget(None);
            remote_command_center.previousTrackCommand().removeTarget(None);
            remote_command_center.changePlaybackPositionCommand().removeTarget(None);
            remote_command_center.changePlaybackRateCommand().removeTarget(None);
            remote_command_center.changeRepeatModeCommand().removeTarget(None);
            remote_command_center.changeShuffleModeCommand().removeTarget(None);
        }

        Ok(())
    }

    pub async fn update(&self, state: MediaPlayerState) -> Result<(), XosmsError> {
        unsafe {
            let now_playing = MPNowPlayingInfoCenter::defaultCenter();
            now_playing.setPlaybackState(match state.playback_status {
                PlaybackStatus::Playing => MPNowPlayingPlaybackState::Playing,
                PlaybackStatus::Paused => MPNowPlayingPlaybackState::Paused,
                PlaybackStatus::Stopped => MPNowPlayingPlaybackState::Stopped,
            });
            let info = match now_playing.nowPlayingInfo() {
                Some(info) => info.mutableCopy(),
                None => NSMutableDictionary::new(),
            };

            if let Some(thumbnail) = state.thumbnail {
                let image = match thumbnail.r#type {
                    MediaPlayerThumbnailType::File => {
                        let ns_thumbnail = NSString::from_str(&thumbnail.platform_data.thumbnail);
                        NSImage::initWithContentsOfFile(NSImage::alloc(), &ns_thumbnail)
                    },
                    MediaPlayerThumbnailType::Uri => {
                        let ns_thumbnail = NSString::from_str(&thumbnail.platform_data.thumbnail);
                        match NSURL::initWithString(NSURL::alloc(), &ns_thumbnail) {
                            Some(ns_url) => NSImage::initWithContentsOfURL(NSImage::alloc(), &ns_url),
                            None => None,
                        }
                    },
                };

                if let Some(image) = image {
                    let image_size = image.size();
                    let handler = RcBlock::new(move |_size: NSSize| -> NonNull<NSImage> {
                        NonNull::from_ref(&*image)
                    });
                    let artwork = MPMediaItemArtwork::initWithBoundsSize_requestHandler(MPMediaItemArtwork::alloc(), image_size, &handler);

                    info.setObject_forKey(&artwork, ProtocolObject::from_ref(MPMediaItemPropertyArtwork));
                }
            }


            info.setObject_forKey(&NSString::from_str(&state.title), ProtocolObject::from_ref(MPMediaItemPropertyTitle));
            info.setObject_forKey(&NSString::from_str(&state.album_title), ProtocolObject::from_ref(MPMediaItemPropertyAlbumTitle));
            info.setObject_forKey(&NSString::from_str(&state.artist.join(", ")), ProtocolObject::from_ref(MPMediaItemPropertyArtist));
            info.setObject_forKey(&NSNumber::initWithDouble(NSNumber::alloc(), state.playback_rate), ProtocolObject::from_ref(MPNowPlayingInfoPropertyPlaybackRate));

            now_playing.setNowPlayingInfo(Some(&info));

            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            remote_command_center.playCommand().setEnabled(state.play_button_enabled);
            remote_command_center.pauseCommand().setEnabled(state.pause_button_enabled);
            remote_command_center.stopCommand().setEnabled(state.stop_button_enabled);
            remote_command_center.nextTrackCommand().setEnabled(state.next_button_enabled);
            remote_command_center.previousTrackCommand().setEnabled(state.previous_button_enabled);
            remote_command_center.changePlaybackPositionCommand().setEnabled(true);
            remote_command_center.changePlaybackRateCommand().setEnabled(true);
            remote_command_center.changeRepeatModeCommand().setEnabled(true);
            remote_command_center.changeShuffleModeCommand().setEnabled(true);

            remote_command_center.changeRepeatModeCommand().setCurrentRepeatType(match state.r#loop {
                LoopType::None => MPRepeatType::Off,
                LoopType::Track => MPRepeatType::One,
                LoopType::Playlist => MPRepeatType::All,
            });
            remote_command_center.changeShuffleModeCommand().setCurrentShuffleType(match state.shuffle {
                true => MPShuffleType::Collections,
                false => MPShuffleType::Items,
            });
        };

        Ok(())
    }

    pub async fn set_timeline(&mut self, duration: f64, position: f64) -> Result<(), XosmsError> {
        unsafe {
            let now_playing = MPNowPlayingInfoCenter::defaultCenter();
            let info = match now_playing.nowPlayingInfo() {
                Some(info) => info.mutableCopy(),
                None => NSMutableDictionary::new(),
            };

            info.setObject_forKey(&NSNumber::initWithDouble(NSNumber::alloc(), duration), ProtocolObject::from_ref(MPMediaItemPropertyPlaybackDuration));
            info.setObject_forKey(&NSNumber::initWithDouble(NSNumber::alloc(), position), ProtocolObject::from_ref(MPNowPlayingInfoPropertyElapsedPlaybackTime));

            now_playing.setNowPlayingInfo(Some(&info));
        }

        Ok(())
    }
}

impl Drop for PlatformBackend {
    fn drop(&mut self) {
        unsafe {
            let now_playing: Retained<MPNowPlayingInfoCenter> = MPNowPlayingInfoCenter::defaultCenter();
            now_playing.setNowPlayingInfo(None);

            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            remote_command_center.playCommand().removeTarget(None);
            remote_command_center.pauseCommand().removeTarget(None);
            remote_command_center.togglePlayPauseCommand().removeTarget(None);
            remote_command_center.stopCommand().removeTarget(None);
            remote_command_center.nextTrackCommand().removeTarget(None);
            remote_command_center.previousTrackCommand().removeTarget(None);
            remote_command_center.changePlaybackPositionCommand().removeTarget(None);
            remote_command_center.changePlaybackRateCommand().removeTarget(None);
            remote_command_center.changeRepeatModeCommand().removeTarget(None);
            remote_command_center.changeShuffleModeCommand().removeTarget(None);
        }
    }
}

pub struct PlatformThumbnailData {
    thumbnail: String
}

static PLATFORM_THUMBNAIL_NEXT_ID: AtomicUsize = AtomicUsize::new(1);
impl PlatformThumbnailBackend {
    pub async fn new(r#type: MediaPlayerThumbnailType, thumbnail: String) -> Result<Self, XosmsError> {
        Ok(Self {
            id: PLATFORM_THUMBNAIL_NEXT_ID.fetch_add(1, Ordering::SeqCst),
            r#type,
            platform_data: PlatformThumbnailData { thumbnail }
        })
    }
}