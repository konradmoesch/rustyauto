use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
//use prost::Message as prostmsg;
use protobuf::Message as protobuf_message;
use rusb::Version;

use crate::{channels, messenger, protos};
use crate::channels::control_service_channel::VersionResponseStatus::{Match, Mismatch};
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::MessengerStatus;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};
use crate::protos::ServiceDiscoveryRequestMessage::ServiceDiscoveryRequest;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub fn create_version_request_message(own_version: &crate::data::android_auto_entity::Version) -> Message {
    log::info!("Creating version request message");
    let mut version_buffer = [0u8; 4];
    version_buffer[0] = own_version.major.to_be_bytes()[0];
    version_buffer[1] = own_version.major.to_be_bytes()[1];
    version_buffer[2] = own_version.minor.to_be_bytes()[0];
    version_buffer[3] = own_version.minor.to_be_bytes()[1];
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::VersionRequest as u16).to_be_bytes().to_vec();
    payload.push(version_buffer[3]);
    payload.push(version_buffer[2]);
    payload.push(version_buffer[1]);
    payload.push(version_buffer[0]);
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
    data.version.remote_version.major = major;
    data.version.remote_version.minor = minor;
    data.version.version_match = version_match_int == 1;
    data.messenger_status = MessengerStatus::VersionRequestDone;
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
    dbg!(service_disc_res);
}

fn handle_audio_focus_request(payload: Vec<u8>) {
    let request = crate::protos::AudioFocusRequestMessage::AudioFocusRequest::parse_from_bytes(payload.as_slice()).unwrap();
    log::debug!("Received service discovery request: {:?}", payload);
    dbg!(request.clone());
    log::info!("Requested audio focus, type: {:?}", request.audio_focus_type());

    let audio_focus_state = match request.audio_focus_type() {
        crate::protos::AudioFocusTypeEnum::audio_focus_type::Enum::RELEASE => {
            crate::protos::AudioFocusStateEnum::audio_focus_state::Enum::LOSS
        },
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
        _ => {}
    }
}

#[derive(Debug)]
pub enum ControlMessageID
{
    None = 0x0000,
    VersionRequest = 0x0001,
    VersionResponse = 0x0002,
    SSLHandshake = 0x0003,
    AuthComplete = 0x0004,
    ServiceDiscoveryRequest = 0x0005,
    ServiceDiscoveryResponse = 0x0006,
    ChannelOpenRequest = 0x0007,
    ChannelOpenResponse = 0x0008,
    PingRequest = 0x000b,
    PingResponse = 0x000c,
    NavigationFocusRequest = 0x000d,
    NavigationFocusResponse = 0x000e,
    ShutdownRequest = 0x000f,
    ShutdownResponse = 0x0010,
    VoiceSessionRequest = 0x0011,
    AudioFocusRequest = 0x0012,
    AudioFocusResponse = 0x0013,
}
impl From<u8> for ControlMessageID {
    fn from(message_id_as_byte: u8) -> Self {
        match message_id_as_byte {
                0x0000 => { ControlMessageID::None }
                0x0001 => { ControlMessageID::VersionRequest }
                0x0002 => { ControlMessageID::VersionResponse }
                0x0003 => { ControlMessageID::SSLHandshake }
                0x0004 => { ControlMessageID::AuthComplete }
                0x0005 => { ControlMessageID::ServiceDiscoveryRequest }
                0x0006 => { ControlMessageID::ServiceDiscoveryResponse }
                0x0007 => { ControlMessageID::ChannelOpenRequest }
                0x0008 => { ControlMessageID::ChannelOpenResponse }
                0x000b => { ControlMessageID::PingRequest }
                0x000c => { ControlMessageID::PingResponse }
                0x000d => { ControlMessageID::NavigationFocusRequest }
                0x000e => { ControlMessageID::NavigationFocusResponse }
                0x000f => { ControlMessageID::ShutdownRequest }
                0x0010 => { ControlMessageID::ShutdownResponse }
                0x0011 => { ControlMessageID::VoiceSessionRequest }
                0x0012 => { ControlMessageID::AudioFocusRequest }
                0x0013 => { ControlMessageID::AudioFocusResponse }
            _ => { ControlMessageID::None }
        }
    }
}
impl Into<u16> for ControlMessageID {
    fn into(self) -> u16 {
        match self {
            ControlMessageID::None => {255}
            ControlMessageID::VersionRequest => {0x0001}
            ControlMessageID::VersionResponse => {0x0002}
            ControlMessageID::SSLHandshake => {0x0003}
            ControlMessageID::AuthComplete => {0x0004}
            ControlMessageID::ServiceDiscoveryRequest => {0x0005}
            ControlMessageID::ServiceDiscoveryResponse => {0x0006}
            ControlMessageID::ChannelOpenRequest => {0x0007}
            ControlMessageID::ChannelOpenResponse => {0x0008}
            ControlMessageID::PingRequest => {0x000b}
            ControlMessageID::PingResponse => {0x000c}
            ControlMessageID::NavigationFocusRequest => {0x000d}
            ControlMessageID::NavigationFocusResponse => {0x000e}
            ControlMessageID::ShutdownRequest => {0x000f}
            ControlMessageID::ShutdownResponse => {0x0010}
            ControlMessageID::VoiceSessionRequest => {0x0011}
            ControlMessageID::AudioFocusRequest => {0x0012}
            ControlMessageID::AudioFocusResponse => {0x0013}
        }
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
