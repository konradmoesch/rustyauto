use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct SystemAudioServiceData {}

impl Service for SystemAudioServiceData {
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
        channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::SystemAudio as u32);

        let mut audio_channel = crate::protos::AVChannelData::AVChannel::default();
        audio_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
        audio_channel.set_audio_type(crate::protos::AudioTypeEnum::audio_type::Enum::SYSTEM);

        audio_channel.set_available_while_in_call(true);

        let mut audio_config = crate::protos::AudioConfigData::AudioConfig::default();
        audio_config.set_sample_rate(16000);
        //audio_config.set_sample_rate(16);
        audio_config.set_bit_depth(16);
        audio_config.set_channel_count(1);

        audio_channel.audio_configs.push(audio_config);

        channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(audio_channel));

        dbg!(channel_descriptor.clone());
        use protobuf::Message as msg;
        println!("SYSTEM_AUDIO:");
        let str = channel_descriptor.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();

        response.channels.push(channel_descriptor);
    }

    fn run(&mut self, data: &mut AndroidAutoEntityData) {
        log::debug!("Running SystemAudioService");
    }
}
