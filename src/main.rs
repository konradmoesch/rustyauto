use std::arch::x86_64::_mm_rcp_ps;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aasdk_rs::cryptor::Cryptor;
use aasdk_rs::data;
use aasdk_rs::data::android_auto_entity::AndroidAutoConfig;
use aasdk_rs::data::services::general::ServiceStatus::Uninitialized;
use aasdk_rs::messenger::messenger::Messenger;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = fern::colors::ColoredLevelConfig::new();
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();
    log::info!("Initialized Logging");
    let aoa_config = aoap_rs::AOAConfig {
        manufacturer: "Android".to_string(),
        model_name: "Android Auto".to_string(),
        description: "Android Auto".to_string(),
        version: "1.0".to_string(),
        uri: "https://github.com/konradmoesch".to_string(),
        serial_number: "001".to_string(),
    };
    aoap_rs::try_starting_aoa_mode(aoa_config);
    std::thread::sleep(Duration::from_secs(5));
    match aoap_rs::search_for_device() {
        Some(device) => {
            log::info!("Found aoa-enabled device!");
            let usb_driver = aasdk_rs::usbdriver::UsbDriver::init(device);

            let auto_entity_config = AndroidAutoConfig {
                head_unit_name: "rustyauto".to_string(),
                car_model: "Universal".to_string(),
                car_year: "2022".to_string(),
                car_serial: "20221124".to_string(),
                left_hand_drive_vehicle: true,
                headunit_manufacturer: "km".to_string(),
                headunit_model: "rustyauto app".to_string(),
                sw_build: "0.1".to_string(),
                sw_version: "0.1".to_string(),
                can_play_native_media_during_vr: false,
                hide_clock: false,
            };

            let mut android_auto_entity_data = aasdk_rs::data::android_auto_entity::AndroidAutoEntityData::new(auto_entity_config);

            let data = Arc::new(Mutex::new(android_auto_entity_data));
            let messenger_data = data.clone();

            let mut messenger = Messenger { cryptor: Cryptor::init(), usb_driver };

            let messenger_thread = std::thread::spawn(move || {
                loop {
                    {
                        let mut recv_data = messenger_data.lock().unwrap();
                        messenger.run(recv_data.deref_mut());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });

            loop {
                {
                    let aa_data = data.lock().unwrap();
                    log::debug!("{:?}", aa_data.status);
                    if aa_data.status == data::android_auto_entity::AutoEntityStatus::Uninitialized {
                        log::debug!("UNINITIALIZED");
                    } else {
                        //run services
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            }

            //let mut android_auto_entity = aasdk_rs::legacy_androidautoentity::LegacyAndroidAutoEntity::new(usb_driver);
            //android_auto_entity.start();
        }
        _ => log::error!("No compatible device found!"),
    };
}
