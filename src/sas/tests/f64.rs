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
