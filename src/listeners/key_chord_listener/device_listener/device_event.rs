use crate::enums::DeviceId;
use crate::enums::KeyId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceEventType {
    RELEASED,
    PRESSED,
    UNKNOWN,
}

impl DeviceEventType {
    pub fn from_value(value: i32) -> DeviceEventType {
        if value == 0 {
            DeviceEventType::RELEASED
        } else if value == 1 {
            DeviceEventType::PRESSED
        } else {
            DeviceEventType::UNKNOWN
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceEvent {
    device_id: DeviceId,
    key_id: KeyId,
    event_type: DeviceEventType,
}

impl DeviceEvent {
    pub fn new(device_id: DeviceId, key_id: KeyId, event_type: DeviceEventType) -> DeviceEvent {
        DeviceEvent {
            device_id,
            key_id,
            event_type,
        }
    }

    pub fn get_device_id(&self) -> DeviceId {
        self.device_id
    }

    pub fn get_key_id(&self) -> KeyId {
        self.key_id
    }

    pub fn get_event_type(&self) -> DeviceEventType {
        self.event_type
    }
}
