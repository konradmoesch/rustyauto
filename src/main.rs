use std::sync::mpsc::Receiver;
use std::time::Duration;

use gstreamer::MessageView;
use gstreamer::prelude::*;

use aasdk_rs::cryptor::Cryptor;
use aasdk_rs::data;
use aasdk_rs::data::android_auto_entity::{AndroidAutoConfig, AndroidAutoEntityData};
use aasdk_rs::data::services::video_service_data::Indication;
use aasdk_rs::messenger::messenger::{Messenger, ReceivalQueue};
use aasdk_rs::services::sensor_service::SensorService;
use aasdk_rs::services::service::{Service, ServiceStatus};

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
            let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

            let mut android_auto_entity_data = aasdk_rs::data::android_auto_entity::AndroidAutoEntityData::new(auto_entity_config, tx);

            let mut messenger_data = android_auto_entity_data.clone();
            let mut view_data = android_auto_entity_data.clone();

            let mut messenger = Messenger { cryptor: Cryptor::init(), usb_driver, receival_queue: ReceivalQueue::new() };

            let messenger_thread = std::thread::spawn(move || {
                loop {
                    {
                        messenger.run(&mut messenger_data);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });

            std::thread::spawn(move || {
                while view_data.video_service_data.read().unwrap().status != aasdk_rs::data::services::general::ServiceStatus::Initialized {}
                let pipeline = create_pipeline(rx).unwrap();
                main_loop(pipeline).unwrap();
            });

            loop {
                {
                    let current_status = (*android_auto_entity_data.status.read().unwrap()).clone();
                    log::debug!("{:?}", current_status);
                    let mut sensor_service = aasdk_rs::services::sensor_service::SensorService {};
                    let mut video_service = aasdk_rs::services::video_service::VideoService {};
                    if current_status == data::android_auto_entity::AutoEntityStatus::Uninitialized {
                        //log::debug!("UNINITIALIZED");
                    } else {
                        //run services
                        sensor_service.run(&mut android_auto_entity_data);
                        video_service.run(&mut android_auto_entity_data);
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

fn create_pipeline(view_receiver: Receiver<Vec<u8>>) -> Result<gstreamer::Pipeline, ()> {
    gstreamer::init().unwrap();

    let pipeline = gstreamer::Pipeline::default();

    let udp_caps = gstreamer::Caps::builder("video/x-h264")
        .field("stream-format", "byte-stream")
        .field("alignment", "nal")
        .build();

    //let src = gstreamer::ElementFactory::make("appsrc").build().unwrap();
    let src = gstreamer::ElementFactory::make("udpsrc").property("address", "127.0.0.1")
        .property("caps", &udp_caps)
        .build().unwrap();
    let parse = gstreamer::ElementFactory::make("h264parse").build().unwrap();
    let decode = gstreamer::ElementFactory::make("avdec_h264").build().unwrap();
    let glup = gstreamer::ElementFactory::make("videoconvert").build().unwrap();
    let sink = gstreamer::ElementFactory::make("autovideosink").build().unwrap();

    // attaching pipeline elements
    pipeline.add_many(&[&src, &parse, &decode, &glup, &sink]).unwrap();
    gstreamer::Element::link_many(&[&src, &parse, &decode, &glup, &sink]).unwrap();

    /*let appsrc = src
        .dynamic_cast::<gstreamer_app::AppSrc>()
        .expect("Source element is expected to be an appsrc!");

    appsrc.set_is_live(true);

    let mut i = 0;
    appsrc.set_callbacks(
        gstreamer_app::AppSrcCallbacks::builder()
            .need_data(move |appsrc, _| {
                println!("Producing frame {}", i);
                //match view_data.video_service_data.write().unwrap().buffer.pop_front() {
                //    None => { log::error!("No NAL frames in buffer!") }
                //    Some(frame) => {
                        //let buffer = gstreamer::Buffer::from_slice(&view_data.video_service_data.write().unwrap().buffer.iter().nth(i).unwrap().0);
                        //let buffer = gstreamer::Buffer::from_mut_slice(&view_receiver.recv());
                        //let buffer = gstreamer::Buffer::from_slice(frame.0.iter().nth(i).as_slice());
                        i += 1;

                        // appsrc already handles the error here
                        //let _ = appsrc.push_buffer(buffer);
                   // }
                //}
            })
            .build(),
    );*/

    Ok(pipeline)
}

fn main_loop(pipeline: gstreamer::Pipeline) -> Result<(), ()> {
    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gstreamer::State::Null).unwrap();
                return Err(());
            }
            _ => (),
        }
    }

    pipeline.set_state(gstreamer::State::Null).unwrap();

    Ok(())
}
