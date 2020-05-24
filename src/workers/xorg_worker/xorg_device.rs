use crate::Error;
use enigo::{Enigo, KeyboardControllable, MouseControllable};

pub struct XorgDevice {
    enigo: Enigo,
}

impl XorgDevice {
    pub fn new() -> XorgDevice {
        XorgDevice {
            enigo: Enigo::new(),
        }
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), Error> {
        self.enigo.key_sequence(text);

        Ok(())
    }

    pub fn mouse_move_by(&mut self, x: i16, y: i16) -> Result<(), Error> {
        self.enigo.mouse_move_relative(x as i32, y as i32);

        Ok(())
    }

    pub fn mouse_move_to(&mut self, x: i16, y: i16) -> Result<(), Error> {
        self.enigo.mouse_move_to(x as i32, y as i32);

        Ok(())
    }
}
