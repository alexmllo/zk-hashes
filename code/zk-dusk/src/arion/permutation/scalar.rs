use dusk_bls12_381::BlsScalar;
use dusk_safe::Safe;

use super::Arion;
use crate::{
    arion::{E_2, WIDTH},
    news::NewableScalar,
};

/// An implementation of the [`Permutation`] for `BlsScalar` as input values.
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

impl Arion<BlsScalar> for ScalarPermutation {
    fn linear_layer(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut w = [BlsScalar::zero(); WIDTH];
        let mut sigma = BlsScalar::zero();
        for val in &mut *state {
            sigma += *val;
        }

        let mut sum = BlsScalar::zero();
        for i in 0..WIDTH {
            sum += BlsScalar::from(i as u64) * state[i];
        }

        w[0] = sigma + sum;

        let mut i = 1;
        while i < WIDTH {
            w[i] = w[i - 1] - sigma + BlsScalar::from(WIDTH as u64) * state[i - 1];
            i += 1;
        }
        state.copy_from_slice(&w);
    }

    fn affine_layer(&mut self, state: &mut [BlsScalar; WIDTH], constants_aff: &[BlsScalar; WIDTH]) {
        Self::linear_layer(self, state);
        let mut inner = [BlsScalar::zero(); WIDTH];
        for i in 0..WIDTH {
            inner[i] += state[i] + constants_aff[i];
        }
        state.copy_from_slice(&inner);
    }

    fn gtds(
        &mut self,
        state: &mut [BlsScalar; WIDTH],
        constants_g: &[BlsScalar; 2 * (WIDTH - 1)],
        constants_h: &[BlsScalar; WIDTH - 1],
    ) {
        let mut output = [BlsScalar::zero(); WIDTH];
        output.copy_from_slice(state);

        Self::inverse_sbox(self, &mut output[WIDTH - 1]);

        let mut sigma = state[WIDTH - 1].clone();
        sigma += output[WIDTH - 1];

        let mut j = 0;
        let mut pos = 20;
        for i in (0..(WIDTH - 1)).rev() {
            Self::sbox_layer(self, &mut output[i]);

            // Evaluate g and h
            let g = sigma.square() + sigma * constants_g[j] + constants_g[j + 1];
            let h = sigma.square() + sigma * constants_h[i];

            // Multiply g and add h
            output[i] = output[i] * g + h;

            sigma = sigma + output[i] + state[i];

            if pos > 0 {
                pos -= 2;
                j += 2;
            }
        }
        state.copy_from_slice(&output);
    }

    fn inverse_sbox(&mut self, value: &mut BlsScalar) {
        *value = value.pow(&E_2)
    }

    fn sbox_layer(&mut self, value: &mut BlsScalar) {
        *value = value.square().square() * *value;
    }
}
