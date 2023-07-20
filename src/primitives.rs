use std::fmt;

use thiserror::Error;

pub trait BitPrimitive: Copy + Default + TryFrom<Self::RustPrimitive> {
    /// The smallest unsigned primitive type supported by Rust (u8, u16, ...) which can hold the entire range of the bit primitive.
    type RustPrimitive;

    const MIN_VALUE: Self::RustPrimitive;
    const MAX_VALUE: Self::RustPrimitive;

    /// The number of bits needed to represent the bit primitive.
    const BIT_WIDTH: u32;

    /// Checks whether the stored value is a valid representation of the given primitive type.
    fn is_valid(self) -> bool;

    /// Returns the value of the bit primitive as its corresponding Rust primitive.
    fn value(self) -> Self::RustPrimitive;

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError>;
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum BitPrimitiveError {
    #[error("The specified value is not valid representation of the bit primitive.")]
    ValueOutOfRange
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Bit {
    value: u8
}

impl BitPrimitive for Bit {
    /// Bits have only two possible values - 0 or 1.
    type RustPrimitive = u8;
    const BIT_WIDTH: u32 = 1;
    const MIN_VALUE: Self::RustPrimitive = 0;
    const MAX_VALUE: Self::RustPrimitive = 2u8.pow(Self::BIT_WIDTH - 1);

    fn is_valid(self) -> bool {
        self.value >= Self::MIN_VALUE && self.value <= Self::MAX_VALUE
    }

    fn value(self) -> Self::RustPrimitive {
        self.value
    }

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            self.value = value;
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }

}

impl TryFrom<u8> for Bit {
    type Error = BitPrimitiveError;
    
    fn try_from(value: u8) -> Result<Self, BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            return Ok(Self {value});
        }
        else {
            return Err(BitPrimitiveError::ValueOutOfRange);
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Crumb {
    value: u8
}

impl BitPrimitive for Crumb {
    type RustPrimitive = u8;
    const BIT_WIDTH: u32 = 2;
    const MIN_VALUE: Self::RustPrimitive = 0;
    const MAX_VALUE: Self::RustPrimitive = 2u8.pow(Self::BIT_WIDTH) - 1;

    fn is_valid(self) -> bool {
        self.value >= Self::MIN_VALUE && self.value <= Self::MAX_VALUE
    }

    fn value(self) -> Self::RustPrimitive {
        self.value
    }

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            self.value = value;
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }
}

impl TryFrom<u8> for Crumb {
    type Error = BitPrimitiveError;

    fn try_from(value: u8) -> Result<Self, BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            return Ok(Self {value});
        }
        else {
            return Err(BitPrimitiveError::ValueOutOfRange);
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct U3 {
    value: u8
}

impl BitPrimitive for U3 {
    type RustPrimitive = u8;
    const BIT_WIDTH: u32 = 3;
    const MIN_VALUE: Self::RustPrimitive = 0;
    const MAX_VALUE: Self::RustPrimitive = 2u8.pow(Self::BIT_WIDTH) - 1;

    fn is_valid(self) -> bool {
        self.value >= Self::MIN_VALUE && self.value <= Self::MAX_VALUE
    }

    fn value(self) -> Self::RustPrimitive {
        self.value
    }

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            self.value = value;
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }
}

impl TryFrom<u8> for U3 {
    type Error = BitPrimitiveError;

    fn try_from(value: u8) -> Result<Self, BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            return Ok(Self {value});
        }
        else {
            return Err(BitPrimitiveError::ValueOutOfRange);
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Nibble {
    value: u8
}

impl BitPrimitive for Nibble {
    type RustPrimitive = u8;
    const BIT_WIDTH: u32 = 4;
    const MIN_VALUE: Self::RustPrimitive = 0;
    const MAX_VALUE: Self::RustPrimitive = 2u8.pow(Self::BIT_WIDTH) - 1;

    fn is_valid(self) -> bool {
        self.value >= Self::MIN_VALUE && self.value <= Self::MAX_VALUE
    }

    fn value(self) -> Self::RustPrimitive {
        self.value
    }

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            self.value = value;
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }
}

impl TryFrom<u8> for Nibble {
    type Error = BitPrimitiveError;

    fn try_from(value: u8) -> Result<Self, BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            return Ok(Self {value});
        }
        else {
            return Err(BitPrimitiveError::ValueOutOfRange);
        }
    }
}

type Byte = u8;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct U12 {
    value: u16
}

impl BitPrimitive for U12 {
    type RustPrimitive = u16;
    const BIT_WIDTH: u32 = 12;
    const MIN_VALUE: Self::RustPrimitive = 0;
    const MAX_VALUE: Self::RustPrimitive = 2u16.pow(Self::BIT_WIDTH) - 1;

