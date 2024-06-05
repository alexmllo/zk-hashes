## zk-dusk
The zk-dusk folder contains the Plonk implementation of the hash functions. Besides, there is also a plain implementation of each one in the BLS12-381 scalar field.

- The 'plain' hashing functionality operates on ```BlsScalar```.
- The 'gadget' hashing functionalities that build a circuit which outputs the hash.

### Benchmars
There are benchmarks for the zero-knowledge and the plain implementation

To run all benchmarks (zero-knowledge and plain), run

```
cargo bench --features=zk
```

## zk-lib
The zk-lib folder contains the Plonky2 implementation of the hash functions. Besides, there is also a plain implementation of each one in the Goldilocks field.

### Benchmars
There are benchmarks for the zero-knowledge and the plain implementation

To run the zero-knowledge implementation benchmarks, run

```
cargo bench --bench zk_benchmark 
```
and, for the plain performance, run
```
cargo bench --bench hash_benchmark
```
