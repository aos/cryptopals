#![allow(dead_code)]
use cryptopals::{
    base64::{Base64, B64_ALPHABET},
    calculate_frequency_score, hamming_distance, hex_to_u8, repeating_key_xor, Result,
};
use std::str;

fn main() -> Result<()> {
    println!("{}", hamming_distance(b"this is a test", b"wokka wokka!!!"));

    Ok(())
}

fn c1(inp: &str) -> Result<Base64> {
    hex_to_u8(inp)?.as_slice().try_into()
}

fn c2(hex1: &str, hex2: &str) -> Result<String> {
    let mut hex1_u8 = hex_to_u8(hex1)?;
    let hex2_u8 = hex_to_u8(hex2)?;

    hex1_u8
        .iter_mut()
        .zip(hex2_u8.iter())
        .for_each(|(x1, x2)| *x1 ^= *x2);

    Ok(hex1_u8
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>())
}

fn c3(input: &str) -> Result<(char, String, f64)> {
    let s = hex_to_u8(input)?;
    let z: Vec<(u8, Vec<u8>)> = B64_ALPHABET
        .as_bytes()
        .iter()
        .map(|x| (*x, s.iter().map(|inp| *x ^ *inp).collect::<Vec<u8>>()))
        .collect();

    let y = z
        .iter()
        .map(|(k, chars)| (k, chars, calculate_frequency_score(chars)))
        .fold((' ', "".to_owned(), 0.0), |acc, (&k, c, v)| {
            if f64::max(acc.2, v) == v {
                (k as char, str::from_utf8(c).unwrap().to_owned(), v)
            } else {
                acc
            }
        });

    Ok(y)
}

fn c4(filename: &str) -> Result<(char, String, String, f64)> {
    let input = std::fs::read_to_string(filename)?;
    let mut max_line: (char, String, String, f64) = (' ', "".to_owned(), "".to_owned(), 0.0);
    for l in input.lines() {
        let s = hex_to_u8(l)?;
        let z: Vec<(u8, Vec<u8>)> = B64_ALPHABET
            .as_bytes()
            .iter()
            .map(|x| (*x, s.iter().map(|inp| *x ^ *inp).collect::<Vec<u8>>()))
            .collect();

        let y = z
            .iter()
            .map(|(k, chars)| (k, chars, calculate_frequency_score(chars)))
            .fold(
                (' ', "".to_owned(), 0.0),
                |acc, (&k, c, v)| match str::from_utf8(c) {
                    Ok(s) => {
                        if f64::max(acc.2, v) == v {
                            (k as char, s.to_owned(), v)
                        } else {
                            acc
                        }
                    }
                    Err(_) => acc,
                },
            );

        if f64::max(y.2, max_line.3) == y.2 {
            max_line = (y.0, l.to_owned(), y.1, y.2);
        }
    }
    Ok(max_line)
}

fn c5(input: &str) -> Result<String> {
    Ok(repeating_key_xor("ICE", input))
}

fn c6(filename: &str) -> Result<String> {
    let input = std::fs::read_to_string(filename)?;
    let input_as_bytes = input.as_bytes();
    assert!(input_as_bytes.len() >= 80);

    for keysize in 2..40 {
        println!("first: {:?}", &input_as_bytes[..keysize]);
        println!("second: {:?}", &input_as_bytes[keysize..keysize * 2]);
        let z = hamming_distance(
            &input_as_bytes[..keysize],
            &input_as_bytes[keysize..keysize * 2],
        );
        println!("keysize: {}, distance: {}", keysize, z / keysize);
    }
    Ok("ok!".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(c3.0, 'X');
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
}
