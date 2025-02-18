#[path = "dialog.rs"]
mod dialog;

use std::env;
use std::fs::{read_to_string, File};
use std::path::Path;
use std::io::Write;
use miniserde::json;
use dialog::Dialogs;

use std::io::{BufRead, BufReader};

fn read_struct() -> String {
    let file = File::open("dialog.rs").unwrap();
    let reader = BufReader::new(file);
    let mut output = String::new();
    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();
        if index >= 2 {
            if line.starts_with("#") {
                continue;
            }
            line = line.replace("String", "&'static str");
            output.push_str(&line);
        }
    }
    
    output
}



fn main() {
    let lang = env::var("LANG".to_string()).unwrap();
    
    let j = read_to_string(format!("defs/{lang}.json")).unwrap();
    let dialogs: Dialogs = json::from_str(&j).unwrap();
 
    let dest_path = Path::new("src").join("lang_info.rs");
    let mut file = File::create(dest_path).unwrap();
    let def = read_struct();

    file.write_all(format!(r#"
        {def}
        pub const DIALOGS: Dialogs = {:?};
    "#, dialogs).as_bytes()).unwrap();

    let paths = std::fs::read_dir("defs").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LANG");
    println!("cargo:rerun-if-changed=dialog.rs");
}

