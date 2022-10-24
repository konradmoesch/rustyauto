use crate::messenger;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use protobuf::Message as protomsg;

pub fn handle_message(message: &Message) {
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
            unimplemented!()
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
                    //this -> handleChannelOpenRequest(payload, std:: move (eventHandler));
                }
                _ => log::error!("message not handled: {:?}", message_id)
            }
        }
    }
}

pub fn create_channel_open_response_message() -> Message {
    log::info!("Creating audio focus response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Control,
        frame_type: FrameType::Bulk,
    };
    let mut channel_open_response = crate::protos::ChannelOpenResponseMessage::ChannelOpenResponse::new();
    channel_open_response.set_status(crate::protos::StatusEnum::status::Enum::OK);
    let mut payload = (crate::channels::control_service_channel::ControlMessageID::ChannelOpenResponse as u16).to_be_bytes().to_vec();
    let mut bytes = channel_open_response.write_to_bytes().unwrap();
    println!("{:x?}", bytes);
    payload.extend(bytes);
    println!("{:x?}", payload);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::AVInput, payload };
    message
}

#[derive(Debug)]
pub enum AVMessageID
{
    AvMediaWithTimestampIndication = 0x0000,
    AvMediaIndication = 0x0001,
    SetupRequest = 0x8000,
    StartIndication = 0x8001,
    StopIndication = 0x8002,
    SetupResponse = 0x8003,
    AvMediaAckIndication = 0x8004,
    AvInputOpenRequest = 0x8005,
    AvInputOpenResponse = 0x8006,
    VideoFocusRequest = 0x8007,
    VideoFocusIndication = 0x8008,
}

impl TryFrom<u16> for AVMessageID {
    type Error = ();

    fn try_from(message_id_as_byte: u16) -> Result<Self, ()> {
        match message_id_as_byte {
            0x0000 => { Ok(AVMessageID::AvMediaWithTimestampIndication) }
            0x0001 => { Ok(AVMessageID::AvMediaIndication) }
            0x8000 => { Ok(AVMessageID::SetupRequest) }
            0x8001 => { Ok(AVMessageID::StartIndication) }
            0x8002 => { Ok(AVMessageID::StopIndication) }
            0x8003 => { Ok(AVMessageID::SetupResponse) }
            0x8004 => { Ok(AVMessageID::AvMediaAckIndication) }
            0x8005 => { Ok(AVMessageID::AvInputOpenRequest) }
            0x8006 => { Ok(AVMessageID::AvInputOpenResponse) }
            0x8007 => { Ok(AVMessageID::VideoFocusRequest) }
            _ => {
                log::error!("Unknown value");
                Err(())
                //todo!()
            }
        }
    }
}
