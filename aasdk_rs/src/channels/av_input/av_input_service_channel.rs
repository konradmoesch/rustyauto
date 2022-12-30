use std::sync::mpsc::Sender;
use crate::{channels, messenger};
use crate::messenger::frame::{ChannelID, EncryptionType, FrameHeader, FrameType, Frame, MessageType};
use protobuf::Message as protomsg;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::messenger::messenger::{Messenger, ReceivalRequest};

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    if data.audio_input_service_data.read().unwrap().channel_status == ChannelStatus::OpenRequest {
        let mut  message = create_channel_open_response_message();
        messenger.cryptor.encrypt_message(&mut message);
        messenger.send_message_via_usb(message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.audio_input_service_data.write().unwrap().channel_status = ChannelStatus::Open;
    }
    if data.audio_input_service_data.read().unwrap().setup_status == SetupStatus::Requested {
        let mut setup_response_message = channels::general_audio::create_av_channel_setup_response(ChannelID::AVInput);
        messenger.cryptor.encrypt_message(&mut setup_response_message);
        messenger.send_message_via_usb(setup_response_message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.media_audio_service_data.write().unwrap().setup_status = SetupStatus::Finished;
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
    println!("{:x?}", bytes);
    payload.extend(bytes);
    println!("{:x?}", payload);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::AVInput, payload };
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
