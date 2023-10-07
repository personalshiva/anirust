fn is_hex_string(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_decrypted(s: &str) -> bool {
    s.starts_with("http") | s.starts_with('/')
}

fn remove_leading_dashes(s: String) -> String {
    s.trim_start_matches('-').to_string()
}

fn decrypt_hex(hex_str: &str, xor_key: u8) -> Result<String, &'static str> {
    // Convert the hex string to bytes
    let byte_data = hex::decode(hex_str).map_err(|_| "Invalid hex string")?;

    // XOR each byte with the key and collect the results
    let decrypted_bytes: Vec<u8> = byte_data.iter().map(|&b| b ^ xor_key).collect();

    // Convert bytes to string
    // 'replace' will replace any invalid UTF-8 characters with the Unicode replacement character (U+FFFD)
    let mut decrypted_str = String::from_utf8_lossy(&decrypted_bytes).into_owned();
    if !decrypted_str.contains("clock.json") {
        decrypted_str = decrypted_str.replace("clock", "clock.json");
    }

    Ok(decrypted_str)
}

pub fn decrypt_url(url_string: String) -> Result<String, &'static str> {
    let xor_key = 56;

    let cleaned_url = remove_leading_dashes(url_string);

    if is_hex_string(&cleaned_url) {
        decrypt_hex(&cleaned_url, xor_key)
    } else if is_decrypted(&cleaned_url) {
        Ok(cleaned_url)
    } else {
        Err("Url string could not be processed")
    }
}
