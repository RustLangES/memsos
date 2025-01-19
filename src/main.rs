mod qemu;
use qemu::QemuBuilder;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    Uefi,
    Bios,
    Dist,
}

fn main() {
    let cli = Cli::from_args();

    match cli.command.unwrap_or(Command::Uefi) {
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
