mod qemu;

use qemu::QemuBuilder;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Memsos basic cli")]
struct Cli {
    #[structopt(short, long)]
    bios: bool,

    #[structopt(short, long)]
    uefi: bool,

    #[structopt(short, long)]
    export: bool,
}

fn main() {
    let cli = Cli::from_args();

    if cli.uefi {
        let qemu = QemuBuilder::new()
            .img(env!("UEFI_PATH").to_string())
            .uefi(true)
            .build();

        qemu.run();
    } else if cli.bios {
        let qemu = QemuBuilder::new()
            .img(env!("BIOS_PATH").to_string())
            .uefi(false)
            .build();
        qemu.run();
    } else if cli.export {
        println!(
            "UEFI PATH: {}, BIOS PATH: {}",
            env!("UEFI_PATH"),
            env!("BIOS_PATH")
        );
    }
}
