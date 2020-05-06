use crate::Error;
use crate::UInputDevice;

pub struct UInputDeviceBuilder {
    device_builder: uinput::device::Builder,
}

impl UInputDeviceBuilder {
    pub fn default() -> Result<UInputDeviceBuilder, Error> {
        let result = uinput::default()
            .map(|device_builder| UInputDeviceBuilder {
                device_builder
            })
            .map_err(|_| Error::uinput_configuration_error("Cannot get default uinput device."));

        result
    }

    pub fn from(device_builder: uinput::device::Builder) -> UInputDeviceBuilder {
        UInputDeviceBuilder {
            device_builder
        }
    }

    pub fn name(self, name: &str) -> Result<UInputDeviceBuilder, Error> {
        let result = self.device_builder.name(name)
            .map_err(|_| Error::uinput_configuration_error("Cannot set uinput device name."))?;

        Ok(UInputDeviceBuilder::from(result))
    }

    pub fn enable_keyboard(self) -> Result<UInputDeviceBuilder, Error> {
        let result = self.device_builder
            .event(uinput::event::Keyboard::All)
            .map_err(|_| Error::uinput_configuration_error("Cannot enable keyboard events for uinput device."))?;

        Ok(UInputDeviceBuilder::from(result))
    }

    pub fn enable_mouse(self) -> Result<UInputDeviceBuilder, Error> {
        use uinput::event::controller::Controller::Mouse;
        use uinput::event::controller::Mouse::*;
        use uinput::event::Event::Controller;

        let result = self.device_builder.event(Controller(Mouse(Left)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable left button of uinput device."))?
            .event(Controller(Mouse(Right)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable right button of uinput device."))?
            .event(Controller(Mouse(Middle)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable middle button of uinput device."))?
            .event(Controller(Mouse(Side)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable side button of uinput device."))?
            .event(Controller(Mouse(Extra)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable extra button of uinput device."))?
            .event(Controller(Mouse(Forward)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable forward button of uinput device."))?
            .event(Controller(Mouse(Back)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable back button of uinput device."))?
            .event(Controller(Mouse(Task)))
            .map_err(|_| Error::uinput_configuration_error("Cannot enable task button of uinput device."))?;

        Ok(UInputDeviceBuilder::from(result))
    }

    pub fn build(self) -> Result<UInputDevice, Error> {
        let device = self.device_builder.create()
            .map_err(|_| Error::uinput_configuration_error("Cannot build uinput device."))?;

        Ok(UInputDevice::new(device))
    }

    pub fn build_default() -> Result<UInputDevice, Error> {
        UInputDeviceBuilder::default()?
            .name("Nia virtual device")?
            .enable_keyboard()?
            .enable_mouse()?
            .build()
    }
}