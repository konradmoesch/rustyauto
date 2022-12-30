use protobuf::Message;
use crate::channels::control::message_ids::ControlMessageID;
use crate::channels::sensor::sensor_service_channel::SensorMessageID;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::services::general::{ChannelStatus, SetupStatus};
use crate::messenger::frame::{Frame, MessageType};

fn handle_channel_open_request(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received channel open request for sensor_channel");
    data.sensor_service_data.write().unwrap().channel_status = ChannelStatus::OpenRequest;
}

fn handle_sensor_start_request(request_message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received sensor start request for sensor_channel");
    let payload = request_message.payload.as_slice().clone();
    let sensor_start_request_message = crate::protos::SensorStartRequestMessage::SensorStartRequestMessage::parse_from_bytes(&payload[2..]).unwrap();
    log::info!("Received sensor start request: {:?}, creating first sensor indication", sensor_start_request_message.sensor_type);
    data.sensor_service_data.write().unwrap().requested_sensor_type = Some(sensor_start_request_message.sensor_type());
    data.sensor_service_data.write().unwrap().setup_status = SetupStatus::Requested;
}

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    log::info!("Received message in sensor service channel: {:?}", message);
    let payload = message.clone().payload;
    let message_id_word = u16::from_be_bytes([payload.as_slice()[0], payload.as_slice()[1]]);
    //TODO: use word correctly
    match message.frame_header.message_type {
        MessageType::Specific => {
            let message_id = SensorMessageID::try_from(message_id_word);
            match message_id {
                Ok(SensorMessageID::SensorStartRequest) => {
                    handle_sensor_start_request(message, data);
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
                    handle_channel_open_request(message, data);
                }
                _ => { unimplemented!() }
            }
        }
    }
    use protobuf::Enum as protoenum;
    //let message_id = crate::protos::MediaAudioChannelMessageIdsEnum::avchannel_message::Enum::from_i32(message_id_word as i32);
    log::info!("Message ID (raw): {:?}", message_id_word);
}