#![no_main]
use libfuzzer_sys::fuzz_target;

// Pull in the Rust library
use ibmfloat::*;

// Pull in the C library
use ibm2ieee_sys as c;

fuzz_target!(|input: [u8; 4]| {
    let v = u32::from_be_bytes(input);
    let rust_result = f64::from(F32::from_bits(v)).to_bits();
    let c_result = unsafe { c::ibm32ieee64(v) };
    assert_eq!(
        rust_result, c_result,
        "\n\n        Testcase(0x{:08x}, 0x{:016x}),\n\n",
        v, c_result,
    );
});
