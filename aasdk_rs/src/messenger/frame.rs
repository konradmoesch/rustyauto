#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EncryptionType {
    Plain = 0,
    //0
    Encrypted = 1 << 3, //1000
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
    //0
    Control = 0b100, //100
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
    Middle = 0, //0
    First = 1, //1
    Last = 2, //10
    Bulk = 3, //11
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
    fn get_frame_size_type(self) -> FrameSizeType {
        match self.frame_type {
            FrameType::First => FrameSizeType::Extended,
            _ => FrameSizeType::Short,
        }
    }
}

enum FrameSizeType {
    Short,
    Extended,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub frame_header: FrameHeader,
    pub channel_id: ChannelID,
    //pub payload: dyn Payload,
    pub payload: Vec<u8>,
}

impl Frame {
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
        let channel_id = ChannelID::from(data_frame[0]);
        let frame_header = FrameHeader::from(data_frame[1]);
        dbg!(frame_header.frame_type);
        let frame_size_type = frame_header.get_frame_size_type();
        let payload_start_byte_index = match frame_size_type {
            FrameSizeType::Short => 4,
            FrameSizeType::Extended => 8,
        };
        log::debug!("Processing data_frame: {:?}", data_frame);
        log::debug!("Starting bytes: {:?}", &data_frame[..payload_start_byte_index]);
        let payload_slice = &data_frame[payload_start_byte_index..];
        log::debug!("payload slice: {:?}", &payload_slice);
        get_size(data_frame);
        let to_return = Self {
            frame_header,
            channel_id,
            payload: payload_slice.to_vec(),
        };
        //log::debug!("Message: {:?}", to_return);
        to_return
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

fn get_size(data_frame: &[u8]) -> u16 {
    let size_bytes = [data_frame[2], data_frame[3]];
    let size = u16::from_be_bytes(size_bytes);
    dbg!(size);
    size
}
