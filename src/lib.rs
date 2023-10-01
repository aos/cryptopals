use std::error::Error;
use std::str;
use bitvec::prelude::*;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_table(alphabet: &str) -> [u8; 64] {
    alphabet.as_bytes().try_into().unwrap()
}

pub fn u8_to_b64(inp: &[u8]) -> Result<String, Box<dyn Error>> {
    let alphabet = encode_table(ALPHABET);
    let mut encoded: Vec<u8> = Vec::new();

    let bits = inp.view_bits::<Msb0>();
    let alignment = align_up(bits.len(), 24);
    let remainder = alignment - bits.len();

    for i in bits.chunks(6) {
        if i.len() < 6 {
            let mut bv = bitvec![u8, Msb0;];
            let pad_zero = 6 - i.len();
            let padding = (remainder - pad_zero) / 6;

            bv.extend(i);
            bv.extend_from_bitslice(&bitvec![u8, Msb0; 0; pad_zero]);

            encoded.push(alphabet[bv.load_be::<u8>() as usize]);
            for _ in 0..padding {
                encoded.push(0x3D)
            }
        } else {
            encoded.push(alphabet[i.load_be::<u8>() as usize]);
        }
    }

    Ok(str::from_utf8(&encoded)?.to_owned())
}

pub fn hex_to_u8(hex: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hex: Result<Vec<_>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16))
        .collect();

    Ok(hex?)
}

fn align_up(num: usize, to: usize) -> usize {
    // By subtracting one, we get "as close as possible"
    // then we do integer division to get the "multiple"
    // and multiply the multiple by the target number
    ((num + to - 1) / to) * to
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn encode_str_b64() {
        let tests = HashMap::from([
            ("Man", "TWFu"),
            ("Ma", "TWE="),
            ("M", "TQ=="),
            ("light work.", "bGlnaHQgd29yay4="),
            ("light work", "bGlnaHQgd29yaw=="),
            ("light wor", "bGlnaHQgd29y"),
            ("light wo", "bGlnaHQgd28="),
            ("light w", "bGlnaHQgdw=="),
        ]);
        for (test, expected) in &tests {
            let b64 = u8_to_b64(test.as_bytes()).unwrap_or(String::from(""));
            assert_eq!(b64, expected.to_owned());
        }
    }

    #[test]
    fn encode_hex_b64() {
        let tests = HashMap::from([
            (
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            ),
        ]);
        for (test, expected) in &tests {
            let to_u8 = hex_to_u8(test).unwrap_or(Vec::new());
            let b64 = u8_to_b64(&to_u8).unwrap_or(String::from(""));
            assert_eq!(b64, expected.to_owned());
        }
    }
}
