mod mds_matrix;
mod permutation;
mod round_constants;

use mds_matrix::MDS_MATRIX;
use round_constants::ROUND_CONSTANTS;

/// Width of the rescue sponge
pub const WIDTH: usize = 8;

// WIDTH = 4 -> Rounds=11
// WIDTH= 5 -> Rounds=9
// WIDTH= 6 -> Rounds=8
// WIDTH= 8 -> Rounds=8
const NUMBER_OF_ROUNDS: usize = 8;

#[cfg(feature = "zk")]
pub use permutation::gadget::GadgetPermutation;
pub use permutation::scalar::ScalarPermutation;

#[allow(dead_code)]
const ALPHA: usize = 5;

/// ALPHA_INV of Rescue
pub const ALPHA_INV: [u64; 4] = {
    let bytes = include_bytes!("../assets/rescue/alpha_inv.bin");

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