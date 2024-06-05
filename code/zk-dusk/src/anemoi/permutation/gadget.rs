use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use crate::{
    anemoi::{
        round_constants::{C, D},
        ALPHA_INV, BETA, DELTA, MDS_MATRIX, NUM_COLUMNS, WIDTH,
    },
    news::NewableSafe,
};

use super::Anemoi;

pub struct GadgetPermutation<'a> {
    composer: &'a mut Composer,
}

impl<'a> GadgetPermutation<'a> {
    /// Constructs a new `GadgetPermutation` with the constraint system.
    pub fn new(composer: &'a mut Composer) -> Self {
        Self { composer }
    }
}

impl<'a> NewableSafe<WIDTH> for GadgetPermutation<'a> {
    type T<'b> = GadgetPermutation<'b>;

    fn new(composer: &mut Composer) -> Self::T<'_> {
        Self::T::new(composer)
    }
}

impl<'a> Safe<Witness, WIDTH> for GadgetPermutation<'a> {
    fn permute(&mut self, state: &mut [Witness; WIDTH]) {
        self.perm(state);
    }

    fn tag(&mut self, input: &[u8]) -> Witness {
        let tag = BlsScalar::hash_to_scalar(input.as_ref());
        // append the tag as a constant
        self.composer.append_constant(tag)
    }

    fn add(&mut self, right: &Witness, left: &Witness) -> Witness {
        let constraint = Constraint::new().left(1).a(*left).right(1).b(*right);
        self.composer.gate_add(constraint)
    }
}

impl<'a> Anemoi<Witness> for GadgetPermutation<'a> {
    fn linear_layer(&mut self, state: &mut [Witness; WIDTH]) {
        let mut x = [Composer::ZERO; NUM_COLUMNS];
        x.copy_from_slice(&state[..NUM_COLUMNS]);
        let mut y = [Composer::ZERO; NUM_COLUMNS];
        y.copy_from_slice(&state[NUM_COLUMNS..]);

        // mds matrix * x
        // WIDTH=6
        if WIDTH == 6 {
            let constraint = Constraint::new()
                .left(7)
                .a(x[0])
                .right(1)
                .b(x[1])
                .fourth(1)
                .d(x[2]);

            x[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(x[0])
                .right(7)
                .b(x[2])
                .fourth(1)
                .d(x[2]);

            x[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(x[1])
                .right(1)
                .b(x[0])
                .fourth(7)
                .d(x[2]);

            x[1] = self.composer.gate_add(constraint);
        }

        // WIDTH=4
        if WIDTH == 4 {
            let constraint = Constraint::new().left(1).a(x[0]).right(7).b(x[1]);
            x[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[1]).right(7).b(x[0]);
            x[1] = self.composer.gate_add(constraint);
        }

        // Width = 8
        if WIDTH == 8 {
            let constraint = Constraint::new().left(1).a(x[0]).right(1).b(x[1]);
            x[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[2]).right(1).b(x[3]);
            x[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[3]).right(7).b(x[0]);
            x[3] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(7).a(x[1]).right(7).b(x[2]);
            x[1] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[0]).right(1).b(x[1]);
            x[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[2]).right(7).b(x[3]);
            x[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[1]).right(1).b(x[2]);
            x[1] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[3]).right(1).b(x[0]);
            x[3] = self.composer.gate_add(constraint);
        }

        // mds matrix * y
        let mut y_rotated = [Composer::ZERO; NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            if i != NUM_COLUMNS - 1 {
                y_rotated[i] = y[i + 1];
            } else {
                y_rotated[i] = y[0];
            }
        }

        // WIDTH = 6
        if WIDTH == 6 {
            let constraint = Constraint::new()
                .left(7)
                .a(y_rotated[0])
                .right(1)
                .b(y_rotated[1])
                .fourth(1)
                .d(y_rotated[2]);

            y_rotated[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(y_rotated[0])
                .right(7)
                .b(y_rotated[2])
                .fourth(1)
                .d(y_rotated[2]);

            y_rotated[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(y_rotated[1])
                .right(1)
                .b(y_rotated[0])
                .fourth(7)
                .d(y_rotated[2]);

            y_rotated[1] = self.composer.gate_add(constraint);
        }

        // WIDTH = 4
        if WIDTH == 4 {
            let constraint = Constraint::new()
                .left(1)
                .a(y_rotated[0])
                .right(7)
                .b(y_rotated[1]);
            y_rotated[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(y_rotated[1])
                .right(7)
                .b(y_rotated[0]);
            y_rotated[1] = self.composer.gate_add(constraint);
        }

        // Width == 8
        let constraint = Constraint::new().left(1).a(y_rotated[0]).right(1).b(y_rotated[1]);
            y_rotated[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[2]).right(1).b(y_rotated[3]);
            y_rotated[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[3]).right(7).b(y_rotated[0]);
            y_rotated[3] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(7).a(y_rotated[1]).right(7).b(y_rotated[2]);
            y_rotated[1] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[0]).right(1).b(y_rotated[1]);
            y_rotated[0] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[2]).right(7).b(y_rotated[3]);
            y_rotated[2] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[1]).right(1).b(y_rotated[2]);
            y_rotated[1] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(y_rotated[3]).right(1).b(y_rotated[0]);
            y_rotated[3] = self.composer.gate_add(constraint);

        // Pseudo-Hadamard transform P
        for i in 0..NUM_COLUMNS {
            let constraint = Constraint::new().left(1).a(y_rotated[i]).right(1).b(x[i]);

            y_rotated[i] = self.composer.gate_add(constraint);

            let constraint = Constraint::new().left(1).a(x[i]).right(1).b(y_rotated[i]);

            x[i] = self.composer.gate_add(constraint);
        }

        state[..NUM_COLUMNS].copy_from_slice(&x);
        state[NUM_COLUMNS..].copy_from_slice(&y_rotated);
    }

