use crate::data::services::general::ServiceStatus;

#[derive(Debug)]
pub enum NightSensorStatus {
    Night,
    Day,
}

#[derive(Copy, Clone)]
pub struct SensorServiceConfig {
    pub location_sensor_present: bool,
}

pub struct SensorServiceData {
    pub service_status: crate::data::services::general::ServiceStatus,
    pub config: SensorServiceConfig,
    pub night_sensor: NightSensorStatus,
    //pub gps_sensor: GPSSensorData,
}

impl SensorServiceData {
    pub fn new() -> Self {
        SensorServiceData {
            service_status: ServiceStatus::Uninitialized,
            config: SensorServiceConfig { location_sensor_present: false },
            night_sensor: NightSensorStatus::Day,
        }
    }
}
