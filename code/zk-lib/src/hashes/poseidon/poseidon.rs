// Implementation of the Poseidon hash function

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

use self::{constants::ALL_ROUND_CONSTANTS, mds::MDS_MATRIX};
use super::*;

pub struct Poseidon;

impl Poseidon {
    /* **********************************
     **********************************
     *     POSEIDON HASH FUNCTION     *
     **********************************
     **********************************
     */
    fn constant_layer<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [F; SPONGE_WIDTH],
        round_ctr: usize,
    ) {
        for i in 0..SPONGE_WIDTH {
            // SPONGE_WIDTH * round_ctr calculates the starting index in the ALL_ROUND_CONSTANTS array for the constants corresponding to the current round. Since each round has SPONGE_WIDTH number of constants, multiplying SPONGE_WIDTH by round_ctr gives the starting index for the constants of the current round.
            // Adding i to this starting index allows the function to access the appropriate constant for the current state element state[i]
            let round_constant = ALL_ROUND_CONSTANTS[i + SPONGE_WIDTH * round_ctr];
            state[i] = state[i].add(F::from_canonical_u64(round_constant));
        }
    }

    fn sbox_monomial<F: RichField + Extendable<D>, const D: usize>(x: F) -> F {
        // x |--> x^7
        let x2 = x.square();
        let x4 = x2.square();
        let x3 = x.mul(x2);
        x3.mul(x4)
    }

    fn sbox_layer<F: RichField + Extendable<D>, const D: usize>(state: &mut [F]) {
        for i in 0..SPONGE_WIDTH {
            state[i] = Self::sbox_monomial(state[i]);
        }
    }

    fn mds_layer<F: RichField + Extendable<D>, const D: usize>(
        state: &[F; SPONGE_WIDTH],
    ) -> [F; SPONGE_WIDTH] {
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

    fn full_rounds<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [F; SPONGE_WIDTH],
        round_ctr: &mut usize,
    ) {
        for _ in 0..HALF_N_FULL_ROUNDS {
            Self::constant_layer(state, *round_ctr);
            Self::sbox_layer(state);
            *state = Self::mds_layer(state);
            *round_ctr += 1;
        }
    }

    fn partial_rounds<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [F; SPONGE_WIDTH],
        round_ctr: &mut usize,
    ) {
        for _ in 0..N_PARTIAL_ROUNDS {
            Self::constant_layer(state, *round_ctr);
            state[0] = Self::sbox_monomial(state[0]);
            *state = Self::mds_layer(state);
            *round_ctr += 1;
        }
    }

    pub fn poseidon_hash<F: RichField + Extendable<D>, const D: usize>(
        input: [F; SPONGE_RATE],
    ) -> F {
        let mut round_ctr = 0;
        let mut state = [F::ZERO; SPONGE_WIDTH];
        state[..SPONGE_RATE].copy_from_slice(&input);
        state[SPONGE_RATE..].fill(F::ZERO);

        Self::full_rounds(&mut state, &mut round_ctr);
        Self::partial_rounds(&mut state, &mut round_ctr);
        Self::full_rounds(&mut state, &mut round_ctr);

        state[0]
    }

    /**********************************
     **********************************
     *      ZERO-KNOWLEDGE PROOF      *
     **********************************
     **********************************/
    fn constant_layer_circuit<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [Target; SPONGE_WIDTH],
        round_ctr: usize,
        builder: &mut CircuitBuilder<F, D>,
    ) {
        for i in 0..SPONGE_WIDTH {
            //let round_constant = round_constants[i + SPONGE_WIDTH * round_ctr];
            //state[i] = builder.add(state[i], round_constant);
            state[i] = builder.add_const(
                state[i],
                F::from_canonical_u64(ALL_ROUND_CONSTANTS[i + SPONGE_WIDTH * round_ctr]),
            );
        }
    }

    // x^7
    fn sbox_monomial_cicruit<F: RichField + Extendable<D>, const D: usize>(
        x: Target,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Target {
        let x2 = builder.mul(x, x);
        let x4 = builder.mul(x2, x2);
        let x6 = builder.mul(x4, x2);
        builder.mul(x6, x)
    }

    fn sbox_layer_circuit<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, D>,
    ) {
        for i in 0..SPONGE_WIDTH {
            state[i] = Self::sbox_monomial_cicruit(state[i], builder);
        }
    }

    fn mds_layer_circuit<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, D>,
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

    fn full_rounds_circuit<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [Target; SPONGE_WIDTH],
        round_ctr: &mut usize,
        builder: &mut CircuitBuilder<F, D>,
    ) {
        for _ in 0..HALF_N_FULL_ROUNDS {
            Self::constant_layer_circuit(state, *round_ctr, builder);
            Self::sbox_layer_circuit(state, builder);
            *state = Self::mds_layer_circuit(state, builder);
            *round_ctr += 1;
        }
    }

    fn partial_rounds_circuit<F: RichField + Extendable<D>, const D: usize>(
        state: &mut [Target; SPONGE_WIDTH],
        round_ctr: &mut usize,
        builder: &mut CircuitBuilder<F, D>,
    ) {
        for _ in 0..N_PARTIAL_ROUNDS {
            Self::constant_layer_circuit(state, *round_ctr, builder);
            state[0] = Self::sbox_monomial_cicruit(state[0], builder);
            *state = Self::mds_layer_circuit(state, builder);
            *round_ctr += 1;
        }
    }

    fn poseidon_permutation<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
        round_ctr: &mut usize,
    ) {
        Self::full_rounds_circuit(state, round_ctr, builder);
        Self::partial_rounds_circuit(state, round_ctr, builder);
        Self::full_rounds_circuit(state, round_ctr, builder);
    }

    /// Generates the circuit for the Poseidon hash
    pub fn circuit_generation<
        F: RichField + Extendable<2>,
        C: GenericConfig<2, F = F>,
        const L: usize,
    >(
        x: [F; SPONGE_RATE],
    ) -> (CircuitData<F, C, 2>, PartialWitness<F>) {
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, 2>::new(config);

        let input = builder.add_virtual_target_arr::<SPONGE_RATE>();
        let mut state = builder.add_virtual_target_arr::<SPONGE_WIDTH>();
        let mut round_ctr = 0;

        for i in 0..SPONGE_WIDTH {
            state[i] = Target::default();
        }

        // The arithmetic circuit
        let mut i = 0;
        for &element in input.iter() {
            state[i] = builder.add(state[i], element);
            i += 1;
            if i % SPONGE_RATE == 0 {
                Self::poseidon_permutation(&mut state, &mut builder, &mut round_ctr);
                i = 0;
            }
        }

        let mut output = [Target::default(); L];
        for i in 0..L {
            output[i] = state[i];
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

    use super::*;

    #[test]
    fn poseidon_hash() {
        // 17291601223193097753 - Correct

        let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
        for i in 0..SPONGE_RATE {
            input[i] = GoldilocksField(i as u64);
        }

        let output = Poseidon::poseidon_hash::<GoldilocksField, 2>(input.clone());
        println!("Poseidon hash output: {}", output);

        // Poseidon circuit
        let (data, pw) = Poseidon::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
            input.clone(),
        );
        let proof =
            Poseidon::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        Poseidon::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &proof);
    }
}
