use std::error::Error;
use cryptopals::{hex_to_u8, u8_to_b64};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    println!("challenge 1: {}", c1()?);

    Ok(())
}

fn c1() -> Result<String> {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    Ok(u8_to_b64(&hex_to_u8(hex)?)?)
}

fn c2() -> Result<()> {
    let hex1_u8 = hex_to_u8("1c0111001f010100061a024b53535009181c")?;
    let hex2_u8 = hex_to_u8("686974207468652062756c6c277320657965")?;
    Ok(())
}
