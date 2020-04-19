use std::thread;
use std::sync::mpsc;

use uinput_sys;
use uinput;

use crate::input_listeners::{KeyChord};
use crate::output_senders::KeyCommand;
use std::sync::mpsc::TryRecvError;

fn uinput_write_key_chord(uinput_device: &mut uinput::Device, key_chord: KeyChord) {
    let modifiers = key_chord.get_modifiers();
    let key = key_chord.get_key();

    for modifier in modifiers {
        uinput_device.write(
            uinput_sys::EV_KEY,
            modifier.get_key_id().get_id() as i32,
            1
        ).expect("nia-events: Failed sending key event.");
    }

    uinput_device.write(
        uinput_sys::EV_KEY,
        key.get_key_id().get_id() as i32,
        1
    ).expect("nia-events: Failed sending key event.");
    uinput_device.write(
        uinput_sys::EV_KEY,
        key.get_key_id().get_id() as i32,
        0
    ).expect("nia-events: Failed sending key event.");

    for modifier in modifiers {
        uinput_device.write(
            uinput_sys::EV_KEY,
            modifier.get_key_id().get_id() as i32,
            0
        ).expect("nia-events: Failed sending key event.");
    }
    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

pub struct KeyWorker {}

impl KeyWorker {
    pub fn new() -> KeyWorker {
        KeyWorker {}
    }

    pub fn start_working(&self) -> (mpsc::Sender<KeyCommand>, mpsc::Sender<()>) {
        let (
            command_sender,
            command_receiver
        ) = mpsc::channel();

        let (tx, rx) = mpsc::channel();

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
                match command_receiver.recv() {
                    Ok(key_command) => {
                        match key_command {
                            KeyCommand::ForwardKeyChord(key_chord) => {
                                uinput_write_key_chord(&mut uinput_device, key_chord);
                            }
                        }
                    },
                    Err(_) => {
                        break;
                    }
                }

                match rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => {
                        break;
                    },
                    Err(TryRecvError::Empty) => {}
                }
            }
        });

        (command_sender, tx)
    }
}
