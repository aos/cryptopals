use openssl::symm::{Cipher, Mode, Crypter};

use crate::{calculate_fitting_quotient, normalized_hamming_distance, Result};
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

pub fn make_repeating_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
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

pub fn encrypt_aes_128_ecb(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let block_size = Cipher::aes_128_ecb().block_size(); // 16 bytes
    assert_eq!(input.len(), block_size);

    let mut encrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Encrypt,
        key,
        None,
    )?;
    encrypter.pad(false);
    let mut buf = vec![0; input.len() + block_size];
    let mut count = encrypter.update(input, &mut buf)?;
    count += encrypter.finalize(&mut buf[count..])?;
    buf.truncate(count);

    Ok(buf)
}

pub fn decrypt_aes_128_ecb(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let block_size = Cipher::aes_128_ecb().block_size(); // 16 bytes
    assert_eq!(input.len(), block_size);

    let mut decrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    )?;
    decrypter.pad(false);
    let mut buf = vec![0; input.len() + block_size];
    let mut count = decrypter.update(input, &mut buf)?;
    count += decrypter.finalize(&mut buf[count..])?;
    buf.truncate(count);

    Ok(buf)
}

pub fn encrypt_aes_128_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    let mut last: Vec<u8> = vec![];

    for (i, chunk) in input.chunks(16).enumerate() {
        let mut start = chunk.to_vec();
        if chunk.len() < 16 {
            start = pkcs7_padding(&start, Cipher::aes_128_ecb().block_size())?;
        }

        if i == 0 {
            start = make_repeating_xor(iv, &start);
        } else {
            start = make_repeating_xor(&last, &start);
        }

        last = encrypt_aes_128_ecb(&start, key)?;
        buf.extend(&last);
    }
    Ok(buf)
}

pub fn decrypt_aes_128_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    let mut ciphertext: Vec<u8> = vec![];

    for (i, chunk) in input.chunks(16).enumerate() {
        let mut to_plain = decrypt_aes_128_ecb(&chunk, key)?;
        ciphertext = chunk.to_vec();

        if i == 0 {
            to_plain = make_repeating_xor(&to_plain, iv);
        } else {
            to_plain = make_repeating_xor(&to_plain, &ciphertext);
        }

        buf.extend(&to_plain);
    }
    Ok(buf)
}

// TODO: easier to just return a new vec with padding added
// than to pull old one I guess
pub fn pkcs7_padding(src: &[u8], block_size: usize) -> Result<Vec<u8>> {
    let padding = block_size - (src.len() % block_size);
    if src.len() > block_size {
        return Err("error: padding length smaller than src length".into());
    }
    let mut new = Vec::new();
    new.extend_from_slice(src);
    new.resize(block_size, padding as u8);

    Ok(new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::symm::{Cipher, encrypt};

    #[test]
    fn pkcs7_pad() -> Result<()> {
        let to_pad = b"YELLOW SUBMARINE";
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = pkcs7_padding(to_pad, 20)?;
        assert_eq!(expected, &result[..]);
        Ok(())
    }

    #[test]
    fn encrypt_decrypt_aes_128_ecb() -> Result<()> {
        let key = b"YELLOW SUBMARINE";
        let message = b"The quick brown ";
        let result = decrypt_aes_128_ecb(&encrypt_aes_128_ecb(message, key)?, key)?;
        assert_eq!(&result, message);
        Ok(())
    }

    #[test]
    fn compare_openssl_aes_128_ecb() -> Result<()> {
        let text = b"The quick brown fox jumps over the lazy dog";
        let key = b"YELLOW SUBMARINE";
        let mut ciphertext = vec![];

        for i in text.chunks(16) {
            if i.len() < 16 {
                let new = pkcs7_padding(i, Cipher::aes_128_ecb().block_size())?;
                let c = encrypt_aes_128_ecb(&new, key)?;
                ciphertext.extend(c);
            } else {
                let c = encrypt_aes_128_ecb(i, key)?;
                ciphertext.extend(c);
            }
        }

        let compare = encrypt(Cipher::aes_128_ecb(), key, None, text)?;
        assert_eq!(ciphertext, compare);
        Ok(())
    }

    #[test]
    fn encrypt_decrypt_aes_128_cbc() -> Result<()> {
        let key = b"YELLOW SUBMARINE";
        let iv = b"\x00".repeat(16);
        let message = b"The quick brown fox jumps over the lazy dog";
        let result = decrypt_aes_128_cbc(
            &encrypt_aes_128_cbc(message, key, &iv)?,
            &iv, key
        )?;
        assert_eq!(&result, message);
        Ok(())
    }
}
