use crate::data::services;
use crate::data::services::general::ServiceStatus;

pub struct InputServiceConfig {}

pub struct InputServiceData {
    pub service_status: crate::data::services::general::ServiceStatus,
}

impl InputServiceData {
    pub fn new() -> Self {
        InputServiceData {
            service_status: ServiceStatus::Uninitialized,
        }
    }
}
