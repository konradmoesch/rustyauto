use crate::channels;
use crate::data::android_auto_entity::AndroidAutoEntityData;
use crate::messenger::frame::{ChannelID, Frame, FrameType};

pub fn handle_message(message: &Frame, data: &mut AndroidAutoEntityData) {
    match message.frame_header.frame_type {
        FrameType::Bulk => {
            log::debug!("bulk message, handling");
            log::debug!("Channel ID: {:?}", message.channel_id);
            match message.channel_id {
                ChannelID::Control => { channels::control::control_channel_message_handler::handle_message(message, data) }
                ChannelID::AVInput => { channels::av_input::av_input_channel_message_handler::handle_message(message, data) }
                ChannelID::MediaAudio => { channels::media_audio::media_audio_channel_message_handler::handle_message(message, data) }
                ChannelID::SpeechAudio => { channels::speech_audio::speech_audio_channel_message_handler::handle_message(message, data) }
                ChannelID::SystemAudio => { channels::system_audio::system_audio_channel_message_handler::handle_message(message, data) }
                ChannelID::Sensor => { channels::sensor::sensor_channel_message_handler::handle_message(message, data) }
                ChannelID::Video => { channels::video::video_channel_message_handler::handle_message(message, data) }
                ChannelID::Input => { channels::input::input_channel_message_handler::handle_message(message, data) }
                _ => { todo!() }
            }
        }
        _ => {
            log::debug!("non-bulk message, handling");
            data.temp_message_storage.write().unwrap().insert_message(message.clone());
            if message.frame_header.frame_type == FrameType::Last {
                let full_message = data.temp_message_storage.write().unwrap().get_message(message.channel_id);
                handle_message(&full_message, data);
            }
            *data.receive_more.write().unwrap() = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::channels::av_input::av_input_service_channel::AVMessageID;
    use crate::data::android_auto_entity::AndroidAutoConfig;
    use crate::messenger::frame::{EncryptionType, FrameHeader, MessageType};

    use super::*;

    #[test]
    fn test_non_bulk_messages() {
        let mut dummy_data = AndroidAutoEntityData::new(AndroidAutoConfig {
            head_unit_name: "".to_string(),
            car_model: "".to_string(),
            car_year: "".to_string(),
            car_serial: "".to_string(),
            left_hand_drive_vehicle: false,
            headunit_manufacturer: "".to_string(),
            headunit_model: "".to_string(),
            sw_build: "".to_string(),
            sw_version: "".to_string(),
            can_play_native_media_during_vr: false,
            hide_clock: false,
        });
        let first_frame = Frame {
            frame_header: FrameHeader {
                encryption_type: EncryptionType::Plain,
                message_type: MessageType::Specific,
                frame_type: FrameType::First,
            },
            channel_id: ChannelID::Video,
            payload: vec![0, AVMessageID::AvMediaIndication as u8, 1, 2, 3],
        };
        assert_eq!(*dummy_data.receive_more.read().unwrap(), false);
        assert_eq!(dummy_data.temp_message_storage.write().unwrap().get_message_count(ChannelID::Video), 0);
        handle_message(&first_frame, &mut dummy_data);
        assert_eq!(*dummy_data.receive_more.read().unwrap(), true);
        assert_eq!(dummy_data.temp_message_storage.write().unwrap().get_message_count(ChannelID::Video), 1);
        *dummy_data.receive_more.write().unwrap() = false;
        let middle_frame_1 = Frame {
            frame_header: FrameHeader {
                encryption_type: EncryptionType::Plain,
                message_type: MessageType::Specific,
                frame_type: FrameType::Middle,
            },
            channel_id: ChannelID::Video,
            payload: vec![4, 5, 6],
        };
        handle_message(&middle_frame_1, &mut dummy_data);
        assert_eq!(*dummy_data.receive_more.read().unwrap(), true);
        assert_eq!(dummy_data.temp_message_storage.write().unwrap().get_message_count(ChannelID::Video), 2);
        *dummy_data.receive_more.write().unwrap() = false;
        let middle_frame_2 = Frame {
            frame_header: FrameHeader {
                encryption_type: EncryptionType::Plain,
                message_type: MessageType::Specific,
                frame_type: FrameType::Middle,
            },
            channel_id: ChannelID::Video,
            payload: vec![7, 8, 9],
        };
        handle_message(&middle_frame_2, &mut dummy_data);
        assert_eq!(*dummy_data.receive_more.read().unwrap(), true);
        assert_eq!(dummy_data.temp_message_storage.write().unwrap().get_message_count(ChannelID::Video), 3);
        *dummy_data.receive_more.write().unwrap() = false;
        let last_frame = Frame {
            frame_header: FrameHeader {
                encryption_type: EncryptionType::Plain,
                message_type: MessageType::Specific,
                frame_type: FrameType::Last,
            },
            channel_id: ChannelID::Video,
            payload: vec![0],
        };
        handle_message(&last_frame, &mut dummy_data);
        assert_eq!(*dummy_data.receive_more.read().unwrap(), true);
        assert_eq!(dummy_data.temp_message_storage.write().unwrap().get_message_count(ChannelID::Video), 0);

        /*let full_message = dummy_data.temp_message_storage.write().unwrap().get_message(ChannelID::Video);
        assert_eq!(full_message.frame_header.frame_type, FrameType::Bulk);
        assert_eq!(full_message.payload, vec![0, AVMessageID::AvMediaIndication as u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);*/
    }
}
