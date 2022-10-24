use crate::protos::ServiceDiscoveryResponseMessage::ServiceDiscoveryResponse;

pub trait Service {
    fn start(&self);
    fn stop(&self);
    fn pause(&self);
    fn resume(&self);
    fn fill_features(&self, response: &mut ServiceDiscoveryResponse);
}

pub type ServiceList = Vec<Box<dyn Service>>;
