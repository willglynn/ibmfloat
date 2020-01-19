use super::*;

struct Testcase(u32, u64);
impl Testcase {
    fn verify(&self) {
        let actual = ibm32ieee64(self.0);
        assert_eq!(
            actual, self.1,
            "ibm64ieee32(0x{:08x}): got 0x{:016x}, expected 0x{:016x}",
            self.0, actual, self.1
        );
    }
}

#[test]
fn python_test_cases() {
    for testcase in &[
        Testcase(0x00000000, 0x0000000000000000),
        Testcase(0x80000000, 0x8000000000000000),
        Testcase(0x00000001, 0x2e70000000000000),
        Testcase(0x80000001, 0xae70000000000000),
        Testcase(0x00ffffff, 0x2fefffffe0000000),
        Testcase(0x80ffffff, 0xafefffffe0000000),
        Testcase(0x41100000, 0x3ff0000000000000),
        Testcase(0xc1100000, 0xbff0000000000000),
        Testcase(0x7fffffff, 0x4fafffffe0000000),
        Testcase(0xffffffff, 0xcfafffffe0000000),
    ] {
        testcase.verify();
    }
}
