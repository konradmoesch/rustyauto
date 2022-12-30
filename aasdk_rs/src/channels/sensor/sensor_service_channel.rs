use std::sync::mpsc::Sender;
use protobuf::Message as protomsg;

use crate::channels::control::message_ids::ControlMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::messenger;
use crate::messenger::frame::{ChannelID, EncryptionType, Frame, FrameHeader, FrameType, MessageType};
use crate::messenger::messenger::{Messenger, ReceivalRequest};
use crate::protos::SensorTypeEnum;
use crate::protos::SensorTypeEnum::SensorType;

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    if data.sensor_service_data.read().unwrap().channel_status == ChannelStatus::OpenRequest {
        let mut message = create_channel_open_response_message();
        messenger.cryptor.encrypt_message(&mut message);
        messenger.send_message_via_usb(message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.sensor_service_data.write().unwrap().channel_status = ChannelStatus::Open;
    }
    if data.sensor_service_data.read().unwrap().setup_status == SetupStatus::Requested {
        let mut first_indication_message = create_first_indication_message(data.sensor_service_data.read().unwrap().requested_sensor_type.unwrap());
        messenger.cryptor.encrypt_message(&mut first_indication_message);
        messenger.send_message_via_usb(first_indication_message);
        let mut sensor_start_response_message = create_sensor_start_response_message();
        messenger.cryptor.encrypt_message(&mut sensor_start_response_message);
        messenger.send_message_via_usb(sensor_start_response_message);
        receival_queue_tx.send(ReceivalRequest).unwrap();
        data.sensor_service_data.write().unwrap().setup_status = SetupStatus::Finished;
    }
}

pub fn create_channel_open_response_message() -> Frame {
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_night_sensor_indication_message() -> Frame {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    let is_night = false;
    log::info!("Setting night mode: {}", is_night);
    let mut night_mode = crate::protos::NightModeData::NightMode::new();
    night_mode.set_is_night(is_night);
    indication.night_mode.push(night_mode);

    let mut payload = (crate::channels::sensor::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_driving_status_sensor_indication_message() -> Frame {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    log::info!("Setting driving status unrestricted");
    let mut driving_status = crate::protos::DrivingStatusData::DrivingStatus::new();
    use protobuf::Enum;
    driving_status.status = Some(crate::protos::DrivingStatusEnum::driving_status::Enum::UNRESTRICTED.value());
    indication.driving_status.push(driving_status);

    let mut payload = (crate::channels::sensor::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_location_sensor_indication_message() -> Frame {
    let mut indication = crate::protos::SensorEventIndicationMessage::SensorEventIndication::new();
    log::info!("Setting location status");
    let mut location = crate::protos::GPSLocationData::GPSLocation::new();
    use protobuf::Enum;
    location.set_timestamp(0);
    location.set_latitude(0);
    location.set_longitude(0);
    location.set_accuracy(0);
    indication.gps_location.push(location);

    let mut payload = (crate::channels::sensor::sensor_service_channel::SensorMessageID::SensorEventIndication as u16).to_be_bytes().to_vec();
    let mut bytes = indication.write_to_bytes().unwrap();
    payload.extend(bytes);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Sensor, payload };
    message
}

pub fn create_first_indication_message(requested_sensor_type: SensorTypeEnum::sensor_type::Enum) -> Frame {
    /*use protobuf::Message as protomsg;
    let payload = request_message.payload.as_slice().clone();
    let sensor_start_request_message = crate::protos::SensorStartRequestMessage::SensorStartRequestMessage::parse_from_bytes(&payload[2..]).unwrap();
    log::info!("Received sensor start request: {:?}, creating first sensor indication", sensor_start_request_message.sensor_type);*/
    match requested_sensor_type {
        crate::protos::SensorTypeEnum::sensor_type::Enum::DRIVING_STATUS => create_driving_status_sensor_indication_message(),
        crate::protos::SensorTypeEnum::sensor_type::Enum::NIGHT_DATA => create_night_sensor_indication_message(),
        crate::protos::SensorTypeEnum::sensor_type::Enum::LOCATION => create_location_sensor_indication_message(),
        _ => {
            log::error!("Unknown Sensor in start request");
            unimplemented!()
        }
    }
}

pub fn create_sensor_start_response_message() -> Frame {
    use protobuf::Message as protomsg;
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut sensor_start_response = crate::protos::SensorStartResponseMessage::SensorStartResponseMessage::new();
    sensor_start_response.set_status(crate::protos::StatusEnum::status::Enum::OK);
    let mut payload = (crate::channels::sensor::sensor_service_channel::SensorMessageID::SensorStartResponse as u16).to_be_bytes().to_vec();
    let mut bytes = sensor_start_response.write_to_bytes().unwrap();
    payload.extend(bytes);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Sensor, payload };
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
