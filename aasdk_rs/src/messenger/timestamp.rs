pub fn get_timestamp_from_bytes(bytes: &[u8]) {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[0..8]);
    let bla = u64::from_be_bytes(arr);
    log::info!("Timestamp: {}", bla);
}