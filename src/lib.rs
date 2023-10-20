pub mod base64;

use bitvec::prelude::*;
use std::collections::HashMap;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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

pub fn hex_to_u8(hex: &str) -> Result<Vec<u8>> {
    let hex: std::result::Result<Vec<_>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();

    Ok(hex?)
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

pub fn repeating_key_xor(key: &str, input: &str) -> String {
    input
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(first, second)| format!("{:02x}", first ^ second))
        .collect::<String>()
}

pub fn hamming_distance(first: &[u8], second: &[u8]) -> usize {
    let f_bits = first.view_bits::<Lsb0>();
    let s_bits = second.view_bits::<Lsb0>();

    f_bits
        .iter()
        .zip(s_bits.iter())
        .map(|(f, s)| (*f as usize) ^ (*s as usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base64::encode_b64;
    use std::collections::HashMap;

    #[test]
    fn encode_hex_b64() -> Result<()> {
        let tests = HashMap::from([
            (
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            ),
        ]);
        for (test, expected) in &tests {
            let to_u8 = hex_to_u8(test)?;
            let b64 = encode_b64(&to_u8)?;
            assert_eq!(b64, expected.to_owned());
        }
        Ok(())
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
