use std::time::Duration;

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

            let mut android_auto_entity = aasdk_rs::androidautoentity::AndroidAutoEntity::new(usb_driver);
            android_auto_entity.start();
        }
        _ => log::error!("No compatible device found!"),
    };
}
