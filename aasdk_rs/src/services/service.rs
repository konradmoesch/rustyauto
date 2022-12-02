use crate::data::android_auto_entity::AndroidAutoEntityData;
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
    fn run(&mut self, data: &mut AndroidAutoEntityData);
}

pub type ServiceList = Vec<Box<dyn Service>>;
