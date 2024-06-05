

use std::{fs, io::Write, marker::PhantomData};
use ff::Field;
use rand::{rngs::StdRng, SeedableRng};
use sha2::{Digest, Sha512};

use dusk_plonk::prelude::*;

use zk_dusk::{anemoi, arion, griffin, hades, rescue, Domain, Hash, HashGadget, HashableGadget};

type HadesHash<'a> = Hash<'a, hades::ScalarPermutation, { hades::WIDTH }>;

type HadesSponge<'a, 'b> = SpongeCircuit<
    HashGadget<'a, hades::GadgetPermutation<'b>, { hades::WIDTH }>,
    { hades::WIDTH - 4 },
>;

type RescueHash<'a> = Hash<'a, rescue::ScalarPermutation, { rescue::WIDTH }>;

type RescueSponge<'a, 'b> = SpongeCircuit<
    HashGadget<'a, rescue::GadgetPermutation<'b>, { rescue::WIDTH }>,
    { rescue::WIDTH - 4 },
>;

type GriffinHash<'a> = Hash<'a, griffin::ScalarPermutation, { griffin::WIDTH }>;

type GriffinSponge<'a, 'b> = SpongeCircuit<
    HashGadget<'a, griffin::GadgetPermutation<'b>, { griffin::WIDTH }>,
    { griffin::WIDTH - 4 },
>;

type AnemoiHash<'a> = Hash<'a, anemoi::ScalarPermutation, { anemoi::WIDTH }>;

type AnemoiSponge<'a, 'b> = SpongeCircuit<
    HashGadget<'a, anemoi::GadgetPermutation<'b>, { anemoi::WIDTH }>,
    { anemoi::WIDTH - 4 },
>;

type ArionHash<'a> = Hash<'a, arion::ScalarPermutation, { arion::WIDTH }>;

type ArionSponge<'a, 'b> = SpongeCircuit<
    HashGadget<'a, arion::GadgetPermutation<'b>, { arion::WIDTH }>,
    { arion::WIDTH - 4 },
>;

const CAPACITY: usize = 14;

struct SpongeCircuit<GadHash, const RATE: usize> {
    message: [BlsScalar; RATE],
    output: BlsScalar,
    phantom: PhantomData<GadHash>,
}

impl<GadHash: HashableGadget, const RATE: usize> SpongeCircuit<GadHash, RATE> {
    pub fn new(message: [BlsScalar; RATE], output: BlsScalar) -> Self {
        SpongeCircuit {
            message,
            output,
            phantom: PhantomData,
        }
    }
}

impl<GadHash: HashableGadget, const RATE: usize> Default for SpongeCircuit<GadHash, RATE> {
    fn default() -> Self {
        SpongeCircuit {
            message: [BlsScalar::default(); RATE],
            output: BlsScalar::default(),
            phantom: PhantomData,
        }
    }
}

impl<GadHash: HashableGadget, const RATE: usize> Circuit for SpongeCircuit<GadHash, RATE> {
    fn circuit(&self, composer: &mut Composer) -> Result<(), Error> {
        let mut w_message = [Composer::ZERO; RATE];
        w_message
            .iter_mut()
            .zip(self.message)
            .for_each(|(witness, scalar)| {
                *witness = composer.append_witness(scalar);
            });

        let output_witness = GadHash::digest(Domain::Merkle4, composer, &w_message);
        composer.assert_equal_constant(output_witness[0], 0, Some(self.output));
        println!("{}", composer.constraints());
        Ok(())
    }
}


// Benchmark for running sponge on 5 BlsScalar, one permutation
fn main() {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (prover, verifier) =
        Compiler::compile::<ArionSponge>(&pp, label).expect("Circuit should compile successfully");
    let mut proof = Proof::default();
    let message = [
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
    ];
    let public_inputs = ArionHash::digest(Domain::Merkle4, &message);
    let circuit = ArionSponge::new(message, public_inputs[0]);

    // Benchmark sponge native
    let _ = ArionHash::digest(Domain::Merkle4, &circuit.message);
 
    // Benchmark proof creation
    (proof, _) = prover
        .prove(&mut rng, &circuit)
        .expect("Proof generation should succeed");

    // Benchmark proof verification
    verifier
        .verify(&proof, &public_inputs)
        .expect("Proof verification should succeed");
}

const WIDTH: usize = hades::WIDTH;

fn mds() -> [[BlsScalar; WIDTH]; WIDTH] {
    let mut matrix = [[BlsScalar::zero(); WIDTH]; WIDTH];
    let mut xs = [BlsScalar::zero(); WIDTH];
    let mut ys = [BlsScalar::zero(); WIDTH];

    // Generate x and y values deterministically for the cauchy matrix, where
    // `x[i] != y[i]` to allow the values to be inverted and there are no
    // duplicates in the x vector or y vector, so that the determinant is always
    // non-zero.
    // [a b]
    // [c d]
    // det(M) = (ad - bc) ; if a == b and c == d => det(M) = 0
    // For an MDS matrix, every possible mxm submatrix, must have det(M) != 0
    (0..WIDTH).for_each(|i| {
        xs[i] = BlsScalar::from(i as u64);
        ys[i] = BlsScalar::from((i + WIDTH) as u64);
    });

    let mut m = 0;
    (0..WIDTH).for_each(|i| {
        (0..WIDTH).for_each(|j| {
            matrix[m][j] = (xs[i] + ys[j]).invert().unwrap();
        });
        m += 1;
    });

    matrix
}

fn write_mds() -> std::io::Result<()> {
    let filename = "mds_4.bin";
    let mut buf: Vec<u8> = vec![];

    mds().iter().for_each(|row| {
        row.iter().for_each(|c| {
            c.internal_repr()
                .iter()
                .for_each(|r| buf.extend_from_slice(&(*r).to_le_bytes()));
        });
    });

    let mut file = fs::File::create(filename)?;
    file.write_all(&buf)?;
    Ok(())
}


const CONSTANTS: usize = (57 + 8) * 8;

fn constants() -> [BlsScalar; CONSTANTS] {
    let mut cnst = [BlsScalar::zero(); CONSTANTS];
    let mut p = BlsScalar::one();
    let mut bytes = b"poseidon-for-plonk".to_vec();

    cnst.iter_mut().for_each(|c| {
        let mut hasher = Sha512::new();
        hasher.update(bytes.as_slice());
        bytes = hasher.finalize().to_vec();

        let mut v = [0x00u8; 64];
        v.copy_from_slice(&bytes[0..64]);

        *c = BlsScalar::from_bytes_wide(&v) + p;
        p = *c;
    });

    cnst
}

fn write_constants() -> std::io::Result<()> {
    let filename = "arc_4.bin";
    let mut buf: Vec<u8> = vec![];

    constants().iter().for_each(|c| {
        c.internal_repr()
            .iter()
            .for_each(|r| buf.extend_from_slice(&(*r).to_le_bytes()));
    });

    let mut file = fs::File::create(filename)?;
    file.write_all(&buf)?;
    Ok(())
}