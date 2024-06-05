// Implementation of the Griffin hash function

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

use self::{
    constants::{ALPHAS, BETAS, ROUND_CONSTANTS},
    mds::MDS_MATRIX,
};
use super::*;

pub struct Griffin;

impl Griffin {
    /* ******************************************
     * INTERNAL GRIFFIN PERMUTATION FUNCITONS *
     ****************************************** */

    // For D = 7
    fn sbox<F: RichField + Extendable<2>>(x: F) -> F {
        let x2 = F::square(&x);
        let x4 = F::mul(x2, x2);
        let x6 = F::mul(x4, x2);
        F::mul(x6, x)
    }

    fn non_linear_layer<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        fn li<F: RichField + Extendable<2>>(z0: &F, z1: &F, z2: &F, i: usize) -> F {
            let prod1 = z0.mul(F::from_canonical_usize(i - 1));
            let prod2 = prod1.add(*z1);
            let prod3 = prod2.add(*z2);
            prod3
        }

        state[0] = state[0].exp_u64(D_INV as u64);
        state[1] = Self::sbox(state[1]);

        let mut l = li(&state[0], &state[1], &F::ZERO, 2);

        state[2] = state[2]
            * (F::square(&l)
                + F::from_canonical_usize(ALPHAS[0]) * l
                + F::from_canonical_usize(BETAS[0]));

