The zk-lib folder contains the Plonky2 implementation of the hash functions. Besides, there is also a plain implementation of each one in the Goldilocks field.

## Benchmars
There are benchmarks for the zero-knowledge and the plain implementation

To run the zero-knowledge implementation benchmarks, run

```
cargo bench --bench zk_benchmark 
```
and, for the plain performance, run
```
cargo bench --bench hash_benchmark
```