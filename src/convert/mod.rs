#[cfg(not(feature = "std"))]
use core::{
    mem::size_of,
    ops::{BitAnd, Not, Shl, Shr, Sub},
};
#[cfg(feature = "std")]
use std::{
    mem::size_of,
    ops::{BitAnd, Not, Shl, Shr, Sub},
};

trait Uint:
    Copy
    + Sized
    + BitAnd<Self, Output = Self>
    + Shr<u8, Output = Self>
    + Shl<u8, Output = Self>
    + Not<Output = Self>
    + Sub<Self, Output = Self>
    + From<u8>
{
    const MAX: Self;
}

impl Uint for u32 {
    const MAX: u32 = u32::max_value();
}
impl Uint for u64 {
    const MAX: u64 = u64::max_value();
}

/// Split an IBM float represented at uint U into its (sign, exponent, fraction).
/// Sign and fraction are left in place, while exponent is slid all the way right.
#[inline]
fn split<U: Uint>(ibm: U) -> (U, U, U) {
    // Sign bit is always the top bit
    let sign_bit_mask = U::MAX & !(U::MAX >> 1);
    // Fraction is always all but the top byte
    let fraction_mask = U::MAX >> 8;
    // Exponent is always the 7 bits in the middle
    let exponent_mask = !sign_bit_mask & !fraction_mask;
    let exponent_shift = (size_of::<U>() * 8 - 8) as u8;
    (
        ibm & sign_bit_mask,
        (ibm & exponent_mask) >> exponent_shift,
        ibm & fraction_mask,
    )
}

/// Convert a native-endian IBM 32-bit float to a native-endian IEEE-754 32-bit float.
pub fn ibm32ieee32(ibm: u32) -> u32 {
    let (sign, ibm_exponent, ibm_fraction) = split(ibm);

    // Quick return for zeros.
    if ibm_fraction == 0 {
        return sign;
    }

    // Normalise significand, adjusting exponent to match, and offsetting by 2
    let (ibm_exponent, ibm_fraction) = {
        let shift = ibm_fraction.leading_zeros() as i32 - 8;
        ((ibm_exponent << 2) as i32 - shift, ibm_fraction << shift)
    };

    // Adjust exponents for the differing biases of the formats: the IBM bias is 64 hex digits, or
    // 256 bits. The IEEE bias is 127. The difference is -129; we get an extra -1 from the different
    // significand representations (0.f for IBM versus 1.f for IEEE), and another -1 to compensate
    // for an evil trick that saves an operation on the fast path: we don't remove the hidden 1-bit
    // from the IEEE significand, so in the final addition that extra bit ends in incrementing the
    // exponent by one.
    let ieee_exponent = ibm_exponent - 131;

    if ieee_exponent >= 254 {
        // overflow
        sign.wrapping_add(0x7f80_0000)
    } else if ieee_exponent >= 0 {
        // normal case; no shift needed
        let ieee_frac = ibm_fraction;
        sign.wrapping_add((ieee_exponent as u32) << 23)
            .wrapping_add(ieee_frac)
    } else if ieee_exponent >= -32 {
        // Possible subnormal result; shift significand right by -ieee_exponent bits, rounding the
        // result with round-ties-to-even.
        //
        // The round-ties-to-even code deserves some explanation: out of the bits we're shifting
        // out, let's call the most significant bit the "rounding bit", and the rest the "trailing
        // bits". We'll call the least significant bit that *isn't* shifted out the "parity bit".
        // So for an example 5-bit shift right, we'd label the bits as follows:
        //
        // Before the shift:
        //         ...xxxprtttt
        //                    ^
        //    msb            lsb
        //
        // After the shift:
        //              ...xxxp
        //                    ^
        //    msb            lsb
        //
        // with the result possibly incremented by one.
        //
        // For round-ties-to-even, we need to round up if both (a) the rounding bit is 1, and
        // (b) either the parity bit is 1, or at least one of the trailing bits is 1. We construct a
        // mask that has 1-bits in the parity bit position and trailing bit positions, and use that
        // to check condition (b). So for example in the 5-bit shift right, the mask looks like
        // this:
        //
        //         ...000101111 : mask
        //         ...xxxprtttt : ibm_frac
        //                    ^
        //    msb            lsb
        //
        // We then shift right by (shift - 1), add 1 if (ibm & mask) is nonzero, and then do a final
        // shift by one to get the rounded value. Note that this approach avoids the possibility of
        // trying to shift a width-32 value by 32, which can be problematic (see C99 6.5.7p3).
        let mask = !((0xffff_fffd) << (-1 - ieee_exponent) as u32);
        let round_up = if ibm_fraction & mask > 0 { 1 } else { 0 };
        let ieee_frac =
            (ibm_fraction >> ((-1i32) - ieee_exponent) as u32).wrapping_add(round_up) >> 1;
        sign.wrapping_add(ieee_frac)
    } else {
        // Underflow to zero
        sign
    }
}

