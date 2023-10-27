#![allow(dead_code)]
use cryptopals::{
    base64::decode_b64,
    cipher::{add_pkcs7_padding, decrypt_aes_128_cbc},
    Result,
};
use std::str;

fn main() -> Result<()> {
    let z = c2("./inputs/s2c10_input.txt")?;
    println!("{}", z);
    Ok(())
}

fn c1(input: &[u8]) -> Result<Vec<u8>> {
    add_pkcs7_padding(input, 20)
}

fn c2(filename: &str) -> Result<String> {
    let data = decode_b64(&std::fs::read_to_string(filename)?);
    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00".repeat(16);

    let decoded = decrypt_aes_128_cbc(&data, key, &iv)?;
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod set2 {
    use super::*;

    #[test]
    fn challenge_1() -> Result<()> {
        let a = c1(b"YELLOW SUBMARINE")?;
        assert_eq!(
            a,
            b"YELLOW SUBMARINE\x04\x04\x04\x04",
        );
        Ok(())
    }

    #[test]
    fn challenge_2() -> Result<()> {
        let decrypted = c2("./inputs/s2c10_input.txt")?;
        let expected = std::fs::read_to_string("./inputs/s1c6_output.txt")?;
        assert_eq!(decrypted, expected);
        Ok(())
    }
}
