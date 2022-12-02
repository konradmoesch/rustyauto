use std::collections::VecDeque;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::u16;

use crate::channels;
use crate::messenger::frame::Frame;
use crate::messenger::frame::EncryptionType::{Encrypted, Plain};
use crate::messenger::frame::FrameType::{Bulk, First, Last, Middle};
use crate::messenger::frame::MessageType::{Control, Specific};
use crate::usbdriver::UsbDriver;


pub struct LegacyMessenger {
    usb_driver: UsbDriver,
}

impl LegacyMessenger {
    pub fn init(usb_driver: UsbDriver) -> Self {
        LegacyMessenger { usb_driver }
    }
    pub fn receive_message(&self, size: usize) -> Frame {
        let mut in_buffer = vec![0u8; size];
        self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        let received_message = Frame::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn receive_message_without_size(&self) -> Frame {
        let mut in_buffer = vec![0u8; 9999];
        let size = self.usb_driver.read_buffer(in_buffer.as_mut_slice());
        in_buffer.truncate(size);
        let received_message = Frame::from_data_frame(in_buffer.as_slice());
        received_message
    }
    pub fn send_message(&mut self, message_to_send: Frame) {
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
    /*pub fn run(&mut self, in_rx: &Receiver<i32>, out_rx: &Receiver<Message>) {
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
                //received_message.handle(&);
            }
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty { log::error!("Error receiving on in_rx: {}",e); }
            }
        }
    }
    pub fn enqueue_receive(&mut self, to_receive: Message) {
        self.in_queue.push_back(to_receive);
    }
    pub fn enqueue_send(&mut self, to_send: Message) {
        self.out_queue.push_back(to_send);
    }*/
}
