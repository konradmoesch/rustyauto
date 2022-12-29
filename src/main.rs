use std::time::Duration;

use aasdk_rs::cryptor::Cryptor;
use aasdk_rs::data;
use aasdk_rs::data::android_auto_entity::AndroidAutoConfig;
use aasdk_rs::messenger::messenger::{Messenger, ReceivalQueue};
use aasdk_rs::services::sensor_service::SensorService;
use aasdk_rs::services::service::Service;

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
    let vendor_ids_to_try: Vec<u16> = vec![0x22d9, 0x18d1];
    aoap_rs::try_starting_aoa_mode(aoa_config, Some(vendor_ids_to_try));
    //aoap_rs::try_starting_aoa_mode(aoa_config, None);
    std::thread::sleep(Duration::from_secs(5));
    match aoap_rs::search_for_device_in_accessory_mode() {
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

            let mut messenger_data = android_auto_entity_data.clone();

            let mut messenger = Messenger { cryptor: Cryptor::init(), usb_driver, receival_queue: ReceivalQueue::new() };

            let messenger_thread = std::thread::spawn(move || {
                loop {
                    {
                        messenger.run(&mut messenger_data);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });

            loop {
                {
                    let current_status = (*android_auto_entity_data.status.read().unwrap()).clone();
                    log::debug!("{:?}", current_status);
                    let mut sensor_service = aasdk_rs::services::sensor_service::SensorService{};
                    sensor_service.run(&mut android_auto_entity_data);
                    if current_status == data::android_auto_entity::AutoEntityStatus::Uninitialized {
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
