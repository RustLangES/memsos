use std::env;
use std::process::Command as Cmd;

#[derive(Debug)]
enum Command {
    Uefi,
    Bios,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = {
        let a = match args.len() {
            n if n < 2 => "uefi",
            n if n > 2 => {
                panic!("Only one argument is expected");
            }
            _ => args[1].as_str(),
        };
        match a {
            "bios" => Command::Bios,
            "uefi" => Command::Uefi,
            _ => panic!("Unknown command"),
        }
    };

    let mut cmd = Cmd::new("just");
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
