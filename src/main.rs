#![allow(dead_code)]
use cryptopals::{
    base64::{decode_b64, encode_b64},
    cipher::repeating_key_xor,
    decrypt_single_byte_xor, hamming_distance, hex_to_u8, Result,
};
use std::str;

fn main() -> Result<()> {
    c6("./inputs/s1c6_input.txt")?;
    // c6("./inputs/s1c6_test.txt")?;

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

    Ok(hex1_u8
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>())
}

fn c3(input: &str) -> Result<(char, String, f64)> {
    let s = hex_to_u8(input)?;
    Ok(decrypt_single_byte_xor(&s))
}

fn c4(filename: &str) -> Result<(char, String, String, f64)> {
    let input = std::fs::read_to_string(filename)?;
    let mut min_line: (char, String, String, f64) = (' ', "".to_owned(), "".to_owned(), f64::MAX);
    for l in input.lines() {
        let s = hex_to_u8(l)?;
        let y = decrypt_single_byte_xor(&s);

        if f64::min(y.2, min_line.3) == y.2 {
            min_line = (y.0, l.to_owned(), y.1, y.2);
        }
    }
    Ok(min_line)
}

fn c5(input: &str) -> Result<String> {
    Ok(repeating_key_xor("ICE", input))
}

fn c6(filename: &str) -> Result<String> {
    let input = decode_b64(&std::fs::read_to_string(filename)?);
    let max_keysize = usize::min(40, input.len() / 2);
    let mut possible_key = (0, usize::MAX); // (keysize, distance)

    for keysize in 2..max_keysize {
        let z = hamming_distance(&input[..keysize], &input[keysize..keysize * 2]) / keysize;
        if usize::min(possible_key.1, z) == z {
            possible_key = (keysize, z);
        }
    }

    let mut chunked: Vec<Vec<u8>> = Vec::new();
    for i in input.chunks(possible_key.0) {
        chunked.push(i.to_vec());
    }

    // Pre-tranpose (keysize 3)
    // [ [ 29, 66, 31 ],
    //   [ 77, 11, 15 ] ]
    // Post-tranpose
    // [ [ 29, 77 ],
    //   [ 66, 11 ],
    //   [ 31, 15 ] ]
    let mut transposed: Vec<Vec<u8>> = vec![Vec::with_capacity(chunked.len()); chunked[0].len()];
    for r in chunked {
        for c in 0..r.len() {
            transposed[c].push(r[c]);
        }
    }
    for block in transposed {
        let z = decrypt_single_byte_xor(&block);
        println!("{:?}", z);
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

    #[test]
    fn challenge_6() -> Result<()> {
        Ok(())
    }
}
