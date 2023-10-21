use crate::{calculate_fitting_quotient, normalized_hamming_distance};
use std::str;

pub fn make_single_byte_xor(input: &[u8], key: u8) -> Vec<u8> {
    // XOR each byte in input with the key
    input.iter().map(|x| x ^ key).collect()
}

pub fn find_single_byte_xor_key(input: &[u8]) -> (u8, String, f64) {
    // XOR each byte in input with all u8 bytes, collect by key
    let z: Vec<(u8, Vec<u8>)> = (0..=255)
        .map(|x| (x, crate::cipher::make_single_byte_xor(input, x)))
        .collect();

    z.iter()
        // Calculate fitting quotient for each single byte XOR
        .map(|(k, chars)| (k, chars, calculate_fitting_quotient(chars)))
        // Make sure it's printable
        .filter(|(_, c, _)| str::from_utf8(c).is_ok())
        .fold(
            (0, String::new(), f64::MAX),
            |acc, (&k, c, v)|
                // get lowest fitting quotient
                if v < acc.2 {
                    // safe because we already filtered to make sure it's okay
                    (k, str::from_utf8(c).unwrap().to_owned(), v)
                } else {
                    acc
                }
        )
}

pub fn make_repeating_xor(key: &[u8], input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|(first, second)| first ^ second)
        .collect::<Vec<u8>>()
}

pub fn find_repeating_xor_size(input: &[u8]) -> usize {
    let max_keysize = usize::min(40, input.len() / 2);
    let mut keysize = 0;
    let mut distance = f64::MAX;
    for k in 2..max_keysize {
        let x = normalized_hamming_distance(input, k);
        if x < distance {
            distance = x;
            keysize = k;
        }
    }
    keysize
}

pub fn find_repeating_xor_key(input: &[u8]) -> Vec<u8> {
    let possible_keysize = find_repeating_xor_size(input);
    let mut key: Vec<u8> = Vec::new();

    let mut chunked: Vec<Vec<u8>> = Vec::new();
    for i in input.chunks(possible_keysize) {
        chunked.push(i.to_vec());
    }

    let mut transposed: Vec<Vec<u8>> = vec![Vec::with_capacity(chunked.len()); chunked[0].len()];
    for r in chunked {
        for c in 0..r.len() {
            transposed[c].push(r[c]);
        }
    }
    for block in transposed {
        let z = find_single_byte_xor_key(&block);
        key.push(z.0);
    }

    key
}
