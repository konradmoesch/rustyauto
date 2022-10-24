use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct InputService {}

impl Service for InputService {
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
        channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Input as u32);

        //TODO: Initialize input and use real values
        //TODO: fix the missing FFFFFF in touch config fields

        let mut input_channel = crate::protos::InputChannelData::InputChannel::default();
        let mut touch_screen_config = crate::protos::TouchConfigData::TouchConfig::default();
        touch_screen_config.set_width(1920);
        //touch_screen_config.set_width(19);
        touch_screen_config.set_height(1080);
        //touch_screen_config.set_height(10);

        input_channel.touch_screen_config = protobuf::MessageField::from_option(Some(touch_screen_config));
        channel_descriptor.input_channel = protobuf::MessageField::from_option(Some(input_channel));

        dbg!(channel_descriptor.clone());
        use protobuf::Message as msg;
        println!("INPUT:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }
}
