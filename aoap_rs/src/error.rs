use thiserror::Error;
use crate::ProtocolVersion;

#[derive(Error, Debug)]
pub enum AccessoryError {
    #[error("libusb error")]
    RusbError(#[from] rusb::Error),
    #[error("invalid length (size: {0})")]
    InvalidLength(usize),
    #[error("unsupported accessory protocol version: {0:?}")]
    UnsupportedProtocol(ProtocolVersion),
}
