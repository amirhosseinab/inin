use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct NationalId(String);

impl TryFrom<&str> for NationalId {
    type Error = NationalIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = format!("{:0>10}", value.trim());
        let digits: Vec<u32> = value.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() != 10 {
            return Err(NationalIdError);
        }

        let sum: u32 = (0..9).map(|i| { digits[i] * (10 - i) as u32 }).sum();
        if sum == 0 { return Err(NationalIdError); }
        let control_digit = *digits.last().unwrap();

        let rem = sum % 11;
        if (rem < 2 && rem == control_digit) || (rem >= 2 && rem + control_digit == 11) {
            return Ok(NationalId(value));
        }
        Err(NationalIdError)
    }
}

impl Deref for NationalId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NationalIdError;

impl Error for NationalIdError {}

impl Display for NationalIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid iranian national id number")
    }
}

#[cfg(test)]
mod tests {
    use crate::{NationalId, NationalIdError};
    use std::convert::{TryFrom, TryInto};

    #[test]
    fn test_length_of_code_should_pad_to_10_digit() {
        assert_eq!(NationalId::try_from("0451726707"), Ok(NationalId(String::from("0451726707"))));
    }

    #[test]
    fn test_invalid_input() {
        assert!(NationalId::try_from("").is_err());
        assert!(NationalId::try_from("123").is_err());
        assert!(NationalId::try_from("123456ab").is_err());
        assert!(NationalId::try_from("12345678ab").is_err());
        assert!(NationalId::try_from("a814659438").is_err());
    }

    #[test]
    fn test_should_deref_to_string() {
        let code = "0814659438";
        let result: Result<NationalId, _> = code.try_into();
        assert_eq!(code, *result.unwrap())
    }

    #[test]
    fn test_validate_national_id() {
        let ni: Result<NationalId, NationalIdError> = "0040010007".try_into();
        assert!(ni.is_ok());

        let ni: Result<NationalId, NationalIdError> = "0814659438".try_into();
        assert!(ni.is_ok());
    }
}