use crate::protos::NightModeData::NightMode;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;
use crate::services::service::{Service, ServiceStatus};
use crate::services::service::ServiceStatus::{Initialized, Uninitialized};

pub struct SensorService {
    pub(crate) service_status: ServiceStatus,
    pub(crate) config: SensorServiceConfig,
    pub(crate) night_sensor_value: NightStatus,
}

pub enum NightStatus {
    Night,
    Day,
}

pub struct SensorServiceConfig {
    pub(crate) location_sensor_present: bool,
}

impl Service for SensorService {
    fn start(&mut self) {
        log::info!("Start");
        //todo: set this via config
        self.config = SensorServiceConfig { location_sensor_present: true };
        self.night_sensor_value = NightStatus::Night;
        self.service_status = Initialized;
    }

    fn stop(&mut self) {
        log::info!("Stop");
        self.service_status = Uninitialized;
    }

    fn pause(&self) {
        log::info!("Pause");
    }

    fn resume(&self) {
        log::info!("Resume");
    }

    fn fill_features(&self, response: &mut ServiceDiscoveryResponse) {
        log::info!("Fill Features");

        let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
        channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Sensor as u32);

        let mut sensor_channel = crate::protos::SensorChannelData::SensorChannel::default();
        let mut driving_status_sensor = crate::protos::SensorData::Sensor::new();
        driving_status_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::DRIVING_STATUS);
        let mut night_data_sensor = crate::protos::SensorData::Sensor::new();
        night_data_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::NIGHT_DATA);
        sensor_channel.sensors.push(driving_status_sensor);
        if self.config.location_sensor_present {
            let mut location_sensor = crate::protos::SensorData::Sensor::new();
            location_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::LOCATION);
            sensor_channel.sensors.push(location_sensor);
        }
        sensor_channel.sensors.push(night_data_sensor);
        //TODO: better first get() the sensorChannel, if possible?

        channel_descriptor.sensor_channel = protobuf::MessageField::from_option(Some(sensor_channel));

        dbg!(channel_descriptor.clone());

        use protobuf::Message as msg;
        println!("SENSOR:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }
}
