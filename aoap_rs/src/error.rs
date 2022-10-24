use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessoryError {
    #[error("libusb error")]
    RusbError(#[from] rusb::Error),
    #[error("invalid length (size: {0})")]
    InvalidLength(usize),
    #[error("unsupported accessory protocol (size: {0})")]
    UnsupportedProtocol(u16),
}
