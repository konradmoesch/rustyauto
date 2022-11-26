use std::sync::MutexGuard;
use crate::channels;
use crate::cryptor::Cryptor;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::data::messenger::MessengerStatus;
use crate::messenger::message::Message;
use crate::usbdriver::UsbDriver;

pub struct Messenger {
    pub cryptor: Cryptor,
    pub usb_driver: UsbDriver,
}

impl Messenger {
    pub fn run(&mut self, data: &mut AndroidAutoEntityData) {
        match data.messenger_status {
            MessengerStatus::Uninitialized => {
                log::info!("Doing version check now");
                let version_request_message = channels::control_service_channel::create_version_request_message(&data.version.own_version);
                self.send_message(version_request_message);
                let received_message = self.receive_message();
                received_message.handle(data);
            }
            MessengerStatus::VersionRequestDone => {
                log::info!("Doing handshake now");

                self.cryptor.do_handshake();

                log::info!("Sending handshake message");
                let handshake_message = channels::control_service_channel::create_handshake_message(self.cryptor.read_handshake_buffer().as_slice());
                log::debug!("{:?}", handshake_message);
                self.send_message(handshake_message);
                let received_message = self.receive_message();
                received_message.handle(data);
                let bla = received_message.payload[2..].to_vec();
                log::info!("{:?}", bla);
                self.cryptor.write_handshake_buffer(bla.as_slice());
                self.cryptor.do_handshake();

                log::info!("Continuing handshake");
                let handshake_message = channels::control_service_channel::create_handshake_message(self.cryptor.read_handshake_buffer().as_slice());
                log::debug!("{:?}", handshake_message);
                self.send_message(handshake_message);

                let received_message = self.receive_message();
                received_message.handle(data);
                let bla = received_message.payload[2..].to_vec();
                log::info!("{:?}", bla);
                self.cryptor.write_handshake_buffer(bla.as_slice());
                self.cryptor.do_handshake();

                //TODO: move to correct place
                data.messenger_status = MessengerStatus::AuthCompleted;
                log::debug!("Auth completed");
            }
            MessengerStatus::AuthCompleted => {}
            MessengerStatus::InitializationDone => {}
        }
    }
    pub fn receive_message(&self) -> Message {
        //todo: Max packet size?
        let mut in_buffer = vec![0u8; 9999];
        let size = self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        in_buffer.truncate(size);
        let received_message = Message::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn send_message(&mut self, message_to_send: Message) {
        self.usb_driver.send_buffer(message_to_send.to_byte_vector().as_slice());
    }
}
