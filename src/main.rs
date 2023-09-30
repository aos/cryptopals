use std::error::Error;
use cryptopals::{hex_to_u8, u8_to_b64};

fn main() -> Result<(), Box<dyn Error>> {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("{}", u8_to_b64(&hex_to_u8(hex)?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn encode_str_b64() {
        let tests = HashMap::from([
            ("Man", "TWFu"),
            ("Ma", "TWE="),
            ("M", "TQ=="),
            ("light work.", "bGlnaHQgd29yay4="),
            ("light work", "bGlnaHQgd29yaw=="),
            ("light wor", "bGlnaHQgd29y"),
            ("light wo", "bGlnaHQgd28="),
            ("light w", "bGlnaHQgdw=="),
        ]);
        for (test, expected) in &tests {
            let b64 = u8_to_b64(test.as_bytes()).unwrap_or(String::from(""));
            assert_eq!(b64, expected.to_owned());
        }
    }

    #[test]
    fn encode_hex_b64() {
        let tests = HashMap::from([
            (
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            ),
        ]);
        for (test, expected) in &tests {
            let to_u8 = hex_to_u8(test).unwrap_or(Vec::new());
            let b64 = u8_to_b64(&to_u8).unwrap_or(String::from(""));
            assert_eq!(b64, expected.to_owned());
        }
    }
}
