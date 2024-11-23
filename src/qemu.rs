use std::process::Command;

#[derive(Debug)]
pub struct Qemu {
    img: String,
    use_uefi: bool,
}

#[derive(Debug)]
pub struct QemuBuilder {
    qemu: Qemu,
}

impl QemuBuilder {
    pub fn new() -> Self {
        QemuBuilder { qemu: Qemu {
            img: String::from(""),
            use_uefi: false,
   
        } }
    }
    pub fn img(mut self, img: String) -> Self {
        self.qemu.img = img;
        self
    }
    pub fn uefi(mut self, use_uefi: bool) -> Self {
        self.qemu.use_uefi = use_uefi;
        self
    }
    pub fn build(self) -> Qemu {
        self.qemu
    }
}

impl Qemu {
   pub fn run(&self) {
        let mut cmd = Command::new("qemu-system-x86_64");  
        if self.use_uefi {
            cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
            cmd.arg("-drive")
            .arg(format!("format=raw,file={}", self.img));

        } else {
            cmd.arg("-drive")
                .arg(format!("format=raw,file={}", self.img));
        }

        let mut child = cmd.spawn().unwrap();            
        child.wait().unwrap();

   }  
}
