use super::*;

struct Testcase(u64, u32);
impl Testcase {
    fn verify(&self) {
        let actual = ibm64ieee32(self.0);
        assert_eq!(
            actual, self.1,
            "ibm64ieee32(0x{:016x}): got 0x{:08x}, expected 0x{:08x}",
            self.0, actual, self.1
        );
    }
}

#[test]
fn overflows() {
    for testcase in &[
        Testcase(0xe1e11a0a0add4c1a, 0xff800000),
        Testcase(0xffff000000000000, 0xff800000),
        Testcase(0xe1e11a0a0add4c1a, 0xff800000),
    ] {
        testcase.verify();
    }
}

#[test]
fn normal_round_ties_to_even() {
    for testcase in &[
        Testcase(0xcc4c00dd4c1a0000, 0xd69801bb),
        Testcase(0x3b2600003b260000, 0x34180001),
        Testcase(0x4c4c4c4cd70a0000, 0x5698989a),
        Testcase(0x4d4cffffffffffff, 0x589a0000),
    ] {
        testcase.verify();
    }
}

#[test]
fn possible_subnormal_round_ties_to_even() {
    for testcase in &[
        Testcase(0x2000ffffffffffff, 0x00002000),
        Testcase(0x1b4c4c2628282828, 0x00000001),
        Testcase(0x22010000fe400004, 0x00200020),
    ] {
        testcase.verify();
    }
}

#[test]
fn underflows() {
    for testcase in &[
        Testcase(0x0000000000260000, 0x00000000),
        Testcase(0x0000ff11ff11ff4c, 0x00000000),
        Testcase(0x80000000000000fc, 0x80000000),
        Testcase(0x8000000000260000, 0x80000000),
    ] {
        testcase.verify();
    }
}

