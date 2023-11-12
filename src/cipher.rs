use crate::base64::decode_b64;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::Rng;

use crate::{calculate_fitting_quotient, normalized_hamming_distance, Result};
use std::str;

#[derive(Debug)]
pub enum CipherMode {
    ECB,
    CBC,
}

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

fn encrypt_aes_128(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let block_size = Cipher::aes_128_ecb().block_size(); // 16 bytes
    assert_eq!(input.len(), block_size);

    let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None)?;
    encrypter.pad(false);
    let mut buf = vec![0; input.len() + block_size];
    let mut count = encrypter.update(input, &mut buf)?;
    count += encrypter.finalize(&mut buf[count..])?;
    buf.truncate(count);

    Ok(buf)
}

fn decrypt_aes_128(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let block_size = Cipher::aes_128_ecb().block_size(); // 16 bytes
    assert_eq!(input.len(), block_size);

    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None)?;
    decrypter.pad(false);
    let mut buf = vec![0; input.len() + block_size];
    let mut count = decrypter.update(input, &mut buf)?;
    count += decrypter.finalize(&mut buf[count..])?;
    buf.truncate(count);

    Ok(buf)
}

fn encrypt_aes_128_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    let mut last = iv.to_vec();

    for chunk in input.chunks(16) {
        let mut start = chunk.to_vec();
        if chunk.len() < 16 {
            start = add_pkcs7_padding(&start, Cipher::aes_128_ecb().block_size())?;
        }

        start = make_repeating_xor(&start, &last);
        last = encrypt_aes_128(&start, key)?;
        buf.extend(&last);
    }
    Ok(buf)
}

fn decrypt_aes_128_cbc(input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    let mut last = iv.to_vec();

    for chunk in input.chunks(16) {
        let mut to_plain = decrypt_aes_128(chunk, key)?;
        to_plain = make_repeating_xor(&to_plain, &last);

        last = chunk.to_vec();
        buf.extend(&to_plain);
    }

    // remove padding
    let pad = *buf.last().ok_or("unavailable")? as usize;
    buf.truncate(buf.len() - pad);
    Ok(buf)
}

fn gen_aes_128_key() -> [u8; 16] {
    let mut v = [0; 16];
    rand::thread_rng().fill(&mut v);
    v
}

pub fn encryption_oracle(input: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let fill_before: u8 = rng.gen_range(5..=10);
    let fill_after: u8 = rng.gen_range(5..=10);
    let mode = if rng.gen::<bool>() {
        CipherMode::ECB
    } else {
        CipherMode::CBC
    };

    let mut start: Vec<u8> = vec![];
    let key = gen_aes_128_key();
    let mut iv = [0; 16];
    rng.fill(&mut iv);

    for _ in 0..fill_before {
        start.push(rng.gen());
    }
    start.extend(input);
    for _ in 0..fill_after {
        start.push(rng.gen());
    }

    encrypt(mode, &start, &key[..], Some(&iv[..]))
}

pub fn oracle_two(input: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let s = decode_b64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
            aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
            dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
            YnkK");
    let fill_before: u8 = rng.gen_range(5..=10);
    let fill_after: u8 = rng.gen_range(5..=10);
    let mode = CipherMode::ECB;

    let mut start: Vec<u8> = vec![];

    for _ in 0..fill_before {
        start.push(rng.gen());
    }
    start.extend(input);
    for _ in 0..fill_after {
        start.push(rng.gen());
    }
    start.extend(s);

    encrypt(mode, &start, key, None)
}

pub fn encrypt(mode: CipherMode, input: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>> {
    match mode {
        CipherMode::ECB => {
            let mut buf: Vec<u8> = vec![];
            for chunk in input.chunks(16) {
                let mut start = chunk.to_vec();
                if chunk.len() < 16 {
                    start = add_pkcs7_padding(&start, Cipher::aes_128_ecb().block_size())?;
                }
                start = encrypt_aes_128(&start, key)?;
                buf.extend(&start);
            }
            Ok(buf)
        }
        CipherMode::CBC => encrypt_aes_128_cbc(input, key, iv.unwrap()),
    }
}

pub fn decrypt(mode: CipherMode, input: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>> {
    match mode {
        CipherMode::ECB => {
            let mut buf: Vec<u8> = vec![];
            for chunk in input.chunks(16) {
                buf.extend(&decrypt_aes_128(chunk, key)?);
            }
            // remove padding
            let pad = *buf.last().ok_or("unavailable")? as usize;
            buf.truncate(buf.len() - pad);
            Ok(buf)
        }
        CipherMode::CBC => decrypt_aes_128_cbc(input, key, iv.unwrap()),
    }
}

// TODO: easier to just return a new vec with padding added
// than to pull old one I guess
pub fn add_pkcs7_padding(src: &[u8], block_size: usize) -> Result<Vec<u8>> {
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

    #[test]
    fn pkcs7_pad() -> Result<()> {
        let to_pad = b"YELLOW SUBMARINE";
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = add_pkcs7_padding(to_pad, 20)?;
        assert_eq!(expected, &result[..]);
        Ok(())
    }

    #[test]
    fn encrypt_decrypt_aes_128() -> Result<()> {
        let key = b"YELLOW SUBMARINE";
        let message = b"The quick brown ";
        let result = decrypt_aes_128(&encrypt_aes_128(message, key)?, key)?;
        assert_eq!(&result, message);
        Ok(())
    }

    #[test]
    fn compare_openssl_aes_128_ecb() -> Result<()> {
        let text = b"The quick brown fox jumps over the lazy dog";
        let key = b"YELLOW SUBMARINE";

        let ciphertext = encrypt(CipherMode::ECB, text, key, None)?;
        let compare =
            openssl::symm::encrypt(openssl::symm::Cipher::aes_128_ecb(), key, None, text)?;
        assert_eq!(ciphertext, compare);
        Ok(())
    }

    #[test]
    fn encrypt_decrypt_aes_128_ecb() -> Result<()> {
        let key = b"YELLOW SUBMARINE";
        let message = b"The quick brown fox jumps over the lazy dog";
        let result = decrypt(
            CipherMode::ECB,
            &encrypt(CipherMode::ECB, message, key, None)?,
            key,
            None,
        )?;
        assert_eq!(&result, message);
        Ok(())
    }

    #[test]
    fn encrypt_decrypt_aes_128_cbc() -> Result<()> {
        let key = b"YELLOW SUBMARINE";
        let iv = b"\x00".repeat(16);
        let message = b"The quick brown fox jumps over the lazy dog";
        let result = decrypt_aes_128_cbc(&encrypt_aes_128_cbc(message, key, &iv)?, key, &iv)?;
        assert_eq!(&result, message);
        Ok(())
    }
}
