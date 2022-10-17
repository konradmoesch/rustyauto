use std::time::Duration;

use rusb::{Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Recipient, request_type, RequestType};

use crate::error::AccessoryError;

mod constants;
pub mod error;

#[derive(Debug, PartialEq)]
enum DeviceType {
    Unknown,
    AOADevice,
    AOADeviceWithADB,
}

///Struct representing the necessary config for the accessory
pub struct AOAConfig {
    pub manufacturer: String,
    pub model_name: String,
    pub description: String,
    pub version: String,
    pub uri: String,
    pub serial_number: String,
}

///Get the supported AOA protocol version
fn get_protocol_version(device_handle: &DeviceHandle<GlobalContext>, data: &mut [u8], timeout: Duration) -> rusb::Result<usize> {
    let request_type = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    device_handle.read_control(request_type, constants::REQUEST_TYPE_GET_PROTOCOL, 0, 0, data, timeout)
}

///Helper for sending a String using the control transfer
fn send_string(device_handle: &DeviceHandle<GlobalContext>, index: u16, str: &String, timeout: Duration) -> Result<(), AccessoryError> {
    let buf = str.as_bytes();
    let size = device_handle.write_control(
        request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
        constants::REQUEST_TYPE_SEND_STRING,
        0,
        index,
        buf,
        timeout,
    )?;
    if size != buf.len() {
        log::error!("Failed to send string {} to device, received invalid length", str);
        return Err(AccessoryError::InvalidLength(size));
    }
    Ok(())
}

///Send the AOA headers given by aoa_config
fn send_headers(device_handle: &rusb::DeviceHandle<GlobalContext>, aoa_config: &AOAConfig, timeout: Duration) -> Result<(), AccessoryError> {
    log::debug!("Sending AOA headers");

    send_string(device_handle, constants::ACCESSORY_STRING_MANUFACTURER, &aoa_config.manufacturer, timeout)?;
    send_string(device_handle, constants::ACCESSORY_STRING_MODEL, &aoa_config.model_name, timeout)?;
    send_string(device_handle, constants::ACCESSORY_STRING_DESCRIPTION, &aoa_config.description, timeout)?;
    send_string(device_handle, constants::ACCESSORY_STRING_VERSION, &aoa_config.version, timeout)?;
    send_string(device_handle, constants::ACCESSORY_STRING_URI, &aoa_config.uri, timeout)?;
    send_string(device_handle, constants::ACCESSORY_STRING_SERIAL, &aoa_config.serial_number, timeout)?;

    log::info!("Successfully sent strings to device");
    Ok(())
}

///Start the device in AOA mode, if possible
fn try_start_aoa_mode(device: rusb::Device<GlobalContext>, aoa_config: &AOAConfig) {
    let timeout = Duration::from_secs(constants::USB_TIMEOUT_SECONDS as u64);
    let handle = device.open().unwrap();

    log::info!("Trying to start AOA on device {:?}", device);

    let mut data = vec![0; 2];
    let protocol_version = match get_protocol_version(&handle, &mut data, timeout) {
        Ok(_num_bytes) => &data,
        Err(e) => {
            log::error!("Fetching protocol version failed: {}", e);
            &data
        }
    };
    log::info!("protocol_version: {:?}", protocol_version);

    if data[0] == 2 || data[0] == 1 {
        match send_headers(&handle, aoa_config, timeout) {
            Ok(_) => log::info!("successfully sent headers"),
            Err(e) => log::error!("Error sending headers: {}", e),
        };
    } else {
        log::error!("Unknown protocol version");
    }
    send_start_request(device);
}

pub fn try_starting_aoa_mode(aoa_config: AOAConfig) {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        let timeout = Duration::from_secs(constants::USB_TIMEOUT_SECONDS as u64);

        let handle = device.open().unwrap();
        /*let language = handle.read_languages(timeout).unwrap()[0];
        log::debug!("Trying new device: Bus {:03} Device {:03} ID {:04x}:{:04x} ({})",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id(),
                 handle.read_manufacturer_string(language, &device_desc, timeout)
                     .expect(format!("unable to read manufacturer string in language {:?} for dev descriptor {:?}",
                                     language, device_desc).as_str()));*/
        //todo: make filter optional & configurable
        if device_desc.vendor_id() == 0x22d9 || device_desc.vendor_id() == 0x18d1 {
            try_start_aoa_mode(device, &aoa_config);
        }
    }
}


pub fn search_for_device() -> Option<Device<GlobalContext>> {
    log::info!("Searching for USB devices now");
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        let timeout = Duration::from_secs(constants::USB_TIMEOUT_SECONDS as u64);

        let device_type = check_usb_ids(&device_desc);
        return if device_type == DeviceType::AOADevice || device_type == DeviceType::AOADeviceWithADB {
            log::debug!("{:?} found at Bus {:03} Device {:03}", device_type, device.bus_number(), device.address());
            Some(device)
        } else {
            None
        }
    }
    None
}

fn check_usb_ids(device_desc: &DeviceDescriptor) -> DeviceType {
    if device_desc.vendor_id() == constants::GOOGLE_VID {
        match device_desc.product_id() {
            constants::AOA_PID => DeviceType::AOADevice,
            constants::AOA_WITH_ADB_PID => DeviceType::AOADeviceWithADB,
            _ => DeviceType::Unknown,
        }
    } else {
        DeviceType::Unknown
    }
}

fn send_start_request(device: Device<GlobalContext>) {
    let device_handle = device.open().unwrap();
    let request_out_vendor_device = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
    let timeout = Duration::from_secs(constants::USB_TIMEOUT_SECONDS as u64);

    log::info!("Start AOAP: {:?}", match (&device_handle).write_control(request_out_vendor_device, crate::constants::REQUEST_TYPE_START_AOA, 0, 0, &[], timeout) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Error: {}", e);
            0
        }
    });
}
