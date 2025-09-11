use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u32 = 0xCF8;
pub const CONFIG_DATA: u32 = 0xCFC;

pub fn pci_read(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let device = device as u32;
    let func = func as u32;
    let offset = offset as u32;

    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xfc) | 0x80000000) as u32;

     unsafe {
        Port::<u32>::new(CONFIG_ADDRESS).write(address);

        Port::<u32>::new(CONFIG_DATA).read()
    }

}

