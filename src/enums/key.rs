use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

use crate::enums::KeyId;
use crate::enums::DeviceId;
use crate::Error;

#[derive(Clone, Copy, Debug, Eq)]
pub enum Key {
    Key1(KeyId),
    Key2(DeviceId, KeyId),
}

impl Key {
    pub fn get_key_id(&self) -> KeyId {
        match self {
            Key::Key1(key_id) => *key_id,
            Key::Key2(_, key_id) => *key_id,
        }
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Key::Key1(key_id) => key_id.hash(state),
            Key::Key2(_, key_id) => key_id.hash(state),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Key::Key1(key_id_1), Key::Key1(key_id_2)) => key_id_1 == key_id_2,
            (Key::Key1(key_id_1), Key::Key2(_, key_id_2)) => key_id_1 == key_id_2,
            (Key::Key2(_, key_id_1), Key::Key1(key_id_2)) => key_id_1 == key_id_2,
            (
                Key::Key2(keyboard_id_1, key_id_1),
                Key::Key2(keyboard_id_2, key_id_2),
            ) => keyboard_id_1 == keyboard_id_2 && key_id_1 == key_id_2,
        }
    }
}

impl Key {
    pub fn from(
        s: &str,
        names_to_keyboard_identifiers: &HashMap<String, DeviceId>,
    ) -> Result<Key, Error> {
        let key_chord_part_parts = s.split(":").collect::<Vec<&str>>();

        match key_chord_part_parts.len() {
            1 => {
                let key_id = key_chord_part_parts[0].parse::<KeyId>()?;

                Ok(Key::Key1(key_id))
            }
            2 => {
                let keyboard_id =
                    DeviceId::from(key_chord_part_parts[0], names_to_keyboard_identifiers)?;

                let key_id = key_chord_part_parts[1].parse::<KeyId>()?;

                Ok(Key::Key2(keyboard_id, key_id))
            }
            _ => {
                return Err(Error::key_parse_error(
                    "Invalid key element count in key chord part.",
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn works_correctly() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), DeviceId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), DeviceId::new(1));

        let specs = vec![
            (Key::Key1(KeyId::from_str("a").unwrap()), "a"),
            (Key::Key1(KeyId::from_str("b").unwrap()), "b"),
            (
                Key::Key2(DeviceId::new(0), KeyId::from_str("b").unwrap()),
                "0:b",
            ),
            (
                Key::Key2(DeviceId::new(1), KeyId::from_str("b").unwrap()),
                "1:b",
            ),
            (
                Key::Key2(DeviceId::new(0), KeyId::from_str("b").unwrap()),
                "first:b",
            ),
            (
                Key::Key2(DeviceId::new(1), KeyId::from_str("b").unwrap()),
                "second:b",
            ),
        ];

        for spec in specs {
            let expected = spec.0;
            let result = Key::from(spec.1, &names_to_keyboard_identifiers).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn fails_when_incorrect_key_chord_part_description() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), DeviceId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), DeviceId::new(1));

        let specs = vec!["bb", "0:bb", "first:bb", "c:b"];

        for spec in specs {
            let result = Key::from(spec, &names_to_keyboard_identifiers);

            assert!(result.is_err());
        }
    }
}
