use crate::{Eui48, Eui64, HEX_CHARS};
use core::fmt;
use serde::de::Visitor;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer};

struct Eui48Visitor;
struct Eui64Visitor;

impl<'de> Visitor<'de> for Eui48Visitor {
    type Value = Eui48;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a 12 byte string with only hexadecimal characters"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 12 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 6];

        for (i, c) in v.chars().enumerate() {
            if let Some(value) = HEX_CHARS.iter().position(|&e| e == (c as u8)) {
                let result_index = i / 2;

                if i % 2 == 0 {
                    result[result_index] = (value as u8) << 4 & 0xF0
                } else {
                    result[result_index] |= value as u8 & 0xF
                }
            } else {
                return Err(Error::invalid_value(Unexpected::Char(c), &self));
            }
        }

        Ok(Eui48(result))
    }
}

impl<'de> Visitor<'de> for Eui64Visitor {
    type Value = Eui64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a 16 byte string with only hexadecimal characters"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 16 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 8];

        for (i, c) in v.chars().enumerate() {
            if let Some(value) = HEX_CHARS.iter().position(|&e| e == (c as u8)) {
                let result_index = i / 2;

                if i % 2 == 0 {
                    result[result_index] = (value as u8) << 4 & 0xF0
                } else {
                    result[result_index] |= value as u8 & 0xF
                }
            } else {
                return Err(Error::invalid_value(Unexpected::Char(c), &self));
            }
        }

        Ok(Eui64(result))
    }
}

impl<'de> Deserialize<'de> for Eui48 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Eui48Visitor)
    }
}

impl<'de> Deserialize<'de> for Eui64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Eui64Visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eui48, Eui64};
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn test_eui48_deserialize() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d7e54972eef")],
        );
    }

    #[test]
    fn test_eui64_deserialize() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d7e540000972eef")],
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972e")],
            "invalid length 10, expected a 12 byte string with only hexadecimal characters",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972eefef4d")],
            "invalid length 16, expected a 12 byte string with only hexadecimal characters",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eaa")],
            "invalid length 12, expected a 16 byte string with only hexadecimal characters",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eefef4ddd")],
            "invalid length 18, expected a 16 byte string with only hexadecimal characters",
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("ad7e54972esa")],
            "invalid value: character `s`, expected a 12 byte string with only hexadecimal characters",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("ad7e54972ea721sa")],
            "invalid value: character `s`, expected a 16 byte string with only hexadecimal characters",
        );
    }
}
