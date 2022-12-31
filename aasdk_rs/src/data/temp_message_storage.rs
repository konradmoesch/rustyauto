use std::collections::VecDeque;
use crate::messenger::frame::{ChannelID, Frame, FrameType};
use crate::services::service::Service;

struct FrameQueue(VecDeque<Frame>);

impl FrameQueue {
    fn get_full_frame(&mut self) -> Frame {
        if self.0.iter().last().unwrap().frame_header.frame_type != FrameType::Last {
            panic!("Last frame in queue is missing");
        } else {
            let first_frame = self.0.pop_front().unwrap();
            let mut frame_header = first_frame.frame_header;
            frame_header.frame_type = FrameType::Bulk;
            let channel_id = first_frame.channel_id;
            let mut size = first_frame.payload.len();
            let mut payload = first_frame.payload;
            while let Some(mut frame) = self.0.pop_front() {
                log::debug!("Appending {:?} frame", frame.frame_header.frame_type);
                size += frame.payload.len();
                payload.append(&mut frame.payload);
            }
            Frame {
                frame_header,
                channel_id,
                payload,
            }
        }
    }
    fn new() -> Self {
        FrameQueue { 0: VecDeque::new() }
    }
}

pub struct TempMessageStorage {
    video_queue: FrameQueue,
}

impl TempMessageStorage {
    pub fn new() -> Self {
        TempMessageStorage {
            video_queue: FrameQueue::new(),
        }
    }
    pub fn insert_message(&mut self, message: Frame) {
        match message.channel_id {
            ChannelID::Video => self.video_queue.0.push_back(message),
            _ => unimplemented!()
        }
    }
    pub fn get_message(&mut self, channel_id: ChannelID) -> Frame {
        match channel_id {
            ChannelID::Video => self.video_queue.get_full_frame(),
            _ => unimplemented!()
        }
    }
    pub fn get_message_count(&mut self, channel_id: ChannelID) -> usize {
        match channel_id {
            ChannelID::Video => self.video_queue.0.len(),
            _ => unimplemented!()
        }
    }
}