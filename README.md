# zk-lib: Zero-knowledge Library for hash functions

This repository constains the implementation of two Zero-Knowledge Proof (ZK) libraries for ZK-Friendly Hash Functions named zk-lib and zk-dusk, developed as part of the bachelor thesis. The libraries provides various hash functions and also their zero-knowledge circuit implementation in Plonky2 and PLONK, proof systems for generating zero-knowledge proofs.

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