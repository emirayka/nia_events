use crate::Error;
use crate::XorgDevice;

pub struct XorgDeviceBuilder {}

impl XorgDeviceBuilder {
    pub fn new() -> XorgDeviceBuilder {
        XorgDeviceBuilder {}
    }

    pub fn build(self) -> Result<XorgDevice, Error> {
        let xorg_device = XorgDevice::new();

        Ok(xorg_device)
    }

    pub fn build_default() -> Result<XorgDevice, Error> {
        XorgDeviceBuilder::new().build()
    }
}
