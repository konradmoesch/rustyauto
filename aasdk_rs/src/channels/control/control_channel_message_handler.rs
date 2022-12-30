use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use protobuf::Message;
use crate::channels::control::control_service_channel::create_audio_focus_response_message;
use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::MessengerStatus;
use crate::data::services::control_service_data::ServiceDiscoveryState;
use crate::messenger::frame::Frame;
use crate::protos::ServiceDiscoveryRequestMessage::ServiceDiscoveryRequest;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

#[derive(Debug)]
enum VersionResponseStatus
{
    Match = 0,
    Mismatch = 0xFFFF,
}

impl From<u16> for VersionResponseStatus {
    fn from(version_response_status_as_word: u16) -> Self {
        match version_response_status_as_word {
            0 => VersionResponseStatus::Match,
            _ => VersionResponseStatus::Mismatch,
        }
    }
}

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in control service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    let message_id = ControlMessageID::from(message_id_word as u8);
    log::info!("Message ID: {:?}", message_id);
    match message_id {
        ControlMessageID::VersionResponse => { handle_version_response(payload[2..].to_vec(), data) }
        ControlMessageID::SSLHandshake => { handle_ssl_handshake(payload[2..].to_vec()) }
        ControlMessageID::ServiceDiscoveryRequest => { handle_service_discovery_request(payload[2..].to_vec(), data) }
        ControlMessageID::AudioFocusRequest => { handle_audio_focus_request(payload[2..].to_vec()) }
        _ => { panic!("error trying to handle unknown message {message_id:?}") }
    }
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

fn handle_service_discovery_request(payload: Vec<u8>, data: &mut AndroidAutoEntityData) {
    log::debug!("Received service discovery request: {:?}", payload);
    let mut payload_slice = payload.clone().as_slice();
    let service_disc_req = ServiceDiscoveryRequest::parse_from_bytes(payload.as_slice()).unwrap();
    log::info!("Discovery request, {}", service_disc_req);
    //log::info!("Discovery request, device name: {}, brand: {}", service_disc_req.device_name, service_disc_req.device_brand);

    data.control_service_data.write().unwrap().service_discovery_state = ServiceDiscoveryState::Requested;
}

fn handle_audio_focus_request(payload: Vec<u8>) {
    let request = crate::protos::AudioFocusRequestMessage::AudioFocusRequest::parse_from_bytes(payload.as_slice()).unwrap();
    log::debug!("Received audio focus request: {:?}", payload);
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
