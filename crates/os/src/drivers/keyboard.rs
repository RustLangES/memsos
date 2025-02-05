use crate::{asm::port::Port, drivers::driver::Driver};

const KEYBOARD_CTRL: Port = Port(0x64);
const KEYBOARD_PORT: Port = Port(0x60);

pub static KEYBOARD: Keyboard = Keyboard {};

pub struct Keyboard;

impl Driver for Keyboard {
    type ReadOutput = Event;
    fn read(&self) -> Self::ReadOutput {
        while KEYBOARD_CTRL.read() & 0x01 == 0 {}
        let scancode = KEYBOARD_PORT.read();

        Event::from(scancode)
    }
}

impl Keyboard {
    pub fn wait_key(&self, key: &Key) {
        loop {
            let event = self.read();

            if &event.key == key {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub enum ResponseCodes {
    Ack,
    Error,
    Echo,
    InvalidCode,
}

impl From<u8> for ResponseCodes {
    fn from(value: u8) -> Self {
        match value {
            0xFA => ResponseCodes::Ack,
            0xFE => ResponseCodes::Error,
            0xEE => ResponseCodes::Echo,
            _ => ResponseCodes::InvalidCode,
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub key: Key,
    pub state: KeyState,
}

impl From<u8> for Event {
    fn from(value: u8) -> Self {
        match value {
            0x39 => Event {
                key: Key::Space,
                state: KeyState::Press,
            },
            0xB9 => Event {
                key: Key::Space,
                state: KeyState::Release,
            },
            _ => Event {
                key: Key::Unknown(value),
                state: KeyState::None,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyState {
    Press,
    Release,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Space,
    Unknown(u8),
}
