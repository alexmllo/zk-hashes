use crate::anemoi::{NUMBER_OF_ROUNDS, WIDTH, NUM_COLUMNS};

#[cfg(feature = "zk")]
pub(crate) mod gadget;

pub(crate) mod scalar;

pub(crate) trait Anemoi<T> {
    fn linear_layer(&mut self, state: &mut[T; WIDTH]);

    fn add_round_constants_leyer(&mut self, state: &mut[T; WIDTH], round: usize, column: usize);

    fn evaluate_sbox_layer(&mut self, state: &mut[T; WIDTH]);
    
    fn inverse_sbox(&mut self, value: &mut T);

    fn perm(&mut self, state: &mut[T; WIDTH]) {
        for round in 0..NUMBER_OF_ROUNDS {
            for column in 0..NUM_COLUMNS {
                self.add_round_constants_leyer(state, round, column);
            }
            self.linear_layer(state);
            self.evaluate_sbox_layer(state);
        }
        self.linear_layer(state);
    }
}