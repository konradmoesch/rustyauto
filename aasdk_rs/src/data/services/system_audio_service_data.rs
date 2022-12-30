use crate::data::audio_services::AudioConfig;
use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};

pub struct SystemAudioServiceData {
    pub service_status: ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub config: crate::data::audio_services::AudioConfig,
}

impl SystemAudioServiceData {
    pub fn new() -> Self {
        SystemAudioServiceData {
            service_status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            config: AudioConfig {
                sample_rate: 16000,
                bit_depth: 16,
                channel_count: 1,
            },
        }
    }
}
