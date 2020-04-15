use std::collections::HashMap;

use crate::input_listeners::{KeyboardId, KeyChordPart};
use crate::str_to_key_id;

fn str_to_keyboard_id(
    s: &str,
    names_to_keyboard_identifiers: &HashMap<String, KeyboardId>
) -> Result<KeyboardId, ()> {
    if let Ok(id) = s.parse() {
        Ok(KeyboardId::new(id))
    } else if names_to_keyboard_identifiers.contains_key(s) {
        Ok(*names_to_keyboard_identifiers.get(s).unwrap())
    } else {
        Err(())
    }
}

pub fn str_to_key_chord_part(
    s: &str,
    names_to_keyboard_identifiers: &HashMap<String, KeyboardId>
) -> Result<KeyChordPart, ()> {
    let key_chord_part_parts = s.split(":").collect::<Vec<&str>>();

    match key_chord_part_parts.len() {
        1 => {
            let key_id = str_to_key_id(key_chord_part_parts[0])?;

            Ok(KeyChordPart::Key1(key_id))
        },
        2 => {
            let keyboard_id = str_to_keyboard_id(
                key_chord_part_parts[0],
                names_to_keyboard_identifiers
            )?;

            let key_id = str_to_key_id(
                key_chord_part_parts[1]
            )?;

            Ok(KeyChordPart::Key2(keyboard_id, key_id))
        },
        _ => {
            return Err(())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_correctly() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec!(
            (KeyChordPart::Key1(str_to_key_id("a").unwrap()), "a"),
            (KeyChordPart::Key1(str_to_key_id("b").unwrap()), "b"),
            (KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("b").unwrap()), "0:b"),
            (KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("b").unwrap()), "1:b"),
            (KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("b").unwrap()), "first:b"),
            (KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("b").unwrap()), "second:b"),
        );

        for spec in specs {
            let expected = spec.0;
            let result = str_to_key_chord_part(spec.1, &names_to_keyboard_identifiers).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn fails_when_incorrect_key_chord_part_description() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec!(
            "bb",
            "0:bb",
            "first:bb",

            "c:b",
        );

        for spec in specs {
            let result = str_to_key_chord_part(
                spec,
                &names_to_keyboard_identifiers
            );

            assert!(result.is_err());
        }
    }
}
