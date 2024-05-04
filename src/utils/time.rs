pub fn now() -> u64 {
    chrono::Utc::now().timestamp() as u64
}
