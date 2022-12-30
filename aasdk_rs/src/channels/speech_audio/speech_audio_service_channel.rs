use std::sync::mpsc::Sender;
use protobuf::Message as protomsg;

use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::{channels, messenger};
use crate::messenger::frame::{ChannelID, EncryptionType, Frame, FrameHeader, FrameType, MessageType};
use crate::messenger::messenger::{Messenger, ReceivalRequest};

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    if data.speech_audio_service_data.read().unwrap().channel_status == ChannelStatus::OpenRequest {
        let mut  message = create_channel_open_response_message();
        messenger.cryptor.encrypt_message(&mut message);
        messenger.send_message_via_usb(message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.speech_audio_service_data.write().unwrap().channel_status = ChannelStatus::Open;
    }
    if data.speech_audio_service_data.read().unwrap().setup_status == SetupStatus::Requested {
        let mut setup_response_message = channels::general_audio::create_av_channel_setup_response(ChannelID::SpeechAudio);
        messenger.cryptor.encrypt_message(&mut setup_response_message);
        messenger.send_message_via_usb(setup_response_message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.speech_audio_service_data.write().unwrap().setup_status = SetupStatus::Finished;
    }
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::SpeechAudio, payload };
    message
}
