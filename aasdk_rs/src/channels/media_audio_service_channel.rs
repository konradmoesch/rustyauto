use openssl_sys::epoll_event;
use protobuf::Message as protomsg;
use crate::channels::av_input_service_channel::AVMessageID;

use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger;
use crate::messenger::frame::{ChannelID, EncryptionType, FrameHeader, FrameType, Frame, MessageType};

fn handle_channel_open_request(message: &Frame) {
    log::info!("Received channel open request for media_audio_channel");
}

fn handle_av_channel_setup_request(message: &Frame) {
    log::info!("Received setup request for media_audio_channel");
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
                    handle_av_channel_setup_request(message);
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

pub fn create_channel_open_response_message() -> Frame {
    log::info!("Creating channel open response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Control,
        frame_type: FrameType::Bulk,
    };
    let mut channel_open_response = crate::protos::ChannelOpenResponseMessage::ChannelOpenResponse::new();
    channel_open_response.set_status(crate::protos::StatusEnum::status::Enum::OK);
    let mut payload = (crate::channels::control::message_ids::ControlMessageID::ChannelOpenResponse as u16).to_be_bytes().to_vec();
    let mut bytes = channel_open_response.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::MediaAudio, payload };
    message
}

pub fn create_av_channel_setup_response(channel_id: ChannelID) -> Frame {
    log::info!("Creating av channel setup response message for channel {:?}", channel_id);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut av_channel_setup_response = crate::protos::AVChannelSetupResponseMessage::AVChannelSetupResponse::new();
    av_channel_setup_response.set_media_status(crate::protos::AVChannelSetupStatusEnum::avchannel_setup_status::Enum::OK);
    av_channel_setup_response.set_max_unacked(1);
    av_channel_setup_response.configs.push(0);
    let mut payload = (AVMessageID::SetupResponse as u16).to_be_bytes().to_vec();
    let mut bytes = av_channel_setup_response.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::frame::Frame { frame_header, channel_id, payload };
    message
}
