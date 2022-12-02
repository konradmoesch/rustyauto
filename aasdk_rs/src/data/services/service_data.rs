pub trait ServiceData {
    fn new() -> Self;
    fn get_type(&self) -> ServiceType;
}

#[derive(Copy, Clone)]
pub enum ServiceType {
    AudioInput,
    Input,
    MediaAudio,
    Sensor,
    SpeechAudio,
    SystemAudio,
    Video,
}