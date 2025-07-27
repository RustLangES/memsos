use x86_64::instructions::port::Port;

pub fn play_sound(frequence: u32) {
    let div: u32 = 1193180 / frequence;

    unsafe {
        let mut p1 = Port::new(0x43);
        p1.write(0xb6 as u8);

        let mut p2 = Port::new(0x42);
        p2.write(div as u8);
        p2.write((div >> 8) as u8);
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
    // TODO: implement a sleep fn to make beep fn
}