#[test]
fn python_test_cases() {
    for testcase in vec![
        Testcase(0x0000000000000001, 0x00000000),
        Testcase(0x8000000000000001, 0x80000000),
        Testcase(0x1effffffffffffff, 0x00002000),
        Testcase(0x9effffffffffffff, 0x80002000),
        Testcase(0x1fffffffffffffff, 0x00020000),
        Testcase(0x9fffffffffffffff, 0x80020000),
        Testcase(0x20ffffffffffffff, 0x00200000),
        Testcase(0xa0ffffffffffffff, 0x80200000),
        Testcase(0x213fffffbfffffff, 0x007fffff),
        Testcase(0xa13fffffbfffffff, 0x807fffff),
        Testcase(0x213fffffc0000000, 0x00800000),
        Testcase(0xa13fffffc0000000, 0x80800000),
        Testcase(0x213fffffffffffff, 0x00800000),
        Testcase(0xa13fffffffffffff, 0x80800000),
        Testcase(0x40ffffff7fffffff, 0x3f7fffff),
        Testcase(0xc0ffffff7fffffff, 0xbf7fffff),
        Testcase(0x40ffffff80000000, 0x3f800000),
        Testcase(0xc0ffffff80000000, 0xbf800000),
        Testcase(0x411fffffefffffff, 0x3fffffff),
        Testcase(0xc11fffffefffffff, 0xbfffffff),
        Testcase(0x411ffffff0000000, 0x40000000),
        Testcase(0xc11ffffff0000000, 0xc0000000),
        Testcase(0x411fffffffffffff, 0x40000000),
        Testcase(0xc11fffffffffffff, 0xc0000000),
        Testcase(0x60ffffff7fffffff, 0x7f7fffff),
        Testcase(0xe0ffffff7fffffff, 0xff7fffff),
        Testcase(0x60ffffff80000000, 0x7f800000),
        Testcase(0xe0ffffff80000000, 0xff800000),
        Testcase(0x60ffffffffffffff, 0x7f800000),
        Testcase(0xe0ffffffffffffff, 0xff800000),
        Testcase(0x7fffffffffffffff, 0x7f800000),
        Testcase(0xffffffffffffffff, 0xff800000),
        Testcase(0x1da7bfffffffffff, 0x0000014f),
        Testcase(0x9da7bfffffffffff, 0x8000014f),
        Testcase(0x1da8400000000001, 0x00000151),
        Testcase(0x9da8400000000001, 0x80000151),
        Testcase(0x1da8bfffffffffff, 0x00000151),
        Testcase(0x9da8bfffffffffff, 0x80000151),
        Testcase(0x1da9400000000001, 0x00000153),
        Testcase(0x9da9400000000001, 0x80000153),
        Testcase(0x1da9bfffffffffff, 0x00000153),
        Testcase(0x9da9bfffffffffff, 0x80000153),
        Testcase(0x1daa400000000001, 0x00000155),
        Testcase(0x9daa400000000001, 0x80000155),
        Testcase(0x0000000000000000, 0x00000000),
        Testcase(0x8000000000000000, 0x80000000),
        Testcase(0x0000000100000000, 0x00000000),
        Testcase(0x8000000100000000, 0x80000000),
        Testcase(0x3f00000000000000, 0x00000000),
        Testcase(0xbf00000000000000, 0x80000000),
        Testcase(0x7f00000000000000, 0x00000000),
        Testcase(0xff00000000000000, 0x80000000),
        Testcase(0x1b10000000000000, 0x00000000),
        Testcase(0x9b10000000000000, 0x80000000),
        Testcase(0x1b20000000000000, 0x00000000),
        Testcase(0x9b20000000000000, 0x80000000),
        Testcase(0x1b40000000000000, 0x00000000),
        Testcase(0x9b40000000000000, 0x80000000),
        Testcase(0x1b40000100000000, 0x00000001),
        Testcase(0x9b40000100000000, 0x80000001),
        Testcase(0x1b80000000000000, 0x00000001),
        Testcase(0x9b80000000000000, 0x80000001),
        Testcase(0x1bbfffff00000000, 0x00000001),
        Testcase(0x9bbfffff00000000, 0x80000001),
        Testcase(0x1bc0000000000000, 0x00000002),
        Testcase(0x9bc0000000000000, 0x80000002),
        Testcase(0x1da7bfff00000000, 0x0000014f),
        Testcase(0x9da7bfff00000000, 0x8000014f),
        Testcase(0x1da7c00000000000, 0x00000150),
        Testcase(0x9da7c00000000000, 0x80000150),
        Testcase(0x1da8400000000000, 0x00000150),
        Testcase(0x9da8400000000000, 0x80000150),
        Testcase(0x1da8400100000000, 0x00000151),
        Testcase(0x9da8400100000000, 0x80000151),
        Testcase(0x1da8bfff00000000, 0x00000151),
        Testcase(0x9da8bfff00000000, 0x80000151),
        Testcase(0x1da8c00000000000, 0x00000152),
        Testcase(0x9da8c00000000000, 0x80000152),
        Testcase(0x1da9400000000000, 0x00000152),
        Testcase(0x9da9400000000000, 0x80000152),
        Testcase(0x1da9400100000000, 0x00000153),
        Testcase(0x9da9400100000000, 0x80000153),
        Testcase(0x1da9bfff00000000, 0x00000153),
        Testcase(0x9da9bfff00000000, 0x80000153),
        Testcase(0x1da9c00000000000, 0x00000154),
        Testcase(0x9da9c00000000000, 0x80000154),
        Testcase(0x1daa400000000000, 0x00000154),
        Testcase(0x9daa400000000000, 0x80000154),
        Testcase(0x1daa400100000000, 0x00000155),
        Testcase(0x9daa400100000000, 0x80000155),
        Testcase(0x1fffffff00000000, 0x00020000),
        Testcase(0x9fffffff00000000, 0x80020000),
        Testcase(0x20fffff400000000, 0x001ffffe),
        Testcase(0xa0fffff400000000, 0x801ffffe),
        Testcase(0x20fffff500000000, 0x001fffff),
        Testcase(0xa0fffff500000000, 0x801fffff),
        Testcase(0x20fffff600000000, 0x001fffff),
        Testcase(0xa0fffff600000000, 0x801fffff),
        Testcase(0x20fffff700000000, 0x001fffff),
        Testcase(0xa0fffff700000000, 0x801fffff),
        Testcase(0x20fffff800000000, 0x001fffff),
        Testcase(0xa0fffff800000000, 0x801fffff),
        Testcase(0x20fffff900000000, 0x001fffff),
        Testcase(0xa0fffff900000000, 0x801fffff),
        Testcase(0x20fffffa00000000, 0x001fffff),
        Testcase(0xa0fffffa00000000, 0x801fffff),
        Testcase(0x20fffffb00000000, 0x001fffff),
        Testcase(0xa0fffffb00000000, 0x801fffff),
        Testcase(0x20fffffc00000000, 0x00200000),
        Testcase(0xa0fffffc00000000, 0x80200000),
        Testcase(0x20fffffd00000000, 0x00200000),
        Testcase(0xa0fffffd00000000, 0x80200000),
        Testcase(0x20fffffe00000000, 0x00200000),
        Testcase(0xa0fffffe00000000, 0x80200000),
        Testcase(0x20ffffff00000000, 0x00200000),
        Testcase(0xa0ffffff00000000, 0x80200000),
        Testcase(0x2110000000000000, 0x00200000),
        Testcase(0xa110000000000000, 0x80200000),
        Testcase(0x2120000000000000, 0x00400000),
        Testcase(0xa120000000000000, 0x80400000),
        Testcase(0x213fffff00000000, 0x007ffffe),
        Testcase(0xa13fffff00000000, 0x807ffffe),
        Testcase(0x2140000000000000, 0x00800000),
        Testcase(0xa140000000000000, 0x80800000),
        Testcase(0x4080000000000000, 0x3f000000),
        Testcase(0xc080000000000000, 0xbf000000),
        Testcase(0x4600000100000000, 0x3f800000),
        Testcase(0xc600000100000000, 0xbf800000),
        Testcase(0x4500001000000000, 0x3f800000),
        Testcase(0xc500001000000000, 0xbf800000),
        Testcase(0x4400010000000000, 0x3f800000),
        Testcase(0xc400010000000000, 0xbf800000),
        Testcase(0x4300100000000000, 0x3f800000),
        Testcase(0xc300100000000000, 0xbf800000),
        Testcase(0x4201000000000000, 0x3f800000),
        Testcase(0xc201000000000000, 0xbf800000),
        Testcase(0x4110000000000000, 0x3f800000),
        Testcase(0xc110000000000000, 0xbf800000),
        Testcase(0x4120000000000000, 0x40000000),
        Testcase(0xc120000000000000, 0xc0000000),
        Testcase(0x4130000000000000, 0x40400000),
        Testcase(0xc130000000000000, 0xc0400000),
        Testcase(0x4140000000000000, 0x40800000),
        Testcase(0xc140000000000000, 0xc0800000),
        Testcase(0x4180000000000000, 0x41000000),
        Testcase(0xc180000000000000, 0xc1000000),
        Testcase(0x4800000100000000, 0x43800000),
        Testcase(0xc800000100000000, 0xc3800000),
        Testcase(0x4800000200000000, 0x44000000),
        Testcase(0xc800000200000000, 0xc4000000),
        Testcase(0x4800000400000000, 0x44800000),
        Testcase(0xc800000400000000, 0xc4800000),
        Testcase(0x4800000800000000, 0x45000000),
        Testcase(0xc800000800000000, 0xc5000000),
        Testcase(0x4800001000000000, 0x45800000),
        Testcase(0xc800001000000000, 0xc5800000),
        Testcase(0x4800002000000000, 0x46000000),
        Testcase(0xc800002000000000, 0xc6000000),
        Testcase(0x4800004000000000, 0x46800000),
        Testcase(0xc800004000000000, 0xc6800000),
        Testcase(0x4800008000000000, 0x47000000),
        Testcase(0xc800008000000000, 0xc7000000),
        Testcase(0x4800010000000000, 0x47800000),
        Testcase(0xc800010000000000, 0xc7800000),
        Testcase(0x4800020000000000, 0x48000000),
        Testcase(0xc800020000000000, 0xc8000000),
        Testcase(0x4800040000000000, 0x48800000),
        Testcase(0xc800040000000000, 0xc8800000),
        Testcase(0x4800080000000000, 0x49000000),
        Testcase(0xc800080000000000, 0xc9000000),
        Testcase(0x4800100000000000, 0x49800000),
        Testcase(0xc800100000000000, 0xc9800000),
        Testcase(0x4800200000000000, 0x4a000000),
        Testcase(0xc800200000000000, 0xca000000),
        Testcase(0x4800400000000000, 0x4a800000),
        Testcase(0xc800400000000000, 0xca800000),
        Testcase(0x4800800000000000, 0x4b000000),
        Testcase(0xc800800000000000, 0xcb000000),
        Testcase(0x4801000000000000, 0x4b800000),
        Testcase(0xc801000000000000, 0xcb800000),
        Testcase(0x4802000000000000, 0x4c000000),
        Testcase(0xc802000000000000, 0xcc000000),
        Testcase(0x4804000000000000, 0x4c800000),
        Testcase(0xc804000000000000, 0xcc800000),
        Testcase(0x4808000000000000, 0x4d000000),
        Testcase(0xc808000000000000, 0xcd000000),
        Testcase(0x4810000000000000, 0x4d800000),
        Testcase(0xc810000000000000, 0xcd800000),
        Testcase(0x4820000000000000, 0x4e000000),
        Testcase(0xc820000000000000, 0xce000000),
        Testcase(0x4840000000000000, 0x4e800000),
        Testcase(0xc840000000000000, 0xce800000),
        Testcase(0x4880000000000000, 0x4f000000),
        Testcase(0xc880000000000000, 0xcf000000),
        Testcase(0x60ffffff00000000, 0x7f7fffff),
        Testcase(0xe0ffffff00000000, 0xff7fffff),
        Testcase(0x6110000000000000, 0x7f800000),
        Testcase(0xe110000000000000, 0xff800000),
        Testcase(0x6120000000000000, 0x7f800000),
        Testcase(0xe120000000000000, 0xff800000),
        Testcase(0x6140000000000000, 0x7f800000),
        Testcase(0xe140000000000000, 0xff800000),
        Testcase(0x6210000000000000, 0x7f800000),
        Testcase(0xe210000000000000, 0xff800000),
        Testcase(0x7fffffff00000000, 0x7f800000),
        Testcase(0xffffffff00000000, 0xff800000),
        Testcase(0xc276a00000000000, 0xc2ed4000),
        Testcase(0x4276a00000000000, 0x42ed4000),
    ] {
        testcase.verify();
    }
}

#[test]
fn python_random_test_cases() {
    for testcase in include!("random_double_to_single.txt") {
        testcase.verify();
    }
}
