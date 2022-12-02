use log::log;
use openssl_sys::bind;
use protobuf::Message as protomsg;

use crate::channels::av_input_service_channel::AVMessageID;
use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};

fn handle_channel_open_request(message: &Message) {
    log::info!("Received channel open request for input_channel");
}

pub fn handle_message(message: &Message, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in input service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = InputMessageID::try_from(message_id_word);
            match message_id {
                Ok(InputMessageID::BindingRequest) => {
                    log::info!("Binding request for input service channel received");
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
                _ => { unimplemented!() }
            }
        }
    }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
}

pub fn create_channel_open_response_message() -> Message {
    log::info!("Creating channel open response message for input channel");
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
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Input, payload };
    message
}

pub fn create_binding_response_message(binding_request_message: Message) -> Message {
    let payload = binding_request_message.payload.as_slice().clone();
    log::debug!("{:?}", payload.to_vec());
    let binding_request = crate::protos::BindingRequestMessage::BindingRequest::parse_from_bytes(&payload[2..]).unwrap();
    let mut status = crate::protos::StatusEnum::status::Enum::OK;
    log::info!("Received binding request: {:?}, scan codes count: {}", binding_request, binding_request.scan_codes.len());

    let mut binding_response = crate::protos::BindingResponseMessage::BindingResponse::new();
    binding_response.set_status(status);
    //TODO: check all scan codes (for being supported)
    //TODO: impl & start input device
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (InputMessageID::BindingResponse as u16).to_be_bytes().to_vec();
    payload.extend(binding_response.write_to_bytes().unwrap());
    crate::messenger::message::Message { frame_header, channel_id: ChannelID::Input, payload }
}

#[derive(Debug)]
pub enum InputMessageID
{
    None = 0x0000,
    InputEventIndication = 0x8001,
    BindingRequest = 0x8002,
    BindingResponse = 0x8003,
}

impl TryFrom<u16> for InputMessageID {
    type Error = ();

    fn try_from(message_id_as_byte: u16) -> Result<Self, ()> {
        match message_id_as_byte {
            0x0000 => { Ok(InputMessageID::None) }
            0x8001 => { Ok(InputMessageID::InputEventIndication) }
            0x8002 => { Ok(InputMessageID::BindingRequest) }
            0x8003 => { Ok(InputMessageID::BindingResponse) }
            _ => {
                log::error!("Unknown value");
                Err(())
                //todo!()
            }
        }
    }
}
