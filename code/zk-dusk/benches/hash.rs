// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dusk_plonk::prelude::*;
use ff::Field;
use rand::rngs::StdRng;
use rand::SeedableRng;
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

const CAPACITY: usize = 16;

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
        Ok(())
    }
}

// Benchmark for running sponge on 5 BlsScalar, one permutation
fn bench_sponge_hades(c: &mut Criterion) {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (mut prover, mut verifier) =
        Compiler::compile::<HadesSponge>(&pp, label).expect("Circuit should compile successfully");
    let mut proof = Proof::default();
    let message = [
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
    ];
    let public_inputs = HadesHash::digest(Domain::Merkle4, &message);
    let circuit = HadesSponge::new(message, public_inputs[0]);

    // Benchmark sponge native
    c.bench_function("Poseidon hash 4 BlsScalar", |b| {
        b.iter(|| {
            let _ = HadesHash::digest(Domain::Merkle4, black_box(&circuit.message));
        })
    });

    // Benchmark circuit generation
    c.bench_function("Rescue hash circuit generation", |b| {
        b.iter(|| {
            (prover, verifier) = Compiler::compile::<HadesSponge>(&pp, label).expect("Circuit should compile successfully");
        })
    });

    // Benchmark proof creation
    c.bench_function("Poseidon hash 4 BlsScalar proof generation", |b| {
        b.iter(|| {
            (proof, _) = prover
                .prove(&mut rng, black_box(&circuit))
                .expect("Proof generation should succeed");
        })
    });

    // Benchmark proof verification
    c.bench_function("Poseidon hash 4 BlsScalar proof verification", |b| {
        b.iter(|| {
            verifier
                .verify(black_box(&proof), &public_inputs)
                .expect("Proof verification should succeed");
        })
    });
}

// Benchmark for running sponge on 5 BlsScalar, one permutation
fn bench_sponge_rescue(c: &mut Criterion) {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (mut prover, mut verifier) =
        Compiler::compile::<RescueSponge>(&pp, label).expect("Circuit should compile successfully");
    let mut proof = Proof::default();
    let message = [
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
    ];
    let public_inputs = RescueHash::digest(Domain::Merkle4, &message);
    let circuit = RescueSponge::new(message, public_inputs[0]);

    // Benchmark sponge native
    c.bench_function(" Rescue hash 4 BlsScalar", |b| {
        b.iter(|| {
            let _ = RescueHash::digest(Domain::Merkle4, black_box(&circuit.message));
        })
    });

    // Benchmark circuit generation
    c.bench_function("Rescue hash circuit generation", |b| {
        b.iter(|| {
            (prover, verifier) = Compiler::compile::<RescueSponge>(&pp, label)
                .expect("Circuit should compile successfully");
        })
    });

    // Benchmark proof creation
    c.bench_function("Rescue hash proof generation", |b| {
        b.iter(|| {
            (proof, _) = prover
                .prove(&mut rng, black_box(&circuit))
                .expect("Proof generation should succeed");
        })
    });

    // Benchmark proof verification
    c.bench_function("Rescue hash proof verification", |b| {
        b.iter(|| {
            verifier
                .verify(black_box(&proof), &public_inputs)
                .expect("Proof verification should succeed");
        })
    });
}

