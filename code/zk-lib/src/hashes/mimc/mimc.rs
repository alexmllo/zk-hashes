// The MiMC hash function and the MiMC circuit

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;

const ROUNDS: usize = 41;

pub struct MiMC<F> {
    constants: Vec<F>,
}

impl<F> MiMC<F>
where
    F: RichField,
{
    /// Create a new MiMC configuration
    //
    // # Panics
    // Number of rounds must match constants.len
    pub fn new(constants: Vec<F>) -> Self {
        assert_eq!(constants.len(), ROUNDS);
        Self {
            constants,
        }
    }

    /// Creates a new MiMC struct providing random generator
    pub fn new_from_rng() -> Self {
        let cts = F::rand_vec(ROUNDS);
        Self {
            constants: cts,
        }
    }

    /* **********************************
       **********************************
       *     MiMC HASH FUNCTION     *
       **********************************
       **********************************
    */

    /// Performs the MiMC hash permutation
    pub fn permute_rounds(&self, x: [F; 2]) -> F {
        let mut hl = x[0].clone();
        let mut hr = x[1].clone();
        for round in 0..ROUNDS {
            let mut x = self.constants[round].clone() + hl;
            
            x = x.exp_u64(7u64);

            let t = hr + x;

            hr = hl.clone();
            hl = t;
        }
        hl
    }

    /* **********************************
       **********************************
       *     ZERO-KNOWLEDGE PROOFS      *
       **********************************
       **********************************
    */

    /// Generates the circuit for the MiMC hash function
    pub fn circuit_generation<C, const D: usize>(&self, x: [F; 2]) -> (CircuitData<F, C, D>, PartialWitness<F>)
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
    {
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // The arithmetic circuit
        let input: [Target; 2] = builder.add_virtual_targets(2).try_into().unwrap();

        let mut hash = input[0];
        let mut x2 = input[1];
        for i in 0..ROUNDS {
            let temp = builder.add_const(hash, self.constants[i]);
            let temp2 = builder.square(temp);
            let temp4 = builder.square(temp2);
            let temp6 = builder.mul(temp4, temp2);
            let temp7 = builder.mul(temp6, temp);

            let t = builder.add(x2, temp7);

            x2 = hash;
            hash = t;
        }
        builder.register_public_inputs(&input);
        builder.register_public_input(hash);

        // Provide initial values
        let mut pw = PartialWitness::new();
        pw.set_target_arr(&input, &x);

        let data = builder.build::<C>();

        (data, pw)
    }

    /// Generates a proof for the MiMC hash 
    pub fn proof_generation<C, const D: usize>(
        data: &CircuitData<F, C, D>,
        pw: &PartialWitness<F>,
    ) -> ProofWithPublicInputs<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
    {
        let proof = data.prove(pw.clone()).unwrap();
        proof
    }

    /// Verifies the proof of the MiMC hash 
    pub fn proof_verification<C, const D: usize>(
        data: &CircuitData<F, C, D>,
        proof: &ProofWithPublicInputs<F, C, D>,
    ) where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
    {
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
        field::goldilocks_field::GoldilocksField,
        plonk::config::PoseidonGoldilocksConfig,
    };

    use super::*;

    #[test]
    fn permute_mimc() {
        let mimc = MiMC::<GoldilocksField>::new_from_rng();
        let hash = mimc.permute_rounds([GoldilocksField(1), GoldilocksField(2)]);

        println!("Message 1: {}", GoldilocksField(1));
        println!("Message 2: {}", GoldilocksField(2));
        println!("Hash: {}", hash);

        type C = PoseidonGoldilocksConfig;

        let (data, pw) = mimc.circuit_generation::<C, 2>([GoldilocksField(1), GoldilocksField(2)]);
        let proof = MiMC::proof_generation(&data, &pw);
        MiMC::proof_verification(&data, &proof);
    }
}