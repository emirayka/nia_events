use evdev_rs::enums::EV_KEY;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ButtonId {
    id: u16,
}

impl ButtonId {
    pub fn new(id: u16) -> ButtonId {
        ButtonId {
            id,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}
