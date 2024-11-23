use crate::{asm::inb::inb, println};
use core::fmt::Write;
use heapless::String;

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Space,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct Event {
    pub key: Key,
    pub state: State,
}

pub struct ScanCode {
    code: u8,
}

impl ScanCode {
    pub fn new(code: u8) -> Self {
        ScanCode { code }
    }
    pub fn to_event(&self) -> Event {
        match self.code {
            0x39 => Event {
                key: Key::Space,
                state: State::Pressed,
            },
            0xF0 => Event {
                key: Key::Space,
                state: State::Released,
            },

            scan_code => panic!("Scan Code {} not recognized", scan_code),
        }
    }
}

pub struct Scanner;

impl Scanner {
    // Warning! potentially unsafe function, be careful adventurer
    pub fn read(&self) -> ScanCode {
        while (inb(0x64) & 0x01) == 0 {}

        let code = inb(0x60);

        ScanCode::new(code)
    }
    pub fn wait_for_key(&self, target_key: Key) {
        let event = self.read().to_event();

        if event.key != target_key {
            self.wait_for_key(target_key);
        }
    }
}
