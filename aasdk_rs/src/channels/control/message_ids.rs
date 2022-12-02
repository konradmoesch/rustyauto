
#[derive(Debug)]
pub enum ControlMessageID
{
    None = 0x0000,
    VersionRequest = 0x0001,
    VersionResponse = 0x0002,
    SSLHandshake = 0x0003,
    AuthComplete = 0x0004,
    ServiceDiscoveryRequest = 0x0005,
    ServiceDiscoveryResponse = 0x0006,
    ChannelOpenRequest = 0x0007,
    ChannelOpenResponse = 0x0008,
    PingRequest = 0x000b,
    PingResponse = 0x000c,
    NavigationFocusRequest = 0x000d,
    NavigationFocusResponse = 0x000e,
    ShutdownRequest = 0x000f,
    ShutdownResponse = 0x0010,
    VoiceSessionRequest = 0x0011,
    AudioFocusRequest = 0x0012,
    AudioFocusResponse = 0x0013,
}
impl From<u8> for ControlMessageID {
    fn from(message_id_as_byte: u8) -> Self {
        match message_id_as_byte {
            0x0000 => { ControlMessageID::None }
            0x0001 => { ControlMessageID::VersionRequest }
            0x0002 => { ControlMessageID::VersionResponse }
            0x0003 => { ControlMessageID::SSLHandshake }
            0x0004 => { ControlMessageID::AuthComplete }
            0x0005 => { ControlMessageID::ServiceDiscoveryRequest }
            0x0006 => { ControlMessageID::ServiceDiscoveryResponse }
            0x0007 => { ControlMessageID::ChannelOpenRequest }
            0x0008 => { ControlMessageID::ChannelOpenResponse }
            0x000b => { ControlMessageID::PingRequest }
            0x000c => { ControlMessageID::PingResponse }
            0x000d => { ControlMessageID::NavigationFocusRequest }
            0x000e => { ControlMessageID::NavigationFocusResponse }
            0x000f => { ControlMessageID::ShutdownRequest }
            0x0010 => { ControlMessageID::ShutdownResponse }
            0x0011 => { ControlMessageID::VoiceSessionRequest }
            0x0012 => { ControlMessageID::AudioFocusRequest }
            0x0013 => { ControlMessageID::AudioFocusResponse }
            _ => { ControlMessageID::None }
        }
    }
}
impl Into<u16> for ControlMessageID {
    fn into(self) -> u16 {
        match self {
            ControlMessageID::None => {255}
            ControlMessageID::VersionRequest => {0x0001}
            ControlMessageID::VersionResponse => {0x0002}
            ControlMessageID::SSLHandshake => {0x0003}
            ControlMessageID::AuthComplete => {0x0004}
            ControlMessageID::ServiceDiscoveryRequest => {0x0005}
            ControlMessageID::ServiceDiscoveryResponse => {0x0006}
            ControlMessageID::ChannelOpenRequest => {0x0007}
            ControlMessageID::ChannelOpenResponse => {0x0008}
            ControlMessageID::PingRequest => {0x000b}
            ControlMessageID::PingResponse => {0x000c}
            ControlMessageID::NavigationFocusRequest => {0x000d}
            ControlMessageID::NavigationFocusResponse => {0x000e}
            ControlMessageID::ShutdownRequest => {0x000f}
            ControlMessageID::ShutdownResponse => {0x0010}
            ControlMessageID::VoiceSessionRequest => {0x0011}
            ControlMessageID::AudioFocusRequest => {0x0012}
            ControlMessageID::AudioFocusResponse => {0x0013}
        }
    }
}