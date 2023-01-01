use crate::data::services;
use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};

#[derive(Copy, Clone)]
pub struct InputServiceConfig {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TouchPosition(pub usize, pub usize);

#[derive(Copy, Clone, Debug)]
pub enum TouchActionType {
    Press = 0,
    Release = 1,
    Drag = 2,
    PointerDown = 5,
    PointerUp = 6,
}

pub struct InputServiceData {
    pub service_status: ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub binding_request: Option<crate::protos::BindingRequestMessage::BindingRequest>,
    pub config: InputServiceConfig,
    pub current_touch_position: Option<TouchPosition>,
    pub current_touch_action: Option<TouchActionType>,
}

impl InputServiceData {
    pub fn new() -> Self {
        InputServiceData {
            service_status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            binding_request: None,
            config: InputServiceConfig {},
            current_touch_position: None,
            current_touch_action: None,
        }
    }
}
