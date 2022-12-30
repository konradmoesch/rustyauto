#[derive(Copy, Clone, Debug)]
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