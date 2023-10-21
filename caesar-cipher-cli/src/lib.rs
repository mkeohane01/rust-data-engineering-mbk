/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value, and returns the ciphertext string. The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}

/*
This code defines two more functions: piglatin_encrypt and piglatin_decrypt.
The encrypt function takes a plaintext string and returns the string translated into piglatin. The decrypt function takes a string and decrypts it from piglatin,
and returns the plaintext string.

*/

pub fn piglatin(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if "aeiouAEIOU".contains(first_char) {
            result.push_str(&format!("{}hay ", word));
        } else {
            result.push_str(&format!("{}{}ay ", chars.as_str(), first_char));
        }
    }
    result.trim_end().to_string()
}

