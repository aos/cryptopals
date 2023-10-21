use crate::Result;

use bitvec::prelude::*;
use std::str;

pub const B64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_b64(input: &[u8]) -> Result<String> {
    let alphabet = encode_table(B64_ALPHABET);
    let mut encoded: Vec<u8> = Vec::new();

    let bits = input.view_bits::<Msb0>();
    let remainder = align_up(bits.len(), 24) - bits.len();

    for i in bits.chunks(6) {
        if i.len() < 6 {
            let mut bv = bitvec![u8, Msb0;];
            let pad_zero = 6 - i.len();
            let padding = (remainder - pad_zero) / 6;

            bv.extend(i);
            bv.extend_from_bitslice(&bitvec![u8, Msb0; 0; pad_zero]);

            encoded.push(alphabet[bv.load_be::<u8>() as usize]);
            encoded.extend(vec![0x3D; padding]);
        } else {
            encoded.push(alphabet[i.load_be::<u8>() as usize]);
        }
    }

    Ok(str::from_utf8(&encoded)?.to_owned())
}

pub fn decode_b64(input: &str) -> Vec<u8> {
    let mut r = Vec::new();
    let mut bv = bitvec![u8, Msb0;];

    // Remove any wrapping
    for i in input.chars().filter(|c| !c.is_whitespace()) {
        let pos = match i {
            '+' => 62u8,
            '/' => 63u8,
            '=' => 64u8,
            'a'..='z' => i as u8 % 32 + 25,
            '0'..='9' => i as u8 % 32 + 36,
            _ => i as u8 % 32 - 1,
        };
        if pos != 64u8 {
            // these are 8 bits, so retrieve the last 6 bits
            bv.extend_from_bitslice(&pos.view_bits::<Msb0>()[2..]);
        }
    }
    for x in bv.chunks_exact(8) {
        r.push(x.load_be::<u8>());
    }

    r
}

fn encode_table(alphabet: &str) -> [u8; 64] {
    alphabet.as_bytes().try_into().unwrap()
}

fn align_up(num: usize, to: usize) -> usize {
    // By subtracting one, we get "as close as possible"
    // then we do integer division to get the "multiple"
    // and multiply the multiple by the target number
    ((num + to - 1) / to) * to
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn encode_str_b64() -> Result<()> {
        let tests = HashMap::from([
            ("Cat", "Q2F0"),
            ("Ca", "Q2E="),
            ("C", "Qw=="),
            ("light work.", "bGlnaHQgd29yay4="),
            ("light work", "bGlnaHQgd29yaw=="),
            ("light wor", "bGlnaHQgd29y"),
            ("light wo", "bGlnaHQgd28="),
            ("light w", "bGlnaHQgdw=="),
        ]);
        for (test, expected) in &tests {
            let b64 = encode_b64(test.as_bytes())?;
            assert_eq!(b64, expected.to_owned());
        }
        Ok(())
    }

    #[test]
    fn decode_b64_str() -> Result<()> {
        let tests = HashMap::from([
            ("Cat", "Q2F0"),
            ("Ca", "Q2E="),
            ("C", "Qw=="),
            ("light work.", "bGlnaHQgd29yay4="),
            ("light work", "bGlnaHQgd29yaw=="),
            ("light wor", "bGlnaHQgd29y"),
            ("light wo", "bGlnaHQgd28="),
            ("light w", "bGlnaHQgdw=="),
        ]);
        for (expected, test) in &tests {
            let to_u8 = decode_b64(test);
            assert_eq!(str::from_utf8(&to_u8)?, expected.to_owned());
        }
        Ok(())
    }

    #[test]
    fn encode_decode_b64() -> Result<()> {
        let tests = [
            "Cat",
            "Ca",
            "C",
            "light work.",
            "light work",
            "light wor",
            "light wo",
            "light w",
        ];
        for test in &tests {
            let to_u8 = decode_b64(&encode_b64(test.as_bytes())?);
            assert_eq!(str::from_utf8(&to_u8)?, test.to_owned());
        }
        Ok(())
    }
}
