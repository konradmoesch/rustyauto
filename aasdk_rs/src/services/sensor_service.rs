use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;
use crate::services::service::Service;

pub struct SensorService {}

impl Service for SensorService {
    fn start(&self) {
        log::info!("Start");
    }

    fn stop(&self) {
        log::info!("Stop");
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
        let mut location_sensor = crate::protos::SensorData::Sensor::new();
        location_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::LOCATION);
        let mut night_data_sensor = crate::protos::SensorData::Sensor::new();
        night_data_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::NIGHT_DATA);
        sensor_channel.sensors.push(driving_status_sensor);
        sensor_channel.sensors.push(location_sensor);
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
