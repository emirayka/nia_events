use std::thread;
use std::sync::mpsc;

use uinput_sys;
use uinput;

use crate::input_listeners::{KeyChord, KeyId};
use crate::output_senders::KeyCommand;
use std::time::Duration;
use std::convert::TryFrom;

fn uinput_write_key_chord(uinput_device: &mut uinput::Device, key_chord: KeyChord) {
    let modifiers = key_chord.get_modifiers();
    let key = key_chord.get_key();

    for modifier in modifiers {
        uinput_device.write(uinput_sys::EV_KEY, modifier.get_key_id().get_id() as i32, 1);
        // uinput_device.synchronize();
    }

    uinput_device.write(uinput_sys::EV_KEY, key.get_key_id().get_id() as i32, 1);
    // uinput_device.synchronize();
    uinput_device.write(uinput_sys::EV_KEY, key.get_key_id().get_id() as i32, 0);
    // uinput_device.synchronize();

    for modifier in modifiers {
        uinput_device.write(uinput_sys::EV_KEY, modifier.get_key_id().get_id() as i32, 0);
        // uinput_device.synchronize();
    }
    uinput_device.synchronize();
}

pub struct KeyWorker {}

impl KeyWorker {
    pub fn new() -> KeyWorker {
        KeyWorker {}
    }

    pub fn start_working(&self) -> mpsc::Sender<KeyCommand> {
        let (
            command_sender,
            command_receiver
        ) = mpsc::channel();

        thread::spawn(move || {
            let mut uinput_device = uinput::default()
                .expect("Can't create default uinput device.")
                .name("test")
                .expect("Can't name device?.")
                .event(uinput::event::Keyboard::All)
                .expect("Can't set flags")
                .create()
                .expect("Can't create default uinput device.");

            loop {
                let key_command = command_receiver.recv()
                    .expect("Failure while reading key command.");

                match key_command {
                    KeyCommand::ForwardKeyChord(key_chord) => {
                        uinput_write_key_chord(&mut uinput_device, key_chord);
                    }
                }
            }
        });

        command_sender
    }
}
