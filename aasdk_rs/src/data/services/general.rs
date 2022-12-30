#[derive(PartialEq)]
pub enum ChannelStatus {
    Closed,
    OpenRequest,
    Open,
}

#[derive(PartialEq)]
pub enum ServiceStatus {
    Uninitialized,
    Initialized,
}

#[derive(PartialEq)]
pub enum SetupStatus {
    NotStarted,
    Requested,
    Finished,
}
