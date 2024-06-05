mod mds_matrix;
mod permutation;
mod round_constants;

use round_constants::{H, G, AFFINE};

// Width = 4 -> Rounds = 5
// Width = 5 -> Rounds = 5
// Width = 6 -> Rounds = 5
const NUMBER_OF_ROUNDS: usize = 4;

/// Width of the Arion sponge
pub const WIDTH: usize = 8;

/// D_1 of Arion
pub const D_1: u64 = 5;

/// D_2 of Arion
pub const D_2: u64 = 257;

#[cfg(feature = "zk")]
pub use permutation::gadget::GadgetPermutation;
pub use permutation::scalar::ScalarPermutation;

/// E_2 of Arion
pub const E_2: [u64; 4] = {
    let bytes = include_bytes!("../assets/arion/E_2.bin");

    let a = u64_from_buffer(bytes, 0);
    let b = u64_from_buffer(bytes, 8);
    let c = u64_from_buffer(bytes, 16);
    let d = u64_from_buffer(bytes, 24);

    [a, b, c, d]
};

const fn u64_from_buffer<const N: usize>(buf: &[u8; N], i: usize) -> u64 {
    u64::from_le_bytes([
        buf[i],
        buf[i + 1],
        buf[i + 2],
        buf[i + 3],
        buf[i + 4],
        buf[i + 5],
        buf[i + 6],
        buf[i + 7],
    ])
}