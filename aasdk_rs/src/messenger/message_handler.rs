use crate::channels;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger::frame::{ChannelID, Frame};

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::debug!("Channel ID: {:?}", message.channel_id);
    match message.channel_id {
        ChannelID::Control => { channels::control::control_channel_message_handler::handle_message(message, data) }
        ChannelID::AVInput => { channels::av_input::av_input_channel_message_handler::handle_message(message, data) }
        ChannelID::MediaAudio => { channels::media_audio::media_audio_channel_message_handler::handle_message(message, data) }
        ChannelID::SpeechAudio => { channels::speech_audio::speech_audio_channel_message_handler::handle_message(message, data) }
        ChannelID::SystemAudio => { channels::system_audio::system_audio_channel_message_handler::handle_message(message, data) }
        ChannelID::Sensor => { channels::sensor::sensor_channel_message_handler::handle_message(message, data) }
        ChannelID::Video => { channels::video::video_channel_message_handler::handle_message(message, data) }
        ChannelID::Input => { channels::input::input_channel_message_handler::handle_message(message, data) }
        _ => { todo!() }
    }
}