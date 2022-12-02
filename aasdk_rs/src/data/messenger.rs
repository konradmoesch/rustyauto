#[derive(Copy, Clone)]
pub enum MessengerStatus {
    Uninitialized,
    VersionRequestDone,
    AuthCompleted,
    InitializationDone,
}

#[derive(PartialEq)]
pub enum HandshakeStatus {
    Unfinished,
    Complete,
    Error,
}