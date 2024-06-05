use crate::rescue::WIDTH;

use dusk_bls12_381::BlsScalar;

pub const MDS_MATRIX: [[BlsScalar; WIDTH]; WIDTH] = {
    let bytes = include_bytes!("../../assets/rescue/mds_matrix_8.bin");
    let mut mds = [[BlsScalar::zero(); WIDTH]; WIDTH];
    let mut k = 0;
    let mut i = 0;

    while i < WIDTH {
        let mut j = 0;
        while j < WIDTH {
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