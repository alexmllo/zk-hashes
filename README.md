# zk-lib: Zero-knowledge Library for hash functions

This repository constains the implementation of two Zero-Knowledge Proof (ZK) libraries for ZK-Friendly Hash Functions named zk-lib and zk-dusk, developed as part of the bachelor thesis. The libraries provides plain and zero-knowledge circuit implementation for various hash functions in the Polygon's Plonky2 and Dusk Plonk's Plonk proof systems for generating zero-knowledge proofs, in the Rust programming language.

Hash functions: MiMC, Poseidon, Rescue-prime, Griffin, Anemoi and Arion.

## Requirements
To use this library, ensure you satisfy one of the following prerequisites:

- Rust compiler with recent nightly toolchain. You can set up with the following command:

```
rustup override set nightly-YYYY-MM-DD
```
Make sure to replace YYYY-MM-DD with the appropriate date for your setup.

- Copy the rust-toolchain file in your project directory. It can be changed to a more recent nightly version.

## Code Structure

- mimc-python: 
    - Implementation of the MiMC hash function Python, primarily used for testing purposes.
- zk-lib: 
    - Library containing implementation of the MiMC, Poseidon, Rescue-Prime, Griffin, Anemoi and Arion hash functions, as well as the zero-knowledge circuit for each one using Plonky2 as the proof system.
    - Benchmarking of the hashes and the zero-knowledge circuit for each hash function.
- zk-dusk
    - Library containing implementation of the MiMC, Poseidon, Rescue-Prime, Griffin, Anemoi and Arion hash functions, as well as the zero-knowledge circuit for each one using PLONK as the proof system.
    - Benchmarking of the hashes and the zero-knowledge circuit for each hash function.

## Benchmarks
Measurements were conducted on a system configured with a 1.4 GHz Quad-Core Intel Core i5 processor, 8 GB LPDDR3 RAM with a transfer rate of 2133 MHz, and running macOS 14.5. The system was clocked at 1.4 GHz and utilized Rust’s Nightly build dated 2024-02-01. Benchmarking was performed using Criterion 0.5, with Plonky2 version 0.1.4 and dusk-plonk version 0.19.

### Plain performance
<div style="display: flex; justify-content: space-between;">
  <img src="https://github.com/alexmllo/zk-hashes/assets/125358906/f0e520b1-7f8f-400b-a025-eb04f3af9bb1" alt="image" width="490" height="170">
  <img src="https://github.com/alexmllo/zk-hashes/assets/125358906/19a6a1f7-ce71-4ed9-96d1-110e3d06459d" alt="image" width="500" height="220">
</div>

### Plonky2 performance
![image](https://github.com/alexmllo/zk-hashes/assets/125358906/35f7896a-befb-46f8-b596-04e4d9ebbae1)

### Plonk performance
#### Number of constraints
<img src="https://github.com/alexmllo/zk-hashes/assets/125358906/712fa09f-aec0-4e71-93b3-8b3e0bc6950b" alt="image" width="500" height="220">

#### Performance
<img src="https://github.com/alexmllo/zk-hashes/assets/125358906/1bbb4b6f-8d3e-464f-a768-ac88e0e5fcac)" alt="image" width="540" height="260">
