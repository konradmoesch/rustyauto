use crate::data::audio_services::AudioConfig;
use crate::data::services::general::ServiceStatus;

pub struct MediaAudioServiceData {
    pub service_status: crate::data::services::general::ServiceStatus,
    pub config: crate::data::audio_services::AudioConfig,
}

impl MediaAudioServiceData {
    pub fn new() -> Self {
        MediaAudioServiceData {
            service_status: ServiceStatus::Uninitialized,
            config: AudioConfig {
                sample_rate: 48000,
                bit_depth: 16,
                channel_count: 2,
            },
        }
    }
}