#[cfg(feature = "zk")]
pub mod gadget;
pub mod scalar;

use crate::Error;

use alloc::vec::Vec;
use dusk_safe::Call;

/// The Domain Separation for Poseidon
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Domain {
    /// Domain to specify hashing of 4-arity merkle tree.
    /// Note that selecting this domain-separator means that the total hash
    /// input must be exactly 4 `BlsScalar` long, and any empty slots of the
    /// merkle tree level need to be filled with the zero element.
    Merkle4,
    /// Domain to specify hashing of 2-arity merkle tree
    /// Note that selecting this domain-separator means that the total hash
    /// input must be exactly 2 `BlsScalar` long, and any empty slots of the
    /// merkle tree level need to be filled with the zero element.
    Merkle2,
    /// Domain to specify hash used for encryption
    Encryption,
    /// Domain to specify hash for any other input
    Other,
}

impl From<Domain> for u64 {
    /// Encryption for the domain-separator are taken from section 4.2 of the
    /// paper adapted to u64.
    /// When `Other` is selected we set the domain-separator to zero. We can do
    /// this since the io-pattern will be encoded in the tag in any case,
    /// ensuring safety from collision attacks.
    fn from(domain: Domain) -> Self {
        match domain {
            // 2^4 - 1
            Domain::Merkle4 => 0x0000_0000_0000_000f,
            // 2^2 - 1
            Domain::Merkle2 => 0x0000_0000_0000_0003,
            // 2^32
            Domain::Encryption => 0x0000_0001_0000_0000,
            // 0
            Domain::Other => 0x0000_0000_0000_0000,
        }
    }
}

// This function, which is called during the finalization step of the hash, will
// always produce a valid io-pattern based on the input.
// The function will return an error if a merkle domain is selected but the
// given input elements don't add up to the specified arity.
fn io_pattern<T>(domain: Domain, input: &[&[T]], output_len: usize) -> Result<Vec<Call>, Error> {
    let mut io_pattern = Vec::new();
    // check total input length against domain
    let input_len = input.iter().fold(0, |acc, input| acc + input.len());
    match domain {
        Domain::Merkle2 if input_len != 2 || output_len != 1 => {
            return Err(Error::IOPatternViolation);
        }
        Domain::Merkle4 if input_len != 4 || output_len != 1 => {
            return Err(Error::IOPatternViolation);
        }
        _ => {}
    }
    for input in input.iter() {
        io_pattern.push(Call::Absorb(input.len()));
    }
    io_pattern.push(Call::Squeeze(output_len));

    Ok(io_pattern)
}
