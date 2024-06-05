use dusk_bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Rescue;

use crate::{
    news::NewableScalar,
    rescue::{ALPHA_INV, MDS_MATRIX, ROUND_CONSTANTS, WIDTH},
};

/// ScalarPermutation of Rescue
#[derive(Default)]
pub struct ScalarPermutation();

impl NewableScalar for ScalarPermutation {
    /// Constructs a new `ScalarPermutation`.
    fn new() -> Self {
        Self()
    }
}

impl Safe<BlsScalar, WIDTH> for ScalarPermutation {
    fn permute(&mut self, state: &mut [BlsScalar; WIDTH]) {
        self.perm(state);
    }

    fn tag(&mut self, input: &[u8]) -> BlsScalar {
        BlsScalar::hash_to_scalar(input.as_ref())
    }

    fn add(&mut self, right: &BlsScalar, left: &BlsScalar) -> BlsScalar {
        right + left
    }
}

impl Rescue<BlsScalar> for ScalarPermutation {
    fn add_round_constants_leyer_1(
        &mut self,
        round: usize,
        state: &mut [BlsScalar; WIDTH],
    ) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
            *w += ROUND_CONSTANTS[round * 2 * WIDTH + i]
        })
    }

    fn add_round_constants_leyer_2(
        &mut self,
        round: usize,
        state: &mut [BlsScalar; WIDTH],
    ) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
            *w += ROUND_CONSTANTS[round * 2 * WIDTH + WIDTH + i]
        })
    }

    fn sbox_layer(&mut self, value: &mut BlsScalar) {
        *value = value.square().square() * *value;
    }

    fn mds_leyer(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut result = [BlsScalar::zero(); WIDTH];

        for (j, value) in state.iter().enumerate() {
            for k in 0..WIDTH {
                result[k] += MDS_MATRIX[k][j] * value;
            }
        }
        state.copy_from_slice(&result);
    }

    fn inverse_sbox(&mut self, value: &mut BlsScalar) {
        *value = value.pow(&ALPHA_INV);
    }
}