        for i in 3..SPONGE_WIDTH {
            l = li(&state[0], &state[1], &state[i - 1], i);
            state[i] = state[i]
                * (F::square(&l)
                    + F::from_canonical_usize(ALPHAS[i - 2]) * l
                    + F::from_canonical_usize(BETAS[i - 2]));
        }
    }

    fn linear_layer<F: RichField + Extendable<2>>(state: &[F; SPONGE_WIDTH]) -> [F; SPONGE_WIDTH] {
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

    fn additive_constants_layer<F: RichField + Extendable<2>>(
        state: &mut [F; SPONGE_WIDTH],
        round: usize,
    ) {
        for j in 0..SPONGE_WIDTH {
            state[j] += F::from_canonical_u64(ROUND_CONSTANTS[round * SPONGE_WIDTH + j] as u64);
        }
    }

    /* ****************************************
     * GRIFFIN HASH AND COMPRESION FUNCTION *
     **************************************** */

    fn griffin_permutation<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        for i in 0..(NUMBER_OF_ROUNDS - 1) {
            Self::non_linear_layer(state);
            *state = Self::linear_layer(state);
            Self::additive_constants_layer(state, i);
        }

        // Last round, round constants are not added to the state
        Self::non_linear_layer(state);
        *state = Self::linear_layer(state);
    }

    pub fn griffin_sponge<F: RichField + Extendable<2>, const L: usize>(
        input: [F; SPONGE_RATE],
    ) -> [F; L] {
        let mut state = [F::ZERO; SPONGE_WIDTH];

        assert!(input.len() > 0);
        assert!(input.len() % SPONGE_RATE == 0);

        // Absorbing
        let mut absorb_index = 0;
        while absorb_index < input.len() {
            for i in 0..SPONGE_RATE {
                state[i] += input[absorb_index];
                absorb_index += 1;
            }
            Self::griffin_permutation(&mut state);
        }

        // Squeezing
        let mut output = [F::ZERO; SPONGE_RATE];
        let mut squeeze_index = 0;
        while squeeze_index < L {
            for i in 0..SPONGE_RATE {
                output[i] = state[i];
                squeeze_index += 1;
            }
            if squeeze_index < L {
                Self::griffin_permutation(&mut state);
            }
        }

        let mut hashes = [F::ZERO; L];
        for i in 0..L {
            hashes[i] = output[i];
        }
        hashes
    }

    /* *******************************
     *   ZERO-KNOWLEDGE CIRCUIT    *
     ******************************* */

    fn sbox_circuit<F: RichField + Extendable<2>>(
        x: Target,
        builder: &mut CircuitBuilder<F, 2>,
    ) -> Target {
        let x2 = builder.mul(x, x);
        let x4 = builder.mul(x2, x2);
        let x6 = builder.mul(x4, x2);
        builder.mul(x6, x)
    }

    fn non_linear_layer_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        fn li<F: RichField + Extendable<2>>(
            builder: &mut CircuitBuilder<F, 2>,
            z0: &Target,
            z1: &Target,
            z2: &Target,
            i: usize,
        ) -> Target {
            let first_op = builder.mul_const_add(F::from_canonical_usize(i - 1), *z0, *z1);
            builder.add(first_op, *z2)
        }

        state[0] = builder.exp_inv(state[0]);
        state[1] = Self::sbox_circuit(state[1], builder);

        let mut l = li(builder, &state[0], &state[1], &Target::default(), 2);

        let exp = builder.square(l);
        let op1 = builder.mul_const_add(F::from_canonical_usize(ALPHAS[0]), l, exp);
        let op2 = builder.add_const(op1, F::from_canonical_usize(BETAS[0]));
        state[2] = builder.mul(state[2], op2);

        for i in 3..SPONGE_WIDTH {
            l = li(builder, &state[0], &state[1], &state[i - 1], i);
            let exp = builder.square(l);
            let op1 = builder.mul_const_add(F::from_canonical_usize(ALPHAS[i - 2]), l, exp);
            let op2 = builder.add_const(op1, F::from_canonical_usize(BETAS[i - 2]));
            state[i] = builder.mul(state[i], op2);
        }
    }

    fn linear_layer_circuit<F: RichField + Extendable<2>>(
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

    fn additive_constants_layer_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
        round: usize,
    ) {
        for j in 0..SPONGE_WIDTH {
            state[j] = builder.add_const(
                state[j],
                F::from_canonical_usize(ROUND_CONSTANTS[round * SPONGE_WIDTH + j]),
            );
        }
    }

    fn griffin_permutation_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        for i in 0..(NUMBER_OF_ROUNDS - 1) {
            Self::non_linear_layer_circuit(state, builder);
            *state = Self::linear_layer_circuit(state, builder);
            Self::additive_constants_layer_circuit(state, builder, i);
        }

        Self::non_linear_layer_circuit(state, builder);
        *state = Self::linear_layer_circuit(state, builder);
    }

    /// Circuit generation for the Griffin hash
    pub fn circuit_generation<
        F: RichField + Extendable<2>,
        C: GenericConfig<2, F = F>,
        const L: usize,
    >(
        x: [F; SPONGE_RATE],
    ) -> (CircuitData<F, C, 2>, PartialWitness<F>) {
        assert!(x.len() > 0);
        assert!(x.len() % SPONGE_RATE == 0);

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, 2>::new(config);
        let input = builder.add_virtual_target_arr::<SPONGE_RATE>();
        let mut state = builder.add_virtual_target_arr::<SPONGE_WIDTH>();

        for i in 0..SPONGE_WIDTH {
            state[i] = Target::default();
        }

        // The arithmetic circuit
        // Absorbing
        let mut absorb_index = 0;
        while absorb_index < x.len() {
            for i in 0..SPONGE_RATE {
                state[i] = builder.add(state[i], input[absorb_index]);
                absorb_index += 1;
            }
            Self::griffin_permutation_circuit(&mut state, &mut builder);
        }

        let mut output = [Target::default(); SPONGE_RATE];
        // Squeezing
        let mut squeeze_index = 0;
        while squeeze_index < L {
            for i in 0..SPONGE_RATE {
                output[i] = state[i];
                squeeze_index += 1;
            }
            if squeeze_index < L {
                Self::griffin_permutation_circuit(&mut state, &mut builder);
            }
        }

        let mut hashes = [Target::default(); L];
        for i in 0..L {
            hashes[i] = output[i];
        }

        builder.register_public_inputs(&hashes);

        // Provide initial values
        let mut pw = PartialWitness::new();
        pw.set_target_arr(&input, &x);

        let data = builder.build::<C>();

        (data, pw)
    }

    /// Proof generartion for Griffin hash
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

    /// Verifies the proof of the Griffin hash
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
mod test {
    use plonky2::{
        field::{goldilocks_field::GoldilocksField, types::Field},
        plonk::config::PoseidonGoldilocksConfig,
    };

    use super::*;

    #[test]
    fn test_griffin_hash() {
        let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
        for i in 0..SPONGE_RATE {
            input[i] = GoldilocksField(i as u64);
        }

        let output = Griffin::griffin_sponge::<GoldilocksField, 4>(input.clone());
        for i in 0..output.len() {
            println!("Output {}: {}", i, output[i]);
        }

        // Griffin circuit
        let (data, pw) = Griffin::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(
            input.clone(),
        );
        let proof =
            Griffin::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        Griffin::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &proof);
    }
}
