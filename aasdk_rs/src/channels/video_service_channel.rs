use crate::messenger;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use protobuf::Message as protomsg;
use crate::channels::av_input_service_channel::AVMessageID;
use crate::channels::control_service_channel::ControlMessageID;
use crate::cryptor::Cryptor;
use crate::usbdriver::UsbDriver;

fn handle_channel_open_request(message: &Message) {
    log::info!("Received channel open request for video_channel");
}

pub fn handle_message(message: &Message) {
    log::info!("Received message in video service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = AVMessageID::try_from(message_id_word);
            match message_id {
                Ok(AVMessageID::SetupRequest) => {
                    log::info!("Received setup request for video service");
                }
                Ok(AVMessageID::StartIndication) => {
                    log::info!("Received start indication for video service");
                }
                Ok(AVMessageID::AvMediaIndication) => {
                    log::info!("Received AV Media Indication");
                    log::debug!("Indication content: {:?}", payload.as_slice());
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
                _ => { unimplemented!() }
            }
        }
    }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
}

pub fn create_channel_open_response_message() -> Message {
    log::info!("Creating channel open response message for video channel");
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
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Video, payload };
    message
}

pub fn create_av_channel_setup_response(video_setup_request: Message) -> Message {
    let payload = video_setup_request.payload.as_slice().clone();
    let video_channel_setup_request = crate::protos::AVChannelSetupRequestMessage::AVChannelSetupRequest::parse_from_bytes(&payload[2..]).unwrap();
    log::info!("Creating av channel setup response message for video service channel, config: {}", video_channel_setup_request.config_index());
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
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Video, payload };
    message
}

pub fn create_video_focus_indication() -> Message {
    log::info!("Creating video focus indication");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut video_focus_indication = crate::protos::VideoFocusIndicationMessage::VideoFocusIndication::new();
    video_focus_indication.set_focus_mode(crate::protos::VideoFocusModeEnum::video_focus_mode::Enum::FOCUSED);
    video_focus_indication.set_unrequested(false);
    let mut payload = (AVMessageID::VideoFocusIndication as u16).to_be_bytes().to_vec();
    let mut bytes = video_focus_indication.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Video, payload };
    message
}
