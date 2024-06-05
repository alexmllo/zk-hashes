// Implementation of the Rescue-prime optimized hash function

use plonky2::{
    field::extension::Extendable,
    hash::hash_types::RichField,
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CircuitData},
        config::GenericConfig,
        proof::ProofWithPublicInputs,
    },
};

use self::{constants::ROUND_CONSTANTS, mds::MDS_MATRIX};
use super::*;

pub struct Rescue;

impl Rescue {
    /* ************************
     *    HASH FUNCTION     *
     ************************ */

    fn mds_layer<F: RichField + Extendable<2>>(state: &[F; SPONGE_WIDTH]) -> [F; SPONGE_WIDTH] {
        let mut new_state: [F; SPONGE_WIDTH] = [F::ZERO; SPONGE_WIDTH];
        for i in 0..SPONGE_WIDTH {
            for j in 0..SPONGE_WIDTH {
                let ct = MDS_MATRIX[i][j];
                let mut temp = state[j].clone();
                temp.mul_assign(F::from_canonical_u64(ct));
                new_state[i].add_assign(temp)
            }
        }
        new_state
    }

    fn rescue_permutation<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        for i in 0..NUMBER_OF_ROUNDS {
            // MDS
            *state = Self::mds_layer(state);

            // Constants
            for j in 0..SPONGE_WIDTH {
                state[j] += F::from_canonical_u64(ROUND_CONSTANTS[i * 2 * SPONGE_WIDTH + j]);
            }
            // S-Box
            for j in 0..SPONGE_WIDTH {
                state[j] = state[j].exp_u64(ALPHA as u64);
            }

            // MDS
            *state = Self::mds_layer(state);

            // Constants
            for j in 0..SPONGE_WIDTH {
                state[j] +=
                    F::from_canonical_u64(ROUND_CONSTANTS[i * 2 * SPONGE_WIDTH + SPONGE_WIDTH + j]);
            }
            // Inverse S-Box
            for j in 0..SPONGE_WIDTH {
                state[j] = state[j].exp_u64(ALPHA_INV as u64);
            }
        }
    }

    pub fn rescue_hash<F: RichField + Extendable<2>, const L: usize>(
        input: [F; L],
    ) -> [F; SPONGE_RATE / 2] {
        assert!(input.len() > 0);
        assert!(input.len() % SPONGE_RATE == 0);

        let mut state = [F::ZERO; SPONGE_WIDTH];

        let mut counter = 0;
        while counter < input.len() {
            for i in 0..SPONGE_RATE {
                state[i + SPONGE_CAPACITY] = input[counter]; // Overwrite mode
                counter += 1;
            }
            // state[SPONGE_RATE..].copy_from_slice(&input[..SPONGE_RATE]);
            Self::rescue_permutation(&mut state);
        }

        let mut output = [F::ZERO; SPONGE_RATE / 2];
        for i in SPONGE_CAPACITY..(SPONGE_CAPACITY + SPONGE_RATE / 2) {
            output[i - SPONGE_CAPACITY] = state[i]
        }
        output
    }

    /* *********************************
     *********************************
     *    ZERO-KNOWLEDGE CIRCUIT     *
     *********************************
     ********************************* */

    fn mds_layer_circuit<F: RichField + Extendable<2>>(
        state: &[Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) -> [Target; SPONGE_WIDTH] {
        let mut new_state = [Target::default(); SPONGE_WIDTH];
        for i in 0..12 {
            let mut sum = Target::default();
            for j in 0..12 {
                sum = builder.mul_const_add(F::from_canonical_u64(MDS_MATRIX[i][j]), state[j], sum);
            }
            new_state[i] = sum;
        }
        new_state
    }

    fn rescue_permutation_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        for i in 0..NUMBER_OF_ROUNDS {
            // MDS
            *state = Self::mds_layer_circuit(state, builder);

            // Constants
            for j in 0..SPONGE_WIDTH {
                state[j] = builder.add_const(
                    state[j],
                    F::from_canonical_u64(ROUND_CONSTANTS[i * 2 * SPONGE_WIDTH + j]),
                );
            }

            // S-box
            for j in 0..SPONGE_WIDTH {
                //state[j] = builder.exp_u64(state[j], ALPHA as u64);
                let x2 = builder.mul(state[j], state[j]);
                let x4 = builder.mul(x2, x2);
                let x6 = builder.mul(x4, x2);
                state[j] = builder.mul(x6, state[j]);
            }

            // MDS
            *state = Self::mds_layer_circuit(state, builder);

            // Constants
            for j in 0..SPONGE_WIDTH {
                state[j] = builder.add_const(
                    state[j],
                    F::from_canonical_u64(ROUND_CONSTANTS[i * 2 * SPONGE_WIDTH + SPONGE_WIDTH + j]),
                );
            }

            // Inverse S-box
            for j in 0..SPONGE_WIDTH {
                //state[j] = builder.exp_u64(state[j], ALPHA_INV as u64);
                state[j] = builder.exp_inv(state[j]);
            }
        }
    }

    pub fn circuit_generation<
        F: RichField + Extendable<2>,
        C: GenericConfig<2, F = F>,
        const L: usize,
    >(
        x: [F; L],
    ) -> (CircuitData<F, C, 2>, PartialWitness<F>) {
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, 2>::new(config);

        let input = builder.add_virtual_target_arr::<L>();
        let mut state = builder.add_virtual_target_arr::<SPONGE_WIDTH>();
        let mut output = [Target::default(); SPONGE_RATE / 2];

        for i in 0..state.len() {
            state[i] = Target::default();
        }

        // The arithmetic circuit
        let mut overwrite_index = 0;
        while overwrite_index < input.len() {
            for i in 0..SPONGE_RATE {
                state[i + SPONGE_CAPACITY] = input[overwrite_index];
                overwrite_index += 1;
            }
            Self::rescue_permutation_circuit(&mut state, &mut builder);
        }

        for i in SPONGE_CAPACITY..(SPONGE_CAPACITY + SPONGE_RATE / 2) {
            output[i - SPONGE_CAPACITY] = state[i]
        }

        builder.register_public_inputs(&output);

        // Provide initial values
        let mut pw = PartialWitness::new();
        pw.set_target_arr(&input, &x);

        let data = builder.build::<C>();

        (data, pw)
    }

    /// Proof generation for the Poseidon hash
    pub fn proof_generation<
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
        const D: usize,
    >(
        data: &CircuitData<F, C, D>,
        pw: &PartialWitness<F>,
    ) -> ProofWithPublicInputs<F, C, D> {
        let proof = data.prove(pw.clone()).unwrap();
        proof
    }

    /// Verifies the proof of the Poseidon hash
    pub fn proof_verification<
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
        const D: usize,
    >(
        data: &CircuitData<F, C, D>,
        proof: &ProofWithPublicInputs<F, C, D>,
    ) {
        /* for i in 0..proof.public_inputs.len() {
            print!("{} ", proof.public_inputs[i]);
        }
        println!(); */
        let _ = data.verify(proof.clone());
    }
}

#[cfg(test)]
mod tests {
    use plonky2::{
        field::{goldilocks_field::GoldilocksField, types::Field},
        plonk::config::PoseidonGoldilocksConfig,
    };

    use super::{Rescue, SPONGE_RATE};

    #[test]
    fn rescue_test() {
        let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
        for i in 0..SPONGE_RATE {
            input[i] = GoldilocksField(i as u64);
        }

        // Rescue hash
        let output = Rescue::rescue_hash(input.clone());
        println!("Rescue-prime output");
        for i in 0..output.len() {
            println!("Hash output {}: {}", i, output[i]);
        }

        // Rescue circuit
        let (data, pw) =
            Rescue::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, SPONGE_RATE>(
                input.clone(),
            );
        let proof =
            Rescue::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        Rescue::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &proof);
    }
}
