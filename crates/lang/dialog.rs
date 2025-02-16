use miniserde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Dialogs {
    test_kind: String,
    advanced: String,
    basic: String
} 
