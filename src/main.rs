use std::env;
use std::process::Command as Cmd;

#[derive(Debug)]
enum Command {
    Uefi,
    Bios,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd = Cmd::new("just");

    let command = {
        if args.len() < 2 {
            Command::Uefi
        } else {
            let a = args[1].as_str();

            if args.len() > 2 {
                cmd.env("LANG", args[2].as_str());
            } else {
                cmd.env("LANG", "en_US");
            }
            match a {
                "bios" => Command::Bios,
                "uefi" => Command::Uefi,
                _ => panic!("Unknown command"),
            }
        }
    };
    match command {
        Command::Uefi => {
            println!("Running in uefi mode");
            let mut child = cmd.spawn().unwrap();
            child.wait().unwrap();
        }
        Command::Bios => {
            println!("Running in bios mode");
            cmd.arg("run-bios");
            let mut child = cmd.spawn().unwrap();
            child.wait().unwrap();
        }
    }
}
