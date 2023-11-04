#![allow(dead_code)]
use cryptopals::{
    base64::decode_b64,
    cipher::{add_pkcs7_padding, decrypt, encryption_oracle, CipherMode},
    Result,
};
use std::collections::HashSet;
use std::str;

fn main() -> Result<()> {
    let z = c11(encryption_oracle)?;
    println!("{:?}", z);
    Ok(())
}

fn c9(input: &[u8]) -> Result<Vec<u8>> {
    add_pkcs7_padding(input, 20)
}

fn c10(filename: &str) -> Result<String> {
    let data = decode_b64(&std::fs::read_to_string(filename)?);
    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00".repeat(16);

    let decoded = decrypt(CipherMode::CBC, &data, key, Some(&iv))?;
    Ok(String::from_utf8(decoded)?)
}

fn c11(f: impl Fn(&[u8]) -> Result<Vec<u8>>) -> Result<CipherMode> {
    let mut set = HashSet::new();
    let input = "A".repeat(160);
    let oracled = f(input.as_bytes())?;

    for l in oracled.chunks_exact(16) {
        if set.contains(l) {
            return Ok(CipherMode::ECB);
        }
        set.insert(l);
    }

    Ok(CipherMode::CBC)
}

#[cfg(test)]
mod set2 {
    use super::*;

    #[test]
    fn challenge_9() -> Result<()> {
        let a = c9(b"YELLOW SUBMARINE")?;
        assert_eq!(a, b"YELLOW SUBMARINE\x04\x04\x04\x04",);
        Ok(())
    }

    #[test]
    fn challenge_10() -> Result<()> {
        let decrypted = c10("./inputs/s2c10_input.txt")?;
        let expected = std::fs::read_to_string("./inputs/s1c6_output.txt")?;
        assert_eq!(decrypted, expected);
        Ok(())
    }
}
