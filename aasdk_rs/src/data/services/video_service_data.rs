use crate::data::services::general::ServiceStatus;

pub enum VideoResolution {
    _480p,
    _720p,
    _1080p,
}

pub enum VideoFPS {
    _30,
    _60,
}

pub struct VideoServiceConfig {
    video_resolution: VideoResolution,
    video_fps: VideoFPS,
    margin_height: usize,
    margin_width: usize,
    dpi: usize,
}

pub struct VideoServiceData {
    status: crate::data::services::general::ServiceStatus,
    config: VideoServiceConfig,
}

impl VideoServiceData {
    pub fn new() -> Self {
        VideoServiceData {
            status: ServiceStatus::Uninitialized,
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
