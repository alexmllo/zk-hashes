use crate::rescue::{NUMBER_OF_ROUNDS, WIDTH};

#[cfg(feature = "zk")]
pub(crate) mod gadget;

pub(crate) mod scalar;

pub(crate) trait Rescue<T> {
    fn add_round_constants_leyer_1(
        &mut self,
        round: usize,
        state: &mut [T; WIDTH],
    );
    
    fn add_round_constants_leyer_2(
        &mut self,
        round: usize,
        state: &mut [T; WIDTH],
    );

    fn sbox_layer(&mut self, value: &mut T);

    fn mds_leyer(&mut self, state: &mut [T; WIDTH]);

    fn inverse_sbox(&mut self, value: &mut T);

    fn perm(&mut self, state: &mut [T; WIDTH]) {
        for round in 0..NUMBER_OF_ROUNDS {
            // Sbox
            state.iter_mut().for_each(|w| self.sbox_layer(w));

            // MDS
            self.mds_leyer(state);

            // Add round constant
            self.add_round_constants_leyer_1(round, state);

            // Inverse Sbox
            state.iter_mut().for_each(|w| self.inverse_sbox(w));

            // MDS
            self.mds_leyer(state);

            // Add round constant
            self.add_round_constants_leyer_2(round, state);
        }
    }
}
