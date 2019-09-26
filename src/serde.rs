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
    use serde::Serialize;

    #[derive(Serialize)]
    struct DeviceEui48 {
        pub eui: Eui48,
        pub version: u32,
        pub blinking: bool,
    }

    #[derive(Serialize)]
    struct DeviceEui64 {
        pub eui: Eui64,
        pub version: u32,
        pub blinking: bool,
    }

    #[test]
    fn test_eui48_to_json() {
        let device = DeviceEui48 {
            eui: Eui48::from(85204980412143),
            version: 3,
            blinking: true,
        };

        let expected_json_string = r#"{"eui":"4d7e54972eef","version":3,"blinking":true}"#;
        let json_string = serde_json::to_string(&device).unwrap();

        assert_eq!(json_string, expected_json_string);
    }

    #[test]
    fn test_eui64_to_json() {
        let device = DeviceEui64 {
            eui: Eui64::from(5583992946972634863),
            version: 3,
            blinking: true,
        };

        let expected_json_string = r#"{"eui":"4d7e540000972eef","version":3,"blinking":true}"#;
        let json_string = serde_json::to_string(&device).unwrap();

        assert_eq!(json_string, expected_json_string);
    }

}
