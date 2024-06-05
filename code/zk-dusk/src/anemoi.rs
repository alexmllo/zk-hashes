mod mds_matrix;
mod permutation;
mod round_constants;

use dusk_bls12_381::BlsScalar;

use mds_matrix::MDS_MATRIX;
use round_constants::{C, D};

const NUM_COLUMNS: usize = WIDTH / 2;

// Width = 4 -> Rounds=12
// Width = 6 -> Rounds=10
// Width = 8 -> Rounds=10
const NUMBER_OF_ROUNDS: usize = 10;

#[cfg(feature = "zk")]
pub use permutation::gadget::GadgetPermutation;
pub use permutation::scalar::ScalarPermutation;

/// Width of the Anemoi sponge
pub const WIDTH: usize = 8;

const ALPHA: u64 = 5;

/// ALPHA_IN of Anemoi
pub const ALPHA_INV: [u64; 4] = {
    let bytes = include_bytes!("../assets/anemoi/alpha_inv.bin");

    let a = u64_from_buffer(bytes, 0);
    let b = u64_from_buffer(bytes, 8);
    let c = u64_from_buffer(bytes, 16);
    let d = u64_from_buffer(bytes, 24);

    [a, b, c, d]
};

const BETA: u64 = 7;

/// DELTA of Anemoi
pub const DELTA: BlsScalar = {
    let bytes = include_bytes!("../assets/anemoi/delta.bin");

    let a = u64_from_buffer(bytes, 0);
    let b = u64_from_buffer(bytes, 8);
    let c = u64_from_buffer(bytes, 16);
    let d = u64_from_buffer(bytes, 24);

    BlsScalar::from_raw([a, b, c, d])
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