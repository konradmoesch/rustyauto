use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use gio::prelude::*;
use gstreamer::MessageView;
use gstreamer::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use aasdk_rs::cryptor::Cryptor;
use aasdk_rs::data;
use aasdk_rs::data::android_auto_entity::{AndroidAutoConfig, AndroidAutoEntityData};
use aasdk_rs::data::services::input_service_data::{TouchActionType, TouchPosition};
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
        //.level(log::LevelFilter::Info)
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
                    //std::thread::sleep(std::time::Duration::from_millis(300));
                }
            });

            std::thread::spawn(move || {
                while view_data.video_service_data.read().unwrap().status != aasdk_rs::data::services::general::ServiceStatus::Initialized {}
                create_pipeline(view_data);
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

fn create_ui(app: &gtk::Application, mut view_data: AndroidAutoEntityData) {
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
    //let sink = gstreamer::ElementFactory::make("autovideosink").build().unwrap();

    let (sink, widget) = if let Ok(gtkglsink) = gstreamer::ElementFactory::make("gtkglsink").build() {
        let glsinkbin = gstreamer::ElementFactory::make("glsinkbin")
            .property("sink", &gtkglsink)
            .build()
            .unwrap();
        let widget = gtkglsink.property::<gtk::Widget>("widget");
        (glsinkbin, widget)
    } else {
        let sink = gstreamer::ElementFactory::make("gtksink").build().unwrap();
        let widget = sink.property::<gtk::Widget>("widget");
        (sink, widget)
    };

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
                match view_data.video_service_data.write().unwrap().buffer.pop_front() {
                    None => { log::error!("No NAL frames in buffer!") }
                    Some(frame) => {
                        //let buffer = gstreamer::Buffer::from_slice(&view_data.video_service_data.write().unwrap().buffer.iter().nth(i).unwrap().0);
                        //let buffer = gstreamer::Buffer::from_mut_slice(&view_receiver.recv());
                        //let buffer = gstreamer::Buffer::from_slice(frame.0.iter().nth(i).as_slice());
                        //let buffer = gstreamer::Buffer::from_mut_slice(view_data.view_buf.lock().unwrap().pop_front().unwrap());
                        let buffer = gstreamer::Buffer::from_mut_slice(frame);
                        i += 1;

                        // appsrc already handles the error here
                        let _ = appsrc.push_buffer(buffer);
                    }
                }
            })
            .build(),
    );*/

    // Create a simple gtk gui window to place our widget into.
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_default_size(720, 480);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // Add our widget to the gui
    //let touch_controller = gtk::GestureSingle;
    //touch_controller.set_touch_only(true);
    widget.add_events(gdk::EventMask::TOUCH_MASK);
    //widget.connect("touch-event", false, |event| {dbg!(event);log::info!("Touch event");Some(event)});
    widget.connect_event(move |widget, event| {
        match event.event_type() {
            gdk::EventType::TouchBegin | gdk::EventType::TouchUpdate | gdk::EventType::TouchEnd => {
                log::info!("Touch event");
                let raw_touch_position = event.coords().unwrap();
                let touch_action = match event.event_type() {
                    gdk::EventType::TouchBegin => Some(TouchActionType::Press),
                    gdk::EventType::TouchUpdate => Some(TouchActionType::Drag),
                    gdk::EventType::TouchEnd => Some(TouchActionType::Release),
                    _ => None,

                };
                let touch_position = TouchPosition(raw_touch_position.0 as usize, raw_touch_position.1 as usize);
                log::info!("Location: {:?}", touch_position);
                view_data.input_service_data.write().unwrap().current_touch_position = Some(touch_position);
                view_data.input_service_data.write().unwrap().current_touch_action = touch_action;
            }
            _ => { dbg!(event); }
        }
        gtk::Inhibit(false)
    });
    vbox.pack_start(&widget, true, true, 0);
    let label = gtk::Label::new(Some("Position: 00:00:00"));
    vbox.pack_start(&label, true, true, 5);
    window.add(&vbox);
    window.show_all();

    app.add_window(&window);

    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop that will periodically (every 500ms) be
    // executed. This will query the current position within the stream from
    // the underlying pipeline, and display it in our gui.
    // Since this closure is called by the mainloop thread, we are allowed
    // to modify the gui widgets here.
    let timeout_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        // Query the current playing position from the underlying pipeline.
        let position = pipeline.query_position::<gstreamer::ClockTime>();
        // Display the playing position in the gui.
        label.set_text(&format!("Position: {:.0}", position.display()));
        // Tell the callback to continue calling this closure.
        glib::Continue(true)
    });

    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let app_weak = app.downgrade();
    bus.add_watch_local(move |_, msg| {
        use gstreamer::MessageView;

        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return glib::Continue(false),
        };

        match msg.view() {
            MessageView::Eos(..) => app.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    })
        .expect("Failed to add bus watch");

    let timeout_id = RefCell::new(Some(timeout_id));
    let pipeline = RefCell::new(Some(pipeline));
    app.connect_shutdown(move |_| {
        // Optional, by manually destroying the window here we ensure that
        // the gst element is destroyed when shutting down instead of having to wait
        // for the process to terminate, allowing us to use the leaks tracer.
        unsafe {
            window.destroy();
        }

        // GTK will keep the Application alive for the whole process lifetime.
        // Wrapping the pipeline in a RefCell<Option<_>> and removing it from it here
        // ensures the pipeline is actually destroyed when shutting down, allowing us
        // to use the leaks tracer for example.
        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline
                .set_state(gstreamer::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
            pipeline.bus().unwrap().remove_watch().unwrap();
        }

        if let Some(timeout_id) = timeout_id.borrow_mut().take() {
            timeout_id.remove();
        }
    });
}

fn create_pipeline(mut view_data: AndroidAutoEntityData) {
    gstreamer::init().unwrap();
    gtk::init().unwrap();

    {
        let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

        app.connect_activate(move |app| { create_ui(app, view_data.clone()) });
        app.run();
    }

    // Optional, can be used to detect leaks using the leaks tracer
    unsafe {
        gstreamer::deinit();
    }
}
