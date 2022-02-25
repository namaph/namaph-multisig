pub fn fit_string(input: &str) -> &[u8] {
    let bytes = input.as_bytes();
    if bytes.len() > 32 {
        &bytes[0..32]
    } else {
        bytes
    }
}


