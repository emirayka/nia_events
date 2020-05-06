use std::collections::HashMap;
use std::str::FromStr;

use evdev_rs::enums::EV_KEY;

use crate::ButtonId;
use crate::Error;

lazy_static! {
    static ref STRING_TO_KEY_ID_MAP: HashMap<&'static str, KeyId> = {
        let mut m = HashMap::new();

        m.insert("Tab", KeyId::from_ev_key(EV_KEY::KEY_TAB));
        m.insert("CapsLock", KeyId::from_ev_key(EV_KEY::KEY_CAPSLOCK));
        m.insert("LeftShift", KeyId::from_ev_key(EV_KEY::KEY_LEFTSHIFT));
        m.insert("LeftControl", KeyId::from_ev_key(EV_KEY::KEY_LEFTCTRL));
        m.insert("LeftMeta", KeyId::from_ev_key(EV_KEY::KEY_LEFTMETA));
        m.insert("LeftAlt", KeyId::from_ev_key(EV_KEY::KEY_LEFTALT));
        m.insert("Space", KeyId::from_ev_key(EV_KEY::KEY_SPACE));
        m.insert("RightShift", KeyId::from_ev_key(EV_KEY::KEY_RIGHTSHIFT));
        m.insert("RightControl", KeyId::from_ev_key(EV_KEY::KEY_RIGHTCTRL));
        m.insert("RightMeta", KeyId::from_ev_key(EV_KEY::KEY_RIGHTMETA));
        m.insert("RightAlt", KeyId::from_ev_key(EV_KEY::KEY_RIGHTALT));
        m.insert("Compose", KeyId::from_ev_key(EV_KEY::KEY_COMPOSE));
        m.insert("Enter", KeyId::from_ev_key(EV_KEY::KEY_ENTER));
        m.insert("BackSpace", KeyId::from_ev_key(EV_KEY::KEY_BACKSPACE));

        m.insert("a", KeyId::from_ev_key(EV_KEY::KEY_A));
        m.insert("b", KeyId::from_ev_key(EV_KEY::KEY_B));
        m.insert("c", KeyId::from_ev_key(EV_KEY::KEY_C));
        m.insert("d", KeyId::from_ev_key(EV_KEY::KEY_D));
        m.insert("e", KeyId::from_ev_key(EV_KEY::KEY_E));
        m.insert("f", KeyId::from_ev_key(EV_KEY::KEY_F));
        m.insert("g", KeyId::from_ev_key(EV_KEY::KEY_G));
        m.insert("h", KeyId::from_ev_key(EV_KEY::KEY_H));
        m.insert("i", KeyId::from_ev_key(EV_KEY::KEY_I));
        m.insert("j", KeyId::from_ev_key(EV_KEY::KEY_J));
        m.insert("k", KeyId::from_ev_key(EV_KEY::KEY_K));
        m.insert("l", KeyId::from_ev_key(EV_KEY::KEY_L));
        m.insert("m", KeyId::from_ev_key(EV_KEY::KEY_M));
        m.insert("n", KeyId::from_ev_key(EV_KEY::KEY_N));
        m.insert("o", KeyId::from_ev_key(EV_KEY::KEY_O));
        m.insert("p", KeyId::from_ev_key(EV_KEY::KEY_P));
        m.insert("q", KeyId::from_ev_key(EV_KEY::KEY_Q));
        m.insert("r", KeyId::from_ev_key(EV_KEY::KEY_R));
        m.insert("s", KeyId::from_ev_key(EV_KEY::KEY_S));
        m.insert("t", KeyId::from_ev_key(EV_KEY::KEY_T));
        m.insert("u", KeyId::from_ev_key(EV_KEY::KEY_U));
        m.insert("v", KeyId::from_ev_key(EV_KEY::KEY_V));
        m.insert("w", KeyId::from_ev_key(EV_KEY::KEY_W));
        m.insert("x", KeyId::from_ev_key(EV_KEY::KEY_X));
        m.insert("y", KeyId::from_ev_key(EV_KEY::KEY_Y));
        m.insert("z", KeyId::from_ev_key(EV_KEY::KEY_Z));

        m.insert("1", KeyId::from_ev_key(EV_KEY::KEY_1));
        m.insert("2", KeyId::from_ev_key(EV_KEY::KEY_2));
        m.insert("3", KeyId::from_ev_key(EV_KEY::KEY_3));
        m.insert("4", KeyId::from_ev_key(EV_KEY::KEY_4));
        m.insert("5", KeyId::from_ev_key(EV_KEY::KEY_5));
        m.insert("6", KeyId::from_ev_key(EV_KEY::KEY_6));
        m.insert("7", KeyId::from_ev_key(EV_KEY::KEY_7));
        m.insert("8", KeyId::from_ev_key(EV_KEY::KEY_8));
        m.insert("9", KeyId::from_ev_key(EV_KEY::KEY_9));
        m.insert("0", KeyId::from_ev_key(EV_KEY::KEY_0));

        m.insert(",", KeyId::from_ev_key(EV_KEY::KEY_COMMA));
        m.insert(".", KeyId::from_ev_key(EV_KEY::KEY_DOT));
        m.insert("/", KeyId::from_ev_key(EV_KEY::KEY_SLASH));
        m.insert(";", KeyId::from_ev_key(EV_KEY::KEY_SEMICOLON));
        m.insert("'", KeyId::from_ev_key(EV_KEY::KEY_APOSTROPHE));
        m.insert("[", KeyId::from_ev_key(EV_KEY::KEY_LEFTBRACE));
        m.insert("]", KeyId::from_ev_key(EV_KEY::KEY_RIGHTBRACE));
        m.insert("\\", KeyId::from_ev_key(EV_KEY::KEY_BACKSLASH));
        m.insert("`", KeyId::from_ev_key(EV_KEY::KEY_GRAVE));
        m.insert("-", KeyId::from_ev_key(EV_KEY::KEY_MINUS));
        m.insert("+", KeyId::from_ev_key(EV_KEY::KEY_EQUAL));

        m.insert("Esc", KeyId::from_ev_key(EV_KEY::KEY_ESC));
        m.insert("F1", KeyId::from_ev_key(EV_KEY::KEY_F1));
        m.insert("F2", KeyId::from_ev_key(EV_KEY::KEY_F2));
        m.insert("F3", KeyId::from_ev_key(EV_KEY::KEY_F3));
        m.insert("F4", KeyId::from_ev_key(EV_KEY::KEY_F4));
        m.insert("F5", KeyId::from_ev_key(EV_KEY::KEY_F5));
        m.insert("F6", KeyId::from_ev_key(EV_KEY::KEY_F6));
        m.insert("F7", KeyId::from_ev_key(EV_KEY::KEY_F7));
        m.insert("F8", KeyId::from_ev_key(EV_KEY::KEY_F8));
        m.insert("F9", KeyId::from_ev_key(EV_KEY::KEY_F9));
        m.insert("F10", KeyId::from_ev_key(EV_KEY::KEY_F10));
        m.insert("F11", KeyId::from_ev_key(EV_KEY::KEY_F11));
        m.insert("F12", KeyId::from_ev_key(EV_KEY::KEY_F12));

        m.insert("Print", KeyId::from_ev_key(EV_KEY::KEY_SYSRQ));
        m.insert("ScrollLock", KeyId::from_ev_key(EV_KEY::KEY_SCROLLLOCK));
        m.insert("Pause", KeyId::from_ev_key(EV_KEY::KEY_PAUSE));
        m.insert("Insert", KeyId::from_ev_key(EV_KEY::KEY_INSERT));
        m.insert("Home", KeyId::from_ev_key(EV_KEY::KEY_HOME));
        m.insert("PageUp", KeyId::from_ev_key(EV_KEY::KEY_PAGEUP));
        m.insert("Delete", KeyId::from_ev_key(EV_KEY::KEY_DELETE));
        m.insert("End", KeyId::from_ev_key(EV_KEY::KEY_END));
        m.insert("PageDown", KeyId::from_ev_key(EV_KEY::KEY_PAGEDOWN));
        m.insert("Up", KeyId::from_ev_key(EV_KEY::KEY_UP));
        m.insert("Down", KeyId::from_ev_key(EV_KEY::KEY_DOWN));
        m.insert("Left", KeyId::from_ev_key(EV_KEY::KEY_UP));
        m.insert("Right", KeyId::from_ev_key(EV_KEY::KEY_RIGHT));

        m.insert("NumLock", KeyId::from_ev_key(EV_KEY::KEY_NUMLOCK));
        m.insert("KPSlash", KeyId::from_ev_key(EV_KEY::KEY_KPSLASH));
        m.insert("KPAsterisk", KeyId::from_ev_key(EV_KEY::KEY_NUMLOCK));
        m.insert("KPMinus", KeyId::from_ev_key(EV_KEY::KEY_NUMLOCK));
        m.insert("KP1", KeyId::from_ev_key(EV_KEY::KEY_KP1));
        m.insert("KP2", KeyId::from_ev_key(EV_KEY::KEY_KP2));
        m.insert("KP3", KeyId::from_ev_key(EV_KEY::KEY_KP3));
        m.insert("KP4", KeyId::from_ev_key(EV_KEY::KEY_KP4));
        m.insert("KP5", KeyId::from_ev_key(EV_KEY::KEY_KP5));
        m.insert("KP6", KeyId::from_ev_key(EV_KEY::KEY_KP6));
        m.insert("KP7", KeyId::from_ev_key(EV_KEY::KEY_KP7));
        m.insert("KP8", KeyId::from_ev_key(EV_KEY::KEY_KP8));
        m.insert("KP9", KeyId::from_ev_key(EV_KEY::KEY_KP9));
        m.insert("KP0", KeyId::from_ev_key(EV_KEY::KEY_KP0));
        m.insert("KPPlus", KeyId::from_ev_key(EV_KEY::KEY_KPPLUS));
        m.insert("KPEnter", KeyId::from_ev_key(EV_KEY::KEY_KPENTER));
        m.insert("KPDot", KeyId::from_ev_key(EV_KEY::KEY_KPDOT));

        m
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyId {
    id: u16,
}

impl KeyId {
    pub fn new(id: u16) -> KeyId {
        KeyId { id }
    }

    pub fn from_ev_key(ev_key: EV_KEY) -> KeyId {
        KeyId { id: ev_key as u16 }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

impl FromStr for KeyId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match STRING_TO_KEY_ID_MAP.get(s) {
            Some(v) => Ok(*v),
            None => Err(Error::key_parse_error(&format!(
                "Cannot parse {} as key identifier.",
                s
            ))),
        }
    }
}

impl From<ButtonId> for KeyId {
    fn from(button_id: ButtonId) -> Self {
        // buttons defined as part of key id enum named as KEY_BUTTON_LEFT, etc
        let key_id = uinput_sys::BTN_LEFT as u16 - 1 + button_id.get_id();

        KeyId::new(key_id)
    }
}
