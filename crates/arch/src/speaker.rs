use core::time::Duration;

use x86_64::instructions::port::Port;

use crate::tsc::sleep;

/// # Panics
/// This function may panic if `div` cannot be converted into a `u8`.
pub fn play_sound(frequence: u32) {
    let div: u32 = 1_193_180 / frequence;

    unsafe {
        let mut p1 = Port::new(0x43);
        p1.write(0xb6_u8);

        let mut p2 = Port::new(0x42);
        p2.write(u8::try_from(div).unwrap());
        p2.write(u8::try_from(div >> 8).unwrap());
    }

    let mut tmp_port = Port::new(0x61);
    let tmp: u8 = unsafe { tmp_port.read() };
    if tmp != (tmp | 3) {
        let mut out = Port::new(0x61);
        unsafe {
            out.write(tmp | 3);
        }
    }
}

pub fn stop_sound() {
    let mut tmp_port = Port::new(0x61);
    unsafe {
        let tmp: u8 = tmp_port.read() & 0xFC;

        tmp_port.write(tmp);
    }
}

pub fn beep() {
    play_sound(1000);
    sleep(Duration::from_secs_f32(0.2));
    stop_sound();
}
