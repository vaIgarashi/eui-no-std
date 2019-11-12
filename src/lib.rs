//! EUI-48 and EUI-64 no-std implementation using heapless.
//!
//! # Example
//!
//! ```rust
//! use eui::Eui48;
//! use eui::Eui64;
//!
//! let eui48 = Eui48::from(85204980412143);
//! let eui64 = Eui64::from(eui48);
//!     
//! assert_eq!(eui48.to_string(), "4D-7E-54-97-2E-EF");
//! assert_eq!(eui64.to_string(), "4D-7E-54-00-00-97-2E-EF");
//! ```
#![no_std]

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

use core::fmt::{Display, Error, Formatter, LowerHex, UpperHex};
use heapless::consts::*;
use heapless::{String, Vec};

const UPPERCASE_HEX_CHARS: &[u8] = b"0123456789ABCDEF";

#[derive(Eq, PartialEq, Copy, Clone, Debug, hash32_derive::Hash32)]
pub struct Eui48([u8; 6]);
#[derive(Eq, PartialEq, Copy, Clone, Debug, hash32_derive::Hash32)]
pub struct Eui64([u8; 8]);

macro_rules! to_hex_string {
    ($eui: expr, $size: ty) => {{
        let mut vec = Vec::<u8, $size>::new();

        for (i, &byte) in $eui.0.iter().enumerate() {
            if i != 0 {
                vec.push('-' as u8).expect("Vector is not long enough");
            }

            vec.push(UPPERCASE_HEX_CHARS[(byte >> 4) as usize])
                .expect("Vector is not long enough");

            vec.push(UPPERCASE_HEX_CHARS[(byte & 0xf) as usize])
                .expect("Vector is not long enough");
        }

        unsafe { String::from_utf8_unchecked(vec) }
    }};
}

impl Eui48 {
    #[inline]
    pub fn to_string(&self) -> String<U17> {
        to_hex_string!(self, U17)
    }
}

impl Eui64 {
    #[inline]
    pub fn to_string(&self) -> String<U23> {
        to_hex_string!(self, U23)
    }
}

impl From<u64> for Eui48 {
    fn from(value: u64) -> Self {
        let b1: u8 = ((value >> 40) & 0xff) as u8;
        let b2: u8 = ((value >> 32) & 0xff) as u8;
        let b3: u8 = ((value >> 24) & 0xff) as u8;
        let b4: u8 = ((value >> 16) & 0xff) as u8;
        let b5: u8 = ((value >> 8) & 0xff) as u8;
        let b6: u8 = (value & 0xff) as u8;

        return Eui48([b1, b2, b3, b4, b5, b6]);
    }
}

impl From<u64> for Eui64 {
    fn from(value: u64) -> Self {
        Eui64(value.to_be_bytes())
    }
}

impl From<Eui48> for Eui64 {
    fn from(eui48: Eui48) -> Self {
        let mut data = [0u8; 8];

        for i in 0..3 {
            data[i] = eui48.0[i]
        }

        for i in 5..8 {
            data[i] = eui48.0[i - 2]
        }

        Eui64(data)
    }
}

impl From<Eui48> for u64 {
    fn from(eui48: Eui48) -> Self {
        let data = eui48.0;

        ((data[0] as u64) << 40)
            + ((data[1] as u64) << 32)
            + ((data[2] as u64) << 24)
            + ((data[3] as u64) << 16)
            + ((data[4] as u64) << 8)
            + ((data[5] as u64) << 0)
    }
}

impl From<Eui64> for u64 {
    fn from(eui64: Eui64) -> Self {
        u64::from_be_bytes(eui64.0)
    }
}

impl Display for Eui48 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Eui64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

impl UpperHex for Eui48 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:X}", u64::from(*self))
    }
}

impl LowerHex for Eui48 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:x}", u64::from(*self))
    }
}

impl UpperHex for Eui64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:X}", u64::from(*self))
    }
}

impl LowerHex for Eui64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:x}", u64::from(*self))
    }
}

#[test]
fn test_eui48_to_string() {
    let eui48 = Eui48::from(85204980412143);

    assert_eq!(eui48.to_string(), "4D-7E-54-97-2E-EF")
}

#[test]
fn test_eui64_to_string() {
    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(eui64.to_string(), "4D-7E-54-00-00-97-2E-EF")
}

#[test]
fn test_eui48_to_eui64() {
    let eui48 = Eui48::from(85204980412143);
    let eui64 = Eui64::from(eui48);

    assert_eq!(eui64.to_string(), "4D-7E-54-00-00-97-2E-EF")
}

#[test]
fn test_u64_from_eui48() {
    let eui48 = Eui48::from(85204980412143);
    assert_eq!(u64::from(eui48), 85204980412143);
}

#[test]
fn test_u64_from_eui64() {
    let eui64 = Eui64::from(5583992946972634863);
    assert_eq!(u64::from(eui64), 5583992946972634863);
}

#[test]
fn test_hash_eui48() {
    use heapless::FnvIndexMap;

    let eui48 = Eui48::from(85204980412143);

    let mut fnv_index_map: FnvIndexMap<Eui48, u8, U1> = FnvIndexMap::new();
    fnv_index_map.insert(eui48, 1).unwrap();

    assert_eq!(1, *fnv_index_map.get(&eui48).unwrap())
}

#[test]
fn test_hash_eui64() {
    use heapless::FnvIndexMap;

    let eui64 = Eui64::from(5583992946972634863);

    let mut fnv_index_map: FnvIndexMap<Eui64, u8, U1> = FnvIndexMap::new();
    fnv_index_map.insert(eui64, 1).unwrap();

    assert_eq!(1, *fnv_index_map.get(&eui64).unwrap())
}

#[test]
fn test_display_eui48() {
    extern crate std;
    use std::format;

    let eui48 = Eui48::from(85204980412143);

    assert_eq!(format!("{}", eui48), "4D-7E-54-97-2E-EF");
}

#[test]
fn test_display_eui64() {
    extern crate std;
    use std::format;

    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(format!("{}", eui64), "4D-7E-54-00-00-97-2E-EF");
}

#[test]
fn test_format_upper_hex_eui48() {
    extern crate std;
    use std::format;

    let eui48 = Eui48::from(85204980412143);

    assert_eq!(format!("{:X}", eui48), "4D7E54972EEF");
}

#[test]
fn test_format_upper_hex_eui64() {
    extern crate std;
    use std::format;

    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(format!("{:X}", eui64), "4D7E540000972EEF");
}

#[test]
fn test_format_lower_hex_eui48() {
    extern crate std;
    use std::format;

    let eui48 = Eui48::from(85204980412143);

    assert_eq!(format!("{:x}", eui48), "4d7e54972eef");
}

#[test]
fn test_format_lower_hex_eui64() {
    extern crate std;
    use std::format;

    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(format!("{:x}", eui64), "4d7e540000972eef");
}
