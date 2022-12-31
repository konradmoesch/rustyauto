use std::sync::{Arc, RwLock};

use crate::data::messenger::MessengerStatus;
use crate::data::services::audio_input_service_data::AudioInputServiceData;
use crate::data::services::control_service_data::ControlServiceData;
use crate::data::services::input_service_data::InputServiceData;
use crate::data::services::media_audio_service_data::MediaAudioServiceData;
use crate::data::services::sensor_service_data::SensorServiceData;
use crate::data::services::speech_audio_service_data::SpeechAudioServiceData;
use crate::data::services::system_audio_service_data::SystemAudioServiceData;
use crate::data::services::video_service_data::VideoServiceData;
use crate::data::temp_message_storage::TempMessageStorage;

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Version {
    pub fn to_bytes(&self) -> [u8; 4] {
        return [self.major.to_be_bytes()[0], self.major.to_be_bytes()[1], self.minor.to_be_bytes()[0], self.minor.to_be_bytes()[1]];
    }
}

pub struct VersionStatus {
    pub own_version: Version,
    pub remote_version: Version,
    pub version_match: bool,
}

pub struct AndroidAutoEntityData {
    pub status: Arc<RwLock<AutoEntityStatus>>,
    pub messenger_status: Arc<RwLock<MessengerStatus>>,
    pub version: Arc<RwLock<VersionStatus>>,
    config: Arc<AndroidAutoConfig>,
    pub control_service_data: Arc<RwLock<ControlServiceData>>,
    pub audio_input_service_data: Arc<RwLock<AudioInputServiceData>>,
    pub media_audio_service_data: Arc<RwLock<MediaAudioServiceData>>,
    pub speech_audio_service_data: Arc<RwLock<SpeechAudioServiceData>>,
    pub system_audio_service_data: Arc<RwLock<SystemAudioServiceData>>,
    pub sensor_service_data: Arc<RwLock<SensorServiceData>>,
    pub video_service_data: Arc<RwLock<VideoServiceData>>,
    //pub bluetooth_service_data: BluetoothServiceData,
    pub input_service_data: Arc<RwLock<InputServiceData>>,
    //pub wifi_service_data: WifiServiceData,
    pub temp_message_storage: Arc<RwLock<TempMessageStorage>>,
    pub receive_more: Arc<RwLock<bool>>,
}

impl Clone for AndroidAutoEntityData {
    fn clone(&self) -> Self {
        AndroidAutoEntityData {
            status: self.status.clone(),
            messenger_status: self.messenger_status.clone(),
            version: self.version.clone(),
            config: self.config.clone(),
            control_service_data: self.control_service_data.clone(),
            audio_input_service_data: self.audio_input_service_data.clone(),
            media_audio_service_data: self.media_audio_service_data.clone(),
            speech_audio_service_data: self.speech_audio_service_data.clone(),
            system_audio_service_data: self.system_audio_service_data.clone(),
            sensor_service_data: self.sensor_service_data.clone(),
            video_service_data: self.video_service_data.clone(),
            input_service_data: self.input_service_data.clone(),
            temp_message_storage: self.temp_message_storage.clone(),
            receive_more: self.receive_more.clone(),
        }
    }
}

impl AndroidAutoEntityData {
    pub fn new(config: AndroidAutoConfig) -> Self {
        AndroidAutoEntityData {
            status: Arc::new(RwLock::new(AutoEntityStatus::Uninitialized)),
            messenger_status: Arc::new(RwLock::new(MessengerStatus::Uninitialized)),
            version: Arc::new(RwLock::new(VersionStatus {
                own_version: Version { major: 1, minor: 0 },
                remote_version: Version { major: 0, minor: 0 },
                version_match: false,
            })),
            config: Arc::new(config),
            control_service_data: Arc::new(RwLock::new(ControlServiceData::new())),
            audio_input_service_data: Arc::new(RwLock::new(AudioInputServiceData::new())),
            media_audio_service_data: Arc::new(RwLock::new(MediaAudioServiceData::new())),
            speech_audio_service_data: Arc::new(RwLock::new(SpeechAudioServiceData::new())),
            system_audio_service_data: Arc::new(RwLock::new(SystemAudioServiceData::new())),
            sensor_service_data: Arc::new(RwLock::new(SensorServiceData::new())),
            video_service_data: Arc::new(RwLock::new(VideoServiceData::new())),
            input_service_data: Arc::new(RwLock::new(InputServiceData::new())),
            temp_message_storage: Arc::new(RwLock::new(TempMessageStorage::new())),
            receive_more: Arc::new(RwLock::new(false)),
        }
    }
}