// Benchmark for running sponge on 5 BlsScalar, one permutation
fn bench_sponge_griffin(c: &mut Criterion) {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (mut prover, mut verifier) = Compiler::compile::<GriffinSponge>(&pp, label)
        .expect("Circuit should compile successfully");
    let mut proof = Proof::default();
    let message = [
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
    ];
    let public_inputs = GriffinHash::digest(Domain::Merkle4, &message);
    let circuit = GriffinSponge::new(message, public_inputs[0]);

    // Benchmark sponge native
    c.bench_function("Griffin hash 4 BlsScalar", |b| {
        b.iter(|| {
            let _ = GriffinHash::digest(Domain::Merkle4, black_box(&circuit.message));
        })
    });

    // Benchmark circuit generation
    c.bench_function("Griffin hash circuit generation", |b| {
        b.iter(|| {
            (prover, verifier) = Compiler::compile::<GriffinSponge>(&pp, label)
                .expect("Circuit should compile successfully");
        })
    });

    // Benchmark proof creation
    c.bench_function("Griffin hash proof generation", |b| {
        b.iter(|| {
            (proof, _) = prover
                .prove(&mut rng, black_box(&circuit))
                .expect("Proof generation should succeed");
        })
    });

    // Benchmark proof verification
    c.bench_function("Griffin hash proof verification", |b| {
        b.iter(|| {
            verifier
                .verify(black_box(&proof), &public_inputs)
                .expect("Proof verification should succeed");
        })
    });
}

// Benchmark for running sponge on 5 BlsScalar, one permutation
fn bench_sponge_anemoi(c: &mut Criterion) {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (mut prover, mut verifier) =
        Compiler::compile::<AnemoiSponge>(&pp, label).expect("Circuit should compile successfully");
    let mut proof = Proof::default();
    let message = [
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
        BlsScalar::random(&mut rng),
    ];
    let public_inputs = AnemoiHash::digest(Domain::Merkle4, &message);
    let circuit = AnemoiSponge::new(message, public_inputs[0]);

    // Benchmark sponge native
    c.bench_function("Anemoi hash 4 BlsScalar", |b| {
        b.iter(|| {
            let _ = AnemoiHash::digest(Domain::Merkle4, black_box(&circuit.message));
        })
    });

    // Benchmark circuit generation
    c.bench_function("Anemoi hash circuit generation", |b| {
        b.iter(|| {
            (prover, verifier) = Compiler::compile::<AnemoiSponge>(&pp, label)
                .expect("Circuit should compile successfully");
        })
    });

    // Benchmark proof creation
    c.bench_function("Anemoi hash proof generation", |b| {
        b.iter(|| {
            (proof, _) = prover
                .prove(&mut rng, black_box(&circuit))
                .expect("Proof generation should succeed");
        })
    });

    // Benchmark proof verification
    c.bench_function("Anemoi hash proof verification", |b| {
        b.iter(|| {
            verifier
                .verify(black_box(&proof), &public_inputs)
                .expect("Proof verification should succeed");
        })
    });
}

// Benchmark for running sponge on 4 BlsScalar, one permutation
fn bench_sponge_arion(c: &mut Criterion) {
    // Prepare benchmarks and initialize variables
    let label = b"sponge benchmark";
    let mut rng = StdRng::seed_from_u64(0xc10d);
    let pp = PublicParameters::setup(1 << CAPACITY, &mut rng).unwrap();
    let (mut prover, mut verifier) =
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
    c.bench_function("Arion hash 4 BlsScalar", |b| {
        b.iter(|| {
            let _ = ArionHash::digest(Domain::Merkle4, black_box(&circuit.message));
        })
    });

    // Benchmark circuit generation
    c.bench_function("Arion hash circuit generation", |b| {
        b.iter(|| {
            (prover, verifier) = Compiler::compile::<ArionSponge>(&pp, label)
                .expect("Circuit should compile successfully");
        })
    });

    // Benchmark proof creation
    c.bench_function("Arion hash proof generation", |b| {
        b.iter(|| {
            (proof, _) = prover
                .prove(&mut rng, black_box(&circuit))
                .expect("Proof generation should succeed");
        })
    });

    // Benchmark proof verification
    c.bench_function("Arion hash proof verification", |b| {
        b.iter(|| {
            verifier
                .verify(black_box(&proof), &public_inputs)
                .expect("Proof verification should succeed");
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_sponge_anemoi, bench_sponge_arion, bench_sponge_griffin, bench_sponge_hades, bench_sponge_rescue
}
criterion_main!(benches);
