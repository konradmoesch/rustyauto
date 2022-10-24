use std::time::Duration;

use rusb::GlobalContext;

pub struct UsbDriver {
    device: rusb::Device<GlobalContext>,
    handle: rusb::DeviceHandle<GlobalContext>,
    in_endpoint_addr: u8,
    out_endpoint_addr: u8,
    timeout: Duration,
}

impl UsbDriver {
    pub fn init(usb_device: rusb::Device<GlobalContext>) -> Self {
        log::info!("Initializing USB Device");

        let device_desc = usb_device.device_descriptor().unwrap();
        let config_desc = usb_device.config_descriptor(0).unwrap();
        let mut handle = usb_device.open().unwrap();
        let mut interfaces = config_desc.interfaces();
        let num_interfaces = config_desc.num_interfaces();
        if num_interfaces > 2 {
            log::error!("Too many interfaces found!");
        } else if num_interfaces == 0 {
            log::error!("No interface found");
        } else if num_interfaces == 2 {
            log::info!("Selecting AOA interface (0)");
            handle.claim_interface(0).unwrap();
        }
        let aoa_interface = interfaces.nth(0).unwrap();
        let interface_desc = aoa_interface.descriptors().nth(0).unwrap();
        if interface_desc.num_endpoints() != 2 { log::error!("incorrect number of endpoints on aoa interface") }
        let endpoint_desc_in = interface_desc.endpoint_descriptors().nth(0).unwrap();
        let endpoint_desc_out = interface_desc.endpoint_descriptors().nth(1).unwrap();

        Self {
            device: usb_device,
            handle,
            in_endpoint_addr: endpoint_desc_in.address(),
            out_endpoint_addr: endpoint_desc_out.address(),
            timeout: Duration::from_secs(crate::constants::USB_TIMEOUT_SECONDS as u64),
        }
    }

    pub fn send_buffer(&self, buffer: &[u8]) {
        log::info!("Sent {} bits, result: {}",
        buffer.len(),
        self.handle.write_bulk(self.out_endpoint_addr, buffer, self.timeout).unwrap());
    }

    pub fn read_buffer(&self, buf: &mut [u8]) {
        match self.handle.read_bulk(self.in_endpoint_addr, buf, self.timeout) {
            Ok(size) => {
                log::info!("Successfully read {size} bits from USB device");
                //buf.to_vec().resize(size, 0);
                buf.to_vec().truncate(size);
            }
            Err(e) => { log::error!("Error reading from USB device: {e}") }
        };
    }
}