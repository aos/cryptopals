pub fn single_byte_xor(input: &[u8], key: u8) -> Vec<u8> {
    // XOR each byte in input with the key
    input.iter().map(|x| x ^ key).collect()
}

pub fn repeating_key_xor(key: &str, input: &str) -> String {
    input
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(first, second)| format!("{:02x}", first ^ second))
        .collect::<String>()
}
