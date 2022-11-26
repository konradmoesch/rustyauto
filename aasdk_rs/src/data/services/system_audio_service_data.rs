use crate::data::audio_services::AudioConfig;
use crate::data::services::general::ServiceStatus;

pub struct SystemAudioServiceData {
    pub service_status: crate::data::services::general::ServiceStatus,
    pub config: crate::data::audio_services::AudioConfig,
}

impl SystemAudioServiceData {
    pub fn new() -> Self {
        SystemAudioServiceData {
            service_status: ServiceStatus::Uninitialized,
            config: AudioConfig {
                sample_rate: 16000,
                bit_depth: 16,
                channel_count: 1,
            },
        }
    }
}
