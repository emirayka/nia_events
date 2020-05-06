use crate::Error;

use crate::enums::KeyId;
use crate::enums::ButtonId;
use crate::enums::KeyChord;

pub struct UInputDevice {
    device: uinput::Device,
}

impl UInputDevice {
    pub fn new(device: uinput::Device) -> UInputDevice {
        UInputDevice {
            device
        }
    }

    fn send_key_down_event(&mut self, code: i32) -> Result<(), Error> {
        self.device
            .write(uinput_sys::EV_KEY, code, 1)
            .map_err(|_| Error::uinput_error(&format!("Error sending key down event: {}.", code)))?;

        Ok(())
    }

    fn send_key_up_event(&mut self, code: i32) -> Result<(), Error> {
        self.device
            .write(uinput_sys::EV_KEY, code, 0)
            .map_err(|_| Error::uinput_error(&format!("Error sending key up event: {}.", code)))?;

        Ok(())
    }

    fn send_sync_event(&mut self) -> Result<(), Error> {
        self.device
            .synchronize()
            .map_err(|_| Error::uinput_error("Cannot send synchronize event."))?;

        Ok(())
    }

    pub fn key_down(&mut self, key_id: KeyId) -> Result<(), Error> {
        self.send_key_down_event(key_id.get_id() as i32)?;
        self.send_sync_event()?;

        Ok(())
    }

    pub fn key_press(&mut self, key_id: KeyId) -> Result<(), Error> {
        self.send_key_down_event(key_id.get_id() as i32)?;
        self.send_sync_event()?;

        self.send_key_up_event(key_id.get_id() as i32)?;
        self.send_sync_event()?;

        Ok(())
    }

    pub fn key_up(&mut self, key_id: KeyId) -> Result<(), Error> {
        self.send_key_up_event(key_id.get_id() as i32)?;
        self.send_sync_event()?;

        Ok(())
    }

    pub fn key_chord_press(&mut self, key_chord: KeyChord) -> Result<(), Error> {
        let modifiers = key_chord.get_modifiers();
        let key = key_chord.get_key();

        for modifier in modifiers {
            self.key_down(modifier.get_key_id())?;
        }

        self.key_down(key.get_key_id())?;
        self.key_up(key.get_key_id())?;

        for modifier in modifiers {
            self.key_up(modifier.get_key_id())?;
        }

        Ok(())
    }

    pub fn mouse_button_down(&mut self, button_id: ButtonId) -> Result<(), Error> {
        let key_id = button_id.into();

        self.key_down(key_id)?;

        Ok(())
    }

    pub fn mouse_button_press(&mut self, button_id: ButtonId) -> Result<(), Error> {
        let key_id = button_id.into();

        self.key_down(key_id)?;
        self.key_up(key_id)?;

        Ok(())
    }

    pub fn mouse_button_up(&mut self, button_id: ButtonId) -> Result<(), Error> {
        let key_id = button_id.into();

        self.key_down(key_id)?;
        self.key_up(key_id)?;

        Ok(())
    }
}