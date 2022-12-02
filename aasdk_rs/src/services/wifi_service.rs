use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct WifiService {}

impl Service for WifiService {
    fn start(&mut self) {
        log::info!("Start");
    }

    fn stop(&mut self) {
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

        let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::new();
        channel_descriptor.set_channel_id(14);

        let mut wifi_channel = crate::protos::WifiChannelData::WifiChannel::new();
        wifi_channel.set_ssid("".to_string());

        channel_descriptor.wifi_channel = protobuf::MessageField::from_option(Some(wifi_channel));

        dbg!(channel_descriptor.clone());

        use protobuf::Message as msg;
        println!("WIFI:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }

    fn run(&mut self, data: &mut AndroidAutoEntityData) {
        todo!()
    }
}
