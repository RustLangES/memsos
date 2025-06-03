use uefi::{boot::{get_handle_for_protocol, memory_map, open_protocol_exclusive, ScopedProtocol}, mem::memory_map::MemoryMapIter, proto::console::gop::GraphicsOutput};

pub struct UefiProtocols {
    pub gop: ScopedProtocol<GraphicsOutput>
}

impl UefiProtocols {
    pub fn get() -> UefiProtocols {
        let gop_handle = get_handle_for_protocol::<GraphicsOutput>().expect("Cannot get handle for GOP");
        let gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle).expect("Cannot get GOP");
        
        UefiProtocols {
            gop,
        }
    }
}
