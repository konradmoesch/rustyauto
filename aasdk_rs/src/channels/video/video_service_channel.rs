use std::sync::mpsc::Sender;

use protobuf::Message as protomsg;

use crate::{channels, messenger};
use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::channels::control::message_ids::ControlMessageID;
use crate::cryptor::Cryptor;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::data::services::video_service_data::VideoIndicationType;
use crate::messenger::frame::{ChannelID, EncryptionType, Frame, FrameHeader, FrameType, MessageType};
use crate::messenger::messenger::{Messenger, ReceivalRequest};
use crate::usbdriver::UsbDriver;

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    if data.video_service_data.read().unwrap().channel_status == ChannelStatus::OpenRequest {
        let mut message = create_channel_open_response_message();
        messenger.cryptor.encrypt_message(&mut message);
        messenger.send_message_via_usb(message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.video_service_data.write().unwrap().channel_status = ChannelStatus::Open;
    } else if data.video_service_data.read().unwrap().setup_status == SetupStatus::Requested {
        log::info!("Sending video focus indication");
        let mut video_focus_message = create_video_focus_indication();
        messenger.cryptor.encrypt_message(&mut video_focus_message);
        messenger.send_message_via_usb(video_focus_message);
        let mut setup_response_message = channels::general_audio::create_av_channel_setup_response(ChannelID::Video);
        messenger.cryptor.encrypt_message(&mut setup_response_message);
        messenger.send_message_via_usb(setup_response_message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.video_service_data.write().unwrap().setup_status = SetupStatus::Finished;
    } else if data.video_service_data.read().unwrap().setup_status == SetupStatus::Finished {
        let last_indication = data.video_service_data.read().unwrap().received_indication.clone();
        match last_indication {
            Some(indication_type) => {
                let mut indication_ack_message = create_av_media_ack_indication();
                messenger.cryptor.encrypt_message(&mut indication_ack_message);
                messenger.send_message_via_usb(indication_ack_message);
                receival_queue_tx.send(ReceivalRequest).unwrap();
                data.video_service_data.write().unwrap().received_indication = None;
            }
            None => {
                log::debug!("No indication received");
            }
        }
    }
}

pub fn create_channel_open_response_message() -> Frame {
    log::info!("Creating channel open response message for video channel");
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Video, payload };
    message
}

pub fn create_av_channel_setup_response(video_setup_request: Frame) -> Frame {
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Video, payload };
    message
}

pub fn create_video_focus_indication() -> Frame {
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Video, payload };
    message
}

pub fn create_av_media_ack_indication() -> Frame {
    log::info!("Creating av media ack indication");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut indication = crate::protos::AVMediaAckIndicationMessage::AVMediaAckIndication::new();
    indication.set_session(0);
    indication.set_value(1);
    let mut payload = (AVMessageID::AvMediaAckIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Video, payload };
    message
}
