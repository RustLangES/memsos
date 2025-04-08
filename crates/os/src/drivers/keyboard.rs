use crate::drivers::driver::Driver;
use memsos_core::Keyboard as CoreKeyboard;

macro_rules! make_keys {
    ($( $name:ident -> $value:literal ),*) => {
        $(
            #[cfg(target_arch = "x86_64")]
            const $name: $crate::asm::port::Port = $crate::asm::port::Port($value);
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            const $name: $crate::asm::port::Port = $crate::asm::port::Port(0);
        )*
    };
}

make_keys!(
    KEYBOARD_CTRL -> 0x64,
    KEYBOARD_PORT -> 0x60
);

pub static KEYBOARD: Keyboard = Keyboard {};

pub struct Keyboard;

impl Driver for Keyboard {
    type ReadOutput = Event;
    fn read(&self) -> Self::ReadOutput {
        KEYBOARD_CTRL.write(0xFF);
        while KEYBOARD_CTRL.read() & 0x01 == 0 {}
        let scancode = KEYBOARD_PORT.read();

        Event::from(scancode)
    }
}

impl CoreKeyboard for Keyboard {
    fn m_pressed(&self) -> bool {
        KEYBOARD_CTRL.write(0xFF);
        let event = Event::from(KEYBOARD_PORT.read());

        event.state == KeyState::Press && event.key == Key::M
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
    pub fn scan(&self, keys: &[Key]) -> Key {
        let mut event = self.read();
        while !keys.contains(&event.key) && event.state == KeyState::Press
            || event.state == KeyState::None
        {
            event = self.read();
        }

        event.key
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
            39 => Event {
                key: Key::Space,
                state: KeyState::Press,
            },
            0xB9 => Event {
                key: Key::Space,
                state: KeyState::Release,
            },
            72 => Event {
                key: Key::Up,
                state: KeyState::Press,
            },
            200 => Event {
                key: Key::Up,
                state: KeyState::Release,
            },
            80 => Event {
                key: Key::Down,
                state: KeyState::Press,
            },
            208 => Event {
                key: Key::Down,
                state: KeyState::Release,
            },
            0x32 => Event {
                key: Key::M,
                state: KeyState::Press,
            },
            0xB2 => Event {
                key: Key::M,
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
    Up,
    Down,
    M,
    Unknown(u8),
}
