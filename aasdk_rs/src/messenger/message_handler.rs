use crate::channels;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger::frame::{ChannelID, Frame};

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::debug!("Channel ID: {:?}", message.channel_id);
    match message.channel_id {
        ChannelID::Control => { channels::control::control_channel_message_handler::handle_message(message, data) }
        ChannelID::AVInput => { channels::av_input_service_channel::handle_message(message, data) }
        ChannelID::MediaAudio => { channels::media_audio_service_channel::handle_message(message, data) }
        ChannelID::SpeechAudio => { channels::speech_audio_service_channel::handle_message(message, data) }
        ChannelID::SystemAudio => { channels::system_audio_service_channel::handle_message(message, data) }
        ChannelID::Sensor => { channels::sensor_service_channel::handle_message(message, data) }
        ChannelID::Video => { channels::video_service_channel::handle_message(message, data) }
        ChannelID::Input => { channels::input_service_channel::handle_message(message, data) }
        _ => { todo!() }
    }
}