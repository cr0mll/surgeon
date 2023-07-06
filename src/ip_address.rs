use std::{fmt, num::ParseIntError};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum IpAddressError
{
    #[error("Invalid IP address")]
    InvalidAddress(#[from] ParseIntError),
    
    #[error("Invalid IP address length")]
    InvalidLength
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd)]
pub struct Ipv4Addr(u8, u8, u8, u8);

impl Ipv4Addr {
    
    /// Constructs an IPv4 address from four octet values
    fn new(a: u8 , b: u8 , c: u8, d: u8) -> Self {
        Ipv4Addr(a, b, c, d)
    }
    
    /// Attempts to construct an IPv4 address from a string.
    fn from_str(string: &str) -> Result<Self, IpAddressError> {
        let octets : Vec<&str> = string.split('.').collect();

        if octets.len() > 4 { return Err(IpAddressError::InvalidLength); }
        
        Ok(Ipv4Addr(octets[0].parse()?, octets[1].parse()?, octets[2].parse()?,octets[3].parse()?))
    }

    /// Constructs a loopback address
    const fn loopback() -> Self {
        Ipv4Addr(127, 0, 0, 1)
    }

    /// Checks if an IP address is a loopback address
    fn is_loopback(&self) -> bool {
        return self == &Self::loopback()
    }
}

impl fmt::Display for Ipv4Addr {
        
    /// Display the IP address as a string with the '.' delimiter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_from_str() {
        
        assert_eq!(Ipv4Addr::new(128, 90, 227, 232), Ipv4Addr::from_str("128.90.227.232").unwrap());
    }

    #[test]
    fn loopback() {
        assert_eq!(Ipv4Addr::loopback(), Ipv4Addr::new(127, 0, 0, 1));
        assert!(Ipv4Addr::new(127,0,0,1).is_loopback());
    }
}