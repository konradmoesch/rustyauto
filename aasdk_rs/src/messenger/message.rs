use crate::channels;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EncryptionType {
    Plain = 0,
    Encrypted = 1 << 3,
}

impl From<u8> for EncryptionType {
    fn from(encryption_type_as_byte: u8) -> Self {
        match encryption_type_as_byte {
            0 => EncryptionType::Plain,
            _ => EncryptionType::Encrypted,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MessageType {
    Specific = 0,
    Control = 0b100,
}

impl From<u8> for MessageType {
    fn from(message_type_as_byte: u8) -> Self {
        match message_type_as_byte {
            0 => MessageType::Specific,
            _ => MessageType::Control,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameType {
    Middle = 0,
    First = 1,
    Last = 2,
    Bulk = 3,
}

impl From<u8> for FrameType {
    fn from(frame_type_as_byte: u8) -> Self {
        match frame_type_as_byte {
            0 => FrameType::Middle,
            1 => FrameType::First,
            2 => FrameType::Last,
            _ => FrameType::Bulk,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FrameHeader {
    pub encryption_type: EncryptionType,
    pub message_type: MessageType,
    pub frame_type: FrameType,
}

impl From<u8> for FrameHeader {
    fn from(frame_header_as_byte: u8) -> Self {
        FrameHeader {
            encryption_type: EncryptionType::from(frame_header_as_byte & EncryptionType::Encrypted as u8),
            message_type: MessageType::from(frame_header_as_byte & MessageType::Control as u8),
            frame_type: FrameType::from(frame_header_as_byte & FrameType::Bulk as u8),
        }
    }
}

impl FrameHeader {
    pub fn to_byte(self) -> u8 {
        let byte = self.encryption_type as u8 | self.message_type as u8 | self.frame_type as u8;
        return byte;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub frame_header: FrameHeader,
    pub channel_id: ChannelID,
    //pub payload: dyn Payload,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn to_byte_vector(self) -> Vec<u8> {
        let mut byte_vector = vec![self.channel_id as u8];
        let payload_length = self.payload.len();
        let lower_byte = payload_length as u8;
        let upper_byte = (payload_length >> 8) as u8;
        byte_vector.push(self.frame_header.to_byte());
        byte_vector.push(upper_byte);
        byte_vector.push(lower_byte);
        byte_vector.extend(self.payload);
        //buffer = byte_vector.as_slice();
        return byte_vector;
    }

    pub fn from_data_frame(data_frame: &[u8]) -> Self {
        //log::debug!("Processing data_frame: {:?}", data_frame);
        let payload_slice = &data_frame[4..];
        let to_return = Self {
            frame_header: FrameHeader::from(data_frame[1]),
            channel_id: ChannelID::from(data_frame[0]),
            payload: payload_slice.to_vec(),
        };
        //log::debug!("Message: {:?}", to_return);
        to_return
    }

    pub fn handle(&self) {
        /*if self.frame_header.encryption_type == EncryptionType::Encrypted {
            log::info!("Encrypted message, decrypting now");
        } else {*/
        log::debug!("Channel ID: {:?}", self.channel_id);
        match self.channel_id {
            ChannelID::Control => { channels::control_service_channel::handle_message(self) }
            ChannelID::AVInput => { channels::av_input_service_channel::handle_message(self) }
            ChannelID::MediaAudio => { channels::media_audio_service_channel::handle_message(self) }
            ChannelID::SpeechAudio => { channels::speech_audio_service_channel::handle_message(self) }
            ChannelID::SystemAudio => { channels::system_audio_service_channel::handle_message(self) }
            ChannelID::Sensor => { channels::sensor_service_channel::handle_message(self) }
            ChannelID::Video => { channels::video_service_channel::handle_message(self) }
            ChannelID::Input => { channels::input_service_channel::handle_message(self) }
            _ => { todo!() }
        }
        //}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChannelID {
    Control = 0,
    Input = 1,
    Sensor = 2,
    Video = 3,
    MediaAudio = 4,
    SpeechAudio = 5,
    SystemAudio = 6,
    AVInput = 7,
    Bluetooth = 8,
    None = 255,
}

impl From<u8> for ChannelID {
    fn from(channel_id_as_byte: u8) -> Self {
        match channel_id_as_byte {
            0 => ChannelID::Control,
            1 => ChannelID::Input,
            2 => ChannelID::Sensor,
            3 => ChannelID::Video,
            4 => ChannelID::MediaAudio,
            5 => ChannelID::SpeechAudio,
            6 => ChannelID::SystemAudio,
            7 => ChannelID::AVInput,
            8 => ChannelID::Bluetooth,
            _ => ChannelID::None
        }
    }
}
