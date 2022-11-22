use crate::services::service::Service;
use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub struct BluetoothService {}

impl Service for BluetoothService {
    fn start(&mut self) {
        log::info!("Start");
    }

    fn stop(&mut self) {
        log::info!("Stop");
    }

    fn pause(&self) {
        log::info!("Pause");
    }

    fn resume(&self) {
        log::info!("Resume");
    }

    fn fill_features(&self, response: &mut ServiceDiscoveryResponse) {
        log::info!("Fill Features");

        let bluetooth_device_available = false;

        if bluetooth_device_available {
            let bluetooth_address = ""; //bluetoothDevice_->getLocalAddress()

            let mut channel_descriptor = crate::protos::ChannelDescriptorData::ChannelDescriptor::default();
            channel_descriptor.set_channel_id(crate::messenger::message::ChannelID::Bluetooth as u32);

            let mut bluetooth_channel = crate::protos::BluetoothChannelData::BluetoothChannel::default();
            bluetooth_channel.set_adapter_address(bluetooth_address.to_string());
            bluetooth_channel.supported_pairing_methods.push(protobuf::EnumOrUnknown::from(crate::protos::BluetoothPairingMethodEnum::bluetooth_pairing_method::Enum::HFP));
            bluetooth_channel.supported_pairing_methods.push(protobuf::EnumOrUnknown::from(crate::protos::BluetoothPairingMethodEnum::bluetooth_pairing_method::Enum::A2DP));

            channel_descriptor.bluetooth_channel = protobuf::MessageField::from_option(Some(bluetooth_channel));

            dbg!(channel_descriptor.clone());
            use protobuf::Message as msg;
            println!("BLUETOOTH:");
            let str = channel_descriptor.write_to_bytes().unwrap();
            for c in str {
                print!("{:X} ", c)
            }
            println!();

            response.channels.push(channel_descriptor);
        }
    }
}
