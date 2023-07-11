use std::fmt;

use thiserror::Error;

pub trait BitPrimitive: {
    /// The smallest unsigned numeric type implemented by Rust which can fit the bit primitive.
    type MinPrimitive; 

    /// Checks whether the stored value is a valid representation of the given primitive type.
    fn is_valid(self) -> bool;
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum BitPrimitiveError {
    #[error("The specified value is not valid representation of the bit primitive.")]
    ValueOutOfRange
}

pub type U1 = u8;
pub type Crumb = u8;
pub type U3 = u8;
pub type Nibble = u8;
pub type U12 = u16;