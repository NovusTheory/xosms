use std::sync::{Arc, atomic::AtomicI64};

use tokio::{sync::{RwLock, mpsc::Sender}, time::Instant};

use crate::{ButtonPressedType, LoopType, MediaPlayerState, MediaPlayerThumbnailType};

pub struct PlatformBackend {
    pub service_name: String,
    pub identity: String,
    pub state: Arc<RwLock<MediaPlayerState>>,
    pub timeline_state: Arc<MediaPlayerTimelineState>,
    pub last_timeline_state_update: Instant,
    pub callback_tx: Arc<Sender<PlatformBackendEvent>>,

    pub platform_data: super::backend::PlatformData
}

pub enum PlatformBackendEvent {
    ButtonPressed {
        button: ButtonPressedType
    },
    PositionChanged {
        position: f64
    },
    PositionSeeked {
        offset: f64
    },
    LoopChanged {
        loop_type: LoopType
    },
    RateChanged {
        rate: f64
    },
    ShuffleChanged {
        shuffle: bool
    },
    VolumeChanged {
        volume: f64
    }
}

#[derive(Default)]
pub struct MediaPlayerTimelineState {
    pub duration: AtomicI64,
    pub position: AtomicI64,
}

pub struct PlatformThumbnailBackend {
    pub id: usize,
    pub r#type: MediaPlayerThumbnailType,

    pub platform_data: super::backend::PlatformThumbnailData
}