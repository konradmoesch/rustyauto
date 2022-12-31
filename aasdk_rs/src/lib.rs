extern crate core;

mod constants;
pub mod usbdriver;
pub mod messenger;
pub mod channels;
pub mod cryptor;
mod utils;
pub mod services;
pub mod data;

/*pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/aasdk.proto.messages.rs"));
}
pub mod data {
    include!(concat!(env!("OUT_DIR"), "/aasdk.proto.data.rs"));
}
pub mod enums {
    include!(concat!(env!("OUT_DIR"), "/aasdk.proto.enums.rs"));
}*/

pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

#[cfg(test)]
mod tests {
    use crate::messenger::frame::{ChannelID, EncryptionType, FrameHeader, FrameType, Frame, MessageType};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_message_conversion() {
        let message = Frame {
            frame_header: FrameHeader {
                encryption_type: EncryptionType::Encrypted,
                message_type: MessageType::Control,
                frame_type: FrameType::Bulk
            },
            channel_id: ChannelID::Video,
            payload: vec![1,2,3]
        };
        let message_as_bytes = message.clone().to_byte_vector();
        assert_eq!(message_as_bytes, vec![3,15,0,4,0,1,2,3]);
        assert_eq!(Frame::from_data_frame(message_as_bytes.as_slice()), message);
    }

}
