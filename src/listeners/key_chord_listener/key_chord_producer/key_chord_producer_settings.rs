use crate::{DeviceInfo, Key};

pub struct KeyChordProducerSettings {
    devices: Vec<DeviceInfo>,
    modifiers: Vec<Key>,
}

impl KeyChordProducerSettings {
    pub fn new(devices: Vec<DeviceInfo>, modifiers: Vec<Key>) -> KeyChordProducerSettings {
        KeyChordProducerSettings { devices, modifiers }
    }

    pub fn get_devices(&self) -> &Vec<DeviceInfo> {
        &self.devices
    }

    pub fn get_modifiers(&self) -> &Vec<Key> {
        &self.modifiers
    }
}
