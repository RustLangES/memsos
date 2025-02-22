#[path = "dialog.rs"]
mod dialog;

use dialog::Dialogs;
use miniserde::json;
use std::env;
use std::fs::{read_dir, read_to_string, File};
use std::io::Write;
use std::path::Path;

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
    let lang = env::var("LANG").unwrap();

    let dest_path = Path::new("src").join("lang_info.rs");
    let paths: Vec<_> = read_dir("defs")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let lang_path = Path::new("defs").join(format!("{}.json", &lang));

    if !lang_path.exists() {
        eprintln!("A translation called {} was not found", lang);
        eprintln!("consider using some of these translations");
        for path in &paths {
            if path.is_file() {
                eprintln!("{}", path.display());
            }
        }
        panic!();
    }

    let j = read_to_string(lang_path).unwrap();
    let dialogs: Dialogs = json::from_str(&j).unwrap();

    let mut file = File::create(dest_path).unwrap();
    let def = read_struct();

    file.write_all(
        format!(
            r#"{def}
pub const DIALOGS: Dialogs = {:?};"#,
            dialogs
        )
        .as_bytes(),
    )
    .unwrap();

    for path in &paths {
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LANG");
    println!("cargo:rerun-if-changed=dialog.rs");
}
