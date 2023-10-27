#![allow(dead_code)]
use cryptopals::{
    cipher::{find_repeating_xor_key, find_single_byte_xor_key, make_repeating_xor, pkcs7_padding},
    hex_to_u8, u8_to_hex, Result,
};

use std::str;

fn main() -> Result<()> {
    let x = b"YELLOW SUBMARINE";
    let z = pkcs7_padding(x, 20)?;
    println!("{:?}", str::from_utf8(&z)?);
    Ok(())
}

fn c1(input: &[u8]) -> Result<Vec<u8>> {
    Ok(pkcs7_padding(input, 20)?)
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
}
