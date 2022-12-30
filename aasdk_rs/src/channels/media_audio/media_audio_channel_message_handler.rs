use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::messenger::frame::{Frame, MessageType};

fn handle_channel_open_request(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received channel open request for media_audio_channel");
    data.media_audio_service_data.write().unwrap().channel_status = ChannelStatus::OpenRequest;
}

fn handle_av_channel_setup_request(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received setup request for media_audio_channel");
    data.media_audio_service_data.write().unwrap().setup_status = SetupStatus::Requested;
}

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in media audio service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = AVMessageID::try_from(message_id_word);
            match message_id {
                Ok(AVMessageID::SetupRequest) => {
                    log::debug!("Setup request");
                    handle_av_channel_setup_request(message, data);
                }
                _ => {
                    log::error!("Error: UnknownMessageID: {:?}", message_id);
                    unimplemented!()
                }
            }
        }
        MessageType::Control => {
            match crate::channels::control::message_ids::ControlMessageID::from(message_id_word as u8) {
                ControlMessageID::ChannelOpenRequest => {
                    handle_channel_open_request(message, data);
                }
                _ => {unimplemented!()}
            }
        }
    }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
}
