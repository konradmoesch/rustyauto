use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::u16;

use crate::channels;
use crate::messenger::message::Message;
use crate::messenger::message::EncryptionType::{Encrypted, Plain};
use crate::messenger::message::FrameType::{Bulk, First, Last, Middle};
use crate::messenger::message::MessageType::{Control, Specific};
use crate::usbdriver::UsbDriver;


pub struct LegacyMessenger {
    usb_driver: UsbDriver,
}

impl LegacyMessenger {
    pub fn init(usb_driver: UsbDriver) -> Self {
        LegacyMessenger { usb_driver }
    }
    pub fn receive_message(&self, size: usize) -> Message {
        let mut in_buffer = vec![0u8; size];
        self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        let received_message = Message::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn send_message(&mut self, message_to_send: Message) {
        self.usb_driver.send_buffer(message_to_send.to_byte_vector().as_slice());
    }
}

pub struct Messenger {
    //in_queue: Receiver<Message>,
    //out_queue: Receiver<Message>,
    usb_driver: UsbDriver,
}

impl Messenger {
    pub fn init(usb_driver: UsbDriver) -> Self {
        Messenger { usb_driver }
    }
    pub fn run(&mut self, in_rx: &Receiver<i32>, out_rx: &Receiver<Message>) {
        //log::debug!("Running");
        if let Ok(message_to_send) = out_rx.try_recv() {
            log::debug!("Received message to send!");
            self.usb_driver.send_buffer(message_to_send.to_byte_vector().as_slice());
        }
        match in_rx.try_recv() {
            Ok(message_to_receive) => {
                log::debug!("Received message to recv!");
                let mut in_buffer = vec![0u8; 10000];
                self.usb_driver.read_buffer(in_buffer.as_mut_slice());
                let received_message = Message::from_data_frame(in_buffer.as_slice());
                received_message.handle();
            }
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty { log::error!("Error receiving on in_rx: {}",e); }
            }
        }
    }/*
    pub fn enqueue_receive(&mut self, to_receive: Message) {
        self.in_queue.push_back(to_receive);
    }
    pub fn enqueue_send(&mut self, to_send: Message) {
        self.out_queue.push_back(to_send);
    }*/
}
