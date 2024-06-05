use dusk_bls12_381::BlsScalar;

use crate::arion::{NUMBER_OF_ROUNDS, WIDTH};

pub const G: [[BlsScalar; 2 * (WIDTH - 1)]; NUMBER_OF_ROUNDS] = {
    let bytes = include_bytes!("../../assets/arion/G_8.bin");

    let mut cnst = [[BlsScalar::zero(); 2 * (WIDTH - 1)]; NUMBER_OF_ROUNDS];

    let mut i = 0;
    let mut j = 0;
    while i < 2 * (WIDTH - 1) * NUMBER_OF_ROUNDS {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j / (2 * (WIDTH - 1))][j % NUMBER_OF_ROUNDS] =
            BlsScalar::from_raw([a, b, c, d]);
        j += 1;
        i += 32;
    }
    cnst
};

pub const H: [[BlsScalar; WIDTH - 1]; NUMBER_OF_ROUNDS] = {
    let bytes = include_bytes!("../../assets/arion/H_8.bin");

    let mut cnst = [[BlsScalar::zero(); WIDTH - 1]; NUMBER_OF_ROUNDS];

    let mut i = 0;
    let mut j = 0;
    while i < (WIDTH - 1) * NUMBER_OF_ROUNDS {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j / (WIDTH - 1)][j % NUMBER_OF_ROUNDS] =
            BlsScalar::from_raw([a, b, c, d]);
        j += 1;
        i += 32;
    }
    cnst
};

pub const AFFINE: [[BlsScalar; WIDTH]; NUMBER_OF_ROUNDS] = {
    let bytes = include_bytes!("../../assets/arion/AFFINE_8.bin");

    let mut cnst = [[BlsScalar::zero(); WIDTH]; NUMBER_OF_ROUNDS];

    let mut i = 0;
    let mut j = 0;
    while i < WIDTH * NUMBER_OF_ROUNDS {
        let a = super::u64_from_buffer(bytes, i);
        let b = super::u64_from_buffer(bytes, i + 8);
        let c = super::u64_from_buffer(bytes, i + 16);
        let d = super::u64_from_buffer(bytes, i + 24);

        cnst[j / WIDTH][j % NUMBER_OF_ROUNDS] =
            BlsScalar::from_raw([a, b, c, d]);
        j += 1;
        i += 32;
    }
    cnst
};
