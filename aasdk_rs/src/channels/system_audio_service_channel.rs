use crate::messenger;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use protobuf::Message as protomsg;
use crate::channels::av_input_service_channel::AVMessageID;
use crate::channels::control_service_channel::ControlMessageID;

fn handle_channel_open_request(message: &Message) {
    log::info!("Received channel open request for system_audio_channel");
}

fn handle_av_channel_setup_request(message: &Message) {
    log::info!("Received setup request for system_audio_channel");
}

pub fn handle_message(message: &Message) {
    log::info!("Received message in system audio service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = AVMessageID::try_from(message_id_word);
            match message_id {
                Ok(AVMessageID::SetupRequest) => {
                    log::debug!("Setup request");
                    handle_av_channel_setup_request(message);
                }
                _ => {
                    log::error!("Error: UnknownMessageID: {:?}", message_id);
                    unimplemented!()
                }
            }
        }
        MessageType::Control => {
            match crate::channels::control_service_channel::ControlMessageID::from(message_id_word as u8) {
                ControlMessageID::ChannelOpenRequest => {
                    handle_channel_open_request(message);
                }
                _ => {unimplemented!()}
            }
        }
    }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
}

pub fn create_channel_open_response_message() -> Message {
    log::info!("Creating channel open response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Control,
        frame_type: FrameType::Bulk,
    };
    let mut channel_open_response = crate::protos::ChannelOpenResponseMessage::ChannelOpenResponse::new();
    channel_open_response.set_status(crate::protos::StatusEnum::status::Enum::OK);
    let mut payload = (crate::channels::control_service_channel::ControlMessageID::ChannelOpenResponse as u16).to_be_bytes().to_vec();
    let mut bytes = channel_open_response.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::SystemAudio, payload };
    message
}
