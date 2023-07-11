pub mod ipv4;
pub use ipv4::*;
pub mod ipv6;
pub use ipv6::*;

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