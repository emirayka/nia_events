use std::collections::HashMap;
use crate::key::{KeyboardId, KeyChord};
use crate::utils::str_to_key_chord_part;

pub fn str_to_key_chord(
    s: &str,
    names_to_keyboard_identifiers: &HashMap<String, KeyboardId>
) -> Result<KeyChord, ()> {
    let key_chord_parts: Vec<&str> = s.split("+").collect();

    if key_chord_parts.len() == 0 {
        return Err(())
    }

    if key_chord_parts.len() == 1 {
        let key_chord_part = str_to_key_chord_part(key_chord_parts[0], names_to_keyboard_identifiers)?;

        Ok(KeyChord::new(
            vec!(),
            key_chord_part
        ))
    } else {
        let (modifier_strs, key_str) = key_chord_parts.split_at(
            key_chord_parts.len() - 1
        );

        let mut modifiers = Vec::new();

        for modifier_str in modifier_strs {
            let modifier = str_to_key_chord_part(
                modifier_str,
                names_to_keyboard_identifiers
            )?;

            modifiers.push(modifier);
        }

        let key = str_to_key_chord_part(
            key_str[0],
            names_to_keyboard_identifiers
        )?;

        Ok(KeyChord::new(
            modifiers,
            key
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::KeyChordPart;
    use crate::utils::str_to_key_id;

    #[test]
    fn works_correctly() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec!(
            (KeyChord::new(vec!(), KeyChordPart::Key1(str_to_key_id("a").unwrap())), "a"),
            (KeyChord::new(vec!(), KeyChordPart::Key1(str_to_key_id("b").unwrap())), "b"),

            (KeyChord::new(vec!(), KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("b").unwrap())), "0:b"),
            (KeyChord::new(vec!(), KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("b").unwrap())), "1:b"),
            (KeyChord::new(vec!(), KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("b").unwrap())), "first:b"),
            (KeyChord::new(vec!(), KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("b").unwrap())), "second:b"),

            (KeyChord::new(
                vec!(KeyChordPart::Key1(str_to_key_id("a").unwrap())),
                KeyChordPart::Key1(str_to_key_id("b").unwrap())),
             "a+b"
            ),

            (KeyChord::new(
                vec!(
                    KeyChordPart::Key1(str_to_key_id("a").unwrap()),
                    KeyChordPart::Key1(str_to_key_id("b").unwrap())
                ),
                KeyChordPart::Key1(str_to_key_id("c").unwrap())),
             "a+b+c"
            ),

            (KeyChord::new(
                vec!(
                    KeyChordPart::Key1(str_to_key_id("a").unwrap()),
                ),
                KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("c").unwrap())),
             "a+0:c"
            ),

            (KeyChord::new(
                vec!(
                    KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("a").unwrap()),
                ),
                KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("c").unwrap())),
             "1:a+0:c"
            ),

            (KeyChord::new(
                vec!(
                    KeyChordPart::Key2(KeyboardId::new(1), str_to_key_id("a").unwrap()),
                ),
                KeyChordPart::Key2(KeyboardId::new(0), str_to_key_id("c").unwrap())),
             "second:a+first:c"
            ),
        );

        for spec in specs {
            let expected = spec.0;
            let result = str_to_key_chord(spec.1, &names_to_keyboard_identifiers).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn fails_when_incorrect_key_chord_was_provided() {
        let mut names_to_keyboard_identifiers = HashMap::new();

        names_to_keyboard_identifiers.insert(String::from("first"), KeyboardId::new(0));
        names_to_keyboard_identifiers.insert(String::from("second"), KeyboardId::new(1));

        let specs = vec!(
            "bb",
            "0:bb",
            "first:bb",
            "c:b",

            "aa+b",
            "0:aa+b",
            "first:aa+b",
            "c:a+b",
        );

        for spec in specs {
            let result = str_to_key_chord(spec, &names_to_keyboard_identifiers);

            assert!(result.is_err());
        }
    }
}