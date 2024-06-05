use dusk_bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Griffin;

use crate::{griffin::{mds_matrix::CIRC_MAT, round_constants::{ALPHAS, BETAS}, D_INV, MDS_MATRIX, ROUND_CONSTANTS, WIDTH}, news::NewableScalar};

/// ScalarPermutation of Griffin
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

impl Griffin<BlsScalar> for ScalarPermutation {
    fn non_liner_layer(&mut self, state: &mut[BlsScalar; WIDTH]) {
        fn li(z0: &BlsScalar, z1: &BlsScalar, z2: &BlsScalar, i: usize) -> BlsScalar {
            let prod1 = z0.mul(&BlsScalar::from((i-1) as u64));
            let prod2 = prod1.add(z1);
            prod2.add(z2)
        }

        self.inverse_sbox(&mut state[0]);
        self.sbox_layer(&mut state[1]);

        let mut l = li(&state[0], &state[1], &BlsScalar::zero(), 2);

        state[2] = state[2] * (l.square() + ALPHAS[0] * l + BETAS[0]);

        for i in 3..WIDTH {
            l = li(&state[0], &state[1], &state[i - 1], i);
            state[i] = state[i] * (l.square() + ALPHAS[i - 2] * l + BETAS[i - 2]);
        }
    }

    fn linear_layer(&mut self, state: &mut[BlsScalar; WIDTH]) {
        let mut sum = [BlsScalar::default(); WIDTH];

        for i in 0..(WIDTH / 4) {
            for j in 0..(WIDTH / 4) {
                let mut off = 0;
                if i != j {
                    off = 4;
                }
                for k in 0..4 {
                    for l in 0..4 {
                        sum[4 * i + k] += BlsScalar::from(CIRC_MAT[off + l]) * state[4 * j + l];
                    }
                }
            }
        }
        state.copy_from_slice(&sum);
    }

    fn add_round_constants_leyer(&mut self, state: &mut[BlsScalar; WIDTH], round: usize) {
        for j in 0..WIDTH {
            state[j] += ROUND_CONSTANTS[round * WIDTH + j];
        }
    }

    fn sbox_layer(&mut self, value: &mut BlsScalar) {
        *value = value.square().square() * *value;
    }

    fn inverse_sbox(&mut self, value: &mut BlsScalar) {
        *value = value.pow(&D_INV);
    }
}