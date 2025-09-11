use x86_64::instructions::port::Port;

pub const CONFIG_ADDRESS: u32 = 0xCF8;
pub const CONFIG_DATA: u32 = 0xCFC;

pub struct PciInfo {
    pub bus: u8,
    pub device: u8,
    pub func: u8,
}


pub fn pci_read(info: PciInfo, offset: u8) -> u32 {
    let bus = info.bus as u32;
    let device = info.device as u32;
    let func = info.func as u32;
    let offset = offset as u32;

    let address = ((bus << 16) | (device << 11) | (func << 8) | (offset & 0xfc) | 0x80000000) as u32;

     unsafe {
        Port::<u32>::new(CONFIG_ADDRESS).write(address);

        Port::<u32>::new(CONFIG_DATA).read()
    }

}

pub fn check_func(info: PciInfo) -> bool {
    assert!(device < 32);
    assert!(function < 8);

    get_ids(info).1 != 0xFFFF
}

pub fn get_ids(info: PciInfo) -> (u16, u16) {
    assert!(device < 32);
    assert!(function < 8);
    let result = pci_read(info, 0);
    let dev_id = ((res >> 16) & 0xFFFF) as u16;
    let vnd_id = (res & 0xFFFF) as u16;
    
    (dev_id, vnd_id)
}
