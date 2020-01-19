use super::*;

struct Testcase(u32, u32);
impl Testcase {
    fn verify(&self) {
        let actual = ibm32ieee32(self.0);
        assert_eq!(
            actual, self.1,
            "ibm32ieee32(0x{:08x}): got 0x{:08x}, expected 0x{:08x}",
            self.0, actual, self.1
        );
    }
}

#[test]
fn overflow() {
    for testcase in &[
        Testcase(0xfffb005d, 0xff800000),
        Testcase(0xfd00fd00, 0xff800000),
        Testcase(0x78ffffff, 0x7f800000),
    ] {
        testcase.verify();
    }
}

#[test]
fn normal_case() {
    for testcase in &[
        Testcase(0xe000000a, 0xf5200000),
        Testcase(0xe0000a00, 0xf9200000),
        Testcase(0x41000010, 0x37800000),
        Testcase(0x24010ac9, 0x03856480),
    ] {
        testcase.verify();
    }
}

#[test]
fn possible_subnormal() {
    for testcase in &[
        Testcase(0xa0ffffff, 0x80200000),
        Testcase(0xa0e0000e, 0x801c0002),
        Testcase(0xa0010a17, 0x80002143),
        Testcase(0xa0014ac5, 0x80002959),
        Testcase(0x2000005d, 0x0000000c),
    ] {
        testcase.verify();
    }
}

