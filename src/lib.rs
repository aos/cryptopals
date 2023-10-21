pub mod base64;
pub mod cipher;

use bitvec::prelude::*;
use std::error::Error;
use std::str;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// This also encodes the space character (' ') on index 0
// https://web.archive.org/web/20170918020907/http://www.data-compression.com/english.html
const LETTER_FREQUENCY: [f64; 27] = [
    19.18182, 8.2389258, 1.5051398, 2.8065007, 4.2904556, 12.813865, 2.2476217, 2.0327458,
    6.1476691, 6.1476691, 0.1543474, 0.7787989, 4.0604477, 2.4271893, 6.8084376, 7.5731132,
    1.9459884, 0.0958366, 6.0397268, 6.3827211, 9.1357551, 2.7822893, 0.9866131, 2.3807842,
    0.151321, 1.9913847, 0.0746517,
];

pub fn hex_to_u8(hex: &str) -> Result<Vec<u8>> {
    let hex: std::result::Result<Vec<_>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();

    Ok(hex?)
}

// Given a stream of text bytes, calculates the letter frequency distribution and then gets the
// absolute diff of the frequency compared to the letters in the english language, normalized by
// the length of text
pub fn calculate_fitting_quotient(input: &[u8]) -> f64 {
    let mut dist_text: Vec<f64> = vec![0.0; LETTER_FREQUENCY.len()];
    for &c in input
        .iter()
        .filter(|&c| c.is_ascii_alphabetic() || (*c as char) == ' ')
    {
        dist_text[c as usize % 32] += 1.0;
    }
    dist_text
        .iter()
        .zip(LETTER_FREQUENCY.iter())
        .map(|(ch, freq)| ((ch * 100.0 / (input.len() as f64)) - freq).abs())
        .sum::<f64>()
        / (dist_text.len() as f64)
}

pub fn decrypt_single_byte_xor(input: &[u8]) -> (char, String, f64) {
    // XOR each byte in input with all u8 bytes
    // collect by alphabet, XOR result
    let z: Vec<(u8, Vec<u8>)> = (0..=255)
        .map(|x| (x, crate::cipher::single_byte_xor(input, x)))
        .collect();

    z.iter()
        // Calculate fitting quotient for each single byte XOR
        .map(|(k, chars)| (k, chars, calculate_fitting_quotient(chars)))
        // Make sure it's printable
        .filter(|(_, c, _)| str::from_utf8(c).is_ok())
        .fold(
            (' ', String::new(), f64::MAX),
            |acc, (&k, c, v)|
                // get lowest fitting quotient
                if f64::min(acc.2, v) == v {
                    (k as char, str::from_utf8(c).unwrap().to_owned(), v)
                } else {
                    acc
                }
        )
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
