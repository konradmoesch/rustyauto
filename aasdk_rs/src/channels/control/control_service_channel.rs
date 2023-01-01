use std::io::Cursor;
use std::sync::mpsc::Sender;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::BufMut;
//use prost::Message as prostmsg;
use protobuf::Message as protobuf_message;
use rusb::Version;

use crate::{channels, messenger, protos};
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::MessengerStatus;
use crate::data::services::control_service_data::{AudioFocusState, ServiceDiscoveryState};
use crate::messenger::frame::{ChannelID, EncryptionType, FrameHeader, FrameType, Frame, MessageType};
use crate::messenger::messenger::{Messenger, ReceivalRequest};
use crate::protos::ServiceDiscoveryRequestMessage::ServiceDiscoveryRequest;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

use super::message_ids::ControlMessageID;

pub fn temp_fill_service_features(sdr: &mut crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse, data: &mut AndroidAutoEntityData) {
    //AudioInput
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::AVInput as u32);

    let mut audio_input_channel = crate::protos::AVInputChannelData::AVInputChannel::default();
    let mut audio_config = crate::protos::AudioConfigData::AudioConfig::new();
    //TODO: Initialize audio input and use real values
    //TODO: fix the missing FFFFFF in sample_rate field
    let saved_audio_input_config = data.audio_input_service_data.read().unwrap().config;
    audio_config.set_sample_rate(saved_audio_input_config.sample_rate as u32);
    //audio_config.set_sample_rate(16);
    audio_config.set_bit_depth(saved_audio_input_config.bit_depth as u32);
    audio_config.set_channel_count(saved_audio_input_config.channel_count as u32);
    audio_input_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
    audio_input_channel.audio_config = protobuf::MessageField::from_option(Some(audio_config));

    channel_descriptor.av_input_channel = protobuf::MessageField::from_option(Some(audio_input_channel));

    sdr.channels.push(channel_descriptor);
    //MediaAudio
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::MediaAudio as u32);

    let mut audio_channel = crate::protos::AVChannelData::AVChannel::default();
    audio_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
    audio_channel.set_audio_type(crate::protos::AudioTypeEnum::audio_type::Enum::MEDIA);

    audio_channel.set_available_while_in_call(true);

    let mut audio_config = crate::protos::AudioConfigData::AudioConfig::default();
    let saved_media_audio_config = data.media_audio_service_data.read().unwrap().config;
    audio_config.set_sample_rate(saved_media_audio_config.sample_rate as u32);
    //audio_config.set_sample_rate(48);
    audio_config.set_bit_depth(saved_media_audio_config.bit_depth as u32);
    audio_config.set_channel_count(saved_media_audio_config.channel_count as u32);

    audio_channel.audio_configs.push(audio_config);

    channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(audio_channel));

    sdr.channels.push(channel_descriptor);
    //SpeechAudio
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::SpeechAudio as u32);

    let mut audio_channel = crate::protos::AVChannelData::AVChannel::default();
    audio_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
    audio_channel.set_audio_type(crate::protos::AudioTypeEnum::audio_type::Enum::SPEECH);

    audio_channel.set_available_while_in_call(true);

    let mut audio_config = crate::protos::AudioConfigData::AudioConfig::default();
    let saved_speech_audio_config = data.speech_audio_service_data.read().unwrap().config;
    audio_config.set_sample_rate(saved_speech_audio_config.sample_rate as u32);
    //audio_config.set_sample_rate(16);
    audio_config.set_bit_depth(saved_speech_audio_config.bit_depth as u32);
    audio_config.set_channel_count(saved_speech_audio_config.channel_count as u32);

    audio_channel.audio_configs.push(audio_config);

    channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(audio_channel));

    sdr.channels.push(channel_descriptor);
    //SystemAudio
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::SystemAudio as u32);

    let mut audio_channel = crate::protos::AVChannelData::AVChannel::default();
    audio_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::AUDIO);
    audio_channel.set_audio_type(crate::protos::AudioTypeEnum::audio_type::Enum::SYSTEM);

    audio_channel.set_available_while_in_call(true);

    let mut audio_config = crate::protos::AudioConfigData::AudioConfig::default();
    let saved_system_audio_config = data.system_audio_service_data.read().unwrap().config;
    audio_config.set_sample_rate(saved_system_audio_config.sample_rate as u32);
    //audio_config.set_sample_rate(16);
    audio_config.set_bit_depth(saved_system_audio_config.bit_depth as u32);
    audio_config.set_channel_count(saved_system_audio_config.channel_count as u32);

    audio_channel.audio_configs.push(audio_config);

    channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(audio_channel));

    sdr.channels.push(channel_descriptor);
    //SensorService
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::Sensor as u32);

    let mut sensor_channel = crate::protos::SensorChannelData::SensorChannel::default();
    let mut driving_status_sensor = crate::protos::SensorData::Sensor::new();
    driving_status_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::DRIVING_STATUS);
    let mut night_data_sensor = crate::protos::SensorData::Sensor::new();
    night_data_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::NIGHT_DATA);
    sensor_channel.sensors.push(driving_status_sensor);
    let sensor_service_config = data.sensor_service_data.read().unwrap().config;
    if sensor_service_config.location_sensor_present {
        let mut location_sensor = crate::protos::SensorData::Sensor::new();
        location_sensor.set_type(crate::protos::SensorTypeEnum::sensor_type::Enum::LOCATION);
        sensor_channel.sensors.push(location_sensor);
    }
    sensor_channel.sensors.push(night_data_sensor);
    //TODO: better first get() the sensorChannel, if possible?

    channel_descriptor.sensor_channel = protobuf::MessageField::from_option(Some(sensor_channel));

    sdr.channels.push(channel_descriptor);
    //VideoService
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::Video as u32);
    //TODO: init video output, use real values
    let mut video_channel = crate::protos::AVChannelData::AVChannel::default();
    video_channel.set_stream_type(crate::protos::AVStreamTypeEnum::avstream_type::Enum::VIDEO);
    video_channel.set_available_while_in_call(true);
    let mut video_config = crate::protos::VideoConfigData::VideoConfig::default();
    let saved_video_config = data.video_service_data.read().unwrap().config;
    //todo: read enums from saved values
    video_config.set_video_resolution(crate::protos::VideoResolutionEnum::video_resolution::Enum::_480p);
    video_config.set_video_fps(crate::protos::VideoFPSEnum::video_fps::Enum::_30);
    video_config.set_margin_height(saved_video_config.margin_height as u32);
    video_config.set_margin_width(saved_video_config.margin_width as u32);
    video_config.set_dpi(saved_video_config.dpi as u32);
    video_channel.video_configs.push(video_config);

    channel_descriptor.av_channel = protobuf::MessageField::from_option(Some(video_channel));

    sdr.channels.push(channel_descriptor);
    //todo BluetoothService
    //InputService
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::frame::ChannelID::Input as u32);

    //TODO: Initialize input and use real values
    //TODO: fix the missing FFFFFF in touch config fields

    let mut input_channel = crate::protos::InputChannelData::InputChannel::default();
    let mut touch_screen_config = crate::protos::TouchConfigData::TouchConfig::default();
    let saved_input_config = data.input_service_data.read().unwrap().config;
    //todo: use saved config
    touch_screen_config.set_width(1920);
    //touch_screen_config.set_width(19);
    touch_screen_config.set_height(1080);
    //touch_screen_config.set_height(10);

    input_channel.touch_screen_config = protobuf::MessageField::from_option(Some(touch_screen_config));
    channel_descriptor.input_channel = protobuf::MessageField::from_option(Some(input_channel));

    sdr.channels.push(channel_descriptor);
    //todo WifiService
}

