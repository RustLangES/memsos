use uefi::{
    boot::{get_handle_for_protocol, open_protocol_exclusive, ScopedProtocol},
    proto::console::{gop::GraphicsOutput, pointer::Pointer},
};

pub struct UefiProtocols {
    pub gop: ScopedProtocol<GraphicsOutput>,
    pub pointer: ScopedProtocol<Pointer>,
}

impl UefiProtocols {
    pub fn get() -> UefiProtocols {
        let gop_handle =
            get_handle_for_protocol::<GraphicsOutput>().expect("Cannot get handle for GOP");
        let gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle).expect("Cannot get GOP");

        let pointer_handle =
            get_handle_for_protocol::<Pointer>().expect("Cannot get handler for Pointer");
        let pointer =
            open_protocol_exclusive::<Pointer>(pointer_handle).expect("Cannot get pointer");

        UefiProtocols { gop, pointer }
    }
}
