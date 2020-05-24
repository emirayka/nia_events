use std::collections::HashMap;

use crate::Error;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeviceId {
    id: u16,
}

impl DeviceId {
    pub fn new(id: u16) -> DeviceId {
        DeviceId { id }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

impl DeviceId {
    pub fn from(
        s: &str,
        names_to_keyboard_identifiers: &HashMap<String, DeviceId>,
    ) -> Result<DeviceId, Error> {
        if let Ok(id) = s.parse() {
            Ok(DeviceId::new(id))
        } else if names_to_keyboard_identifiers.contains_key(s) {
            Ok(*names_to_keyboard_identifiers.get(s).unwrap())
        } else {
            Err(Error::key_parse_error(""))
        }
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
