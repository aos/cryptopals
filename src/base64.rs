use crate::{encode_table, Result};

use bitvec::prelude::*;
use std::str;

pub const B64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_b64(input: &[u8]) -> Result<String> {
    let alphabet = encode_table(B64_ALPHABET);
    let mut encoded: Vec<u8> = Vec::new();

    let bits = input.view_bits::<Msb0>();
    let remainder = align_up(bits.len(), 24) - bits.len();

    for i in bits.chunks(6) {
        if i.len() < 6 {
            let mut bv = bitvec![u8, Msb0;];
            let pad_zero = 6 - i.len();
            let padding = (remainder - pad_zero) / 6;

            bv.extend(i);
            bv.extend_from_bitslice(&bitvec![u8, Msb0; 0; pad_zero]);

            encoded.push(alphabet[bv.load_be::<u8>() as usize]);
            encoded.extend(vec![0x3D; padding]);
        } else {
            encoded.push(alphabet[i.load_be::<u8>() as usize]);
        }
    }

    Ok(str::from_utf8(&encoded)?.to_owned())
}

fn align_up(num: usize, to: usize) -> usize {
    // By subtracting one, we get "as close as possible"
    // then we do integer division to get the "multiple"
    // and multiply the multiple by the target number
    ((num + to - 1) / to) * to
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn encode_str_b64() -> Result<()> {
        let tests = HashMap::from([
            ("Cat", "Q2F0"),
            ("Ca", "Q2E="),
            ("C", "Qw=="),
            ("light work.", "bGlnaHQgd29yay4="),
            ("light work", "bGlnaHQgd29yaw=="),
            ("light wor", "bGlnaHQgd29y"),
            ("light wo", "bGlnaHQgd28="),
            ("light w", "bGlnaHQgdw=="),
        ]);
        for (test, expected) in &tests {
            let b64 = encode_b64(test.as_bytes())?;
            assert_eq!(b64, expected.to_owned());
        }
        Ok(())
    }
}
