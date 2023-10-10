use bitvec::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::str;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
// This also encodes the space character (' ') on index 0
// https://web.archive.org/web/20170918020907/http://www.data-compression.com/english.html
const LETTER_FREQUENCY: [f64; 27] = [
    0.1918182, 0.0651738, 0.0124248, 0.0217339, 0.0349835, 0.1041442, 0.0197881, 0.0158610,
    0.0492888, 0.0558094, 0.0009033, 0.0050529, 0.0331490, 0.0202124, 0.0564513, 0.0596302,
    0.0137645, 0.0008606, 0.0497563, 0.0515760, 0.0729357, 0.0225134, 0.0082903, 0.0171272,
    0.0013692, 0.0145984, 0.0007836,
];

pub fn encode_table(alphabet: &str) -> [u8; 64] {
    alphabet.as_bytes().try_into().unwrap()
}

pub trait Base64 {
    fn to_base64(&self) -> Result<String>;
}

impl Base64 for [u8] {
    fn to_base64(&self) -> Result<String> {
        let alphabet = encode_table(ALPHABET);
        let mut encoded: Vec<u8> = Vec::new();

        let bits = self.view_bits::<Msb0>();
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
}

pub fn hex_to_u8(hex: &str) -> Result<Vec<u8>> {
    let hex: std::result::Result<Vec<_>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();

    Ok(hex?)
}

fn align_up(num: usize, to: usize) -> usize {
    // By subtracting one, we get "as close as possible"
    // then we do integer division to get the "multiple"
    // and multiply the multiple by the target number
    ((num + to - 1) / to) * to
}

pub fn calculate_frequency_score(input: &[u8]) -> f64 {
    input
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, &c| {
            if (c as char).is_ascii_alphabetic() || (c as char) == ' ' {
                let count = acc.entry(c as char).or_insert(0);
                *count += 1;
            }
            acc
        })
        .iter()
        .fold(0.0, |total, (&ch, &num)| {
            total + (LETTER_FREQUENCY[ch as usize % 32] * num as f64)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn encode_str_b64() {
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
            let b64 = test.as_bytes().to_base64().unwrap_or(String::from(""));
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
            let b64 = &to_u8.to_base64().unwrap_or(String::from(""));
            assert_eq!(b64, expected.to_owned());
        }
    }
}
