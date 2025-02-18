use miniserde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Dialogs {
    pub ask: Ask,
    pub info: MemsosInfo
} 

#[derive(Deserialize, Serialize, Debug)]
pub struct Ask {
    pub test_kind: String,
    pub advanced: String,
    pub basic: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemsosInfo {
    pub bootloader_version: String,
    pub love_message: String

}
