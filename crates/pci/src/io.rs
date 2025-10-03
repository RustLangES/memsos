use x86_64::instructions::port::Port;

use crate::PciDevice;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

#[derive(Clone, Copy)]
pub struct PciInfo {
    pub bus: u8,
    pub device: u8,
}

// I'll leave this as a preliminary design, but if there is no XHCI controller, the best thing to do would be to try different controllers. If they are not available, then don't use USB.
pub fn xhci_scan() -> PciDevice {
    for bus in 0u8..=255 {
        for device in 0u8..32 {
            if let Some(info) = check_device(bus, device)
                && info.class == 3075
            {
                return info;
            }
        }
    }

    panic!("No xhci found!");
}

pub fn check_device(bus: u8, device: u8) -> Option<PciDevice> {
    assert!(device < 32);

    let info = PciInfo { bus, device };

    let (device_id, vendor_id) = get_ids(info, 0);

    if vendor_id == 0xFFFF {
        return None;
    }

    let class = pci_read(info, 0, 0x8);
    let class = (class >> 16) & 0x0000FFFF;
    let header_type = get_header_type(info, 0);

    let mut supported_fns = [true, false, false, false, false, false, false, false];
    if (header_type & 0x80) != 0 {
        for function in 0u8..8 {
            if get_ids(info, 0).1 != 0xFFFF && check_func(info, function) {
                supported_fns[function as usize] = true;
            }
        }
    }

    let mut bars = [0, 0, 0, 0, 0, 0];

    bars[0] = pci_read(info, 0, 0x10);
    bars[1] = pci_read(info, 0, 0x14);
    bars[2] = pci_read(info, 0, 0x18);
    bars[3] = pci_read(info, 0, 0x1C);
    bars[4] = pci_read(info, 0, 0x20);
    bars[5] = pci_read(info, 0, 0x24);

    let last_row = pci_read(info, 0, 0x3C);

    Some(PciDevice {
        device,
        bus,
        device_id,
        vendor_id,
        class: class as u16,
        header_type,
        bars,
        supported_fns,
        interrupt_line: (last_row & 0xFF) as u8,
        interrupt_pin: ((last_row >> 8) & 0xFF) as u8,
    })
}

pub fn pci_read(info: PciInfo, func: u8, offset: u8) -> u32 {
    let bus = info.bus as u32;
    let device = info.device as u32;
    let func = func as u32;
    let offset = offset as u32;

    let address = (bus << 16) | (device << 11) | (func << 8) | (offset & 0xfc) | 0x80000000;

    unsafe {
        Port::<u32>::new(CONFIG_ADDRESS).write(address);

        Port::<u32>::new(CONFIG_DATA).read()
    }
}

pub fn check_func(info: PciInfo, func: u8) -> bool {
    get_ids(info, func).1 != 0xFFFF
}

pub fn get_ids(info: PciInfo, func: u8) -> (u16, u16) {
    assert!(info.device < 32);
    assert!(func < 8);
    let result = pci_read(info, func, 0);
    let dev_id = ((result >> 16) & 0xFFFF) as u16;
    let vnd_id = (result & 0xFFFF) as u16;

    (dev_id, vnd_id)
}

fn get_header_type(info: PciInfo, function: u8) -> u8 {
    assert!(info.device < 32);
    assert!(function < 8);
    let res = pci_read(info, function, 0x0C);
    ((res >> 16) & 0xFF) as u8
}
