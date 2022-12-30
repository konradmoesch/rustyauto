use crate::data::services::general::{ChannelStatus, ServiceStatus, SetupStatus};
use crate::protos::SensorTypeEnum::SensorType;

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
    pub service_status: ServiceStatus,
    pub channel_status: ChannelStatus,
    pub setup_status: SetupStatus,
    pub requested_sensor_type: Option<crate::protos::SensorTypeEnum::sensor_type::Enum>,
    pub config: SensorServiceConfig,
    pub night_sensor: NightSensorStatus,
    //pub gps_sensor: GPSSensorData,
}

impl SensorServiceData {
    pub fn new() -> Self {
        SensorServiceData {
            service_status: ServiceStatus::Uninitialized,
            channel_status: ChannelStatus::Closed,
            setup_status: SetupStatus::NotStarted,
            requested_sensor_type: None,
            config: SensorServiceConfig { location_sensor_present: false },
            night_sensor: NightSensorStatus::Day,
        }
    }
}
