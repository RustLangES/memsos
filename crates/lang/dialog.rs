use miniserde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Dialogs {
    pub ask: Ask,
    pub info: MemsosInfo,
    pub memtest_info: MemtestInfo,
    pub mem_info: MemInfo,
    pub cpu_info: CpuInfo,
    pub debug_info: MemTestDebug
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ask {
    pub test_kind: String,
    pub advanced: String,
    pub basic: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemsosInfo {
    pub bootloader_version: String,
    pub love_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemtestInfo {
    pub info: String,
    pub actual_tests: String,
    pub kind_test: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemInfo {
    pub info: String,
    pub size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuInfo {
    pub info: String,
    pub model: String,
    pub vendor: String,
    pub family: String,
    pub stepping: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemTestDebug {
   pub checking: String,
   pub omitting: String
}
