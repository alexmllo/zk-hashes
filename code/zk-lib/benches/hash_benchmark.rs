use std::any::type_name;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    hash::hash_types::RichField,
};
use zk_lib::hashes::{
    anemoi::{anemoi::Anemoi, SPONGE_RATE as SPONGE_RATE_ANE, SPONGE_WIDTH as SPONGE_WIDTH_ANE},
    arion::{arion::Arion, SPONGE_RATE as SPONGE_RATE_ARI, SPONGE_WIDTH as SPONGE_WIDTH_ARI},
    griffin::griffin::Griffin,
    mimc::mimc::MiMC,
    poseidon::{poseidon::Poseidon, SPONGE_RATE, SPONGE_WIDTH},
    rescue_prime::{
        rescue_prime::Rescue, SPONGE_RATE as SPONGE_RATE_RESC, SPONGE_WIDTH as SPONGE_WIDTH_RESC,
    },
};

// Only can use GoldilocksField field type
// You can use a type 'T' that implements both PrimeField64 and Poseidon
fn bench_mimc(c: &mut Criterion) {
    mimc::<GoldilocksField, 41>(c);
}

fn mimc<F, const ROUNDS: usize>(c: &mut Criterion)
where
    F: RichField,
{
    let mimc = MiMC::<F>::new_from_rng();
    //let input = [F::rand(), F::rand()];
    let input: [F; 2] = [F::ONE, F::TWO];
    let name = format!("MiMC::<{}>", type_name::<F>().split("::").last().unwrap());
    let id = BenchmarkId::new(name, ROUNDS);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| mimc.permute_rounds(input))
    });
}

fn bench_poseidon(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE];
    for i in 0..SPONGE_RATE {
        input[i] = GoldilocksField(0 as u64);
    }
    let id = BenchmarkId::new("Poseidon Hash", SPONGE_WIDTH);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| Poseidon::poseidon_hash::<GoldilocksField, 2>(input))
    });
}

fn bench_rescue(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_RESC];
    for i in 0..SPONGE_RATE {
        input[i] = GoldilocksField(0 as u64);
    }
    let id = BenchmarkId::new("Rescue Hash", SPONGE_WIDTH);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| {
            Rescue::rescue_hash::<GoldilocksField, SPONGE_RATE_RESC>(input);
        })
    });
}

fn bench_griffin(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_RESC];
    for i in 0..SPONGE_RATE_RESC {
        input[i] = GoldilocksField(0 as u64);
    }
    let id = BenchmarkId::new("Griffin Hash", SPONGE_WIDTH_RESC);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| {
            Griffin::griffin_sponge::<GoldilocksField, SPONGE_RATE_RESC>(input);
        })
    });
}

fn bench_anemoi(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_ANE];
    for i in 0..SPONGE_RATE_ANE {
        input[i] = GoldilocksField(0 as u64);
    }
    let id = BenchmarkId::new("Anemoi Hash", SPONGE_WIDTH_ANE);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| {
            Anemoi::anemoi_hash::<GoldilocksField, SPONGE_RATE_ANE>(input);
        })
    });
}

fn bench_arion(c: &mut Criterion) {
    let mut input = [GoldilocksField::ZERO; SPONGE_RATE_ARI];
    for i in 0..SPONGE_RATE_ARI {
        input[i] = GoldilocksField(0 as u64);
    }
    let id = BenchmarkId::new("Arion Hash", SPONGE_WIDTH_ARI);
    c.bench_with_input(id, &input, |b, &input| {
        b.iter(|| {
            Arion::arion_hash::<GoldilocksField, SPONGE_RATE_ARI>(input);
        })
    });
}

criterion_group!(
    benches,
    bench_mimc,
    bench_poseidon,
    bench_rescue,
    bench_griffin,
    bench_anemoi,
    bench_arion
);
criterion_main!(benches);
