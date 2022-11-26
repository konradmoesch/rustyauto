use crate::data::messenger::MessengerStatus;
use crate::data::services::audio_input_service_data::AudioInputServiceData;
use crate::data::services::input_service_data::InputServiceData;
use crate::data::services::media_audio_service_data::MediaAudioServiceData;
use crate::data::services::sensor_service_data::SensorServiceData;
use crate::data::services::speech_audio_service_data::SpeechAudioServiceData;
use crate::data::services::system_audio_service_data::SystemAudioServiceData;
use crate::data::services::video_service_data::VideoServiceData;

#[derive(PartialEq, Debug)]
pub enum AutoEntityStatus {
    Uninitialized,
    Initialized,
}

pub struct AndroidAutoConfig {
    pub head_unit_name: String,
    pub car_model: String,
    pub car_year: String,
    pub car_serial: String,
    pub left_hand_drive_vehicle: bool,
    pub headunit_manufacturer: String,
    pub headunit_model: String,
    pub sw_build: String,
    pub sw_version: String,
    pub can_play_native_media_during_vr: bool,
    pub hide_clock: bool,
}

pub struct Version {
    pub major: u16,
    pub minor: u16,
}

pub struct VersionStatus {
    pub own_version: Version,
    pub remote_version: Version,
    pub version_match: bool,
}

pub struct AndroidAutoEntityData {
    pub status: AutoEntityStatus,
    pub messenger_status: MessengerStatus,
    pub version: VersionStatus,
    config: AndroidAutoConfig,
    audio_input_service_data: AudioInputServiceData,
    media_audio_service_data: MediaAudioServiceData,
    speech_audio_service_data: SpeechAudioServiceData,
    system_audio_service_data: SystemAudioServiceData,
    sensor_service_data: SensorServiceData,
    video_service_data: VideoServiceData,
    //bluetooth_service_data: BluetoothServiceData,
    input_service_data: InputServiceData,
    //wifi_service_data: WifiServiceData,
}

impl AndroidAutoEntityData {
    pub fn new(config: AndroidAutoConfig) -> Self {
        AndroidAutoEntityData {
            status: AutoEntityStatus::Uninitialized,
            messenger_status: MessengerStatus::Uninitialized,
            version: VersionStatus {
                own_version: Version { major: 1, minor: 0 },
                remote_version: Version { major: 0, minor: 0 },
                version_match: false,
            },
            config,
            audio_input_service_data: AudioInputServiceData::new(),
            media_audio_service_data: MediaAudioServiceData::new(),
            speech_audio_service_data: SpeechAudioServiceData::new(),
            system_audio_service_data: SystemAudioServiceData::new(),
            sensor_service_data: SensorServiceData::new(),
            video_service_data: VideoServiceData::new(),
            input_service_data: InputServiceData::new(),
        }
    }
}
