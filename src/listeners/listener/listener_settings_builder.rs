use crate::enums::Key;

use crate::{DeviceId, KeyId, ListenerSettings};
use crate::{DeviceInfo, KeyChordProducerSettings};

pub struct ListenerSettingsBuilder {
    devices: Vec<DeviceInfo>,
    modifiers: Vec<Key>,
}

impl ListenerSettingsBuilder {
    pub fn new() -> ListenerSettingsBuilder {
        ListenerSettingsBuilder {
            devices: Vec::new(),
            modifiers: Vec::new(),
        }
    }

    pub fn add_device(mut self, device_path: String, device_id: u16) -> ListenerSettingsBuilder {
        let device_info = DeviceInfo::new(DeviceId::new(device_id), device_path);
        self.devices.push(device_info);

        self
    }

    pub fn add_modifier_1(mut self, key_code: u16) -> ListenerSettingsBuilder {
        let modifier = Key::Key1(KeyId::new(key_code));

        self.modifiers.push(modifier);

        self
    }

    pub fn add_modifier_2(mut self, device_id: u16, key_code: u16) -> ListenerSettingsBuilder {
        let modifier = Key::Key2(DeviceId::new(device_id), KeyId::new(key_code));

        self.modifiers.push(modifier);

        self
    }

    pub fn build(self) -> ListenerSettings {
        let key_chord_producer_settings =
            KeyChordProducerSettings::new(self.devices, self.modifiers);

        ListenerSettings::new(key_chord_producer_settings)
    }
}
