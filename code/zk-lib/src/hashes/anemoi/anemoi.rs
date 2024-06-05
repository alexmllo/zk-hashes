// Implementation of the Anemoi hash funciton and zk circuit

use self::{
    mds::MDS_MATRIX,
    round_constants::{C, D},
    sbox::{ALPHA_INV, BETA, DELTA},
};
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

pub struct Anemoi;

impl Anemoi {
    fn linear_layer<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        let mut x = [F::ZERO; NUM_COLUMNS];
        x.copy_from_slice(&state[..NUM_COLUMNS]);
        let mut y = [F::ZERO; NUM_COLUMNS];
        y.copy_from_slice(&state[NUM_COLUMNS..]);

        // MDS_MATRIX * x
        let mut x_vec = [F::ZERO; NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            for j in 0..NUM_COLUMNS {
                let ct = MDS_MATRIX[i][j];
                let mut temp = state[j].clone();
                temp.mul_assign(F::from_canonical_usize(ct));
                x_vec[i].add_assign(temp);
            }
        }

        // MDS_MATRIX * y
        let mut y_vec = [F::ZERO; NUM_COLUMNS];
        let mut y_rotated = [F::ZERO; NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            if i != NUM_COLUMNS - 1 {
                y_rotated[i] = y[i + 1];
            } else {
                y_rotated[i] = y[0];
            }
        }
        for i in 0..NUM_COLUMNS {
            for j in 0..NUM_COLUMNS {
                let ct = MDS_MATRIX[i][j];
                let mut temp = y_rotated[j].clone();
                temp.mul_assign(F::from_canonical_usize(ct));
                y_vec[i].add_assign(temp);
            }
        }

        // Pseudo-Hadamard transform P
        for i in 0..NUM_COLUMNS {
            y_vec[i] += x_vec[i];
            x_vec[i] += y_vec[i];
        }

