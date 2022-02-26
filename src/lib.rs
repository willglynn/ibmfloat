#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", deny(missing_debug_implementations))]

//! A Rust library for [IBM floating point
//! numbers](https://en.wikipedia.org/wiki/IBM_hexadecimal_floating_point), specifically focused on
//! converting them to IEEE-754 floating point values.
//!
//! See [`F32`](struct.F32.html) for 32-bit floats and [`F64`](struct.F64.html) for 64-bit floats.
//!
//! ## Usage
//!
//! ### 32-bit floats
//!
//! `ibmfloat::F32` represents a 32-bit IBM floating point number. It supports the conversions:
//!
//! * Transmuting to/from a `u32` via `from_bits()`, `to_bits()`
//! * Transmuting to/from a big-endian `[u8; 4]` via `from_be_bytes()`/`to_be_bytes()`
//! * Lossily converting to an `f32` via `From`/`Into`
//! * Losslessly converting to an `f64` via `From`/`Into`
//!
//! IBM `F32` floats have slightly less precision than IEEE-754 `f32` floats, but it covers a slightly larger domain. `F32`s
//! of typical magnitude can be converted to `f32` without rounding or other loss of precision. Converting `F32`s of large
//! magnitude to `f32` will cause rounding; `F32`s of extreme magnitude can also cause overflow and underflow to occur.
//!
//! Every `F32` can be precisely represented as an `f64`, without rounding, overflow, or underflow. Those seeking a lossless
//! path to IEEE-754 should convert `F32` to `f64`.
//!
//! ```rust
//! // Use the example -118.625:
//! //   https://en.wikipedia.org/wiki/IBM_hexadecimal_floating_point#Example
//! let foreign_float = ibmfloat::F32::from_bits(0b1_1000010_0111_0110_1010_0000_0000_0000);
//!
//! let native_float = f32::from(foreign_float);
//! assert_eq!(native_float, -118.625f32);
//!
//! let native_float: f32 = foreign_float.into();
//! assert_eq!(native_float, -118.625f32);
//! ```
//!
//! ### 64-bit floats
//!
//! `ibmfloat::F64` represents a 64-bit IBM floating point number. It supports the conversions:
//!
//! * Transmuting to/from a `u64` via `from_bits()`, `to_bits()`
//! * Transmuting to/from a big-endian `[u8; 8]` via `from_be_bytes()`/`to_be_bytes()`
//! * Lossily converting to an `f32` via `From`/`Into`
//! * Lossily converting to an `f64` via `From`/`Into`
//!
//! IBM `F64` floats have slightly more precision than IEEE-754 `f64` floats, but they cover a slightly smaller domain. Most
//! conversions will require rounding, but there is no risk of overflow or underflow.
//!
//! ```rust
//! let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);
//!
//! let native_float = f64::from(foreign_float);
//! assert_eq!(native_float, 1.0f64);
//!
//! let native_float: f64 = foreign_float.into();
//! assert_eq!(native_float, 1.0f64);
//! ```

#[cfg(feature = "std")]
use std::{cmp, fmt};

#[cfg(not(feature = "std"))]
use core::cmp;

mod convert;

/// A 32-bit IBM floating point number.
///
/// This type supports the conversions:
///
/// * Transmuting to/from a `u32` via `from_bits()`, `to_bits()`
/// * Transmuting to/from a big-endian `[u8; 4]` via `from_be_bytes()`/`to_be_bytes()`
/// * Lossily converting to an `f32` via `From`/`Into`
/// * Losslessly converting to an `f64` via `From`/`Into`
///
/// IBM `F32` floats have slightly less precision than IEEE-754 `f32` floats, but it covers a
/// slightly larger domain. `F32`s of typical magnitude can be converted to `f32` without rounding
/// or other loss of precision. Converting `F32`s of large magnitude to `f32` will cause rounding;
/// `F32`s of extreme magnitude can also cause overflow and underflow to occur.
///
/// Every `F32` can be precisely represented as an `f64`, without rounding, overflow, or underflow.
/// Those seeking a lossless path to IEEE-754 should convert `F32` to `f64`.
///
/// ```
/// // Use the example -118.625:
/// //   https://en.wikipedia.org/wiki/IBM_hexadecimal_floating_point#Example
/// let foreign_float = ibmfloat::F32::from_bits(0b1_1000010_0111_0110_1010_0000_0000_0000);
///
/// let native_float = f32::from(foreign_float);
/// assert_eq!(native_float, -118.625f32);
///
/// let native_float: f32 = foreign_float.into();
/// assert_eq!(native_float, -118.625f32);
/// ```
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct F32(u32);

