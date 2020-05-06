use std::collections::HashMap;

use crate::Error;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyboardId {
    id: u16,
}

impl KeyboardId {
    pub fn new(id: u16) -> KeyboardId {
        KeyboardId { id }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

impl KeyboardId {
    pub fn from(
        s: &str,
        names_to_keyboard_identifiers: &HashMap<String, KeyboardId>
    ) -> Result<KeyboardId, Error> {
        if let Ok(id) = s.parse() {
            Ok(KeyboardId::new(id))
        } else if names_to_keyboard_identifiers.contains_key(s) {
            Ok(*names_to_keyboard_identifiers.get(s).unwrap())
        } else {
            Err(Error::key_parse_error(""))
        }
    }
}

impl std::fmt::Display for KeyboardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
