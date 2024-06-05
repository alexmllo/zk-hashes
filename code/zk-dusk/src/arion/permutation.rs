use crate::arion::{NUMBER_OF_ROUNDS, WIDTH, H, G, AFFINE};

use dusk_bls12_381::BlsScalar;

#[cfg(feature = "zk")]
pub(crate) mod gadget;

pub(crate) mod scalar;

pub(crate) trait Arion<T> {
    fn linear_layer(&mut self, state: &mut[T; WIDTH]);
    
    fn affine_layer(&mut self, state: &mut[T; WIDTH], constants_aff: &[BlsScalar; WIDTH]);

    fn gtds(&mut self, state: &mut[T; WIDTH], constants_g: &[BlsScalar; 2 * (WIDTH - 1)], constants_h: &[BlsScalar; WIDTH - 1]);

    fn inverse_sbox(&mut self, state: &mut T);

    fn sbox_layer(&mut self, value: &mut T);

    fn perm(&mut self, state: &mut[T; WIDTH]) {
        self.linear_layer(state);
        self.affine_layer(state, &[BlsScalar::zero(); WIDTH]);
        for round in 0..NUMBER_OF_ROUNDS {
            self.gtds(state, &G[round], &H[round]);
            self.affine_layer(state, &AFFINE[round]);
        }
    }
}