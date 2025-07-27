use x86_64::instructions::port::Port;

const CMOS_DISABLE_NMI: u8 = 1 << 7;

pub fn enable_nmi() {
    let mut cmos_port: Port<u8> = Port::new(0x70);

    unsafe {
        let tmp = cmos_port.read();
        cmos_port.write(tmp & 0x7F);

        let mut tmp_port: Port<u8> = Port::new(0x71);
        tmp_port.read();
    }
}

pub fn disable_nmi(register: u8) {
    let mut cmos_port: Port<u8> = Port::new(0x70);

    unsafe {
        cmos_port.write(CMOS_DISABLE_NMI | register);

        let mut tmp_port: Port<u8> = Port::new(0x71);
        tmp_port.read();
    }
}
