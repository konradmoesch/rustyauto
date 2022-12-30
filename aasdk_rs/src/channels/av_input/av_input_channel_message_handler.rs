use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::messenger::frame::Frame;

//TODO: clean up and refactor!

fn handle_av_channel_setup_request(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received setup request for av_input_channel");
    data.audio_input_service_data.write().unwrap().setup_status = SetupStatus::Requested;
}

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in av input service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::AVChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
    let message_id = AVMessageID::try_from(message_id_word);
    log::info!("Message ID: {:?}", message_id);
    match message_id {
        Ok(AVMessageID::SetupRequest) => {
            log::debug!("Setup request");
            handle_av_channel_setup_request(message, data);
            //handleAVChannelSetupRequest(payload, std:: move (eventHandler));
        }
        Ok(AVMessageID::AvInputOpenRequest) => {
            log::debug!("AV input open request");
            unimplemented!()
            //this -> handleAVInputOpenRequest(payload, std:: move (eventHandler));
        }
        Ok(AVMessageID::AvMediaAckIndication) => {
            log::debug!("AV media ack indication");
            unimplemented!();
            //this -> handleAVMediaAckIndication(payload, std:: move (eventHandler));
        }
        _ => {
            match message_id_word {
                7 => {
                    //crate::channels::control_service_channel::ControlMessageID::ChannelOpenRequest::to_u16() => {
                    log::debug!("Channel open request");
                    data.audio_input_service_data.write().unwrap().channel_status = ChannelStatus::OpenRequest;
                    //this -> handleChannelOpenRequest(payload, std:: move (eventHandler));
                }
                _ => log::error!("message not handled: {:?}", message_id)
            }
        }
    }
}