    fn is_valid(self) -> bool {
        self.value >= Self::MIN_VALUE && self.value <= Self::MAX_VALUE
    }

    fn value(self) -> Self::RustPrimitive {
        self.value
    }

    fn set(&mut self, value: Self::RustPrimitive) -> Result<(), BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            self.value = value;
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }
}

impl TryFrom<u16> for U12 {
    type Error = BitPrimitiveError;

    fn try_from(value: u16) -> Result<Self, BitPrimitiveError> {
        if value >= Self::MIN_VALUE && value <= Self::MAX_VALUE {
            return Ok(Self {value});
        }
        else {
            return Err(BitPrimitiveError::ValueOutOfRange);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::primitive;

    use super::*;

    #[test]
    fn bit() {
        assert_eq!(Bit::MIN_VALUE, 0);
        assert_eq!(Bit::MAX_VALUE, 1);
        assert_eq!(Bit::BIT_WIDTH, 1);

        assert_eq!(Bit::default(), Bit {value: 0});

        assert_eq!(Bit::try_from(0), Ok(Bit {value: 0}));
        assert_eq!(Bit::try_from(1), Ok(Bit {value: 1}));
        assert_eq!(Bit::try_from(Bit::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));

        let mut primitive = Bit::try_from(0).unwrap();
        assert_eq!(primitive.value(), 0);
        primitive.set(1).unwrap();
        assert_eq!(primitive.value(), 1);
        assert_eq!(primitive.set(Bit::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));
        assert_ne!(primitive.value(), Bit::MAX_VALUE + 1);

        let primitive = Bit{value: 0};
        assert!(primitive.is_valid());
        let primitive =  Bit {value: Bit::MAX_VALUE + 1};
        assert!(!primitive.is_valid());
    }

    #[test]
    fn crumb() {
        assert_eq!(Crumb::MIN_VALUE, 0);
        assert_eq!(Crumb::MAX_VALUE, 3);
        assert_eq!(Crumb::BIT_WIDTH, 2);

        assert_eq!(Crumb::default(), Crumb {value: 0});

        assert_eq!(Crumb::try_from(0b00), Ok(Crumb {value: 0}));
        assert_eq!(Crumb::try_from(0b01), Ok(Crumb {value: 1}));
        assert_eq!(Crumb::try_from(0b10), Ok(Crumb {value: 2}));
        assert_eq!(Crumb::try_from(0b11), Ok(Crumb {value: 3}));

        assert_eq!(Crumb::try_from(Crumb::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));

        let mut primitive = Crumb::try_from(0).unwrap();
        assert_eq!(primitive.value(), 0);
        primitive.set(1).unwrap();
        assert_eq!(primitive.value(), 1);
        assert_eq!(primitive.set(Crumb::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));
        assert_ne!(primitive.value(), Crumb::MAX_VALUE + 1);

        let primitive = Crumb{value: 0};
        assert!(primitive.is_valid());
        let primitive =  Crumb {value: Crumb::MAX_VALUE + 1};
        assert!(!primitive.is_valid());
    }

    #[test]
    fn u3() {
        assert_eq!(U3::MIN_VALUE, 0);
        assert_eq!(U3::MAX_VALUE, 7);
        assert_eq!(U3::BIT_WIDTH, 3);

        assert_eq!(U3::default(), U3 {value: 0});

        assert_eq!(U3::try_from(0b000), Ok(U3 {value: 0}));
        assert_eq!(U3::try_from(0b001), Ok(U3 {value: 1}));
        assert_eq!(U3::try_from(0b010), Ok(U3 {value: 2}));
        assert_eq!(U3::try_from(0b011), Ok(U3 {value: 3}));
        assert_eq!(U3::try_from(0b100), Ok(U3 {value: 4}));
        assert_eq!(U3::try_from(0b101), Ok(U3 {value: 5}));
        assert_eq!(U3::try_from(0b110), Ok(U3 {value: 6}));
        assert_eq!(U3::try_from(0b111), Ok(U3 {value: 7}));

        assert_eq!(U3::try_from(U3::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));

        let mut primitive = U3::try_from(0).unwrap();
        assert_eq!(primitive.value(), 0);
        primitive.set(1).unwrap();
        assert_eq!(primitive.value(), 1);
        assert_eq!(primitive.set(U3::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));
        assert_ne!(primitive.value(), U3::MAX_VALUE + 1);

        let primitive = U3{value: 0};
        assert!(primitive.is_valid());
        let primitive =  U3 {value: U3::MAX_VALUE + 1};
        assert!(!primitive.is_valid());

    }

    #[test]
    fn nibble() {
        assert_eq!(Nibble::MIN_VALUE, 0);
        assert_eq!(Nibble::MAX_VALUE, 15);
        assert_eq!(Nibble::BIT_WIDTH, 4);

        assert_eq!(Nibble::default(), Nibble {value: 0});

        assert_eq!(Nibble::try_from(0b0000), Ok(Nibble {value: 0}));
        assert_eq!(Nibble::try_from(0b0001), Ok(Nibble {value: 1}));
        assert_eq!(Nibble::try_from(0b0010), Ok(Nibble {value: 2}));
        assert_eq!(Nibble::try_from(0b0011), Ok(Nibble {value: 3}));
        assert_eq!(Nibble::try_from(0b0100), Ok(Nibble {value: 4}));
        assert_eq!(Nibble::try_from(0b0101), Ok(Nibble {value: 5}));
        assert_eq!(Nibble::try_from(0b0110), Ok(Nibble {value: 6}));
        assert_eq!(Nibble::try_from(0b0111), Ok(Nibble {value: 7}));
        assert_eq!(Nibble::try_from(0b1000), Ok(Nibble {value: 8}));
        assert_eq!(Nibble::try_from(0b1001), Ok(Nibble {value: 9}));
        assert_eq!(Nibble::try_from(0b1010), Ok(Nibble {value: 10}));
        assert_eq!(Nibble::try_from(0b1011), Ok(Nibble {value: 11}));
        assert_eq!(Nibble::try_from(0b1100), Ok(Nibble {value: 12}));
        assert_eq!(Nibble::try_from(0b1101), Ok(Nibble {value: 13}));
        assert_eq!(Nibble::try_from(0b1110), Ok(Nibble {value: 14}));
        assert_eq!(Nibble::try_from(0b1111), Ok(Nibble {value: 15}));

        assert_eq!(Nibble::try_from(Nibble::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));

        let mut primitive = Nibble::try_from(0).unwrap();
        assert_eq!(primitive.value(), 0);
        primitive.set(1).unwrap();
        assert_eq!(primitive.value(), 1);
        assert_eq!(primitive.set(Nibble::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));
        assert_ne!(primitive.value(), Nibble::MAX_VALUE + 1);

        let primitive = Nibble{value: 0};
        assert!(primitive.is_valid());
        let primitive =  Nibble {value: Nibble::MAX_VALUE + 1};
        assert!(!primitive.is_valid());
    }

    #[test]
    fn u12() {
        assert_eq!(U12::MIN_VALUE, 0);
        assert_eq!(U12::MAX_VALUE, 4095);
        assert_eq!(U12::BIT_WIDTH, 12);

        assert_eq!(U12::default(), U12 {value: 0});

        assert_eq!(U12::try_from(0b0000), Ok(U12 {value: 0}));
        assert_eq!(U12::try_from(0b0001), Ok(U12 {value: 1}));
        assert_eq!(U12::try_from(0b0010), Ok(U12 {value: 2}));
        assert_eq!(U12::try_from(0b0011), Ok(U12 {value: 3}));
        assert_eq!(U12::try_from(0b0100), Ok(U12 {value: 4}));
        assert_eq!(U12::try_from(0b0101), Ok(U12 {value: 5}));
        assert_eq!(U12::try_from(0b0110), Ok(U12 {value: 6}));
        assert_eq!(U12::try_from(0b0111), Ok(U12 {value: 7}));
        assert_eq!(U12::try_from(0b1000), Ok(U12 {value: 8}));
        assert_eq!(U12::try_from(0b1001), Ok(U12 {value: 9}));
        assert_eq!(U12::try_from(0b1010), Ok(U12 {value: 10}));
        assert_eq!(U12::try_from(0b1011), Ok(U12 {value: 11}));
        assert_eq!(U12::try_from(0b1100), Ok(U12 {value: 12}));
        assert_eq!(U12::try_from(0b1101), Ok(U12 {value: 13}));
        assert_eq!(U12::try_from(0b1110), Ok(U12 {value: 14}));
        assert_eq!(U12::try_from(0b1111), Ok(U12 {value: 15}));

        assert_eq!(U12::try_from(U12::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));

        let mut primitive = U12::try_from(0).unwrap();
        assert_eq!(primitive.value(), 0);
        primitive.set(1).unwrap();
        assert_eq!(primitive.value(), 1);
        assert_eq!(primitive.set(U12::MAX_VALUE + 1), Err(BitPrimitiveError::ValueOutOfRange));
        assert_ne!(primitive.value(), U12::MAX_VALUE + 1);

        let primitive = U12{value: 0};
        assert!(primitive.is_valid());
        let primitive =  U12 {value: U12::MAX_VALUE + 1};
        assert!(!primitive.is_valid());
    }

}