use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub enum ServiceStatus {
    Uninitialized,
    Initialized,
}

pub trait Service {
    fn start(&mut self);
    fn stop(&mut self);
    fn pause(&self);
    fn resume(&self);
    fn fill_features(&self, response: &mut ServiceDiscoveryResponse);
}

pub type ServiceList = Vec<Box<dyn Service>>;
