use crate::{Eui48, Eui64, HEX_CHARS};
use core::fmt;
use serde::de::{Error, Unexpected};
use serde::de::{Expected, Visitor};
use serde::{Deserialize, Deserializer};

struct Eui48Visitor;
struct Eui64Visitor;

impl<'de> Visitor<'de> for Eui48Visitor {
    type Value = Eui48;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 12 && v.len() != 17 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 6];
        to_hexadecimal(v, &mut result[..], &self)?;

        Ok(Eui48(result))
    }
}

impl<'de> Visitor<'de> for Eui64Visitor {
    type Value = Eui64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v.len() != 16 && v.len() != 23 {
            return Err(Error::invalid_length(v.len(), &self));
        }

        let mut result = [0; 8];
        to_hexadecimal(v, &mut result[..], &self)?;

        Ok(Eui64(result))
    }
}

fn to_hexadecimal<E>(v: &str, result: &mut [u8], exp: &dyn Expected) -> Result<(), E>
where
    E: Error,
{
    let mut separator_type = None;
    let mut separators = 0;

    for (i, c) in v.to_lowercase().chars().enumerate() {
        let hex_char_index = HEX_CHARS.iter().position(|&e| e == (c as u8));

        match hex_char_index {
            Some(value) => {
                let current_pos = i - separators;
                let index = current_pos / 2;

                if index > result.len() - 1 {
                    return Err(Error::invalid_length(v.len() - separators, exp));
                }

                if current_pos % 2 == 0 {
                    result[index] = (value as u8) << 4 & 0xF0
                } else {
                    result[index] |= value as u8 & 0xF
                }
            }
            None if c == ':' || c == '-' => {
                // String may contain separator after every second character.
                if i == 0 || i == v.len() || (i + 1) % 3 != 0 {
                    return Err(Error::custom(
                        "Separator must be placed after every second character",
                    ));
                }

                match separator_type {
                    Some(t) => {
                        if t != c {
                            return Err(Error::custom("Only one type of separator should be used"));
                        }
                    }
                    None => separator_type = Some(c),
                }

                separators += 1;
            }
            None => return Err(Error::invalid_value(Unexpected::Char(c), exp)),
        }
    }

    Ok(())
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
    fn test_eui48_deserialize_lowercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d7e54972eef")],
        );
    }

    #[test]
    fn test_eui48_deserialize_uppercase() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4D7E54972EEF")],
        );
    }

    #[test]
    fn test_eui64_deserialize_lowercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d7e540000972eef")],
        );
    }

    #[test]
    fn test_eui64_deserialize_uppercase() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4D7E540000972EEF")],
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972e")],
            "invalid length 10, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972eefef4d")],
            "invalid length 16, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d7e54972eefef4da")],
            "invalid length 17, expected 12 byte string with only hexadecimal characters or \
             17 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_length() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eaa")],
            "invalid length 12, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d7e54972eefef4ddd")],
            "invalid length 18, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str("ad7e54972esa")],
            "invalid value: character `s`, expected 12 byte string with only hexadecimal characters or \
            17 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_character() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("ad7e54972ea721sa")],
            "invalid value: character `s`, expected 16 byte string with only hexadecimal characters or \
             23 byte string with hexadecimal characters and separator after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_with_separator() {
        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d:7e:54:97:2e:ef")],
        );

        assert_de_tokens(
            &Eui48::from(85204980412143),
            &[Token::String("4d-7e-54-97-2e-ef")],
        );
    }

    #[test]
    fn test_eui64_deserialize_with_separator() {
        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d:7e:54:00:00:97:2e:ef")],
        );

        assert_de_tokens(
            &Eui64::from(5583992946972634863),
            &[Token::String("4d-7e-54-00-00-97-2e-ef")],
        );
    }

    #[test]
    fn test_eui48_deserialize_invalid_separator_position() {
        assert_de_tokens_error::<Eui48>(
            &[Token::Str(":4d7e:54:97:2e:ef")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d:7e:54:97:2eef:")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui48>(
            &[Token::Str("4d::7e54:97:2e:ef")],
            "Separator must be placed after every second character",
        );
    }

    #[test]
    fn test_eui64_deserialize_invalid_separator_position() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str(":4d7e:54:00:00:97:2e:ef")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d:7e:54:00:00:97:2eef:")],
            "Separator must be placed after every second character",
        );

        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d::7e54:00:00:97:2e:ef")],
            "Separator must be placed after every second character",
        );
    }

    #[test]
    fn test_eui48_deserialize_different_separators() {
        assert_de_tokens_error::<Eui64>(
            &[Token::Str("4d:7e-54:00:00:97:2e-ef")],
            "Only one type of separator should be used",
        );
    }
}
