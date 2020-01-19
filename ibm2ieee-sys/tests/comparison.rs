use ibm2ieee_sys;
use ibmfloat::*;
use rand::prelude::*;

fn random_values<T>(n: usize) -> Vec<T>
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let mut rng = thread_rng();
    let mut values = Vec::with_capacity(n);
    values.resize_with(n, || rng.gen());
    values
}

// ten million ought to be enough for anybody
const N: usize = 10_000_000;

#[test]
fn ibm32ieee32() {
    let values = random_values(N);
    let c = values
        .iter()
        .map(|v| unsafe { ibm2ieee_sys::ibm32ieee32(*v) });
    let rust = values
        .iter()
        .map(|v| f32::from(F32::from_bits(*v)).to_bits());
    for ((value, c), rust) in values.iter().zip(c).zip(rust) {
        assert_eq!(
            c, rust,
            "input 0x{:08x}: C returned 0x{:08x}, Rust returned 0x{:08x}",
            value, c, rust
        );
    }
}

#[test]
fn ibm32ieee64() {
    let values = random_values(N);
    let c = values
        .iter()
        .map(|v| unsafe { ibm2ieee_sys::ibm32ieee64(*v) });
    let rust = values
        .iter()
        .map(|v| f64::from(F32::from_bits(*v)).to_bits());
    for ((value, c), rust) in values.iter().zip(c).zip(rust) {
        assert_eq!(
            c, rust,
            "input 0x{:08x}: C returned 0x{:16x}, Rust returned 0x{:16x}",
            value, c, rust
        );
    }
}

#[test]
fn ibm64ieee32() {
    let values = random_values(N);
    let c = values
        .iter()
        .map(|v| unsafe { ibm2ieee_sys::ibm64ieee32(*v) });
    let rust = values
        .iter()
        .map(|v| f32::from(F64::from_bits(*v)).to_bits());
    for ((value, c), rust) in values.iter().zip(c).zip(rust) {
        assert_eq!(
            c, rust,
            "input 0x{:16x}: C returned 0x{:08x}, Rust returned 0x{:08x}",
            value, c, rust
        );
    }
}

#[test]
fn ibm64ieee64() {
    let values = random_values(N);
    let c = values
        .iter()
        .map(|v| unsafe { ibm2ieee_sys::ibm64ieee64(*v) });
    let rust = values
        .iter()
        .map(|v| f64::from(F64::from_bits(*v)).to_bits());
    for ((value, c), rust) in values.iter().zip(c).zip(rust) {
        assert_eq!(
            c, rust,
            "input 0x{:16x}: C returned 0x{:16x}, Rust returned 0x{:16x}",
            value, c, rust
        );
    }
}
