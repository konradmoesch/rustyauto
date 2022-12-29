use crate::data::services::general::ServiceStatus;
use crate::data::services::service_data::{ServiceData, ServiceType};

#[derive(Clone, PartialEq)]
pub enum AudioFocusState {
    Lost,
    Requested,
    Gained,
}

#[derive(Clone, PartialEq)]
pub enum ServiceDiscoveryState {
    Idle,
    Requested,
}

#[derive(Clone)]
pub struct ControlServiceData {
    pub audio_focus_state: AudioFocusState,
    pub service_discovery_state: ServiceDiscoveryState,
}

impl ControlServiceData {
    pub fn new() -> Self {
        ControlServiceData {
            audio_focus_state: AudioFocusState::Lost,
            service_discovery_state: ServiceDiscoveryState::Idle,
        }
    }
}