use miniserde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Dialogs {
    pub test_kind: String,
    pub advanced: String,
    pub basic: String
} 
