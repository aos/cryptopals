#![allow(dead_code)]
use cryptopals::{
    base64::{decode_b64, encode_b64},
    cipher::{find_repeating_xor_key, find_single_byte_xor_key, make_repeating_xor},
    hex_to_u8, u8_to_hex, Result,
};
use openssl::symm::{decrypt, Cipher};
use std::collections::HashSet;
use std::str;

fn main() -> Result<()> {
    let (num, line) = c8("./inputs/s1c8_input.txt")?;
    println!("{}: {}", num, line);
    Ok(())
}

fn c1(inp: &str) -> Result<String> {
    encode_b64(&hex_to_u8(inp)?)
}

fn c2(hex1: &str, hex2: &str) -> Result<String> {
    let mut hex1_u8 = hex_to_u8(hex1)?;
    let hex2_u8 = hex_to_u8(hex2)?;

    hex1_u8
        .iter_mut()
        .zip(hex2_u8.iter())
        .for_each(|(x1, x2)| *x1 ^= *x2);

    Ok(u8_to_hex(&hex1_u8))
}

fn c3(input: &str) -> Result<(u8, String, f64)> {
    let s = hex_to_u8(input)?;
    Ok(find_single_byte_xor_key(&s))
}

fn c4(filename: &str) -> Result<(u8, String, String, f64)> {
    let input = std::fs::read_to_string(filename)?;
    let mut min_line: (u8, String, String, f64) = (0, "".to_owned(), "".to_owned(), f64::MAX);
    for l in input.lines() {
        let s = hex_to_u8(l)?;
        let y = find_single_byte_xor_key(&s);

        if y.2 < min_line.3 {
            min_line = (y.0, l.to_owned(), y.1, y.2);
        }
    }
    Ok(min_line)
}

fn c5(input: &str) -> Result<String> {
    Ok(u8_to_hex(&make_repeating_xor(
        "ICE".as_bytes(),
        input.as_bytes(),
    )))
}

fn c6(filename: &str) -> Result<String> {
    let input = decode_b64(&std::fs::read_to_string(filename)?);
    let key = find_repeating_xor_key(&input);

    Ok(String::from_utf8(make_repeating_xor(&key, &input))?)
}

fn c7(filename: &str) -> Result<String> {
    let data = decode_b64(&std::fs::read_to_string(filename)?);
    let cipher = Cipher::aes_128_ecb();
    let ciphertext = decrypt(cipher, b"YELLOW SUBMARINE", None, &data)?;

    Ok(String::from_utf8(ciphertext)?)
}

fn c8(filename: &str) -> Result<(usize, String)> {
    let ciphertexts = std::fs::read_to_string(filename)?;
    for (num, line) in ciphertexts.lines().enumerate() {
        let mut set = HashSet::new();
        let decoded = hex_to_u8(line)?;
        for c in decoded.chunks(16) {
            if set.contains(c) {
                return Ok((num + 1, line.to_owned()));
            }
            set.insert(c);
        }
    }
    Err("Not detected!".into())
}

#[cfg(test)]
mod set1 {
    use super::*;
    use openssl::symm::{decrypt, Cipher};

    #[test]
    fn challenge_1() -> Result<()> {
        let c1 = c1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?;
        assert_eq!(
            *c1,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned(),
        );
        Ok(())
    }

    #[test]
    fn challenge_2() -> Result<()> {
        let c2 = c2(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        )?;
        assert_eq!(c2, "746865206b696420646f6e277420706c6179".to_owned());
        Ok(())
    }

    #[test]
    fn challenge_3() -> Result<()> {
        let c3 = c3("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;
        assert_eq!(c3.0 as char, 'X');
        Ok(())
    }

    #[test]
    fn challenge_4() -> Result<()> {
        let c4 = c4("./inputs/s1c4_input.txt")?;
        assert_eq!(
            c4.1,
            "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f"
        );
        Ok(())
    }

    #[test]
    fn challenge_5() -> Result<()> {
        let c5 = c5("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal")?;
        assert_eq!(
            c5,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
        Ok(())
    }

    #[test]
    fn challenge_6() -> Result<()> {
        let a = c6("./inputs/s1c6_input.txt")?;
        let expected = std::fs::read_to_string("./inputs/s1c6_output.txt")?;
        assert_eq!(a, expected);
        Ok(())
    }

    #[test]
    fn challenge_7() -> Result<()> {
        let data = decode_b64(&std::fs::read_to_string("./inputs/s1c7_input.txt")?);
        let cipher = Cipher::aes_128_ecb();
        let ciphertext = decrypt(cipher, b"YELLOW SUBMARINE", None, &data)?;
        let expected = std::fs::read_to_string("./inputs/s1c6_output.txt")?;
        assert_eq!(str::from_utf8(&ciphertext)?, expected);
        Ok(())
    }

    #[test]
    fn challenge_8() -> Result<()> {
        let (num, line) = c8("./inputs/s1c8_input.txt")?;
        assert_eq!(num, 133);
        assert_eq!(
            line,
            "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a".to_owned()
        );
        Ok(())
    }
}
