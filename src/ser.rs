use crate::{Eui48, Eui64};
use serde::{Serialize, Serializer};

impl Serialize for Eui48 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for Eui64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Eui48, Eui64};
    use serde_test::{assert_ser_tokens, Token};

    #[test]
    fn test_eui48_serialize() {
        let eui48 = Eui48::from(85204980412143);
        assert_ser_tokens(&eui48, &[Token::String("4D-7E-54-97-2E-EF")]);
    }

    #[test]
    fn test_eui64_serialize() {
        let eui64 = Eui64::from(5583992946972634863);
        assert_ser_tokens(&eui64, &[Token::String("4D-7E-54-00-00-97-2E-EF")]);
    }
}
