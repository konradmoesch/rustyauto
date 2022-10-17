use crate::messenger;
use crate::messenger::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use protobuf::Message as protomsg;

fn handle_channel_open_request(message: &Message) {
    log::info!("Received channel open request for speech_audio_channel");
}

pub fn handle_message(message: &Message) {
    log::info!("Received message in speech audio service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    if message_id_word == crate::channels::control_service_channel::ControlMessageID::ChannelOpenRequest.into() {
        handle_channel_open_request(message);
    } else { unimplemented!() }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
    //let message_id = AVMessageID::try_from(message_id_word);
    //log::info!("Message ID: {:?}", message_id);
    /*match message_id {
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
    }*/
}

pub fn create_channel_open_response_message(channel_open_response_message: crate::protos::ChannelOpenResponseMessage::ChannelOpenResponse) -> Message {
    log::info!("Creating audio focus response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (crate::channels::control_service_channel::ControlMessageID::ChannelOpenRequest as u16).to_be_bytes().to_vec();
    let mut bytes = channel_open_response_message.write_to_bytes().unwrap();
    println!("{:x?}", bytes);
    payload.extend(bytes);
    println!("{:x?}", payload);
    let message = messenger::Message { frame_header, channel_id: ChannelID::AVInput, payload };
    message
}
