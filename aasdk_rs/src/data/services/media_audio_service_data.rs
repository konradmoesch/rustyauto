use crate::data::audio_services::AudioConfig;
use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};

pub struct MediaAudioServiceData {
    pub service_status: ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub config: crate::data::audio_services::AudioConfig,
}

impl MediaAudioServiceData {
    pub fn new() -> Self {
        MediaAudioServiceData {
            service_status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            config: AudioConfig {
                sample_rate: 48000,
                bit_depth: 16,
                channel_count: 2,
            },
        }
    }
}