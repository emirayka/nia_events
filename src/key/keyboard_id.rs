#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardId {
    id: u16,
}

impl KeyboardId {
    pub fn new(id: u16) -> KeyboardId {
        KeyboardId {
            id,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}