use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::Constraint;
use dusk_safe::Safe;

use super::Anemoi;

use crate::{
    anemoi::{ALPHA_INV, BETA, C, D, DELTA, MDS_MATRIX, NUM_COLUMNS, WIDTH},
    news::NewableScalar,
};

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

impl Anemoi<BlsScalar> for ScalarPermutation {
    fn linear_layer(&mut self, state: &mut [BlsScalar; WIDTH]) {
        let mut x = [BlsScalar::zero(); NUM_COLUMNS];
        x.copy_from_slice(&state[..NUM_COLUMNS]);
        let mut y = [BlsScalar::zero(); NUM_COLUMNS];
        y.copy_from_slice(&state[NUM_COLUMNS..]);

        // mds matrix * x

        // WIDTH = 6
        if WIDTH == 6 {
            x[2] = x[2] + x[1] + BlsScalar::from(7) * x[0];
            x[0] = x[0] + BlsScalar::from(7) * x[2] + x[2];
            x[1] = x[1] + x[0] + BlsScalar::from(7) * x[2];
        }

        // WIDTH = 4
        if WIDTH == 4 {
            x[0] = x[0] + BlsScalar::from(7) * x[1];
            x[1] = x[1] + BlsScalar::from(7) * x[0];
        }

        // WIDTH == 8
        if WIDTH == 8 {
            x[0] += x[1];
            x[2] += x[3];
            x[3] += BlsScalar::from(7) * x[0];
            x[1] = BlsScalar::from(7) * (x[1] + x[2]);
            x[0] += x[1];
            x[2] += BlsScalar::from(7) * x[3];
            x[1] += x[2];
            x[3] += x[0];
        }

        // mds matrix * y
        let mut y_rotated = [BlsScalar::zero(); NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            if i != NUM_COLUMNS - 1 {
                y_rotated[i] = y[i + 1];
            } else {
                y_rotated[i] = y[0];
            }
        }

        // WIDTH = 6
        if WIDTH == 6 {
            y_rotated[2] = y_rotated[2] + y_rotated[1] + BlsScalar::from(7) * y_rotated[0];
            y_rotated[0] = y_rotated[0] + BlsScalar::from(7) * y_rotated[2] + y_rotated[2];
            y_rotated[1] = y_rotated[1] + y_rotated[0] + BlsScalar::from(7) * y_rotated[2];
        }

        // WIDTH = 4
        if WIDTH == 4 {
            y_rotated[0] = y_rotated[0] + BlsScalar::from(7) * y_rotated[1];
            y_rotated[1] = y_rotated[1] + BlsScalar::from(7) * y_rotated[0];
        }

        // WIDTH ==8
        if WIDTH == 8 {
            y_rotated[0] += y_rotated[1];
            y_rotated[2] += y_rotated[3];
            y_rotated[3] += BlsScalar::from(7) * y_rotated[0];
            y_rotated[1] = BlsScalar::from(7) * (y_rotated[1] + y_rotated[2]);
            y_rotated[0] += y_rotated[1];
            y_rotated[2] += BlsScalar::from(7) * y_rotated[3];
            y_rotated[1] += y_rotated[2];
            y_rotated[3] += y_rotated[0];
        }

        // Pseudo-Hadamard transform P
        for i in 0..NUM_COLUMNS {
            y_rotated[i] += x[i];
            x[i] += y_rotated[i];
        }

        state[..NUM_COLUMNS].copy_from_slice(&x);
        state[NUM_COLUMNS..].copy_from_slice(&y_rotated);
    }

    fn add_round_constants_leyer(
        &mut self,
        state: &mut [BlsScalar; WIDTH],
        round: usize,
        column: usize,
    ) {
        state[column] += C[round][column];
        state[NUM_COLUMNS + column] += D[round][column];
    }

    fn evaluate_sbox_layer(&mut self, state: &mut [BlsScalar; WIDTH]) {
        for i in 0..NUM_COLUMNS {
            // x = x - BETA * y^QUAD
            state[i] -= BlsScalar::from(BETA) * state[NUM_COLUMNS + i] * state[NUM_COLUMNS + i];

            // y = y - x^ALPHA_INV
            let mut exp = BlsScalar::default();
            Self::inverse_sbox(self, &mut exp);
            state[NUM_COLUMNS + i] -= exp;

            // x = x + BETA * y^QUAD + DELTA
            state[i] +=
                BlsScalar::from(BETA) * state[NUM_COLUMNS + i] * state[NUM_COLUMNS + i] + DELTA;
        }
    }

    fn inverse_sbox(&mut self, value: &mut BlsScalar) {
        *value = value.pow(&ALPHA_INV);
    }
}
