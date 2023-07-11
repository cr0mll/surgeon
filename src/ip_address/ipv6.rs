use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, Ord, PartialOrd)]
pub struct Ipv6Addr(u16, u16, u16, u16, u16, u16);

impl Ipv6Addr {
    
    /// Constructs an IPv6 address from six hextets.
    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16) -> Self {
        Ipv6Addr(a, b, c, d, e, f)
    }
    
    /// Attempts to construct an IPv6 address from a string.
    fn from_str(string: &str) -> Result<Self, IpAddressError> {
        let hextets : Vec<&str> = string.split(':').collect();

        if hextets.len() != 6 { return Err(IpAddressError::InvalidLength); }
        
        Ok(Ipv6Addr(hextets[0].parse()?, hextets[1].parse()?, hextets[2].parse()?,hextets[3].parse()?, hextets[4].parse()?, hextets[5].parse()?))
    }

    /// Constructs a loopback address.
    const fn loopback() -> Self {
        Ipv6Addr(0, 0, 0, 0, 0, 0)
    }

    /// Checks if an IP address is a loopback address
    fn is_loopback(&self) -> bool {
        return self == &Self::loopback()
    }
}

impl fmt::Display for Ipv6Addr {
        
    /// Display the IPv6 address as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}", self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}