        state[..NUM_COLUMNS].copy_from_slice(&x_vec);
        state[NUM_COLUMNS..].copy_from_slice(&y_vec);
    }

    fn sbox_exp_comp<F: RichField + Extendable<2>>(x: F) -> F {
        // QUAD = 2
        F::mul(x, x)
    }

    fn evaluate_sbox<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        for i in 0..NUM_COLUMNS {
            state[i] -= F::from_canonical_usize(BETA) * Self::sbox_exp_comp(state[NUM_COLUMNS + i]);
            state[NUM_COLUMNS + i] -= state[i].exp_u64(ALPHA_INV as u64);
            state[i] += F::from_canonical_usize(BETA) * Self::sbox_exp_comp(state[NUM_COLUMNS + i])
                + F::from_canonical_usize(DELTA);
        }
    }

    fn anemoi_permutation<F: RichField + Extendable<2>>(state: &mut [F; SPONGE_WIDTH]) {
        for j in 0..NUMBER_OF_ROUNDS {
            for i in 0..NUM_COLUMNS {
                state[i] += F::from_canonical_usize(C[j][i]);
                state[NUM_COLUMNS + i] += F::from_canonical_usize(D[j][i]);
            }
            Self::linear_layer(state);
            Self::evaluate_sbox(state);
        }
        // Final call to the linear layer
        Self::linear_layer(state);
    }

    pub fn anemoi_hash<F: RichField + Extendable<2>, const H: usize>(
        input: [F; SPONGE_RATE],
    ) -> Vec<F> {
        assert!(input.len() != 0);

        let mut state = [F::ZERO; SPONGE_WIDTH];

        let sigma = if input.len() % SPONGE_RATE == 0 {
            F::ONE
        } else {
            F::ZERO
        };

        // Absorbing
        let mut i = 0;
        for &element in input.iter() {
            state[i] += element;
            i += 1;
            if i % SPONGE_RATE == 0 {
                Self::anemoi_permutation(&mut state);
                i = 0;
            }
        }

        // We then add sigma to the last capacity register of the capacity.
        state[SPONGE_WIDTH - 1] += sigma;

        // Squeezing
        let mut digest = Vec::new();
        let mut pos = 0;
        while digest.len() < H {
            digest.push(state[pos]);
            pos += 1;

            if pos == SPONGE_RATE {
                pos = 0;
                Self::anemoi_permutation(&mut state);
            }
        }
        digest
    }

    /* ************************************
     *      ZERO-KNOWLEDGE PROOF        *
     ***********************************
     */
    fn linear_layer_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        let mut x = [Target::default(); NUM_COLUMNS];
        x.copy_from_slice(&state[..NUM_COLUMNS]);
        let mut y = [Target::default(); NUM_COLUMNS];
        y.copy_from_slice(&state[NUM_COLUMNS..]);

        // mds_matrix * x
        let mut x_vec = [Target::default(); NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            let mut sum = Target::default();
            for j in 0..NUM_COLUMNS {
                sum = builder.mul_const_add(F::from_canonical_usize(MDS_MATRIX[i][j]), x[j], sum);
            }
            x_vec[i] = sum;
        }

        // mds_matrix * y
        let mut y_vec = [Target::default(); NUM_COLUMNS];
        let mut y_rotated = [Target::default(); NUM_COLUMNS];
        for i in 0..NUM_COLUMNS {
            if i != NUM_COLUMNS - 1 {
                y_rotated[i] = y[i + 1];
            } else {
                y_rotated[i] = y[0];
            }
        }
        for i in 0..NUM_COLUMNS {
            let mut sum = Target::default();
            for j in 0..NUM_COLUMNS {
                sum = builder.mul_const_add(
                    F::from_canonical_usize(MDS_MATRIX[i][j]),
                    y_rotated[j],
                    sum,
                );
            }
            y_vec[i] = sum;
        }

        // Pseudo-Hadamard transform P
        for i in 0..NUM_COLUMNS {
            y_vec[i] = builder.add(y_vec[i], x_vec[i]);
            x_vec[i] = builder.add(x_vec[i], y_vec[i]);
        }

        state[..NUM_COLUMNS].copy_from_slice(&x_vec);
        state[NUM_COLUMNS..].copy_from_slice(&y_vec);
    }

    fn sbox_exp_comp_circuit<F: RichField + Extendable<2>>(
        x: Target,
        builder: &mut CircuitBuilder<F, 2>,
    ) -> Target {
        // QUAD = 2
        builder.square(x)
    }

    fn evaluate_sbox_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        for i in 0..NUM_COLUMNS {
            let exp = Self::sbox_exp_comp_circuit(state[NUM_COLUMNS + i], builder);
            let op1 = builder.mul_const(F::from_canonical_usize(BETA), exp);
            state[i] = builder.sub(state[i], op1);

            //let exp = builder.exp_u64(state[i], ALPHA_INV as u64);
            let exp = builder.exp_inv(state[i]);
            state[NUM_COLUMNS + i] = builder.sub(state[NUM_COLUMNS + i], exp);

            let exp = Self::sbox_exp_comp_circuit(state[NUM_COLUMNS + i], builder);
            let op1 = builder.mul_const_add(F::from_canonical_usize(BETA), exp, state[i]);
            state[i] = builder.add_const(op1, F::from_canonical_usize(DELTA));
        }
    }

    fn anemoi_permutation_circuit<F: RichField + Extendable<2>>(
        state: &mut [Target; SPONGE_WIDTH],
        builder: &mut CircuitBuilder<F, 2>,
    ) {
        for j in 0..NUMBER_OF_ROUNDS {
            for i in 0..NUM_COLUMNS {
                state[i] = builder.add_const(state[i], F::from_canonical_usize(C[j][i]));
                state[NUM_COLUMNS + i] =
                    builder.add_const(state[NUM_COLUMNS + i], F::from_canonical_usize(D[j][i]));
            }
            Self::linear_layer_circuit(state, builder);
            Self::evaluate_sbox_circuit(state, builder);
        }
        // Final call to the linear layer
        Self::linear_layer_circuit(state, builder);
    }

    pub fn circuit_generation<
        F: RichField + Extendable<2>,
        C: GenericConfig<2, F = F>,
        const H: usize,
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
        // Absorbing
        let mut i = 0;
        for &element in input.iter() {
            state[i] = builder.add(state[i], element);
            i += 1;
            if i % SPONGE_RATE == 0 {
                Self::anemoi_permutation_circuit(&mut state, &mut builder);
                i = 0;
            }
        }

        if input.len() % SPONGE_RATE == 0 {
            let sigma = builder.one();
            state[SPONGE_WIDTH - 1] = builder.add(state[SPONGE_WIDTH - 1], sigma);
        } else {
            let sigma = builder.zero();
            state[SPONGE_WIDTH - 1] = builder.add(state[SPONGE_WIDTH - 1], sigma);
        }

        // Squeezing
        let mut digest: Vec<Target> = Vec::new();
        let mut pos = 0;
        while digest.len() < H {
            digest.push(state[pos]);
            pos += 1;

            if pos == SPONGE_RATE {
                pos = 0;
                Self::anemoi_permutation_circuit(&mut state, &mut builder);
            }
        }
        let mut output = [Target::default(); H];
        for (i, &item) in digest.iter().enumerate() {
            output[i] = item;
        }
        builder.register_public_inputs(&output);

        let mut pw = PartialWitness::new();
        pw.set_target_arr(&input, &x);

        let data = builder.build::<C>();

        (data, pw)
    }

    /// Proof generartion for Anemoi hash
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

    /// Verifies the proof of the Anemoi hash
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
mod test {
    use plonky2::{
        field::{goldilocks_field::GoldilocksField, types::Field},
        plonk::config::PoseidonGoldilocksConfig,
    };

    use super::{Anemoi, SPONGE_RATE};

    #[test]
    fn test_anemoi_hash() {
        let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
        for i in 0..SPONGE_RATE {
            input[i] = GoldilocksField(i as u64);
        }

        let output = Anemoi::anemoi_hash::<GoldilocksField, 4>(input.clone());
        println!("{:?}", output);

        // Anemoi circuit
        let (data, pw) = Anemoi::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(
            input.clone(),
        );
        let proof =
            Anemoi::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        Anemoi::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &proof);
    }
}
