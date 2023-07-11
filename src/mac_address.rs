use std::fmt;

use thiserror::Error;

#[derive(Error, Copy, Debug, PartialEq, Eq, Clone)]
pub enum MacAddressError
{
    #[error("Invalid MAC address")]
    InvalidAddress
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd)]
pub struct MacAddr(u8, u8, u8, u8, u8, u8);

impl MacAddr {
    
    /// Constructs a MAC address from 6 values.
    pub fn new(a: u8, b: u8, c:u8, d:u8, e:u8, f:u8) -> Self {
        Self(a,b,c,d,e,f)
    }

    /// Attempts to construct a MAC address from a string.
    pub fn from_str(string: &str) -> Result<Self, MacAddressError> {
        let mut instance = MacAddr::default();

        let bytes : Vec<&str> = string.trim().split(['.', ':', '-'].as_ref()).collect();

        if bytes.len() != 6 {
            return Err(MacAddressError::InvalidAddress);
        }

        if let Ok(bytes) = hex::decode(bytes.concat()) {
            instance.0 = bytes[0];
            instance.1 = bytes[1];
            instance.2 = bytes[2];
            instance.3 = bytes[3];
            instance.4 = bytes[4];
            instance.5 = bytes[5];

            return Ok(instance);
        }
        else {
            return Err(MacAddressError::InvalidAddress);
        }
    }

    /// Constructs a broadcast MAC address.
    pub const fn broadcast() -> Self {
        MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff)
    }

}

impl From<pnet_datalink::MacAddr> for MacAddr {
    fn from(pnet_mac_addr: pnet_datalink::MacAddr) -> Self {
        Self(pnet_mac_addr.0, pnet_mac_addr.1, pnet_mac_addr.2, pnet_mac_addr.3, pnet_mac_addr.4, pnet_mac_addr.5)
    }
}

impl fmt::Display for MacAddr {
    
    /// Display the MAC address as a string with the ':' delimiter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_from_str() {
        
        assert_eq!(MacAddr::from_str("FF:FF:FF:FF:FF:FF"), Ok(MacAddr::broadcast()));
        assert_eq!(MacAddr::from_str("00:12:FF:E3:A4:78"), Ok(MacAddr::new(0x00, 0x12, 0xff, 0xe3, 0xa4,0x78)));
        assert_eq!(MacAddr::from_str("AX:BR:13:FA:98:KO"), Err(MacAddressError::InvalidAddress));

    }
}