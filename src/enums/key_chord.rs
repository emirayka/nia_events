use std::collections::HashMap;
use std::hash::Hash;

use crate::Error;
use crate::KeyChordPart;
use crate::KeyboardId;

#[derive(Clone, Debug, Eq, Hash)]
pub struct KeyChord {
    modifiers: Vec<KeyChordPart>,
    key: KeyChordPart,
}

impl PartialEq for KeyChord {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key {
            return false;
        }

        if self.modifiers.len() != other.modifiers.len() {
            return false;
        }

        for key_chord_part in &self.modifiers {
            if !other.modifiers.contains(key_chord_part) {
                return false;
            }
        }

        true
    }
}

impl KeyChord {
    pub fn new(modifiers: Vec<KeyChordPart>, key: KeyChordPart) -> KeyChord {
        KeyChord { modifiers, key }
    }

    pub fn get_modifiers(&self) -> &Vec<KeyChordPart> {
        &self.modifiers
    }

    pub fn get_key(&self) -> &KeyChordPart {
        &self.key
    }
}

impl KeyChord {
    pub fn from(
        s: &str,
        names_to_keyboard_identifiers: &HashMap<String, KeyboardId>,
    ) -> Result<KeyChord, Error> {
        let mut key_chord_parts: Vec<&str> = s.split("+").collect();

        if key_chord_parts.len() == 0 {
            return Err(Error::key_parse_error("Invalid key chord specification."));
        }

        let key_chord_part = KeyChordPart::from(
            key_chord_parts.remove(key_chord_parts.len() - 1),
            names_to_keyboard_identifiers,
        )?;

        let mut modifiers = Vec::new();

        for key_chord_part in key_chord_parts {
            let result = KeyChordPart::from(key_chord_part, names_to_keyboard_identifiers)?;
            modifiers.push(result);
        }

        Ok(KeyChord::new(modifiers, key_chord_part))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use crate::KeyId;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn works_correctly() {
            let mut names_to_keyboard_identifiers = HashMap::new();

            names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
            names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

            let specs = vec![
                (
                    KeyChord::new(vec![], KeyChordPart::Key1(KeyId::from_str("a").unwrap())),
                    "a",
                ),
                (
                    KeyChord::new(vec![], KeyChordPart::Key1(KeyId::from_str("b").unwrap())),
                    "b",
                ),
                (
                    KeyChord::new(
                        vec![],
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("b").unwrap()),
                    ),
                    "0:b",
                ),
                (
                    KeyChord::new(
                        vec![],
                        KeyChordPart::Key2(KeyboardId::new(1), KeyId::from_str("b").unwrap()),
                    ),
                    "1:b",
                ),
                (
                    KeyChord::new(
                        vec![],
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("b").unwrap()),
                    ),
                    "first:b",
                ),
                (
                    KeyChord::new(
                        vec![],
                        KeyChordPart::Key2(KeyboardId::new(1), KeyId::from_str("b").unwrap()),
                    ),
                    "second:b",
                ),
                (
                    KeyChord::new(
                        vec![KeyChordPart::Key1(KeyId::from_str("a").unwrap())],
                        KeyChordPart::Key1(KeyId::from_str("b").unwrap()),
                    ),
                    "a+b",
                ),
                (
                    KeyChord::new(
                        vec![
                            KeyChordPart::Key1(KeyId::from_str("a").unwrap()),
                            KeyChordPart::Key1(KeyId::from_str("b").unwrap()),
                        ],
                        KeyChordPart::Key1(KeyId::from_str("c").unwrap()),
                    ),
                    "a+b+c",
                ),
                (
                    KeyChord::new(
                        vec![KeyChordPart::Key1(KeyId::from_str("a").unwrap())],
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("c").unwrap()),
                    ),
                    "a+0:c",
                ),
                (
                    KeyChord::new(
                        vec![KeyChordPart::Key2(
                            KeyboardId::new(1),
                            KeyId::from_str("a").unwrap(),
                        )],
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("c").unwrap()),
                    ),
                    "1:a+0:c",
                ),
                (
                    KeyChord::new(
                        vec![KeyChordPart::Key2(
                            KeyboardId::new(1),
                            KeyId::from_str("a").unwrap(),
                        )],
                        KeyChordPart::Key2(KeyboardId::new(0), KeyId::from_str("c").unwrap()),
                    ),
                    "second:a+first:c",
                ),
            ];

            for spec in specs {
                let expected = spec.0;
                let result = KeyChord::from(spec.1, &names_to_keyboard_identifiers).unwrap();

                assert_eq!(expected, result);
            }
        }

        #[test]
        fn fails_when_incorrect_key_chord_was_provided() {
            let mut names_to_keyboard_identifiers = HashMap::new();

            names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
            names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

            let specs = vec![
                "bb",
                "0:bb",
                "first:bb",
                "c:b",
                "aa+b",
                "0:aa+b",
                "first:aa+b",
                "c:a+b",
            ];

            for spec in specs {
                let result = KeyChord::from(spec, &names_to_keyboard_identifiers);

                assert!(result.is_err());
            }
        }
    }
}
