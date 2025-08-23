#![no_std]
use core::{error::Error, fmt::Display, str::FromStr};

#[derive(Clone, Copy)]
pub struct MacAddress {
    pub bytes: [u8; 6],
}

impl MacAddress {
    pub fn unicast(self) -> bool {
        self.bytes[0] & 1 == 0
    }

    pub fn multicast(self) -> bool {
        !self.unicast()
    }

    pub fn local(self) -> bool {
        self.bytes[0] & 2 != 0
    }
}

impl From<MacAddress> for u64 {
    fn from(val: MacAddress) -> Self {
        let mut bytes = [0u8; 8];
        bytes[..6].copy_from_slice(&val.bytes);
        u64::from_le_bytes(bytes)
    }
}

impl From<u64> for MacAddress {
    fn from(value: u64) -> Self {
        let mut bytes = [0u8; 6];
        bytes.copy_from_slice(&value.to_le_bytes()[0..6]);
        Self { bytes }
    }
}

impl From<[u8; 6]> for MacAddress {
    fn from(value: [u8; 6]) -> Self {
        Self { bytes: value }
    }
}

impl From<MacAddress> for [u8; 6] {
    fn from(value: MacAddress) -> Self {
        value.bytes
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5]
        )
    }
}

impl FromStr for MacAddress {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const SEPARATORS: [char; 2] = [' ', ':'];
        const BASE: u32 = 16;
        let mut bytes = [0u8; 6];
        let mut chars = s.chars();

        if s.len() < 12 {
            return Err(ParseError);
        }

        let mut i = 0;
        while i < 6 {
            let mut left = chars.next().ok_or(ParseError)?;
            let mut right = chars.next().ok_or(ParseError)?;

            if SEPARATORS.contains(&left) {
                left = right;
                right = chars.next().ok_or(ParseError)?;
            }

            let high = (left.to_digit(BASE).ok_or(ParseError)? * BASE) as u8;
            let low = right.to_digit(BASE).ok_or(ParseError)? as u8;

            bytes[i] = high + low;
            i += 1;
        }

        Ok(Self { bytes })
    }
}

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ParseError: Could not parse mac address.")
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;

    #[test]
    pub fn deser_test() -> Result<(), ParseError> {
        let bytes = [0, 1, 20, 30, 40, 50];
        let mac = alloc::format!("{}", MacAddress { bytes }).parse::<MacAddress>();
        assert_eq!(bytes, mac?.bytes);

        let str = "0f0f0f0f0f0f";
        assert_eq!([15; 6], str.parse::<MacAddress>()?.bytes);

        let str = "0F0F0F0F0F0F";
        assert_eq!([15; 6], str.parse::<MacAddress>()?.bytes);
        Ok(())
    }

    #[test]
    pub fn unicast_test() {
        let bytes = [0, 1, 2, 3, 4, 5];
        let mut mac = MacAddress { bytes };
        assert!(mac.unicast());

        mac.bytes[0] = 1;
        assert!(!mac.unicast());
    }

    #[test]
    pub fn multicast_test() {
        let bytes = [0, 1, 2, 3, 4, 5];
        let mut mac = MacAddress { bytes };
        assert!(!mac.multicast());

        mac.bytes[0] = 1;
        assert!(mac.multicast());
    }

    #[test]
    pub fn local_test() {
        let bytes = [0, 1, 2, 3, 4, 5];
        let mut mac = MacAddress { bytes };
        assert!(!mac.local());

        mac.bytes[0] = 3;
        assert!(mac.local());
    }

    #[test]
    pub fn tofrom_u64_test() {
        let int = u32::MAX as u64;
        let mac: MacAddress = int.into();
        assert_eq!(int, mac.into());
    }
}
