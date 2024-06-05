use crate::griffin::{NUMBER_OF_ROUNDS, WIDTH};

#[cfg(feature = "zk")]
pub(crate) mod gadget;

pub(crate) mod scalar;

pub(crate) trait Griffin<T> {
    fn non_liner_layer(&mut self, state: &mut[T; WIDTH]);

    fn linear_layer(&mut self, state: &mut[T; WIDTH]);

    fn add_round_constants_leyer(&mut self, state: &mut[T; WIDTH], round: usize);

    fn sbox_layer(&mut self, value: &mut T);

    fn inverse_sbox(&mut self, value: &mut T);

    fn perm(&mut self, state: &mut[T; WIDTH]) {
        for round in 0..(NUMBER_OF_ROUNDS - 1) {
            self.non_liner_layer(state);
            self.linear_layer(state);
            self.add_round_constants_leyer(state, round);
        }
        self.non_liner_layer(state);
        self.linear_layer(state);
    }
}