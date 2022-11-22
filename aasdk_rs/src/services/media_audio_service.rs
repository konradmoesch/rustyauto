use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct MediaAudioService {}

impl Service for MediaAudioService {
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
        channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::MediaAudio as u32);

        let mut audio_channel = crate::protos::AVChannelData::AVChannel::default();
        audio_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
        audio_channel.set_audio_type(crate::protos::AudioTypeEnum::audio_type::Enum::MEDIA);

        audio_channel.set_available_while_in_call(true);

        let mut audio_config = crate::protos::AudioConfigData::AudioConfig::default();
        audio_config.set_sample_rate(48000);
        //audio_config.set_sample_rate(48);
        audio_config.set_bit_depth(16);
        audio_config.set_channel_count(2);

        audio_channel.audio_configs.push(audio_config);

        channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(audio_channel));

        dbg!(channel_descriptor.clone());
        use protobuf::Message as msg;
        println!("MEDIA_AUDIO:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }
}
