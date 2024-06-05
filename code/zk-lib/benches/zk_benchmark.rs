use criterion::{criterion_group, criterion_main, Criterion};

use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    plonk::config::PoseidonGoldilocksConfig,
};
use zk_lib::hashes::{
    anemoi::{anemoi::Anemoi, SPONGE_RATE as SPONGE_RATE_ANE},
    arion::{arion::Arion, SPONGE_RATE as SPONGE_RATE_ARI},
    griffin::{griffin::Griffin, SPONGE_RATE as SPONGE_RATE_GRI},
    mimc::mimc::MiMC,
    poseidon::{poseidon::Poseidon, SPONGE_RATE as SPONGE_RATE_POS},
    rescue_prime::{rescue_prime::Rescue, SPONGE_RATE as SPONGE_RATE_RESC},
};

fn mimc(c: &mut Criterion) {
    bench_mimc(c);
}

fn poseidon(c: &mut Criterion) {
    bench_poseidon(c);
}

fn rescue(c: &mut Criterion) {
    bench_rescue(c);
}

fn griffin(c: &mut Criterion) {
    bench_griffin(c);
}

fn anemoi(c: &mut Criterion) {
    bench_anemoi(c);
}

fn arion(c: &mut Criterion) {
    bench_arion(c);
}

fn bench_mimc(c: &mut Criterion) {
    let mimc = MiMC::<GoldilocksField>::new_from_rng();
    let init_value = [GoldilocksField(1), GoldilocksField(2)];
    let (mut data, mut pw) = mimc.circuit_generation::<PoseidonGoldilocksConfig, 2>(init_value);
    let mut proof =
        MiMC::<GoldilocksField>::proof_generation::<PoseidonGoldilocksConfig, 2>(&data, &pw);

    // Benchmark circuit generation
    c.bench_function("circuit_generation_mimc", |b| {
        b.iter(|| {
            (data, pw) = mimc.circuit_generation::<PoseidonGoldilocksConfig, 2>(init_value);
        })
    });

    // Benchmark proof creation
    c.bench_function("proof_generation_mimc", |b| {
        b.iter(|| {
            proof = MiMC::<GoldilocksField>::proof_generation::<PoseidonGoldilocksConfig, 2>(
                &data, &pw,
            );
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_mimc", |b| {
        b.iter(|| {
            MiMC::<GoldilocksField>::proof_verification::<PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

fn bench_poseidon(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_POS];
    for i in 0..SPONGE_RATE_POS {
        input[i] = GoldilocksField(i as u64);
    }
    let (mut data, mut pw) =
        Poseidon::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(input);
    let mut proof =
        Poseidon::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
    // Benchmark circuit generation
    c.bench_function("circuit_generation_poseidon", |b| {
        b.iter(|| {
            (data, pw) =
                Poseidon::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(input);
        })
    });

    // Benchmark proof generation
    c.bench_function("proof_generation_poseidon", |b| {
        b.iter(|| {
            proof = Poseidon::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &pw,
            );
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_poseidon", |b| {
        b.iter(|| {
            Poseidon::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

fn bench_rescue(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_RESC];
    for i in 0..SPONGE_RATE_RESC {
        input[i] = GoldilocksField(i as u64);
    }
    let (mut data, mut pw) =
        Rescue::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 8>(input);
    let mut proof =
        Rescue::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);

    // Benchmark circuit generation
    c.bench_function("circuit_generation_rescue", |b| {
        b.iter(|| {
            (data, pw) =
                Rescue::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 8>(input);
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_generation_rescue", |b| {
        b.iter(|| {
            proof = Rescue::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &pw,
            );
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_rescue", |b| {
        b.iter(|| {
            Rescue::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

fn bench_griffin(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_GRI];
    for i in 0..SPONGE_RATE_GRI {
        input[i] = GoldilocksField(i as u64);
    }
    let (mut data, mut pw) =
        Griffin::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
    let mut proof =
        Griffin::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);

    // Benchmark circuit generation
    c.bench_function("circuit_generation_griffin", |b| {
        b.iter(|| {
            (data, pw) =
                Griffin::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_generation_griffin", |b| {
        b.iter(|| {
            proof = Griffin::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &pw,
            );
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_griffin", |b| {
        b.iter(|| {
            Griffin::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

fn bench_anemoi(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_ANE];
    for i in 0..SPONGE_RATE_ANE {
        input[i] = GoldilocksField(i as u64);
    }
    let (mut data, mut pw) =
        Anemoi::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
    let mut proof =
        Anemoi::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);

    // Benchmark circuit generation
    c.bench_function("circuit_generation_anemoi", |b| {
        b.iter(|| {
            (data, pw) =
                Anemoi::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_generation_anemoi", |b| {
        b.iter(|| {
            proof = Anemoi::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &pw,
            );
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_anemoi", |b| {
        b.iter(|| {
            Anemoi::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

fn bench_arion(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_ARI];
    for i in 0..SPONGE_RATE_ARI {
        input[i] = GoldilocksField(i as u64);
    }
    let (mut data, mut pw) =
        Arion::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
    let mut proof =
        Arion::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);

    // Benchmark circuit generation
    c.bench_function("circuit_generation_arion", |b| {
        b.iter(|| {
            (data, pw) =
                Arion::circuit_generation::<GoldilocksField, PoseidonGoldilocksConfig, 4>(input);
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_generation_arion", |b| {
        b.iter(|| {
            proof =
                Arion::proof_generation::<GoldilocksField, PoseidonGoldilocksConfig, 2>(&data, &pw);
        })
    });

    // Benchmark proof verification
    c.bench_function("proof_verification_arion", |b| {
        b.iter(|| {
            Arion::proof_verification::<GoldilocksField, PoseidonGoldilocksConfig, 2>(
                &data, &proof,
            );
        })
    });
}

criterion_group!(benches, mimc, poseidon, rescue, griffin, anemoi, arion);
criterion_main!(benches);
