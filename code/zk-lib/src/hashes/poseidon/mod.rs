pub mod poseidon;
mod constants;
mod mds;

pub const SPONGE_RATE: usize = 8;
pub const SPONGE_CAPACITY: usize = 4;
pub const SPONGE_WIDTH: usize = SPONGE_RATE + SPONGE_CAPACITY; // Number of permutations

pub const HALF_N_FULL_ROUNDS: usize = 4;
pub const N_FULL_ROUNDS_TOTAL: usize = 2 * HALF_N_FULL_ROUNDS;
pub const N_PARTIAL_ROUNDS: usize = 22;
pub const N_ROUNDS: usize = N_FULL_ROUNDS_TOTAL + N_PARTIAL_ROUNDS; // 30
pub const MAX_WIDTH: usize = 12;
pub const N_CONSTANTS: usize = MAX_WIDTH * N_ROUNDS;