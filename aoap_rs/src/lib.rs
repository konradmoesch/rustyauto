use std::time::Duration;

use rusb::{Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Recipient, request_type, RequestType};

use crate::error::AccessoryError;

mod constants;
pub mod error;

//TODO: stop iterating devices when successful
//TODO: better error handling

#[derive(Debug)]
#[allow(dead_code)]
pub struct ProtocolVersion {
    major: u8,
    minor: u8,
}

impl From<[u8; 2]> for ProtocolVersion {
    fn from(buffer: [u8; 2]) -> Self {
        Self {
            major: buffer[0],
            minor: buffer[1],
        }
    }
}

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
fn get_protocol_version(device_handle: &DeviceHandle<GlobalContext>, timeout: Duration) -> Result<ProtocolVersion, AccessoryError> {
    let request_type = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let mut data_buffer = [0u8; 2];
    match device_handle.read_control(request_type, constants::REQUEST_TYPE_GET_PROTOCOL, 0, 0, &mut data_buffer, timeout) {
        Ok(_) => {
            let version = ProtocolVersion::from(data_buffer);
            Ok(version)
        }
        Err(rusb_error) => {
            log::error!("An error occurred during the protocol version request {}", rusb_error);
            Err(AccessoryError::RusbError(rusb_error))
        }
    }
}

///Helper for sending a String using the control transfer
fn send_string(device_handle: &DeviceHandle<GlobalContext>, index: u16, str: &String, timeout: Duration) -> Result<(), AccessoryError> {
    let buf = str.as_bytes();
    let size = device_handle.write_control(
        rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
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
fn send_headers(device_handle: &DeviceHandle<GlobalContext>, aoa_config: &AOAConfig, timeout: Duration) -> Result<(), AccessoryError> {
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
    let device_desc = device.device_descriptor().unwrap();

    log::info!("Trying new device: Bus {:03} Device {:03} ID {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id());

    match get_protocol_version(&handle, timeout) {
        Ok(version) => {
            log::info!("protocol_version: {:?}", version);
            if version.major == 2 || version.major == 1 {
                match send_headers(&handle, aoa_config, timeout) {
                    Ok(_) => {
                        log::info!("successfully sent headers");
                        send_start_request(device);
                    }
                    Err(e) => {
                        log::error!("Error sending headers: {:?}; skipping this device", e);
                        return;
                    }
                };
            } else {
                log::error!("Unknown protocol version; skipping this device");
                //AccessoryError(UnsupportedProtocol(protocol_version))
                return;
            }
        }
        Err(e) => {
            log::error!("Fetching protocol version failed: {:?}", e);
            return;
        }
    };
}

pub fn try_starting_aoa_mode(aoa_config: AOAConfig, vendor_ids_to_try: Option<Vec<u16>>) {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        match vendor_ids_to_try {
            Some(ref filter_list) => {
                if filter_list.contains(&device_desc.vendor_id()) {
                    try_start_aoa_mode(device, &aoa_config);
                }
            }
            None => {
                log::debug!("No filter specified, trying all devices");
                try_start_aoa_mode(device, &aoa_config);
            }
        }
    }
}

///Search for a Android-powered device in accessory mode
pub fn search_for_device_in_accessory_mode() -> Option<Device<GlobalContext>> {
    log::info!("Searching for a accessory mode USB device now");
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        let device_type = get_device_type(&device_desc);
        return if device_type == DeviceType::AOADevice || device_type == DeviceType::AOADeviceWithADB {
            log::debug!("{:?} found at Bus {:03} Device {:03}", device_type, device.bus_number(), device.address());
            Some(device)
        } else {
            None
        };
    }
    None
}

///Get device type by comparing usb ids
fn get_device_type(device_desc: &DeviceDescriptor) -> DeviceType {
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

    log::info!("sending aoa mode start request: {:?}", match (&device_handle).write_control(request_out_vendor_device, crate::constants::REQUEST_TYPE_START_AOA, 0, 0, &[], timeout) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Error: {}", e);
            0
        }
    });
}
