// Implementation of the Arion hash function and zero-knowledge circuit

use arion::constants::{AFFINE_CONSTANTS, G_VALUES, H_VALUES};
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

use super::*;

pub struct Arion;

impl Arion {
    fn mul_matrix<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        let mut w = [F::ZERO; SPONGE_WIDTH];
        let mut sigma = F::ZERO;
        for val in &mut *state {
            sigma += *val;
        }

        let mut sum = F::ZERO;
        for i in 0..SPONGE_WIDTH {
            sum += F::from_canonical_usize(i) * state[i];
        }

        w[0] = sigma + sum;

        let mut i = 1;
        while i < SPONGE_WIDTH {
            w[i] = w[i - 1] - (sigma + F::from_canonical_usize(SPONGE_WIDTH) * state[i - 1]);
            i += 1;
        }

        state.copy_from_slice(&w);
    }

    fn affine_layer<F: RichField + Extendable<2>>(
        state: &mut [F; SPONGE_WIDTH],
        constants_aff: &[u64; SPONGE_WIDTH],
    ) {
        Self::mul_matrix(state);
        let mut inner = [F::ZERO; SPONGE_WIDTH];
        for i in 0..SPONGE_WIDTH {
            inner[i] += state[i] + F::from_canonical_u64(constants_aff[i]);
        }
        state.copy_from_slice(&inner);
    }

    fn s_box<F: RichField + Extendable<2>>(x: &mut F) {
        let x2 = F::square(x);
        let x4 = F::mul(x2, x2);
        let x6 = F::mul(x4, x2);
        *x = F::mul(x6, *x)
    }

    fn gtds<F: RichField + Extendable<2>>(
        state: &mut [F; SPONGE_WIDTH],
        constants_g: &[[u64; 2]; SPONGE_WIDTH - 1],
        constants_h: &[u64; SPONGE_WIDTH - 1],
    ) {
        let mut output = [F::ZERO; SPONGE_WIDTH];
        output.copy_from_slice(state);

        output[SPONGE_WIDTH - 1] = output[SPONGE_WIDTH - 1].exp_u64(E as u64);

        let mut sigma = state[SPONGE_WIDTH - 1].clone();
        sigma += output[SPONGE_WIDTH - 1];

        for i in (0..SPONGE_WIDTH - 1).rev() {
            Self::s_box(&mut output[i]);

            // Evaluate g and h
            // Linear term
            let mut g = sigma.clone();
            g *= F::from_canonical_u64(constants_g[i][0]);
            let mut h = sigma.clone();
            h *= F::from_canonical_u64(constants_h[i]);

            // Quadratic term
            g += sigma.square();
            h += sigma.square();

            // Add constant term
            g += F::from_canonical_u64(constants_g[i][1]);

            // Multiply g and add h
            output[i] *= g;
            output[i] += h;

            sigma += output[i];
            sigma += state[i];
        }
        state.copy_from_slice(&output);
    }

    fn arion_permutation<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        Self::mul_matrix(state);
        Self::affine_layer(state, &[0u64; SPONGE_WIDTH]);
        for r in 0..NUMBER_OF_ROUNDS {
            Self::gtds(state, &G_VALUES[r], &H_VALUES[r]);
            Self::affine_layer(state, &AFFINE_CONSTANTS[r]);
        }
    }

    pub fn arion_hash<F: RichField + Extendable<2>, const L: usize>(
        input: [F; SPONGE_RATE],
    ) -> [F; L] {
        let mut state = [F::ZERO; SPONGE_WIDTH];

        assert!(input.len() > 0);
        assert!(input.len() % SPONGE_RATE == 0);

        let mut absorb_index = 0;
        while absorb_index < input.len() {
            for i in 0..SPONGE_RATE {
                state[i] += input[absorb_index];
                absorb_index += 1;
            }
            Self::arion_permutation(&mut state);
        }

        // Output
        let mut output = [F::ZERO; L];
        for i in 0..L {
            output[i] = state[i].clone();
        }

        output
    }

    /* ************************************
     *      ZERO-KNOWLEDGE PROOF        *
     ***********************************
     */

    fn mul_matrix_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        let mut w = [Target::default(); SPONGE_WIDTH];
        let mut sigma = Target::default();
        for val in &mut *state {
            sigma = builder.add(sigma, *val);
        }

        let mut sum = Target::default();
        for i in 0..SPONGE_WIDTH {
            sum = builder.mul_const_add(F::from_canonical_usize(i), state[i], sum);
        }
        w[0] = builder.add(sigma, sum);

        let mut i = 1;
        while i < SPONGE_WIDTH {
            let op1 =
                builder.mul_const_add(F::from_canonical_usize(SPONGE_WIDTH), state[i - 1], sigma);
            w[i] = builder.sub(w[i - 1], op1);
            i += 1;
        }

        state.copy_from_slice(&w);
    }

    fn affine_layer_circit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        constants_aff: &[u64; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        Self::mul_matrix_circuit(state, builder);
        let mut inner = [Target::default(); SPONGE_WIDTH];
        for i in 0..SPONGE_WIDTH {
            let op1 = builder.add_const(state[i], F::from_canonical_u64(constants_aff[i]));
            inner[i] = builder.add(inner[i], op1);
        }
        state.copy_from_slice(&inner);
    }

    fn s_box_circuit<F: RichField + Extendable<2>>(
        x: &mut Target,
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        //*x = builder.exp_u64(*x, D_1 as u64);
        let x2 = builder.mul(*x, *x);
        let x4 = builder.mul(x2, x2);
        let x6 = builder.mul(x4, x2);
        *x = builder.mul(x6, *x);
    }

    fn gtds_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
        constants_g: &[[u64; 2]; SPONGE_WIDTH - 1],
        constants_h: &[u64; SPONGE_WIDTH - 1],
    ) {
        let mut output = [Target::default(); SPONGE_WIDTH];
        output.copy_from_slice(state);

        //output[SPONGE_WIDTH - 1] = builder.exp_u64(output[SPONGE_WIDTH - 1], E as u64);
        output[SPONGE_WIDTH - 1] = builder.exp_inv(output[SPONGE_WIDTH - 1]);

        let mut sigma = state[SPONGE_WIDTH - 1].clone();
        sigma = builder.add(sigma, output[SPONGE_WIDTH - 1]);

        let mut pos = 20;
        for i in (0..(SPONGE_WIDTH - 1)).rev() {
            Self::s_box_circuit(&mut output[i], builder);

            // Evaluate h and g
            let tmp = builder.square(sigma);

            let g_op1 = builder.mul_const_add(F::from_canonical_u64(constants_g[i][0]), sigma, tmp);
            let g = builder.add_const(g_op1, F::from_canonical_u64(constants_g[i][1]));

            let h = builder.mul_const_add(F::from_canonical_u64(constants_h[i]), sigma, tmp);

            //Multiply g and add h
            output[i] = builder.mul_add(output[i], g, h);

            sigma = builder.add(sigma, output[i]);
            sigma = builder.add(sigma, state[i]);

            if pos > 0 {
                pos -= 2;
            }
        }

        state.copy_from_slice(&output);
    }

    fn arion_permutation_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        Self::mul_matrix_circuit(state, builder);
        Self::affine_layer_circit(state, &[0u64; SPONGE_WIDTH], builder);
        for r in 0..NUMBER_OF_ROUNDS {
            Self::gtds_circuit(state, builder, &G_VALUES[r], &H_VALUES[r]);
            Self::affine_layer_circit(state, &AFFINE_CONSTANTS[r], builder);
        }
    }

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

        for i in 0..SPONGE_WIDTH {
            state[i] = Target::default();
        }

        // The arithmetic circuit
        let mut absorb_index = 0;
        while absorb_index < input.len() {
            for i in 0..SPONGE_RATE {
                state[i] = builder.add(state[i], input[absorb_index]);
                absorb_index += 1;
            }
            Self::arion_permutation_circuit(&mut state, &mut builder);
        }

        let mut output = [Target::default(); L];
        for i in 0..L {
            output[i] = state[i];
        }
        builder.register_public_inputs(&output);

        let mut pw = PartialWitness::new();
        pw.set_target_arr(&input, &x);

        /* println!("{}", builder.num_gates()); */

        let data = builder.build::<C>();
        /* println!("{}", data.common.gates.len());
        println!("{:?}", data.common.gates); */

        (data, pw)
    }

    /// Proof generartion for Arion hash
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

    /// Verifies the proof of the Arion hash
    pub fn proof_verification<
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
        const D: usize,
    >(
        data: &CircuitData<F, C, D>,
        proof: &ProofWithPublicInputs<F, C, D>,
    ) {
        /* for i in 0..4 {
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

    use super::{Arion, SPONGE_RATE};

    #[test]
    fn arion_test() {
        let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
        for i in 0..SPONGE_RATE {
            input[i] = GoldilocksField(i as u64);
        }

        // Rescue hash
        let output = Arion::arion_hash::<GoldilocksField, 4>(input.clone());
        println!("Arion output");
        for i in 0..output.len() {
            println!("Hash output {}: {}", i, output[i]);
        }

        // Arion circuit
        let (data, pw) =
            Arion::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, SPONGE_RATE>(
                input.clone(),
            );
        let proof =
            Arion::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        Arion::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &proof);
    }
}
