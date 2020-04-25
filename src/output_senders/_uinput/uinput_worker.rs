use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

use uinput_sys;

use crate::input_listeners::{KeyChord};
use crate::output_senders::UinputCommand;
use crate::{KeyId, ButtonId};
use std::time::Duration;

fn uinput_make_device() -> uinput::Device {
    use uinput;
    use uinput::event::Event::Controller;
    use uinput::event::controller::Controller::Mouse;
    use uinput::event::controller::Mouse::*;

    use uinput::event::Event::Relative;
    use uinput::event::Event::Absolute;
    use uinput::event::relative;
    use uinput::event::absolute;

    uinput::default()
        .expect("Can't create default uinput device.")
        .name("nia virtual input device")
        .expect("Can't name device.")
        .event(uinput::event::Keyboard::All).expect("Can't set flags")
        .event(Controller(Mouse(Left))).expect("Can't set flags")
        .event(Controller(Mouse(Right))).expect("Can't set flags")
        .event(Controller(Mouse(Middle))).expect("Can't set flags")
        .event(Controller(Mouse(Side))).expect("Can't set flags")
        .event(Controller(Mouse(Extra))).expect("Can't set flags")
        .event(Controller(Mouse(Forward))).expect("Can't set flags")
        .event(Controller(Mouse(Back))).expect("Can't set flags")
        .event(Controller(Mouse(Task))).expect("Can't set flags")
        .event(Relative(relative::Relative::Position(relative::Position::X))).expect("Can't set flags")
        .event(Relative(relative::Relative::Position(relative::Position::Y))).expect("Can't set flags")
        // .event(Absolute(absolute::Absolute::Position(absolute::Position::X))).expect("Can't set flags")
        // .event(Absolute(absolute::Absolute::Position(absolute::Position::Y))).expect("Can't set flags")
        .create()
        .expect("Can't create default uinput device.")
}

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

fn uinput_write_type_text_event(uinput_device: &mut uinput::Device, text: String) {
    use crate::utils;

    let shift_key_id = match utils::str_to_key_id("LeftShift") {
        Ok(key_id) => key_id,
        _ => return
    };

    for c in text.chars() {
        let key_id = match utils::str_to_key_id(&c.to_string()) {
            Ok(key_id) => key_id,
            _ => break
        };

        if c.is_uppercase() {
            uinput_write_key_down_event(uinput_device, shift_key_id);
        }

        uinput_write_key_press_event(uinput_device, key_id);

        if c.is_uppercase() {
            uinput_write_key_up_event(uinput_device, shift_key_id);
        }
    }
}

fn uinput_write_key_down_event(uinput_device: &mut uinput::Device, key_id: KeyId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        key_id.get_id() as i32,
        1
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_key_press_event(uinput_device: &mut uinput::Device, key_id: KeyId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        key_id.get_id() as i32,
        1
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");

    uinput_device.write(
        uinput_sys::EV_KEY,
        key_id.get_id() as i32,
        0
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_key_up_event(uinput_device: &mut uinput::Device, key_id: KeyId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        key_id.get_id() as i32,
        0
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_mouse_button_down_event(uinput_device: &mut uinput::Device, button_id: ButtonId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        uinput_sys::BTN_LEFT - 1 + button_id.get_id() as i32,
        1
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_mouse_button_press_event(uinput_device: &mut uinput::Device, button_id: ButtonId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        uinput_sys::BTN_LEFT - 1 + button_id.get_id() as i32,
        1
    ).expect("nia-events: Failed sending key event.");
    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");

    uinput_device.write(
        uinput_sys::EV_KEY,
        uinput_sys::BTN_LEFT - 1 + button_id.get_id() as i32,
        0
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_mouse_button_up_event(uinput_device: &mut uinput::Device, button_id: ButtonId) {
    uinput_device.write(
        uinput_sys::EV_KEY,
        uinput_sys::BTN_LEFT - 1 + button_id.get_id() as i32,
        0
    ).expect("nia-events: Failed sending key event.");

    uinput_device.synchronize().expect("nia-events: Failed to synchronize.");
}

fn uinput_write_mouse_move_by_event(uinput_device: &mut uinput::Device, x: i16, y: i16) {
    use xcb::*;
    use xcb::xproto::*;

    let (connection, _) = xcb::Connection::connect(None).unwrap();

    xcb::xproto::warp_pointer(
        &connection,
        xcb::base::NONE,
        xcb::base::NONE,
        0,
        0,
        0,
        0,
        x,
        y
    ).request_check();
}

fn uinput_write_mouse_move_to_event(uinput_device: &mut uinput::Device, x: i16, y: i16) {
    use xcb::*;
    use xcb::xproto::*;

    let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

    let setup = connection.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    xcb::xproto::warp_pointer(
        &connection,
        xcb::base::NONE,
        root,
        0,
        0,
        0,
        0,
        x,
        y
    ).request_check();
}

pub struct KeyWorker {}

impl KeyWorker {
    pub fn new() -> KeyWorker {
        KeyWorker {}
    }

    pub fn start_working(&self) -> (mpsc::Sender<UinputCommand>, mpsc::Sender<()>) {
        let (
            command_sender,
            command_receiver
        ) = mpsc::channel();

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut uinput_device = uinput_make_device();

            loop {
                match command_receiver.recv() {
                    Ok(key_command) => {
                        match key_command {
                            UinputCommand::ForwardKeyChord(key_chord) => {
                                uinput_write_key_chord(&mut uinput_device, key_chord);
                            },
                            UinputCommand::KeyDown(key_id) => {
                                uinput_write_key_down_event(&mut uinput_device, key_id);
                            },
                            UinputCommand::KeyPress(key_id) => {
                                uinput_write_key_press_event(&mut uinput_device, key_id);
                            },
                            UinputCommand::KeyUp(key_id) => {
                                uinput_write_key_up_event(&mut uinput_device, key_id);
                            },
                            UinputCommand::MouseButtonDown(button_id) => {
                                uinput_write_mouse_button_down_event(&mut uinput_device, button_id);
                            },
                            UinputCommand::MouseButtonPress(button_id) => {
                                uinput_write_mouse_button_press_event(&mut uinput_device, button_id);
                            },
                            UinputCommand::MouseButtonUp(button_id) => {
                                uinput_write_mouse_button_up_event(&mut uinput_device, button_id);
                            },
                            UinputCommand::MouseMoveBy(x, y) => {
                                uinput_write_mouse_move_by_event(&mut uinput_device, x, y);
                            },
                            UinputCommand::MouseMoveTo(x, y) => {
                                uinput_write_mouse_move_to_event(&mut uinput_device, x, y);
                            },
                            UinputCommand::TextType(string) => {
                                uinput_write_type_text_event(&mut uinput_device,string);
                            },
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
