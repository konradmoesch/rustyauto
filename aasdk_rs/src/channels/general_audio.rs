use protobuf::Message;
use crate::channels::av_input::av_input_service_channel::AVMessageID;
use crate::messenger::frame::{ChannelID, EncryptionType, Frame, FrameHeader, FrameType, MessageType};

pub fn create_av_channel_setup_response(channel_id: ChannelID) -> Frame {
    log::info!("Creating av channel setup response message for channel {:?}", channel_id);
    let frame_header = FrameHeader {
        encryption_type: EncryptionType::Encrypted,
        message_type: MessageType::Specific,
        frame_type: FrameType::Bulk,
    };
    let mut av_channel_setup_response = crate::protos::AVChannelSetupResponseMessage::AVChannelSetupResponse::new();
    av_channel_setup_response.set_media_status(crate::protos::AVChannelSetupStatusEnum::avchannel_setup_status::Enum::OK);
    av_channel_setup_response.set_max_unacked(1);
    av_channel_setup_response.configs.push(0);
    let mut payload = (AVMessageID::SetupResponse as u16).to_be_bytes().to_vec();
    let mut bytes = av_channel_setup_response.write_to_bytes().unwrap();
    //println!("{:x?}", bytes);
    payload.extend(bytes);
    //println!("{:x?}", payload);
    let message = crate::messenger::frame::Frame { frame_header, channel_id, payload };
    message
}