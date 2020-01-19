# `ibm2ieee-sys`

This crate exposes `ibm2ieee.c` to Rust. It is not needed outside `ibmfloat` development and is therefore
`publish = false`.

`cargo test` runs millions of random inputs through `ibm2ieee.c` and verifies that `ibmfloat` returns the same results
when asked to perform the same conversion.

```console
$ cargo test
…
running 4 tests
test ibm32ieee64 ... ok
test ibm32ieee32 ... ok
test ibm64ieee64 ... ok
test ibm64ieee32 ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo bench` here benchmarks `ibm2ieee.c` and compares it against `ibmfloat`.

```
$ cargo bench
…
C: F32 to f32           time:   [6.6305 ns 6.7005 ns 6.7766 ns]                           
C: F32 to f64           time:   [2.5299 ns 2.5688 ns 2.6116 ns]                           
C: F64 to f32           time:   [6.9990 ns 7.0843 ns 7.1680 ns]                           
C: F64 to f64           time:   [3.0458 ns 3.1388 ns 3.2464 ns]                           
…
F32 to f32              time:   [6.7092 ns 6.7734 ns 6.8454 ns]                        
F32 to f64              time:   [2.4642 ns 2.4965 ns 2.5326 ns]                        
F64 to f32              time:   [7.2500 ns 7.3315 ns 7.4169 ns]                        
F64 to f64              time:   [2.7761 ns 2.8028 ns 2.8342 ns]                        
```
