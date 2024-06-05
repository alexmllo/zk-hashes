use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use crate::{
    arion::{E_2, WIDTH},
    news::NewableSafe,
};

use super::Arion;

/// GadgetPermutation of Arion
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

impl<'a> Arion<Witness> for GadgetPermutation<'a> {
    fn linear_layer(&mut self, state: &mut [Witness; WIDTH]) {
        let mut w = [Composer::ZERO; WIDTH];
        let mut sigma = Composer::ZERO;
        for val in &mut *state {
            let constraint = Constraint::new().left(1).a(sigma).right(1).b(*val);
            sigma = self.composer.gate_add(constraint);
        }

        let mut sum = Composer::ZERO;
        for i in 0..WIDTH {
            let constraint = Constraint::new()
                .left(BlsScalar::from(i as u64))
                .a(state[i])
                .right(1)
                .b(sum);
            sum = self.composer.gate_add(constraint);
        }
        let constraint = Constraint::new().left(1).a(sigma).right(1).b(sum);
        w[0] = self.composer.gate_add(constraint);

        let mut i = 1;
        // w[i] = w[i-1] - sigma + WIDTH * state[i-1]
        while i < WIDTH {
            let constraint = Constraint::new()
                .left(1)
                .a(w[i - 1])
                .right(BlsScalar::from(1).neg())
                .b(sigma)
                .fourth(BlsScalar::from(WIDTH as u64))
                .d(state[i - 1]);

            w[i] = self.composer.gate_add(constraint);
            i += 1;
        }
        state.copy_from_slice(&w);
    }

    fn affine_layer(&mut self, state: &mut [Witness; WIDTH], constants_aff: &[BlsScalar; WIDTH]) {
        self.linear_layer(state);
        let mut inner = [Composer::ZERO; WIDTH];
        for i in 0..WIDTH {
            let constraint = Constraint::new()
                .left(1)
                .a(inner[i])
                .right(1)
                .b(state[i])
                .constant(constants_aff[i]);
            inner[i] = self.composer.gate_add(constraint);
        }
        state.copy_from_slice(&inner);
    }

    fn gtds(
        &mut self,
        state: &mut [Witness; WIDTH],
        constants_g: &[BlsScalar; 2 * (WIDTH - 1)],
        constants_h: &[BlsScalar; WIDTH - 1],
    ) {
        let mut output = [Composer::ZERO; WIDTH];
        output.copy_from_slice(state);

        Self::inverse_sbox(self, &mut output[WIDTH - 1]);

        let mut sigma = state[WIDTH - 1].clone();
        let constraint = Constraint::new()
            .left(1)
            .a(sigma)
            .right(1)
            .b(output[WIDTH - 1]);
        sigma = self.composer.gate_add(constraint);

        let mut j = 0;
        let mut pos = 20;
        for i in (0..(WIDTH - 1)).rev() {
            Self::sbox_layer(self, &mut output[i]);

            // Evaluate g
            let constraint = Constraint::new()
                .mult(1)
                .a(sigma)
                .b(sigma)
                .right(constants_g[j])
                .b(sigma)
                .constant(constants_g[j + 1]);
            let g = self.composer.gate_add(constraint);

            // Evaluate h
            let constraint = Constraint::new()
                .left(constants_h[i])
                .a(sigma)
                .mult(1)
                .a(sigma)
                .b(sigma);
            let h = self.composer.gate_add(constraint);

            // Multiply g and add h
            let constraint = Constraint::new()
                .mult(1)
                .a(output[i])
                .b(g)
                .fourth(1)
                .d(h);
            output[i] = self.composer.gate_mul(constraint);

            let constraint = Constraint::new()
                .left(1)
                .a(sigma)
                .right(1)
                .b(output[i])
                .fourth(1)
                .d(state[i]);
            sigma = self.composer.gate_add(constraint);

            if pos > 0 {
                pos -= 2;
                j += 2;
            }
        }
        state.copy_from_slice(&output);
    }

    fn inverse_sbox(&mut self, value: &mut Witness) {
        let tmp = self.composer[*value];
        let tmp = tmp.pow_vartime(&E_2);

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

    fn sbox_layer(&mut self, value: &mut Witness) {
        let constraint = Constraint::new().mult(1).a(*value).b(*value);
        let v2 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v2).b(v2);
        let v4 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v4).b(*value);
        *value = self.composer.gate_mul(constraint);
    }
}
