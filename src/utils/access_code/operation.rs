#[derive(Serialize, Deserialize)]
pub enum Operation {
    #[serde(rename="UL")]
    Upload,
    #[serde(rename="DL")]
    Download,
}