#[cfg(feature = "std")]
#[test]
fn python_test_cases() {
    for testcase in vec![
        Testcase(0x00000000, 0x00000000),
        Testcase(0x80000000, 0x80000000),
        Testcase(0x00000001, 0x00000000),
        Testcase(0x80000001, 0x80000000),
        Testcase(0x3f000000, 0x00000000),
        Testcase(0xbf000000, 0x80000000),
        Testcase(0x7f000000, 0x00000000),
        Testcase(0xff000000, 0x80000000),
        Testcase(0x1b100000, 0x00000000),
        Testcase(0x9b100000, 0x80000000),
        Testcase(0x1b200000, 0x00000000),
        Testcase(0x9b200000, 0x80000000),
        Testcase(0x1b400000, 0x00000000),
        Testcase(0x9b400000, 0x80000000),
        Testcase(0x1b400001, 0x00000001),
        Testcase(0x9b400001, 0x80000001),
        Testcase(0x1b800000, 0x00000001),
        Testcase(0x9b800000, 0x80000001),
        Testcase(0x1bbfffff, 0x00000001),
        Testcase(0x9bbfffff, 0x80000001),
        Testcase(0x1bc00000, 0x00000002),
        Testcase(0x9bc00000, 0x80000002),
        Testcase(0x1da7bfff, 0x0000014f),
        Testcase(0x9da7bfff, 0x8000014f),
        Testcase(0x1da7c000, 0x00000150),
        Testcase(0x9da7c000, 0x80000150),
        Testcase(0x1da84000, 0x00000150),
        Testcase(0x9da84000, 0x80000150),
        Testcase(0x1da84001, 0x00000151),
        Testcase(0x9da84001, 0x80000151),
        Testcase(0x1da8bfff, 0x00000151),
        Testcase(0x9da8bfff, 0x80000151),
        Testcase(0x1da8c000, 0x00000152),
        Testcase(0x9da8c000, 0x80000152),
        Testcase(0x1da94000, 0x00000152),
        Testcase(0x9da94000, 0x80000152),
        Testcase(0x1da94001, 0x00000153),
        Testcase(0x9da94001, 0x80000153),
        Testcase(0x1da9bfff, 0x00000153),
        Testcase(0x9da9bfff, 0x80000153),
        Testcase(0x1da9c000, 0x00000154),
        Testcase(0x9da9c000, 0x80000154),
        Testcase(0x1daa4000, 0x00000154),
        Testcase(0x9daa4000, 0x80000154),
        Testcase(0x1daa4001, 0x00000155),
        Testcase(0x9daa4001, 0x80000155),
        Testcase(0x1fffffff, 0x00020000),
        Testcase(0x9fffffff, 0x80020000),
        Testcase(0x20fffff4, 0x001ffffe),
        Testcase(0xa0fffff4, 0x801ffffe),
        Testcase(0x20fffff5, 0x001fffff),
        Testcase(0xa0fffff5, 0x801fffff),
        Testcase(0x20fffff6, 0x001fffff),
        Testcase(0xa0fffff6, 0x801fffff),
        Testcase(0x20fffff7, 0x001fffff),
        Testcase(0xa0fffff7, 0x801fffff),
        Testcase(0x20fffff8, 0x001fffff),
        Testcase(0xa0fffff8, 0x801fffff),
        Testcase(0x20fffff9, 0x001fffff),
        Testcase(0xa0fffff9, 0x801fffff),
        Testcase(0x20fffffa, 0x001fffff),
        Testcase(0xa0fffffa, 0x801fffff),
        Testcase(0x20fffffb, 0x001fffff),
        Testcase(0xa0fffffb, 0x801fffff),
        Testcase(0x20fffffc, 0x00200000),
        Testcase(0xa0fffffc, 0x80200000),
        Testcase(0x20fffffd, 0x00200000),
        Testcase(0xa0fffffd, 0x80200000),
        Testcase(0x20fffffe, 0x00200000),
        Testcase(0xa0fffffe, 0x80200000),
        Testcase(0x20ffffff, 0x00200000),
        Testcase(0xa0ffffff, 0x80200000),
        Testcase(0x21100000, 0x00200000),
        Testcase(0xa1100000, 0x80200000),
        Testcase(0x21200000, 0x00400000),
        Testcase(0xa1200000, 0x80400000),
        Testcase(0x213fffff, 0x007ffffe),
        Testcase(0xa13fffff, 0x807ffffe),
        Testcase(0x21400000, 0x00800000),
        Testcase(0xa1400000, 0x80800000),
        Testcase(0x40800000, 0x3f000000),
        Testcase(0xc0800000, 0xbf000000),
        Testcase(0x46000001, 0x3f800000),
        Testcase(0xc6000001, 0xbf800000),
        Testcase(0x45000010, 0x3f800000),
        Testcase(0xc5000010, 0xbf800000),
        Testcase(0x44000100, 0x3f800000),
        Testcase(0xc4000100, 0xbf800000),
        Testcase(0x43001000, 0x3f800000),
        Testcase(0xc3001000, 0xbf800000),
        Testcase(0x42010000, 0x3f800000),
        Testcase(0xc2010000, 0xbf800000),
        Testcase(0x41100000, 0x3f800000),
        Testcase(0xc1100000, 0xbf800000),
        Testcase(0x41200000, 0x40000000),
        Testcase(0xc1200000, 0xc0000000),
        Testcase(0x41300000, 0x40400000),
        Testcase(0xc1300000, 0xc0400000),
        Testcase(0x41400000, 0x40800000),
        Testcase(0xc1400000, 0xc0800000),
        Testcase(0x41800000, 0x41000000),
        Testcase(0xc1800000, 0xc1000000),
        Testcase(0x48000001, 0x43800000),
        Testcase(0xc8000001, 0xc3800000),
        Testcase(0x48000002, 0x44000000),
        Testcase(0xc8000002, 0xc4000000),
        Testcase(0x48000004, 0x44800000),
        Testcase(0xc8000004, 0xc4800000),
        Testcase(0x48000008, 0x45000000),
        Testcase(0xc8000008, 0xc5000000),
        Testcase(0x48000010, 0x45800000),
        Testcase(0xc8000010, 0xc5800000),
        Testcase(0x48000020, 0x46000000),
        Testcase(0xc8000020, 0xc6000000),
        Testcase(0x48000040, 0x46800000),
        Testcase(0xc8000040, 0xc6800000),
        Testcase(0x48000080, 0x47000000),
        Testcase(0xc8000080, 0xc7000000),
        Testcase(0x48000100, 0x47800000),
        Testcase(0xc8000100, 0xc7800000),
        Testcase(0x48000200, 0x48000000),
        Testcase(0xc8000200, 0xc8000000),
        Testcase(0x48000400, 0x48800000),
        Testcase(0xc8000400, 0xc8800000),
        Testcase(0x48000800, 0x49000000),
        Testcase(0xc8000800, 0xc9000000),
        Testcase(0x48001000, 0x49800000),
        Testcase(0xc8001000, 0xc9800000),
        Testcase(0x48002000, 0x4a000000),
        Testcase(0xc8002000, 0xca000000),
        Testcase(0x48004000, 0x4a800000),
        Testcase(0xc8004000, 0xca800000),
        Testcase(0x48008000, 0x4b000000),
        Testcase(0xc8008000, 0xcb000000),
        Testcase(0x48010000, 0x4b800000),
        Testcase(0xc8010000, 0xcb800000),
        Testcase(0x48020000, 0x4c000000),
        Testcase(0xc8020000, 0xcc000000),
        Testcase(0x48040000, 0x4c800000),
        Testcase(0xc8040000, 0xcc800000),
        Testcase(0x48080000, 0x4d000000),
        Testcase(0xc8080000, 0xcd000000),
        Testcase(0x48100000, 0x4d800000),
        Testcase(0xc8100000, 0xcd800000),
        Testcase(0x48200000, 0x4e000000),
        Testcase(0xc8200000, 0xce000000),
        Testcase(0x48400000, 0x4e800000),
        Testcase(0xc8400000, 0xce800000),
        Testcase(0x48800000, 0x4f000000),
        Testcase(0xc8800000, 0xcf000000),
        Testcase(0x60ffffff, 0x7f7fffff),
        Testcase(0xe0ffffff, 0xff7fffff),
        Testcase(0x61100000, 0x7f800000),
        Testcase(0xe1100000, 0xff800000),
        Testcase(0x61200000, 0x7f800000),
        Testcase(0xe1200000, 0xff800000),
        Testcase(0x61400000, 0x7f800000),
        Testcase(0xe1400000, 0xff800000),
        Testcase(0x62100000, 0x7f800000),
        Testcase(0xe2100000, 0xff800000),
        Testcase(0x7fffffff, 0x7f800000),
        Testcase(0xffffffff, 0xff800000),
        Testcase(0xc276a000, 0xc2ed4000),
        Testcase(0x4276a000, 0x42ed4000),
    ] {
        testcase.verify();
    }
}

#[cfg(feature = "std")]
#[test]
fn python_random_test_cases() {
    for testcase in include!("random_single_to_single.txt") {
        testcase.verify();
    }
}
