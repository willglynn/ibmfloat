use std::convert::TryFrom;

/// Represents a numeric value that is missing. SAS® supports multiple flavors
/// of missing values including period (.), underscore (._), and the letters
/// A through Z (.A - .Z).
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MissingValue {
    /// A missing value represented using a period (.). This is the default.
    #[default]
    Period = b'.',
    /// A missing value represented using an underscore (_).
    Underscore = b'_',
    /// A missing value represented using the ASCII letter 'A'.
    A = b'A',
    /// A missing value represented using the ASCII letter 'B'.
    B = b'B',
    /// A missing value represented using the ASCII letter 'C'.
    C = b'C',
    /// A missing value represented using the ASCII letter 'D'.
    D = b'D',
    /// A missing value represented using the ASCII letter 'E'.
    E = b'E',
    /// A missing value represented using the ASCII letter 'F'.
    F = b'F',
    /// A missing value represented using the ASCII letter 'G'.
    G = b'G',
    /// A missing value represented using the ASCII letter 'H'.
    H = b'H',
    /// A missing value represented using the ASCII letter 'I'.
    I = b'I',
    /// A missing value represented using the ASCII letter 'J'.
    J = b'J',
    /// A missing value represented using the ASCII letter 'K'.
    K = b'K',
    /// A missing value represented using the ASCII letter 'L'.
    L = b'L',
    /// A missing value represented using the ASCII letter 'M'.
    M = b'M',
    /// A missing value represented using the ASCII letter 'N'.
    N = b'N',
    /// A missing value represented using the ASCII letter 'O'.
    O = b'O',
    /// A missing value represented using the ASCII letter 'P'.
    P = b'P',
    /// A missing value represented using the ASCII letter 'Q'.
    Q = b'Q',
    /// A missing value represented using the ASCII letter 'R'.
    R = b'R',
    /// A missing value represented using the ASCII letter 'S'.
    S = b'S',
    /// A missing value represented using the ASCII letter 'T'.
    T = b'T',
    /// A missing value represented using the ASCII letter 'U'.
    U = b'U',
    /// A missing value represented using the ASCII letter 'V'.
    V = b'V',
    /// A missing value represented using the ASCII letter 'W'.
    W = b'W',
    /// A missing value represented using the ASCII letter 'X'.
    X = b'X',
    /// A missing value represented using the ASCII letter 'Y'.
    Y = b'Y',
    /// A missing value represented using the ASCII letter 'Z'.
    Z = b'Z',
}

impl MissingValue {
    /// Return the ASCII code representation for the missing value.
    #[inline]
    pub const fn code(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u8> for MissingValue {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(MissingValue::Period),
            b'_' => Ok(MissingValue::Underscore),
            b'A' => Ok(MissingValue::A),
            b'B' => Ok(MissingValue::B),
            b'C' => Ok(MissingValue::C),
            b'D' => Ok(MissingValue::D),
            b'E' => Ok(MissingValue::E),
            b'F' => Ok(MissingValue::F),
            b'G' => Ok(MissingValue::G),
            b'H' => Ok(MissingValue::H),
            b'I' => Ok(MissingValue::I),
            b'J' => Ok(MissingValue::J),
            b'K' => Ok(MissingValue::K),
            b'L' => Ok(MissingValue::L),
            b'M' => Ok(MissingValue::M),
            b'N' => Ok(MissingValue::N),
            b'O' => Ok(MissingValue::O),
            b'P' => Ok(MissingValue::P),
            b'Q' => Ok(MissingValue::Q),
            b'R' => Ok(MissingValue::R),
            b'S' => Ok(MissingValue::S),
            b'T' => Ok(MissingValue::T),
            b'U' => Ok(MissingValue::U),
            b'V' => Ok(MissingValue::V),
            b'W' => Ok(MissingValue::W),
            b'X' => Ok(MissingValue::X),
            b'Y' => Ok(MissingValue::Y),
            b'Z' => Ok(MissingValue::Z),
            _ => Err("Not a missing value"),
        }
    }
}

/// A 64-bit SAS® floating point number.
///
/// SAS® floating point numbers are based on IBM floating point numbers, with
/// different representations for missing values. When converted to an IEEE-754
/// floating point number, the missing value representation is encoded as a
/// valid NaN encoding.
#[derive(Debug, Copy, Clone)]
pub struct F64(super::F64);

impl F64 {
    /// Create a floating point value from its representation as a byte array in big endian.
    ///
    /// ```
    /// use ibmfloat::sas::F64;
    ///
    /// let foreign_float = F64::from_be_bytes([0x2E, 0x00, 0, 0, 0, 0, 0, 0]);
    ///
    /// assert_eq!(foreign_float.to_bits(), 0x2E000000_00000000);
    ///
    /// let native_float = f64::from(foreign_float);
    /// assert!(native_float.is_nan());
    /// ```
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(super::F64::from_be_bytes(bytes))
    }

    /// Transmute this `F64` to a native-endian `u64`.
    #[inline]
    pub const fn to_bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Return the missing value representation, if applicable.
    pub fn missing_value(&self) -> Option<MissingValue> {
        let high = ((self.0 .0 >> 56) & 0x7F) as u8;
        MissingValue::try_from(high).ok()
    }

    /// Return whether the value represents a missing numeric value.
    pub fn is_missing_value(&self) -> bool {
        self.missing_value().is_some()
    }

    /// Return whether the value is a NaN representation.
    pub fn is_nan(&self) -> bool {
        f64::from(self.0).is_nan() || self.is_missing_value()
    }
}

impl From<F64> for f64 {
    fn from(value: F64) -> f64 {
        if let Some(_which) = value.missing_value() {
            let code = !_which.code() as u64;
            let bits = 0xFFFF0000_00000000 | (code << 40);
            f64::from_bits(bits)
        } else {
            value.0.into()
        }
    }
}

#[cfg(test)]
mod tests;
