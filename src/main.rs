mod qemu;
use qemu::QemuBuilder;
use std::env;

#[derive(Debug)]
enum Command {
    Uefi,
    Bios,
    Dist,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = {
        if args.len() < 2 {
            Command::Uefi
        } else if args.len() > 2 {
            panic!("Only one argument is expected")
        } 
        else {
            match args[1].as_str() {
                "bios" => Command::Bios,
                "dist" => Command::Dist,
                "uefi" => Command::Uefi,
                _ => panic!("Unknown command"),
            }
        }
    };

    match command {
        Command::Uefi => {
            let qemu = QemuBuilder::new()
                .img(env!("UEFI_PATH").to_string())
                .uefi(true)
                .build();
            qemu.run();
        }
        Command::Bios => {
            let qemu = QemuBuilder::new()
                .img(env!("BIOS_PATH").to_string())
                .uefi(false)
                .build();
            qemu.run();
        }
        Command::Dist => {
            println!(
                "UEFI PATH: {}, BIOS PATH: {}",
                env!("UEFI_PATH"),
                env!("BIOS_PATH")
            );
        }
    }
}
