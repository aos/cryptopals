use cryptopals::cipher::encryption_oracle;
use cryptopals::Result;

fn main() -> Result<()> {
    let message = b"The quick brown fox jumps over the lazy dog";
    let z = encryption_oracle(message)?;
    Ok(())
}
