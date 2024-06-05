use dusk_bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use crate::{
    news::NewableSafe,
    rescue::{ALPHA_INV, MDS_MATRIX, ROUND_CONSTANTS, WIDTH},
};

use super::Rescue;

/// Gadget Permutation of the Rescue
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

impl<'a> Rescue<Witness> for GadgetPermutation<'a> {
    fn add_round_constants_leyer_1(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
            let constant = ROUND_CONSTANTS[round * 2 * WIDTH + i];
            let constraint = Constraint::new().left(1).a(*w).constant(constant);

            *w = self.composer.gate_add(constraint);
        })
    }

    fn add_round_constants_leyer_2(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
            let constant = ROUND_CONSTANTS[round * 2 * WIDTH + WIDTH + i];
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

    fn mds_leyer(&mut self, state: &mut [Witness; WIDTH]) {
        // For t = 5, r = 4
        let mut result = [Composer::ZERO; WIDTH];
        
        /* for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][0])
                .a(state[0])
                .right(MDS_MATRIX[j][1])
                .b(state[1])
                .fourth(MDS_MATRIX[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][3])
                .a(state[3])
                .right(MDS_MATRIX[j][4])
                .b(state[4])
                .fourth(1)
                .d(result[j]);

            result[j] = self.composer.gate_add(constraint);
        }
        state.copy_from_slice(&result); */

        // For t=4, r=2
        /* let mut result = [Composer::ZERO; WIDTH];

        for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][0])
                .a(state[0])
                .right(MDS_MATRIX[j][1])
                .b(state[1])
                .fourth(MDS_MATRIX[j][2])
                .d(state[2]);
        
            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][3])
                .a(state[3])
                .right(1)
                .b(result[j]);

            result[j] = self.composer.gate_add(constraint);
        }
        state.copy_from_slice(&result); */

        // For t = 6, r=4
        /* for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][0])
                .a(state[0])
                .right(MDS_MATRIX[j][1])
                .b(state[1])
                .fourth(MDS_MATRIX[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][3])
                .a(state[3])
                .right(MDS_MATRIX[j][4])
                .b(state[4])
                .fourth(1)
                .d(result[j]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][5])
                .a(state[5])
                .right(1)
                .b(result[j]);

            result[j] = self.composer.gate_add(constraint);
        }
        state.copy_from_slice(&result); */

        // For t = 8, r = 4
        for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][0])
                .a(state[0])
                .right(MDS_MATRIX[j][1])
                .b(state[1])
                .fourth(MDS_MATRIX[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][3])
                .a(state[3])
                .right(MDS_MATRIX[j][4])
                .b(state[4])
                .fourth(1)
                .d(result[j]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][5])
                .a(state[5])
                .right(MDS_MATRIX[j][6])
                .b(state[6])
                .fourth(1)
                .d(result[j]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MDS_MATRIX[j][7])
                .a(state[7])
                .fourth(1)
                .d(result[j]);

            result[j] = self.composer.gate_add(constraint);
        }
        state.copy_from_slice(&result);
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