impl F32 {
    /// Transmute a native-endian `u64` into an `F64`.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F32::from_bits(0x46000001);
    ///
    /// let native_float = f32::from(foreign_float); // potential loss of precision
    /// assert_eq!(native_float, 1.0f32);
    ///
    /// let native_float = f64::from(foreign_float); // always exact
    /// assert_eq!(native_float, 1.0f64);
    /// ```
    #[inline]
    pub const fn from_bits(value: u32) -> Self {
        Self(value)
    }

    /// Transmute this `F32` to a native-endian `u32`.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F32::from_bits(0x46000001);
    ///
    /// assert_eq!(foreign_float.to_bits(), 0x46000001);
    /// ```
    #[inline]
    pub const fn to_bits(self) -> u32 {
        self.0
    }

    /// Create a floating point value from its representation as a byte array in big endian.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F32::from_be_bytes([0x46, 0, 0, 1]);
    ///
    /// assert_eq!(foreign_float.to_bits(), 0x46000001);
    ///
    /// let native_float = f32::from(foreign_float);
    /// assert_eq!(native_float, 1.0f32);
    /// ```
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }

    /// Return the memory representation of this floating point number as a byte array in big-endian
    /// (network) byte order.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F32::from_bits(0x46000001);
    ///
    /// assert_eq!(foreign_float.to_be_bytes(), [0x46, 0, 0, 1]);
    /// ```
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

/// A 64-bit IBM floating point number.
///
/// This type supports the conversions:
///
/// * Transmuting to/from a `u64` via `from_bits()`, `to_bits()`
/// * Transmuting to/from a big-endian `[u8; 8]` via `from_be_bytes()`/`to_be_bytes()`
/// * Lossily converting to an `f32` via `From`/`Into`
/// * Lossily converting to an `f64` via `From`/`Into`
///
/// IBM `F64` floats have slightly more precision than IEEE-754 `f64` floats, but they cover a
/// slightly smaller domain. Most conversions will require rounding, but there is no risk of
/// overflow or underflow.
///
/// ```
/// let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);
///
/// let native_float = f64::from(foreign_float);
/// assert_eq!(native_float, 1.0f64);
///
/// let native_float: f64 = foreign_float.into();
/// assert_eq!(native_float, 1.0f64);
/// ```
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct F64(u64);

impl F64 {
    /// Transmute a native-endian `u64` into an `F64`.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);
    ///
    /// let native_float = f64::from(foreign_float);
    /// assert_eq!(native_float, 1.0f64);
    /// ```
    #[inline]
    pub const fn from_bits(value: u64) -> Self {
        Self(value)
    }

    /// Transmute this `F64` to a native-endian `u64`.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);
    ///
    /// assert_eq!(foreign_float.to_bits(), 0x4110000000000000);
    /// ```
    #[inline]
    pub const fn to_bits(self) -> u64 {
        self.0
    }

    /// Create a floating point value from its representation as a byte array in big endian.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F64::from_be_bytes([0x41, 0x10, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(foreign_float.to_bits(), 0x4110000000000000);
    ///
    /// let native_float = f64::from(foreign_float);
    /// assert_eq!(native_float, 1.0f64);
    /// ```
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    /// Return the memory representation of this floating point number as a byte array in big-endian
    /// (network) byte order.
    ///
    /// ```
    /// let foreign_float = ibmfloat::F64::from_bits(0x4110000000000000);
    ///
    /// assert_eq!(foreign_float.to_be_bytes(), [0x41, 0x10, 0, 0, 0, 0, 0, 0]);
    /// ```
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

macro_rules! float {
    ($t:ty) => {
        // Convert everything to an f64 and implement Debug, PartialEq, PartialOrd over top

        #[cfg(feature = "std")]
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f64::from(*self).fmt(f)
            }
        }

        #[cfg(feature = "std")]
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f64::from(*self).fmt(f)
            }
        }

        #[cfg(feature = "std")]
        impl fmt::LowerExp for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f64::from(*self).fmt(f)
            }
        }

        #[cfg(feature = "std")]
        impl fmt::UpperExp for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f64::from(*self).fmt(f)
            }
        }

        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                f64::from(*self).eq(&f64::from(*other))
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                f64::from(*self).partial_cmp(&f64::from(*other))
            }
        }
    };
}
float!(F32);
float!(F64);

impl From<F32> for f32 {
    #[inline]
    fn from(v: F32) -> Self {
        f32::from_bits(convert::ibm32ieee32(v.0))
    }
}

impl From<F32> for f64 {
    #[inline]
    fn from(v: F32) -> Self {
        f64::from_bits(convert::ibm32ieee64(v.0))
    }
}

impl From<F64> for f32 {
    #[inline]
    fn from(v: F64) -> Self {
        f32::from_bits(convert::ibm64ieee32(v.0))
    }
}

impl From<F64> for f64 {
    #[inline]
    fn from(v: F64) -> Self {
        f64::from_bits(convert::ibm64ieee64(v.0))
    }
}

#[cfg(all(test, feature = "std"))]
mod std_tests;