pub fn run(data: &mut AndroidAutoEntityData, receival_queue_tx: Sender<ReceivalRequest>, messenger: &mut Messenger) {
    let current_data = data.control_service_data.read().unwrap().clone();
    if current_data.service_discovery_state == ServiceDiscoveryState::Requested {

        let mut service_discovery_response = crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse::new();
        service_discovery_response.head_unit_name = Some("rustyauto".to_string());
        service_discovery_response.car_model = Some("Universal".to_string());
        service_discovery_response.car_year = Some("2022".to_string());
        service_discovery_response.car_serial = Some("20221004".to_string());
        service_discovery_response.left_hand_drive_vehicle = Some(true);
        service_discovery_response.headunit_manufacturer = Some("km".to_string());
        service_discovery_response.headunit_model = Some("rustyauto app".to_string());
        service_discovery_response.sw_build = Some("1".to_string());
        service_discovery_response.sw_version = Some("1.0".to_string());
        service_discovery_response.can_play_native_media_during_vr = Some(false);
        service_discovery_response.hide_clock = Some(false);

        temp_fill_service_features(&mut service_discovery_response, data);

        create_service_discovery_response_message(service_discovery_response);
        data.control_service_data.write().unwrap().service_discovery_state = ServiceDiscoveryState::Idle;
        receival_queue_tx.send(ReceivalRequest).unwrap();
    }
    if current_data.audio_focus_state == AudioFocusState::Requested {
        create_audio_focus_response_message();
        data.control_service_data.write().unwrap().audio_focus_state = AudioFocusState::Gained;
        receival_queue_tx.send(ReceivalRequest).unwrap();
    }
    if current_data.navigation_focus_requested {
        let mut navigation_focus_response_message = create_navigation_focus_response_message();
        messenger.cryptor.encrypt_message(&mut navigation_focus_response_message);
        messenger.send_message_via_usb(navigation_focus_response_message);
        data.control_service_data.write().unwrap().navigation_focus_requested = false;
        receival_queue_tx.send(ReceivalRequest).unwrap();
    }
}

