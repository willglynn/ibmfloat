`ibmfloat`
==========

A Rust library for [IBM floating point numbers](https://en.wikipedia.org/wiki/IBM_hexadecimal_floating_point),
specifically focused on converting them to IEEE-754 floating point values.
 
This crate has no Rust dependencies, no C dependencies, and no `unsafe` code. Its `std` feature is enabled by default,
and it can be disabled to support `#![no_std]` environments.

The conversion processes and much of the test suite are derived from the
[Python `ibm2ieee` library](https://github.com/enthought/ibm2ieee).

## Performance

Representative results from a laptop:

```console
$ cargo bench
…
F32 to f32              time:   [6.7323 ns 6.8545 ns 6.9941 ns]                        

F32 to f64              time:   [2.6318 ns 2.6932 ns 2.7597 ns]                        

F64 to f32              time:   [7.2901 ns 7.3978 ns 7.5129 ns]                        

F64 to f64              time:   [3.3132 ns 3.3889 ns 3.4657 ns]                        
```

Conversions to `f32` are more expensive than conversions to `f64`.

## Usage

### 32-bit floats

`ibmfloat::F32` represents a 32-bit IBM floating point number. It supports the conversions:

* Transmuting to/from a `u32` via `from_bits()`, `to_bits()`
* Transmuting to/from a big-endian `[u8; 4]` via `from_be_bytes()`/`to_be_bytes()`
* Lossily converting to an `f32` via `From`/`Into`
* Losslessly converting to an `f64` via `From`/`Into`

IBM `F32` floats have slightly less precision than IEEE-754 `f32` floats, but it covers a slightly larger domain. `F32`s
of typical magnitude can be converted to `f32` without rounding or other loss of precision. Converting `F32`s of large
magnitude to `f32` will cause rounding; `F32`s of extreme magnitude can also cause overflow and underflow to occur.

Every `F32` can be precisely represented as an `f64`, without rounding, overflow, or underflow. Those seeking a lossless
path to IEEE-754 should convert `F32` to `f64`.

```rust
// Use the example -118.625:
//   https://en.wikipedia.org/wiki/IBM_hexadecimal_floating_point#Example
let foreign_float = ibmfloat::F32::from_bits(0b1_1000010_0111_0110_1010_0000_0000_0000);

let native_float = f32::from(foreign_float);
assert_eq!(native_float, -118.625f32);

let native_float: f32 = foreign_float.into();
assert_eq!(native_float, -118.625f32);
```

### 64-bit floats

`ibmfloat::64` represents a 64-bit IBM floating point number. It supports the conversions:

* Transmuting to/from a `u64` via `from_bits()`, `to_bits()`
* Transmuting to/from a big-endian `[u8; 8]` via `from_be_bytes()`/`to_be_bytes()`
* Lossily converting to an `f32` via `From`/`Into`
* Lossily converting to an `f64` via `From`/`Into`

IBM `F64` floats have slightly more precision than IEEE-754 `f64` floats, but they cover a slightly smaller domain. Most
conversions will require rounding, but there is no risk of overflow or underflow.

```rust
let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);

let native_float = f64::from(foreign_float);
assert_eq!(native_float, 1.0f64);

let native_float: f64 = foreign_float.into();
assert_eq!(native_float, 1.0f64);
```

## Development

Please use `cargo test`, `cargo clippy`, and `cargo fmt` as you go. Please also `cargo test --no-default-features` to
prevent accidental breakage for `#![no_std]` users. GitHub Actions runs each of these commands on push. 

[`cargo fuzz`](https://github.com/rust-fuzz/cargo-fuzz) covers each of the four IBM to IEEE conversion paths, comparing
them to `ibm2ieee.c`'s output. Please run them as needed if you tinker with that logic.

```console
$ cargo +nightly fuzz run ibm32ieee32
$ cargo +nightly fuzz run ibm32ieee64
$ cargo +nightly fuzz run ibm64ieee32
$ cargo +nightly fuzz run ibm64ieee64
```

Additional references:

* [ESA/390 Enhanced Floating Point Support: An
  Overview](ftp://public.dhe.ibm.com/software/websphere/awdtools/hlasm/sh93fpov.pdf) is an excellent introduction
* [Hercules](http://www.hercules-390.org) implements IBM floating point hardware in C
