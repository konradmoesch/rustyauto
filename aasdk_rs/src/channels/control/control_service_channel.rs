use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::BufMut;
//use prost::Message as prostmsg;
use protobuf::Message as protobuf_message;
use rusb::Version;

use crate::{channels, messenger, protos};
use crate::channels::control::control_service_channel::VersionResponseStatus::{Match, Mismatch};
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::MessengerStatus;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use crate::protos::ServiceDiscoveryRequestMessage::ServiceDiscoveryRequest;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

use super::message_ids::ControlMessageID;

pub fn create_version_request_message(own_version: crate::data::android_auto_entity::Version) -> Message {
    log::info!("Creating version request message");
    let version_buffer = own_version.to_bytes();
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::VersionRequest as u16).to_be_bytes().to_vec();
    payload.extend_from_slice(&version_buffer);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_handshake_message(handshake_buffer: &[u8]) -> Message {
    log::info!("Creating ssl handshake message");
    log::debug!("Handshake buffer: {:?}", handshake_buffer);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::SSLHandshake as u16).to_be_bytes().to_vec();
    payload.extend_from_slice(handshake_buffer);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_auth_complete_message() -> Message {
    log::info!("Creating auth complete message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::AuthComplete as u16).to_be_bytes().to_vec();
    payload.push(0x8);
    payload.push(0);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_service_discovery_response_message(service_discovery_response_message: crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse) -> Message {
    log::info!("Creating service discovery response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::ServiceDiscoveryResponse as u16).to_be_bytes().to_vec();
    //payload.push(0);
    let mut bytes = service_discovery_response_message.write_to_bytes().unwrap();
    println!("{:x?}", bytes);
    payload.extend(bytes);
    println!("{:x?}", payload);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_audio_focus_response_message() -> Message {
    log::info!("Creating audio focus response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut audio_focus_response = crate::protos::AudioFocusResponseMessage::AudioFocusResponse::new();
    audio_focus_response.set_audio_focus_state(crate::protos::AudioFocusStateEnum::audio_focus_state::Enum::LOSS);
    let mut payload = (ControlMessageID::AudioFocusResponse as u16).to_be_bytes().to_vec();
    //payload.push(0);
    let mut bytes = audio_focus_response.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Control, payload };
    message
}

fn handle_version_response(payload: Vec<u8>, data: &mut AndroidAutoEntityData) {
    log::debug!("Received raw version response {:?}", payload);
    let mut payload_slice = payload.clone().as_slice();
    let mut rdr = Cursor::new(payload);

    let major = rdr.read_u16::<BigEndian>().unwrap();
    let minor = rdr.read_u16::<BigEndian>().unwrap();
    let version_match_int = rdr.read_u16::<BigEndian>().unwrap();
    let version_match = VersionResponseStatus::from(version_match_int);
    let mut version = data.version.write().unwrap();
    version.remote_version.major = major;
    version.remote_version.minor = minor;
    version.version_match = version_match_int == 1;
    *data.messenger_status.write().unwrap() = MessengerStatus::VersionRequestDone;
    log::info!("Received version response: {}.{} ({:?})", major, minor, version_match);
}

fn handle_ssl_handshake(payload: Vec<u8>) {
    log::debug!("Received raw ssl handshake: {:?}", payload);
    let mut payload_slice = payload.clone().as_slice();
}

fn handle_service_discovery_request(payload: Vec<u8>) {
    log::debug!("Received service discovery request: {:?}", payload);
    let mut payload_slice = payload.clone().as_slice();
    let service_disc_req = ServiceDiscoveryRequest::parse_from_bytes(payload.as_slice()).unwrap();
    dbg!(service_disc_req);
    //log::info!("Discovery request, {}", service_disc_req);
    //log::info!("Discovery request, device name: {}, brand: {}", service_disc_req.device_name, service_disc_req.device_brand);

    let mut service_disc_res = ServiceDiscoveryResponse::new();
    //service_disc_res.mutable_channels()->Reserve(256);
    service_disc_res.head_unit_name = Some("rustyauto".to_string());
    service_disc_res.car_model = Some("Universal".to_string());
    service_disc_res.car_year = Some("2022".to_string());
    service_disc_res.car_serial = Some("20221004".to_string());
    service_disc_res.left_hand_drive_vehicle = Some(true);
    service_disc_res.headunit_manufacturer = Some("km".to_string());
    service_disc_res.headunit_model = Some("rustyauto app".to_string());
    service_disc_res.sw_build = Some("1".to_string());
    service_disc_res.sw_version = Some("1.0".to_string());
    service_disc_res.can_play_native_media_during_vr = Some(false);
    service_disc_res.hide_clock = Some(false);
    //dbg!(service_disc_res);
    //todo send this message!
}

fn handle_audio_focus_request(payload: Vec<u8>) {
    let request = crate::protos::AudioFocusRequestMessage::AudioFocusRequest::parse_from_bytes(payload.as_slice()).unwrap();
    log::debug!("Received service discovery request: {:?}", payload);
    dbg!(request.clone());
    log::info!("Requested audio focus, type: {:?}", request.audio_focus_type());

    let audio_focus_state = match request.audio_focus_type() {
        crate::protos::AudioFocusTypeEnum::audio_focus_type::Enum::RELEASE => {
            crate::protos::AudioFocusStateEnum::audio_focus_state::Enum::LOSS
        }
        _ => {
            crate::protos::AudioFocusStateEnum::audio_focus_state::Enum::GAIN
        }
    };

    log::info!("audio focus state: {:?}", audio_focus_state);

    let mut response = crate::protos::AudioFocusResponseMessage::AudioFocusResponse::new();
    response.set_audio_focus_state(audio_focus_state);

    //let message = create_audio_focus_response_message(response);
    let message = create_audio_focus_response_message();
    dbg!(message);
}

pub fn handle_message(message: &Message, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in control service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    let message_id = ControlMessageID::from(message_id_word as u8);
    log::info!("Message ID: {:?}", message_id);
    match message_id {
        ControlMessageID::VersionResponse => { handle_version_response(payload[2..].to_vec(), data) }
        ControlMessageID::SSLHandshake => { handle_ssl_handshake(payload[2..].to_vec()) }
        ControlMessageID::ServiceDiscoveryRequest => { handle_service_discovery_request(payload[2..].to_vec()) }
        ControlMessageID::AudioFocusRequest => { handle_audio_focus_request(payload[2..].to_vec()) }
        _ => { panic!("error trying to handle unknown message {message_id:?}") }
    }
}

#[derive(Debug)]
enum VersionResponseStatus
{
    Match = 0,
    Mismatch = 0xFFFF,
}

impl From<u16> for VersionResponseStatus {
    fn from(version_response_status_as_word: u16) -> Self {
        match version_response_status_as_word {
            0 => Match,
            _ => Mismatch,
        }
    }
}
