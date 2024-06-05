use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use crate::{
    griffin::{
        mds_matrix::CIRC_MAT, round_constants::{ALPHAS, BETAS}, D_INV, MDS_MATRIX, ROUND_CONSTANTS, WIDTH
    },
    news::NewableSafe,
};

use super::Griffin;

/// GadgetPermutation of Griffin
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

impl<'a> Griffin<Witness> for GadgetPermutation<'a> {
    fn non_liner_layer(&mut self, state: &mut [Witness; WIDTH]) {
        fn li(z0: &Witness, z1: &Witness, z2: &Witness, i: usize) -> Constraint {
            Constraint::new()
                .left(BlsScalar::from((i - 1) as u64))
                .a(*z0)
                .right(1)
                .b(*z1)
                .fourth(1)
                .d(*z2)
        }

        self.inverse_sbox(&mut state[0]);
        self.sbox_layer(&mut state[1]);

        let constraint = li(&state[0], &state[1], &Composer::ZERO, 2);
        let mut l = self.composer.gate_add(constraint);

        let constraint = Constraint::new()
            .left(ALPHAS[0])
            .a(l)
            .mult(1)
            .a(l)
            .b(l)
            .constant(BETAS[0]);
        let op1 = self.composer.gate_add(constraint);
        let constraint = Constraint::new().mult(1).a(state[2]).b(op1);
        state[2] = self.composer.gate_add(constraint);

        for i in 3..WIDTH {
            let constraint = li(&state[0], &state[1], &state[i - 1], i);
            l = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(ALPHAS[i - 2])
                .a(l)
                .mult(1)
                .a(l)
                .b(l)
                .constant(BETAS[i - 2]);
            let op1 = self.composer.gate_add(constraint);
            let constraint = Constraint::new().mult(1).a(state[i]).b(op1);
            state[i] = self.composer.gate_add(constraint);
        }
    }

    fn linear_layer(&mut self, state: &mut [Witness; WIDTH]) {
        let mut sum = [Composer::ZERO; WIDTH];

        for i in 0..(WIDTH / 4) {
            for j in 0..(WIDTH / 4) {
                let mut off = 0;
                if i != j {
                    off = 4;
                }
                for k in 0..4 {
                    for l in 0..4 {
                        let constraint = Constraint::new()
                            .left(CIRC_MAT[off + l])
                            .a(state[4 * j + l])
                            .right(1)
                            .b(sum[4 * i + k]);

                        sum[4 * i + k] = self.composer.gate_add(constraint);
                    }
                }
            }
        }
        state.copy_from_slice(&sum);
    }

    fn add_round_constants_leyer(&mut self, state: &mut [Witness; WIDTH], round: usize) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
            let constant = ROUND_CONSTANTS[round * WIDTH + i];
            let constraint = Constraint::new().left(1).a(*w).constant(constant);

            *w = self.composer.gate_add(constraint);
        })
    }

    fn sbox_layer(&mut self, value: &mut Witness) {
        let constraint = Constraint::new().mult(1).a(*value).b(*value);
        let v2 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v2).b(v2);
        let v4 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v4).b(*value);
        *value = self.composer.gate_mul(constraint);
    }

    fn inverse_sbox(&mut self, value: &mut Witness) {
        let tmp = self.composer[*value];
        let tmp = tmp.pow_vartime(&D_INV);

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
