#![no_main]
use libfuzzer_sys::fuzz_target;

// Pull in the Rust library
use ibmfloat::*;

// Pull in the C library
mod c {
    extern "C" {
        pub fn ibm64ieee64(ibm: u64) -> u64;
    }
}

fuzz_target!(|input: [u8; 8]| {
    let v = u64::from_be_bytes(input);
    let rust_result = f64::from(F64::from_bits(v)).to_bits();
    let c_result = unsafe { c::ibm64ieee64(v) };
    assert_eq!(
        rust_result, c_result,
        "\n\n        Testcase(0x{:016x}, 0x{:016x}),\n\n",
        v, c_result,
    );
});
