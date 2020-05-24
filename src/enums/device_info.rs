use crate::DeviceId;

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    device_id: DeviceId,
    device_path: String,
}

impl DeviceInfo {
    pub fn new(device_id: DeviceId, device_path: String) -> DeviceInfo {
        DeviceInfo {
            device_id,
            device_path,
        }
    }

    pub fn get_device_id(&self) -> DeviceId {
        self.device_id
    }

    pub fn get_device_path(&self) -> &String {
        &self.device_path
    }
}
