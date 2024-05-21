// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * Create a program to convert a hex code (as &str) into an Rgb struct
// * Implement TryFrom to perform the conversion
// * Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type

use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Error, Debug)]
enum HexError {
    #[error("Invalid hex string supplied for conversion")]
    InvalidHex,
}

#[derive(Error, Debug)]
enum ConvertError {
    #[error("Hex Error: {0}")]
    HexIssue(#[from] HexError),
    #[error("Hex Parse Error: {0}")]
    ParseIssue(#[from] ParseIntError),
}

impl TryFrom<&str> for Rgb {
    type Error = ConvertError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 7 || !value.starts_with("#") {
            Err(HexError::InvalidHex)?
        } else {
            let r = u8::from_str_radix(&value[1..=2], 16)?;
            let g = u8::from_str_radix(&value[3..=4], 16)?;
            let b = u8::from_str_radix(&value[5..=6], 16)?;

            Ok(Rgb(r, g, b))
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
