use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct VideoService {}

impl Service for VideoService {
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

        let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
        channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Video as u32);
        //TODO: init video output, use real values
        let mut video_channel = crate::protos::AVChannelData::AVChannel::default();
        video_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::VIDEO);
        video_channel.set_available_while_in_call(true);
        let mut video_config = crate::protos::VideoConfigData::VideoConfig::default();
        video_config.set_video_resolution(crate::protos::VideoResolutionEnum::video_resolution::Enum::_480p);
        video_config.set_video_fps(crate::protos::VideoFPSEnum::video_fps::Enum::_30);
        video_config.set_margin_height(0);
        video_config.set_margin_width(0);
        video_config.set_dpi(140);
        video_channel.video_configs.push(video_config);

        channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(video_channel));

        dbg!(channel_descriptor.clone());
        use protobuf::Message as msg;
        println!("VIDEO:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }
}
