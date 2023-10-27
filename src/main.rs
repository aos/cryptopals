use cryptopals::Result;
use cryptopals::cipher::{encrypt_aes_128_cbc, decrypt_aes_128_cbc};
use openssl::symm::{Cipher, encrypt, decrypt};

fn main() -> Result<()> {
    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00".repeat(16);
    let message = b"The quick brown fox jumps over the lazy dog";
    let encrypted = encrypt_aes_128_cbc(message, key, &iv)?;
    let result = decrypt_aes_128_cbc(&encrypted, key, &iv)?;

    let compare = decrypt(Cipher::aes_128_cbc(), key, Some(&iv), &encrypted)?;
    println!("openssl: {:?}", std::str::from_utf8(&compare));

    // println!("{:?}", std::str::from_utf8(&result));
    Ok(())
}
