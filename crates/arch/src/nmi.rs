use x86_64::instructions::port::Port;

pub fn enable_nmi() {
    let mut cmos_port: Port<u8> = Port::new(0x70);

    unsafe {
        let tmp = cmos_port.read();
        cmos_port.write(tmp & 0x7F);
        
        let mut tmp_port: Port<u8> = Port::new(0x71);
        tmp_port.read();
    }
}

pub fn disable_nmi() {
    let mut cmos_port: Port<u8> = Port::new(0x70);

    unsafe {
        let tmp = cmos_port.read();
        cmos_port.write(tmp | 0x80);

        let mut tmp_port: Port<u8> = Port::new(0x71);
        tmp_port.read();
    }
}
