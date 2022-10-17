use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct SystemAudioService {}

impl Service for SystemAudioService {
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
        channel_descriptor.set_channel_id(crate::messenger::ChannelID::SystemAudio as u32);

        dbg!(channel_descriptor.clone());


        response.channels.push(channel_descriptor);
    }
}
