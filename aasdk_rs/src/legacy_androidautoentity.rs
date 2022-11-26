use std::time::Duration;
use openssl_sys::uselocale;
use crate::channels;
use crate::cryptor;
use crate::cryptor::Cryptor;
use crate::data::android_auto_entity::Version;
use crate::messenger::message::{ChannelID, EncryptionType, Message};
use crate::messenger::legacy_messenger::LegacyMessenger;
use crate::services::audio_input_service::AudioInputService;
use crate::services::bluetooth_service::BluetoothService;
use crate::services::input_service::InputService;
use crate::services::media_audio_service::MediaAudioService;
use crate::services::sensor_service::{SensorService, SensorServiceConfig};
use crate::services::service::{ServiceList, ServiceStatus};
use crate::services::speech_audio_service::SpeechAudioService;
use crate::services::system_audio_service::SystemAudioServiceData;
use crate::services::video_service::VideoService;
use crate::services::wifi_service::WifiService;
use crate::usbdriver::UsbDriver;

pub struct LegacyAndroidAutoEntity {
    cryptor: Cryptor,
    messenger: LegacyMessenger,
    service_list: ServiceList,
    //config: AndroidAutoConfig,
}

impl LegacyAndroidAutoEntity {
    pub fn new(usb_driver: UsbDriver) -> Self {
        Self {
            cryptor: Cryptor::init(),
            messenger: LegacyMessenger::init(usb_driver),//, &cryptor),
            service_list: ServiceList::new(),
            //config: AndroidAutoConfig {},
        }
    }

    pub fn start(&mut self) {
        //TODO: Audio Services
        //TODO: sensor service
        //TODO: pinger?
        //TODO: video service
        //TODO: bluetooth service
        //TODO: input service
        //TODO: wifi service

        //let mut cryptor = Cryptor::init();

        /*let (in_queue_tx, in_queue_rx) = std::sync::mpsc::channel();
        let (out_queue_tx, out_queue_rx) = std::sync::mpsc::channel();

        let mut messenger = crate::messenger::Messenger::init(self.usb_driver);

        let _usb_thread = std::thread::spawn(move || {
            loop {
                messenger.run(&in_queue_rx, &out_queue_rx);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        let version_request_message = channels::control_service_channel::create_version_request_message();
        out_queue_tx.send(version_request_message.clone()).unwrap();
        in_queue_tx.send((1)).unwrap();
        loop {}*/
        let own_version = Version { major: 1, minor: 0 };
        let version_request_message = channels::control_service_channel::create_version_request_message(&own_version);
        self.messenger.send_message(version_request_message);

        let received_message = self.messenger.receive_message(12);
        ////received_message.handle();
        log::info!("Doing handshake now");
        self.cryptor.do_handshake();

        log::info!("Sending handshake message");
        let handshake_message = channels::control_service_channel::create_handshake_message(self.cryptor.read_handshake_buffer().as_slice());
        log::debug!("{:?}", handshake_message);
        self.messenger.send_message(handshake_message);

        let received_message = self.messenger.receive_message(2288);
        //received_message.handle();
        let bla = received_message.payload[2..].to_vec();
        log::info!("{:?}", bla);
        self.cryptor.write_handshake_buffer(bla.as_slice());
        self.cryptor.do_handshake();

        log::info!("Continuing handshake");
        let handshake_message = channels::control_service_channel::create_handshake_message(self.cryptor.read_handshake_buffer().as_slice());
        log::debug!("{:?}", handshake_message);
        self.messenger.send_message(handshake_message);

        let received_message = self.messenger.receive_message(57);
        //received_message.handle();
        let bla = received_message.payload[2..].to_vec();
        log::info!("{:?}", bla);
        self.cryptor.write_handshake_buffer(bla.as_slice());
        self.cryptor.do_handshake();
        // check successful

        let auth_complete_message = channels::control_service_channel::create_auth_complete_message();
        log::debug!("{:?}", auth_complete_message);
        self.messenger.send_message(auth_complete_message);

        let mut received_message = self.messenger.receive_message(870);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut service_disc_res = crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse::new();
        //service_disc_res.mutable_channels()->Reserve(256);
        //service_disc_res.head_unit_name = Some("rustyauto".to_string());
        service_disc_res.head_unit_name = Some("Crankshaft-NG".to_string());
        service_disc_res.car_model = Some("Universal".to_string());
        //service_disc_res.car_year = Some("2022".to_string());
        service_disc_res.car_year = Some("2018".to_string());
        //service_disc_res.car_serial = Some("20221004".to_string());
        service_disc_res.car_serial = Some("20180301".to_string());
        service_disc_res.left_hand_drive_vehicle = Some(true);
        //service_disc_res.headunit_manufacturer = Some("km".to_string());
        service_disc_res.headunit_manufacturer = Some("f1x".to_string());
        //service_disc_res.headunit_model = Some("rustyauto app".to_string());
        service_disc_res.headunit_model = Some("Crankshaft-NG Autoapp".to_string());
        service_disc_res.sw_build = Some("1".to_string());
        service_disc_res.sw_version = Some("1.0".to_string());
        service_disc_res.can_play_native_media_during_vr = Some(false);
        service_disc_res.hide_clock = Some(false);
        dbg!(service_disc_res.clone());

        let mut service_list = ServiceList::new();
        let audio_input_service = AudioInputService {};
        let media_audio_service = MediaAudioService {};
        let speech_audio_service = SpeechAudioService {};
        let system_audio_service = SystemAudioServiceData {};
        let sensor_service = SensorService {
            service_status: ServiceStatus::Uninitialized,
            config: SensorServiceConfig { location_sensor_present: false },
            night_sensor_value: crate::services::sensor_service::NightStatus::Night
        };
        let video_service = VideoService {};
        let bluetooth_service = BluetoothService {};
        let input_service = InputService {};
        let wifi_service = WifiService {};
        service_list.push(Box::new(audio_input_service));
        service_list.push(Box::new(media_audio_service));
        service_list.push(Box::new(speech_audio_service));
        service_list.push(Box::new(system_audio_service));
        service_list.push(Box::new(sensor_service));
        service_list.push(Box::new(video_service));
        service_list.push(Box::new(bluetooth_service));
        service_list.push(Box::new(input_service));
        service_list.push(Box::new(wifi_service));

        for mut service in service_list {
            service.start();
            service.fill_features(&mut service_disc_res);
            //dbg!(service_disc_res.clone().channels);
            //service.stop();
        }

        /*use protobuf::Message as msg;
        let str = service_disc_res.write_to_bytes().unwrap();
        for c in str {
            print!("{:X} ", c)
        }
        println!();*/

        let mut service_discovery_response_message = channels::control_service_channel::create_service_discovery_response_message(service_disc_res);
        self.cryptor.encrypt_message(&mut service_discovery_response_message);
        self.messenger.send_message(service_discovery_response_message);

        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut audio_focus_response_message = channels::control_service_channel::create_audio_focus_response_message();
        self.cryptor.encrypt_message(&mut audio_focus_response_message);
        self.messenger.send_message(audio_focus_response_message);

        log::warn!("Opening channels now");
        //ChannelOpenRequest for AudioInputChannel
        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::av_input_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //Media Audio Channel Open Request
        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::media_audio_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //Speech audio open request
        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::speech_audio_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //System audio open request
        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::system_audio_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //sensor channel open request
        let mut received_message = self.messenger.receive_message_without_size();
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::sensor_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //Input channel open request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::input_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        //Video channel open request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        let mut channel_open_response_message = channels::video_service_channel::create_channel_open_response_message();
        self.cryptor.encrypt_message(&mut channel_open_response_message);
        self.messenger.send_message(channel_open_response_message);

        log::warn!("Doing setup now");
        let mut video_setup_done = false;
        //Media audio setup request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //Speech audio setup request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //System audio setup request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //input setup request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //video setup request
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //sensor setup request: location
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //video start indication
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //sensor setup request: driving status
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //sensor setup request: night data
        let mut received_message = self.messenger.receive_message(39);
        self.cryptor.decrypt_message(&mut received_message);
        //received_message.handle();

        match received_message.channel_id {
            ChannelID::Video => {
                if !video_setup_done {
                    log::warn!("Received video channel setup request");
                    log::info!("Sending video focus indication");
                    let mut video_focus_message = channels::video_service_channel::create_video_focus_indication();
                    self.cryptor.encrypt_message(&mut video_focus_message);
                    self.messenger.send_message(video_focus_message);

                    let mut setup_response_message = create_correct_setup(received_message);
                    self.cryptor.encrypt_message(&mut setup_response_message);
                    self.messenger.send_message(setup_response_message);
                    video_setup_done = true;
                } else {
                    log::warn!("Video setup done; received start indication");
                    log::info!("Would receive more");
                    /*let mut received_message = self.messenger.receive_message(39);
                    self.cryptor.decrypt_message(&mut received_message);
                    //received_message.handle();*/
                }
            },
            ChannelID::Sensor => {
                log::warn!("Received sensor channel setup request");
                log::info!("Sending sensor indication");
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);

                log::info!("Sent some sensor response, will now send sensor start message");
                let mut message = channels::sensor_service_channel::create_sensor_start_response_alternate();
                self.cryptor.encrypt_message(&mut message);
                self.messenger.send_message(message);
            },
            _ => {
                log::warn!("Sending setup response for channel {:?}", received_message.channel_id);
                let mut setup_response_message = create_correct_setup(received_message);
                self.cryptor.encrypt_message(&mut setup_response_message);
                self.messenger.send_message(setup_response_message);
            }
        }

