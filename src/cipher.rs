use crate::{calculate_fitting_quotient, hamming_distance};
use std::str;

pub fn single_byte_xor(input: &[u8], key: u8) -> Vec<u8> {
    // XOR each byte in input with the key
    input.iter().map(|x| x ^ key).collect()
}

pub fn find_single_byte_xor(input: &[u8]) -> (u8, String, f64) {
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
            (0, String::new(), f64::MAX),
            |acc, (&k, c, v)|
                // get lowest fitting quotient
                if f64::min(acc.2, v) == v {
                    (k, str::from_utf8(c).unwrap().to_owned(), v)
                } else {
                    acc
                }
        )
}

pub fn repeating_xor(key: &[u8], input: &[u8]) -> String {
    input
        .iter()
        .zip(key.iter().cycle())
        .map(|(first, second)| format!("{:02x}", first ^ second))
        .collect::<String>()
}

pub fn find_repeating_xor_size(input: &[u8]) -> usize {
    let max_keysize = usize::min(40, input.len() / 2);
    let mut possible_key = (0, usize::MAX); // (keysize, distance)

    for keysize in 2..max_keysize {
        // TODO(als): this needs to be fixed, we can't hardcode the keysize
        let z =
            hamming_distance(&input[..keysize * 4], &input[keysize * 4..keysize * 4 * 2]) / keysize;
        if usize::min(possible_key.1, z) == z {
            possible_key = (keysize, z);
        }
    }

    possible_key.0
}

pub fn find_repeating_xor_key(input: &[u8]) -> Vec<u8> {
    let possible_keysize = find_repeating_xor_size(input);
    let mut key: Vec<u8> = Vec::with_capacity(possible_keysize);

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
        let z = find_single_byte_xor(&block);
        key.push(z.0);
    }

    key
}
