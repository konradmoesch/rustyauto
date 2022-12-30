use crate::data::services;
use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};

#[derive(Copy, Clone)]
pub struct InputServiceConfig {}

pub struct InputServiceData {
    pub service_status: ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub binding_request: Option<crate::protos::BindingRequestMessage::BindingRequest>,
    pub config: InputServiceConfig,
}

impl InputServiceData {
    pub fn new() -> Self {
        InputServiceData {
            service_status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            binding_request: None,
            config: InputServiceConfig {},
        }
    }
}
