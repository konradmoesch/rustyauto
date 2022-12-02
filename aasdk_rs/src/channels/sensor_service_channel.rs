use protobuf::Message as protomsg;

use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger;
use crate::messenger::message::{ChannelID, EncryptionType, FrameHeader, FrameType, Message, MessageType};

fn handle_channel_open_request(message: &Message) {
    log::info!("Received channel open request for sensor_channel");
}

fn handle_sensor_start_request(message: &Message) {
    log::info!("Received sensor start request for sensor_channel");
}

pub fn handle_message(message: &Message, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in sensor service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = SensorMessageID::try_from(message_id_word);
            match message_id {
                Ok(SensorMessageID::SensorStartRequest) => {
                    handle_sensor_start_request(message);
                }
                _ => {
                    log::error!("Error: UnknownMessageID: {:?}", message_id);
                    unimplemented!()
                }
            }
        }
        MessageType::Control => {
            match crate::channels::control::message_ids::ControlMessageID::from(message_id_word as u8) {
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
    log::info!("Creating channel open response message for sensor channel");
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
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_night_sensor_indication_message() -> Message {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    let is_night = false;
    log::info!("Setting night mode: {}", is_night);
    let mut night_mode = crate::protos::NightModeData::NightMode::new();
    night_mode.set_is_night(is_night);
    indication.night_mode.push(night_mode);

    let mut payload = (crate::channels::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_driving_status_sensor_indication_message() -> Message {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    log::info!("Setting driving status unrestricted");
    let mut driving_status = crate::protos::DrivingStatusData::DrivingStatus::new();
    use protobuf::Enum;
    driving_status.status = Some(crate::protos::DrivingStatusEnum::driving_status::Enum::UNRESTRICTED.value());
    indication.driving_status.push(driving_status);

    let mut payload = (crate::channels::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_location_sensor_indication_message() -> Message {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    log::info!("Setting location status");
    let mut location = crate::protos::GPSLocationData::GPSLocation::new();
    use protobuf::Enum;
    location.set_timestamp(0);
    location.set_latitude(0);
    location.set_longitude(0);
    location.set_accuracy(0);
    indication.gps_location.push(location);

    let mut payload = (crate::channels::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_sensor_start_response_message(request_message: Message) -> Message {
    use protobuf::Message as protomsg;
    let payload = request_message.payload.as_slice().clone();
    let sensor_start_request_message = crate::protos::SensorStartRequestMessage::SensorStartRequestMessage::parse_from_bytes(&payload[2..]).unwrap();
    log::info!("Received sensor start request: {:?}, creating first sensor indication", sensor_start_request_message.sensor_type);
    match sensor_start_request_message.sensor_type() {
        crate::protos::SensorTypeEnum::sensor_type::Enum::DRIVING_STATUS => create_driving_status_sensor_indication_message(),
        crate::protos::SensorTypeEnum::sensor_type::Enum::NIGHT_DATA => create_night_sensor_indication_message(),
        crate::protos::SensorTypeEnum::sensor_type::Enum::LOCATION => create_location_sensor_indication_message(),
        _ => {
            log::error!("Unknown Sensor in start request");
            unimplemented!()
        }
    }
}

pub fn create_sensor_start_response_alternate() -> Message {
    use protobuf::Message as protomsg;
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut sensor_start_response = crate::protos::SensorStartResponseMessage::SensorStartResponseMessage::new();
    sensor_start_response.set_status(crate::protos::StatusEnum::status::Enum::OK);
    let mut payload = (crate::channels::sensor_service_channel::SensorMessageID::SensorStartResponse as u16).to_be_bytes().to_vec();
    let mut bytes = sensor_start_response.write_to_bytes().unwrap();
    payload.extend(bytes);
    let message = messenger::message::Message { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

#[derive(Debug)]
pub enum SensorMessageID
{
    None = 0x0000,
    SensorStartRequest = 0x8001,
    SensorStartResponse = 0x8002,
    SensorEventIndication = 0x8003,
}

impl TryFrom<u16> for SensorMessageID {
    type Error = ();

    fn try_from(message_id_as_byte: u16) -> Result<Self, ()> {
        match message_id_as_byte {
            0x0000 => { Ok(SensorMessageID::None) }
            0x8001 => { Ok(SensorMessageID::SensorStartRequest) }
            0x8002 => { Ok(SensorMessageID::SensorStartResponse) }
            0x8003 => { Ok(SensorMessageID::SensorEventIndication) }
            _ => {
                log::error!("Unknown value");
                Err(())
                //todo!()
            }
        }
    }
}
