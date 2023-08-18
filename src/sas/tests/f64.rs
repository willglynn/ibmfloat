use super::*;

#[test]
fn from_be_bytes_missing_value() {
    let bytes: [u8; 8] = [0x2E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let sas = F64::from_be_bytes(bytes);
    assert!(sas.is_missing_value());
    assert_eq!(MissingValue::Period, sas.missing_value().unwrap());
    assert!(sas.is_nan());
}

#[test]
fn from_missing_value() {
    let bytes: [u8; 8] = [0x2E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let sas = F64::from_be_bytes(bytes);
    let ieee = f64::from(sas);
    assert!(ieee.is_nan());
}

#[test]
fn from_non_missing_value() {
    let foreign_float = F64::from_be_bytes([
        0b1100_0010,
        0b0111_0110,
        0b1010_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_00000,
    ]);
    let native_float = f64::from(foreign_float);
    assert_eq!(native_float, -118.625f64);

    let native_float: f64 = foreign_float.into();
    assert_eq!(native_float, -118.625f64);

    assert!(!foreign_float.is_nan());
    assert_eq!(None, foreign_float.missing_value());
}
