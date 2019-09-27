#![no_std]

#[cfg(feature = "serde")]
mod de;
#[cfg(feature = "serde")]
mod ser;

use core::fmt::{Display, Error, Formatter};
use heapless::consts::*;
use heapless::{String, Vec};

const HEX_CHARS: &[u8] = b"0123456789abcdef";

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Eui48([u8; 6]);
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Eui64([u8; 8]);

macro_rules! to_hex_string {
    ($eui: expr, $size: ty) => {{
        let mut vec = Vec::<u8, $size>::new();

        for &byte in $eui.0.iter() {
            vec.push(HEX_CHARS[(byte >> 4) as usize]).unwrap();
            vec.push(HEX_CHARS[(byte & 0xf) as usize]).unwrap();
        }

        unsafe { String::from_utf8_unchecked(vec) }
    }};
}

impl Eui48 {
    #[inline]
    pub fn to_string(&self) -> String<U12> {
        to_hex_string!(self, U12)
    }
}

impl Eui64 {
    #[inline]
    pub fn to_string(&self) -> String<U16> {
        to_hex_string!(self, U16)
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

        for i in 0..6 {
            if i < 3 {
                data[i] = eui48.0[i]
            } else {
                data[i + 2] = eui48.0[i]
            }
        }

        Eui64(data)
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

#[test]
fn test_eui48_to_string() {
    let eui48 = Eui48::from(85204980412143);

    assert_eq!(eui48.to_string(), "4d7e54972eef")
}

#[test]
fn test_eui64_to_string() {
    let eui64 = Eui64::from(5583992946972634863);

    assert_eq!(eui64.to_string(), "4d7e540000972eef")
}

#[test]
fn test_eui48_to_eui64() {
    let eui48 = Eui48::from(85204980412143);
    let eui64 = Eui64::from(eui48);

    assert_eq!(eui64.to_string(), "4d7e540000972eef")
}
