use miniserde::{json, Deserialize, Serialize};
use std::env;
use std::fs::{read_to_string, File};
use std::path::Path;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
pub struct Dialogs {
    test_kind: String,
    advanced: String,
    basic: String
} 

fn main() {
    let lang = env::var("LANG".to_string()).unwrap();
    
    let j = read_to_string(format!("defs/{lang}.json")).unwrap();
    let dialogs: Dialogs = json::from_str(&j).unwrap();
 
    let dest_path = Path::new("src").join("lang_info.rs");
    let mut file = File::create(dest_path).unwrap();

    file.write_all(format!(r#"
        pub struct Dialogs {{
            pub test_kind: &'static str,
            pub advanced: &'static str,
            pub basic: &'static str
        }}

        pub const DIALOGS: Dialogs = {:?};
    "#, dialogs).as_bytes()).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=TEST_KIND={}", dialogs.test_kind);
}

