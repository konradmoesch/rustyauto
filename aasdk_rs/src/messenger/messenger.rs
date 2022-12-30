use std::sync::mpsc::{channel, Receiver, Sender};

use crate::channels;
use crate::cryptor::Cryptor;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::{HandshakeStatus, MessengerStatus};
use crate::data::services::control_service_data::{AudioFocusState, ServiceDiscoveryState};
use crate::data::services::service_data::{ServiceData, ServiceType};
use crate::messenger::frame::Frame;
use crate::usbdriver::UsbDriver;

#[derive(Debug)]
pub struct ReceivalRequest;

pub struct ReceivalQueue {
    tx: Sender<ReceivalRequest>,
    rx: Receiver<ReceivalRequest>,
}

impl ReceivalQueue {
    pub fn new() -> Self {
        let (tx, rx) = channel::<ReceivalRequest>();
        Self {
            tx,
            rx,
        }
    }
}

pub struct Messenger {
    pub cryptor: Cryptor,
    pub usb_driver: UsbDriver,
    pub receival_queue: ReceivalQueue,
}

impl Messenger {
    fn receive_messages(&mut self, data: &mut AndroidAutoEntityData) {
        while let Ok(ReceivalRequest) = self.receival_queue.rx.try_recv() {
            log::info!("Have to receive a message, doing this now");
            let received_message = self.receive_and_decrypt_message();
            crate::messenger::message_handler::handle_message(&received_message, data);
        }
        log::debug!("Finished receiving messages");
    }
    pub fn run(&mut self, data: &mut AndroidAutoEntityData) {
        let current_messenger_status = (*data.messenger_status.read().unwrap()).clone();
        log::debug!("Messenger running now; current status: {:?}", current_messenger_status);
        self.receive_messages(data);
        match current_messenger_status {
            MessengerStatus::Uninitialized => {
                log::info!("Doing version check now");
                let version_request_message = channels::control::control_service_channel::create_version_request_message(data.version.read().unwrap().own_version);
                self.send_message_via_usb(version_request_message);
                let received_message = self.receive_and_decrypt_message();
                crate::messenger::message_handler::handle_message(&received_message, data);
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
                            let received_message = self.receive_and_decrypt_message();
                            crate::messenger::message_handler::handle_message(&received_message, data);
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

                let mut received_message = self.receive_and_decrypt_message();
                crate::messenger::message_handler::handle_message(&received_message, data);

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

                crate::channels::control::control_service_channel::temp_fill_service_features(&mut service_discovery_response, data);

                let mut service_discovery_response_message = channels::control::control_service_channel::create_service_discovery_response_message(service_discovery_response);
                self.cryptor.encrypt_message(&mut service_discovery_response_message);
                self.send_message_via_usb(service_discovery_response_message);

                data.control_service_data.write().unwrap().service_discovery_state = ServiceDiscoveryState::Idle;
                log::info!("Sent SD-response");

                let mut received_message = self.receive_message_via_usb();
                self.cryptor.decrypt_message(&mut received_message);
                crate::messenger::message_handler::handle_message(&received_message, data);

                let mut audio_focus_response_message = channels::control::control_service_channel::create_audio_focus_response_message();
                self.cryptor.encrypt_message(&mut audio_focus_response_message);
                self.send_message_via_usb(audio_focus_response_message);

                data.control_service_data.write().unwrap().audio_focus_state = AudioFocusState::Lost;
                log::info!("Sent audio focus response");

                self.receival_queue.tx.send(ReceivalRequest).unwrap();

                *data.messenger_status.write().unwrap() = MessengerStatus::InitializationDone;
            }
            MessengerStatus::InitializationDone => {
                //run channels
                channels::control::control_service_channel::run(data, self.receival_queue.tx.clone());
                channels::av_input::av_input_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::media_audio::media_audio_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::speech_audio::speech_audio_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::system_audio::system_audio_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::sensor::sensor_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::input::input_service_channel::run(data, self.receival_queue.tx.clone(), self);
                channels::video::video_service_channel::run(data, self.receival_queue.tx.clone(), self);
            }
        }
    }
    fn receive_message_via_usb(&self) -> Frame {
        //todo: Max packet size?
        let mut in_buffer = vec![0u8; 9999];
        let size = self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        in_buffer.truncate(size);
        let received_message = Frame::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn receive_and_decrypt_message(&mut self) -> Frame {
        let mut raw_message = self.receive_message_via_usb();
        self.cryptor.decrypt_message(&mut raw_message);
        raw_message
    }
    pub fn send_message_via_usb(&mut self, message_to_send: Frame) {
        self.usb_driver.send_buffer(message_to_send.to_byte_vector().as_slice());
    }
}
