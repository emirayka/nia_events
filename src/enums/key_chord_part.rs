use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

use crate::enums::KeyId;
use crate::enums::KeyboardId;
use crate::Error;

#[derive(Clone, Copy, Debug, Eq)]
pub enum KeyChordPart {
    Key1(KeyId),
    Key2(KeyboardId, KeyId),
}

impl KeyChordPart {
    pub fn get_key_id(&self) -> KeyId {
        match self {
            KeyChordPart::Key1(key_id) => *key_id,
            KeyChordPart::Key2(_, key_id) => *key_id,
        }
    }
}

impl Hash for KeyChordPart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            KeyChordPart::Key1(key_id) => key_id.hash(state),
            KeyChordPart::Key2(_, key_id) => key_id.hash(state),
        }
    }
}

impl PartialEq for KeyChordPart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyChordPart::Key1(key_id_1), KeyChordPart::Key1(key_id_2)) => key_id_1 == key_id_2,
            (KeyChordPart::Key1(key_id_1), KeyChordPart::Key2(_, key_id_2)) => key_id_1 == key_id_2,
            (KeyChordPart::Key2(_, key_id_1), KeyChordPart::Key1(key_id_2)) => key_id_1 == key_id_2,
            (
                KeyChordPart::Key2(keyboard_id_1, key_id_1),
                KeyChordPart::Key2(keyboard_id_2, key_id_2),
            ) => keyboard_id_1 == keyboard_id_2 && key_id_1 == key_id_2,
        }
    }
}

impl KeyChordPart {
    pub fn from(
        s: &str,
        names_to_keyboard_identifiers: &HashMap<String, KeyboardId>,
    ) -> Result<KeyChordPart, Error> {
        let key_chord_part_parts = s.split(":").collect::<Vec<&str>>();

        match key_chord_part_parts.len() {
            1 => {
                let key_id = key_chord_part_parts[0].parse::<KeyId>()?;

                Ok(KeyChordPart::Key1(key_id))
            }
            2 => {
                let keyboard_id =
                    KeyboardId::from(key_chord_part_parts[0], names_to_keyboard_identifiers)?;

                let key_id = key_chord_part_parts[1].parse::<KeyId>()?;

                Ok(KeyChordPart::Key2(keyboard_id, key_id))
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

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec![
            (KeyChordPart::Key1(KeyId::from_str("a").unwrap()), "a"),
            (KeyChordPart::Key1(KeyId::from_str("b").unwrap()), "b"),
            (
                KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("b").unwrap()),
                "0:b",
            ),
            (
                KeyChordPart::Key2(KeyboardId::new(1), KeyId::from_str("b").unwrap()),
                "1:b",
            ),
            (
                KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("b").unwrap()),
                "first:b",
            ),
            (
                KeyChordPart::Key2(KeyboardId::new(1), KeyId::from_str("b").unwrap()),
                "second:b",
            ),
        ];

        for spec in specs {
            let expected = spec.0;
            let result = KeyChordPart::from(spec.1, &names_to_keyboard_identifiers).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn fails_when_incorrect_key_chord_part_description() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec!["bb", "0:bb", "first:bb", "c:b"];

        for spec in specs {
            let result = KeyChordPart::from(spec, &names_to_keyboard_identifiers);

            assert!(result.is_err());
        }
    }
}