        //video av media indication
        let mut received_message = self.messenger.receive_message(64);
        self.cryptor.decrypt_message(&mut received_message);
        log::error!("Received blub: {:?}", received_message);
        //received_message.handle();

        let mut av_ack_message = channels::video_service_channel::create_av_media_ack_indication();
        self.cryptor.encrypt_message(&mut av_ack_message);
        self.messenger.send_message(av_ack_message);

        //video av media indication
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let mut received_message = self.messenger.receive_message(1561);
            self.cryptor.decrypt_message(&mut received_message);
            log::error!("Received blub: {:?}", received_message);
            //received_message.handle();

            let mut av_ack_message = channels::video_service_channel::create_av_media_ack_indication();
            self.cryptor.encrypt_message(&mut av_ack_message);
            self.messenger.send_message(av_ack_message);
        }
    }
}

fn create_correct_setup(message: Message) -> Message {
    match message.channel_id {
        ChannelID::Control => {unimplemented!()}
        ChannelID::Input => {channels::input_service_channel::create_binding_response_message(message)}
        ChannelID::Sensor => {channels::sensor_service_channel::create_sensor_start_response_message(message)}
        ChannelID::Video => {channels::video_service_channel::create_av_channel_setup_response(message)}
        ChannelID::MediaAudio => {channels::media_audio_service_channel::create_av_channel_setup_response(message.channel_id)}
        ChannelID::SpeechAudio => {channels::media_audio_service_channel::create_av_channel_setup_response(message.channel_id)}
        ChannelID::SystemAudio => {channels::media_audio_service_channel::create_av_channel_setup_response(message.channel_id)}
        ChannelID::AVInput => {channels::media_audio_service_channel::create_av_channel_setup_response(message.channel_id)}
        ChannelID::Bluetooth => {unimplemented!()}
        ChannelID::None => {unimplemented!()}
    }
}
