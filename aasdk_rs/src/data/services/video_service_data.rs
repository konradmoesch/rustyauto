use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};

#[derive(Copy, Clone)]
pub enum VideoResolution {
    _480p,
    _720p,
    _1080p,
}

#[derive(Copy, Clone)]
pub enum VideoFPS {
    _30,
    _60,
}

#[derive(Copy, Clone)]
pub struct VideoServiceConfig {
    pub video_resolution: VideoResolution,
    pub video_fps: VideoFPS,
    pub margin_height: usize,
    pub margin_width: usize,
    pub dpi: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum VideoIndicationType {
    StartIndication,
    VideoIndication,
    VideoIndicationWithTimestamp,
}

pub struct VideoServiceData {
    pub status: crate::data::services::general::ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub received_indication: Option<VideoIndicationType>,
    pub config: VideoServiceConfig,
}

impl VideoServiceData {
    pub fn new() -> Self {
        VideoServiceData {
            status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            received_indication: None,
            config: VideoServiceConfig {
                video_resolution: VideoResolution::_480p,
                video_fps: VideoFPS::_30,
                margin_height: 0,
                margin_width: 0,
                dpi: 140,
            },
        }
    }
}
