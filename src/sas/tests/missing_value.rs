use super::MissingValue;
use std::convert::TryFrom;

#[test]
fn code() {
    assert_eq!(b'.', MissingValue::Period.code());
}

#[test]
fn default() {
    assert_eq!(MissingValue::Period, Default::default());
}

#[test]
fn try_from_known() {
    assert_eq!(MissingValue::Period, MissingValue::try_from(b'.').unwrap());
}

#[test]
fn try_from_unknown() {
    assert!(MissingValue::try_from(0u8).is_err());
}
