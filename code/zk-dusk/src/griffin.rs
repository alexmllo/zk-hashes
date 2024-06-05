mod mds_matrix;
mod permutation;
mod round_constants;

use mds_matrix::MDS_MATRIX;
use round_constants::ROUND_CONSTANTS;

/// Width of the Griffin sponge
pub const WIDTH: usize = 8;

const D: usize = 5;

// Width = 4 -> ROUNDS = 11
// Width = 8 -> ROUNDS= 9
const NUMBER_OF_ROUNDS: usize = 9;

#[cfg(feature = "zk")]
pub use permutation::gadget::GadgetPermutation;
pub use permutation::scalar::ScalarPermutation;

/// D_INV of Griffin
pub const D_INV: [u64; 4] = {
    let bytes = include_bytes!("../assets/griffin/d_inv.bin");

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