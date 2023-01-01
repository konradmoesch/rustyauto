use std::sync::mpsc::Sender;

use log::log;
use openssl_sys::bind;
use protobuf::Message as protomsg;

use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::data::services::input_service_data::{TouchActionType, TouchPosition};
use crate::messenger;
use crate::messenger::frame::{ChannelID, EncryptionType, Frame, FrameHeader, FrameType, MessageType};
use crate::messenger::messenger::{Messenger, ReceivalRequest};

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    if data.input_service_data.read().unwrap().channel_status == ChannelStatus::OpenRequest {
        let mut message = create_channel_open_response_message();
        messenger.cryptor.encrypt_message(&mut message);
        messenger.send_message_via_usb(message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.input_service_data.write().unwrap().channel_status = ChannelStatus::Open;
    }
    if data.input_service_data.read().unwrap().setup_status == SetupStatus::Requested {
        let mut binding_response_message = create_binding_response_message(data.input_service_data.read().unwrap().binding_request.clone().unwrap());
        messenger.cryptor.encrypt_message(&mut binding_response_message);
        messenger.send_message_via_usb(binding_response_message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.input_service_data.write().unwrap().setup_status = SetupStatus::Finished;
    }
    if let Some(position) = data.input_service_data.read().unwrap().current_touch_position {
        log::error!("Touch position recv! {:?}", position);
        let action = data.input_service_data.read().unwrap().current_touch_action.unwrap();
        let mut touch_input_indication = create_touch_event_indication(position, action);
        messenger.cryptor.encrypt_message(&mut touch_input_indication);
        messenger.send_message_via_usb(touch_input_indication);
        log::info!("Sent touch event indication");
    } else {
        log::error!("No Touch");
    }
    //TODO: reset properly, also capture event type
    data.input_service_data.write().unwrap().current_touch_position = None;
}

pub fn create_channel_open_response_message() -> Frame {
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Input, payload };
    message
}

pub fn create_binding_response_message(binding_request: crate::protos::BindingRequestMessage::BindingRequest) -> Frame {
    /*let payload = binding_request_message.payload.as_slice().clone();
    log::debug!("{:?}", payload.to_vec());
    let binding_request = crate::protos::BindingRequestMessage::BindingRequest::parse_from_bytes(&payload[2..]).unwrap();*/
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
    crate::messenger::frame::Frame { frame_header, channel_id: ChannelID::Input, payload }
}

pub fn create_touch_event_indication(touch_position: TouchPosition, touch_action: TouchActionType) -> Frame {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let mut input_indication = crate::protos::InputEventIndicationMessage::InputEventIndication::new();
    input_indication.set_timestamp(timestamp);

    let mut touch_event = crate::protos::TouchEventData::TouchEvent::new();
    use protobuf::Enum;
    let proto_action = crate::protos::TouchActionEnum::touch_action::Enum::from_i32(touch_action as i32).unwrap();
    touch_event.set_touch_action(proto_action);//event.type;
    let mut touch_location = crate::protos::TouchLocationData::TouchLocation::new();
    touch_location.x = Some(touch_position.0 as u32);
    touch_location.y = Some(touch_position.1 as u32);
    touch_location.set_pointer_id(0);
    touch_event.touch_location.push(touch_location);
    input_indication.touch_event = Some(touch_event).into();

    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (InputMessageID::InputEventIndication as u16).to_be_bytes().to_vec();
    payload.extend(input_indication.write_to_bytes().unwrap());
    crate::messenger::frame::Frame { frame_header, channel_id: ChannelID::Input, payload }
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
