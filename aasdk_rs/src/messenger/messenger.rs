use std::sync::MutexGuard;

use crate::channels;
use crate::cryptor::Cryptor;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::{HandshakeStatus, MessengerStatus};
use crate::data::services::service_data::{ServiceData, ServiceType};
use crate::messenger::message::Message;
use crate::usbdriver::UsbDriver;

pub struct Messenger {
    pub cryptor: Cryptor,
    pub usb_driver: UsbDriver,
}

fn fill_service_features(sdr: &mut crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse, data: &mut AndroidAutoEntityData) {
    //AudioInput
    let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::AVInput as u32);

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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::MediaAudio as u32);

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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::SpeechAudio as u32);

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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::SystemAudio as u32);

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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Sensor as u32);

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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Video as u32);
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
    channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Input as u32);

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

impl Messenger {
    pub fn run(&mut self, data: &mut AndroidAutoEntityData) {
        let current_messenger_status = (*data.messenger_status.read().unwrap()).clone();
        match current_messenger_status {
            MessengerStatus::Uninitialized => {
                log::info!("Doing version check now");
                let version_request_message = channels::control::control_service_channel::create_version_request_message(data.version.read().unwrap().own_version);
                self.send_message_via_usb(version_request_message);
                let received_message = self.receive_message();
                received_message.handle(data);
            }
            MessengerStatus::VersionRequestDone => {
                log::info!("Doing handshake now");

                let mut handshake_status = HandshakeStatus::Unfinished;
                while handshake_status != HandshakeStatus::Complete {
                    handshake_status = self.cryptor.do_handshake();

                    match handshake_status {
                        HandshakeStatus::Unfinished => {
                            log::info!("continuing handshake");
                            let handshake_message = channels::control::control_service_channel::create_handshake_message(self.cryptor.read_handshake_buffer().as_slice());
                            log::debug!("{:?}", handshake_message);
                            self.send_message_via_usb(handshake_message);
                            let received_message = self.receive_message();
                            received_message.handle(data);
                            let received_handshake_message = received_message.payload[2..].to_vec();
                            log::info!("{:?}", received_handshake_message);
                            self.cryptor.write_handshake_buffer(received_handshake_message.as_slice());
                        }
                        HandshakeStatus::Complete => {
                            log::info!("handshake complete!");
                        }
                        HandshakeStatus::Error => { panic!("Unrecoverable error in ssl handshake!") }
                    }
                };

                //TODO: move to correct place
                *data.messenger_status.write().unwrap() = MessengerStatus::AuthCompleted;
                log::debug!("Auth completed");
            }
            MessengerStatus::AuthCompleted => {
                let auth_complete_message = channels::control::control_service_channel::create_auth_complete_message();
                log::debug!("auth complete message: {:?}", auth_complete_message);
                self.send_message_via_usb(auth_complete_message);

                let mut received_message = self.receive_message();
                received_message.handle(data);

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

                fill_service_features(&mut service_discovery_response, data);

                let mut service_discovery_response_message = channels::control::control_service_channel::create_service_discovery_response_message(service_discovery_response);
                self.cryptor.encrypt_message(&mut service_discovery_response_message);
                self.send_message_via_usb(service_discovery_response_message);

                let mut received_message = self.receive_message_via_usb();
                self.cryptor.decrypt_message(&mut received_message);
                received_message.handle(data);

                let mut audio_focus_response_message = channels::control::control_service_channel::create_audio_focus_response_message();
                self.cryptor.encrypt_message(&mut audio_focus_response_message);
                self.send_message_via_usb(audio_focus_response_message);

                *data.messenger_status.write().unwrap() = MessengerStatus::InitializationDone;
            }
            MessengerStatus::InitializationDone => {
                //run channels
            }
        }
    }
    fn receive_message_via_usb(&self) -> Message {
        //todo: Max packet size?
        let mut in_buffer = vec![0u8; 9999];
        let size = self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        in_buffer.truncate(size);
        let received_message = Message::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn receive_message(&mut self) -> Message {
        let mut raw_message = self.receive_message_via_usb();
        self.cryptor.decrypt_message(&mut raw_message);
        raw_message
    }
    pub fn send_message_via_usb(&mut self, message_to_send: Message) {
        self.usb_driver.send_buffer(message_to_send.to_byte_vector().as_slice());
    }
}