pub fn create_version_request_message(own_version: crate::data::android_auto_entity::Version) -> Frame {
    log::info!("Creating version request message");
    let version_buffer = own_version.to_bytes();
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::VersionRequest as u16).to_be_bytes().to_vec();
    payload.extend_from_slice(&version_buffer);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_handshake_message(handshake_buffer: &[u8]) -> Frame {
    log::info!("Creating ssl handshake message");
    log::debug!("Handshake buffer: {:?}", handshake_buffer);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::SSLHandshake as u16).to_be_bytes().to_vec();
    payload.extend_from_slice(handshake_buffer);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_auth_complete_message() -> Frame {
    log::info!("Creating auth complete message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Plain,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut payload = (ControlMessageID::AuthComplete as u16).to_be_bytes().to_vec();
    payload.push(0x8);
    payload.push(0);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_service_discovery_response_message(service_discovery_response_message: crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse) -> Frame {
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
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_audio_focus_response_message() -> Frame {
    log::info!("Creating audio focus response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut audio_focus_response = crate::protos::AudioFocusResponseMessage::AudioFocusResponse::new();
    audio_focus_response.set_audio_focus_state(crate::protos::AudioFocusStateEnum::audio_focus_state::Enum::LOSS);
    let mut payload = (ControlMessageID::AudioFocusResponse as u16).to_be_bytes().to_vec();
    let mut bytes = audio_focus_response.write_to_bytes().unwrap();
    payload.extend(bytes);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}

pub fn create_navigation_focus_response_message() -> Frame {
    log::info!("Creating navigation focus response message");
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut navigation_focus_response = crate::protos::NavigationFocusResponseMessage::NavigationFocusResponse::new();
    //TODO: figure out types
    navigation_focus_response.set_type(2);
    let mut payload = (ControlMessageID::NavigationFocusResponse as u16).to_be_bytes().to_vec();
    let mut bytes = navigation_focus_response.write_to_bytes().unwrap();
    payload.extend(bytes);
    let message = messenger::frame::Frame { frame_header, channel_id: ChannelID::Control, payload };
    message
}