pub fn ibm32ieee64(ibm: u32) -> u64 {
    // This is the simplest of the four cases: there's no need to check for overflow or underflow,
    // no possibility of subnormal output, and never any rounding.
    let (sign, ibm_exponent, ibm_fraction) = split(ibm);

    // Extend the sign
    let sign = (sign as u64) << 32;

    // Quick return for zeros.
    if ibm_fraction == 0 {
        return sign;
    }

    // Normalise significand, adjusting exponent to match, and offsetting by 2
    let (ibm_exponent, ibm_fraction) = {
        let shift = ibm_fraction.leading_zeros() as i32 - 8;
        ((ibm_exponent << 2) as i32 - shift, ibm_fraction << shift)
    };

    // Adjust exponents for the differing biases of the formats: the IBM bias is 64 hex digits, or
    // 256 bits. The IEEE bias is 1023. The difference is 767; we get an extra -1 from the different
    // significand representations (0.f for IBM versus 1.f for IEEE), and another -1 to compensate
    // for an evil trick that saves an operation: we don't remove the hidden 1-bit from the IEEE
    // significand, so in the final addition that extra bit ends in incrementing the exponent by
    // one.

    let ieee_exponent = ibm_exponent + 765;
    let ieee_fraction = (ibm_fraction as u64) << 29;
    sign.wrapping_add((ieee_exponent as u64) << 52)
        .wrapping_add(ieee_fraction)
}

// IBM double-precision bit pattern to IEEE single-precision bit pattern.
pub fn ibm64ieee32(ibm: u64) -> u32 {
    // Overflow and underflow possible; rounding can occur in both normal and subnormal cases.
    let (sign, ibm_exponent, ibm_fraction) = split(ibm);

    // Trim the sign
    let sign = (sign >> 32) as u32;

    // Quick return for zeros.
    if ibm_fraction == 0 {
        return sign;
    }

    let (ibm_exponent, ibm_fraction) = {
        let shift = ibm_fraction.leading_zeros() as i32 - 8;
        ((ibm_exponent << 2) as i32 - shift, ibm_fraction << shift)
    };

    let ieee_exponent = ibm_exponent - 131;
    if ieee_exponent >= 254 {
        // Overflow
        sign.wrapping_add(0x7f80_0000)
    } else if ieee_exponent >= 0 {
        // Normal case; shift right 32, with round-ties-to-even
        let round_up = if ibm_fraction & 0x0001_7fff_ffff > 0 {
            1
        } else {
            0
        };
        let ieee_frac = ((ibm_fraction >> 31).wrapping_add(round_up) >> 1) as u32;
        sign.wrapping_add((ieee_exponent as u32) << 23)
            .wrapping_add(ieee_frac)
    } else if ieee_exponent >= -32 {
        // Possible subnormal; shift right with round-ties-to-even
        let mask: u64 = !(0xffff_ffff_ffff_fffdu64 << (31 - ieee_exponent) as u64);
        let round_up: u32 = if ibm_fraction & mask > 0 { 1 } else { 0 };
        let ieee_frac = ((ibm_fraction >> (31 - ieee_exponent) as u64)
            .wrapping_add(round_up as u64)
            >> 1) as u32;
        sign.wrapping_add(ieee_frac)
    } else {
        // Underflow to zero
        sign
    }
}

// IBM double-precision bit pattern to IEEE double-precision bit pattern.
pub fn ibm64ieee64(ibm: u64) -> u64 {
    // No overflow or underflow possible, but the precision of the so we'll frequently need to
    // round.
    let (sign, ibm_exponent, ibm_fraction) = split(ibm);

    // Quick return for zeros.
    if ibm_fraction == 0 {
        return sign;
    }

    let (ibm_exponent, ibm_fraction) = {
        let shift = ibm_fraction.leading_zeros() as i32 - 8;
        ((ibm_exponent << 2) as i32 - shift, ibm_fraction << shift)
    };

    let ieee_exponent = ibm_exponent + 765;

    // Right-shift by 3 bits (the difference between the IBM and IEEE significand lengths), rounding
    // with round-ties-to-even.
    let round_up = if (ibm_fraction & 0xb) > 0 { 1 } else { 0 };
    let ieee_fraction = (ibm_fraction >> 2).wrapping_add(round_up) >> 1;
    sign.wrapping_add((ieee_exponent as u64) << 52)
        .wrapping_add(ieee_fraction)
}

#[cfg(test)]
mod tests;
