use dusk_plonk::prelude::*;

use super::{NUM_COLUMNS, NUMBER_OF_ROUNDS};

pub const C: [[BlsScalar; NUM_COLUMNS]; NUMBER_OF_ROUNDS] = {
    let bytes = include_bytes!("../../assets/anemoi/c_8.bin");
    let mut mds = [[BlsScalar::zero(); NUM_COLUMNS]; NUMBER_OF_ROUNDS];
    let mut k = 0;
    let mut i = 0;

    while i < NUMBER_OF_ROUNDS {
        let mut j = 0;
        while j < NUM_COLUMNS {
            let a = super::u64_from_buffer(bytes, k);
            let b = super::u64_from_buffer(bytes, k + 8);
            let c = super::u64_from_buffer(bytes, k + 16);
            let d = super::u64_from_buffer(bytes, k + 24);
            k += 32;

            mds[i][j] = BlsScalar::from_raw([a, b, c, d]);
            j += 1;
        }
        i += 1;
    }

    mds
};

pub const D: [[BlsScalar; NUM_COLUMNS]; NUMBER_OF_ROUNDS] = {
    let bytes = include_bytes!("../../assets/anemoi/d_8.bin");
    let mut mds = [[BlsScalar::zero(); NUM_COLUMNS]; NUMBER_OF_ROUNDS];
    let mut k = 0;
    let mut i = 0;

    while i < NUMBER_OF_ROUNDS {
        let mut j = 0;
        while j < NUM_COLUMNS {
            let a = super::u64_from_buffer(bytes, k);
            let b = super::u64_from_buffer(bytes, k + 8);
            let c = super::u64_from_buffer(bytes, k + 16);
            let d = super::u64_from_buffer(bytes, k + 24);
            k += 32;

            mds[i][j] = BlsScalar::from_raw([a, b, c, d]);
            j += 1;
        }
        i += 1;
    }

    mds
};