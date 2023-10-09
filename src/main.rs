#![allow(dead_code)]
use cryptopals::{calculate_frequency_score, hex_to_u8, u8_to_b64, Result, ALPHABET};
use std::str;

fn main() -> Result<()> {
    println!("{:?}", c4("./inputs/s1c4_input.txt")?);
    Ok(())
}

fn c1(inp: &str) -> Result<String> {
    u8_to_b64(&hex_to_u8(inp)?)
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
    let z: Vec<(u8, Vec<u8>)> = ALPHABET
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
        let z: Vec<(u8, Vec<u8>)> = ALPHABET
            .as_bytes()
            .iter()
            .map(|x| (*x, s.iter().map(|inp| *x ^ *inp).collect::<Vec<u8>>()))
            .collect();

        let y = z
            .iter()
            .map(|(k, chars)| (k, chars, calculate_frequency_score(chars)))
            .fold((' ', "".to_owned(), 0.0), |acc, (&k, c, v)| {
                match str::from_utf8(c) {
                    Ok(s) => if f64::max(acc.2, v) == v {
                                (k as char, s.to_owned(), v)
                            } else {
                                acc
                            }
                    Err(_) => acc
                }
            });

        if f64::max(y.2, max_line.3) == y.2 {
            max_line = (y.0, l.to_owned(), y.1, y.2);
        }
    }
    Ok(max_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() -> Result<()> {
        let c1 = c1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")?;
        assert_eq!(
            c1,
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
        assert_eq!(c4.1, "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f");
        Ok(())
    }
}