    fn add_round_constants_leyer(
        &mut self,
        state: &mut [Witness; WIDTH],
        round: usize,
        column: usize,
    ) {
        let constraint = Constraint::new()
            .left(1)
            .a(state[column])
            .constant(C[round][column]);
        state[column] = self.composer.gate_add(constraint);

        let constraint = Constraint::new()
            .left(1)
            .a(state[NUM_COLUMNS + column])
            .constant(D[round][column]);
        state[NUM_COLUMNS + column] = self.composer.gate_add(constraint);
    }

    fn evaluate_sbox_layer(&mut self, state: &mut [Witness; WIDTH]) {
        for i in 0..NUM_COLUMNS {
            // x = x - BETA * y^QUAD
            let constraint = Constraint::new()
                .mult(BlsScalar::from(BETA).neg())
                .a(state[NUM_COLUMNS + i])
                .b(state[NUM_COLUMNS + i])
                .fourth(1)
                .d(state[i]);
            state[i] = self.composer.gate_add(constraint);

            // y = y - x^ALPHA_INV
            let mut wit = Composer::ZERO;
            Self::inverse_sbox(self, &mut wit);
            let constraint = Constraint::new()
                .left(1)
                .a(state[NUM_COLUMNS + i])
                .right(BlsScalar::from(1u64).neg())
                .b(wit);
            state[NUM_COLUMNS + i] = self.composer.gate_add(constraint);

            // x = x + BETA * y^QUAD + DELTA
            let constraint = Constraint::new()
                .mult(1)
                .a(state[NUM_COLUMNS + i])
                .b(state[NUM_COLUMNS + i]);
            let exp = self.composer.gate_mul(constraint);
            let constraint = Constraint::new()
                .left(1)
                .a(state[i])
                .right(BETA)
                .b(exp)
                .constant(DELTA);
            state[i] = self.composer.gate_add(constraint);
        }
    }

    fn inverse_sbox(&mut self, value: &mut Witness) {
        let tmp = self.composer[*value];
        let tmp = tmp.pow_vartime(&ALPHA_INV);

        let wit = self.composer.append_witness(tmp);
        // y^2
        let constraint = Constraint::new().mult(1).a(wit).b(wit);
        let tmp_wit = self.composer.gate_mul(constraint);
        // y^4
        let constraint = Constraint::new().mult(1).a(tmp_wit).b(tmp_wit);
        let tmp_wit = self.composer.gate_mul(constraint);
        // y^5
        let constraint = Constraint::new().mult(1).a(tmp_wit).b(wit);
        *value = self.composer.gate_mul(constraint);
        *value = wit;
    }
}
