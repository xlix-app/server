#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(into = "u8")]
#[serde(from = "u8")]
pub enum FileState {
    Uploading = 0,
    Finished = 1,
}

impl Into<u8> for FileState {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for FileState {
    fn from(value: u8) -> Self {
        if value > FileState::Finished as u8 {
            Self::Uploading
        } else {
            unsafe { std::mem::transmute(value) }
        }
    }
}
