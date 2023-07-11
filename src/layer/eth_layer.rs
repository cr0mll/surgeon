use super::*;

use crate::{MacAddr, primitives::*};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthLayer {
    pub src_mac: MacAddr,
    pub dst_mac: MacAddr,
    pub _802_1q_tag: Option<Q802_1Tag>,
    pub ether_type: EtherType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Q802_1Tag {
    tpid: u16,
    tic: u16,
}

impl Q802_1Tag {
    /// Tag Protocol Identifier (TPID) constant as specified by the 802.1Q encapsulation standard.
    const TPID: u16 = 0x8100;

    /// Constructs a new 802.1Q tag with the specified Tag Control Information (TIC).
    fn new(tic: u16) -> Self {
        Self {
            tpid: Self::TPID,
            tic,
        }
    }

    /// Checks if the tag is valid by comparing its tag protocol identifier (TPID).
    fn is_valid(self) -> bool {
        self.tpid == Self::TPID
    }

    /// Checks the Drop Eligible Indicator (DEI) bit.
    fn is_drop_eligible(self) -> bool {
        ((self.tic & 0b0001_0000_0000_0000) >> 12) == 1
    }

    /// Sets the Drop Eligible Indicator to the specified value.
    fn set_drop_eligible(&mut self, value: bool) {
        self.tic = (self.tic & !(1 << 12)) | ((value as u16) << 12);
    }

    /// Returns the value of the 3-bit Priority Code Point (PCP) field of the TIC.
    fn pcp(self) -> U3 {
        ((self.tic & 0b1110_0000_0000_0000) >> 13) as U3
    }

    /// Sets the Priority Code Point field. If the provided PCP cannot fit into a 3-bit value, returns an error.
    fn set_pcp(&mut self, pcp: U3) -> Result<(), BitPrimitiveError> {
        if pcp <= 0b0000_0111 {
            self.tic = (self.tic & !(0b111 << 13)) | ((pcp as u16) << 13);
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }

    /// Returns the value of the VLAN Identifier (VID).
    fn vid(self) -> U12 {
        self.tic & 0b0000_1111_1111_1111
    }

    /// Sets the VLAN identifier. If the provided VID cannot fit into a 12-bit value, returns an error.
    fn set_vid(&mut self, vid: U12) -> Result<(), BitPrimitiveError> {
        
        if vid <= 0b0000_1111_1111_1111
        {
            self.tic |= vid; 
            Ok(())
        }
        else {
            Err(BitPrimitiveError::ValueOutOfRange)
        }
    }
}

impl Default for Q802_1Tag {
    fn default() -> Self {
        Self {
            tpid: Self::TPID,
            tic: 0,
        }
    }
}

impl Layer for EthLayer {
    const NAME: &'static str = "Ethernet";
    const TYPE: LayerType = LayerType::EthLayer;
    const OSI_LEVEL: OsiLevel = OsiLevel::DataLink;
}

impl EthLayer {
    fn new(
        src_mac: MacAddr,
        dst_mac: MacAddr,
        ether_type: EtherType,
        _802_1q_tag: Option<Q802_1Tag>,
    ) -> Self {
        EthLayer {
            src_mac,
            dst_mac,
            _802_1q_tag,
            ether_type,
        }
    }
}

impl Default for EthLayer {
    /// Constructs a default Ethernet layer.
    fn default() -> Self {
        EthLayer {
            src_mac: MacAddr::default(),
            dst_mac: MacAddr::default(),
            _802_1q_tag: None,
            ether_type: EtherType::Empty,
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum EthError {
    #[error("The specified EtherType represents a length and not an actual type.")]
    EtherTypeIsLength,

    #[error("The specified EtherType is unknown.")]
    UnknownEtherType,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherType {
    IPv4 = 0x0800,
    Empty = 0x0000,
}

impl TryFrom<u16> for EtherType {
    type Error = EthError;

    fn try_from(value: u16) -> Result<Self, EthError> {
        match value {
            _ if value <= 1500 => Err(EthError::EtherTypeIsLength),
            0x0800 => Ok(EtherType::IPv4),
            _ => Err(EthError::UnknownEtherType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(EthLayer::NAME, "Ethernet");
        assert_eq!(EthLayer::TYPE, LayerType::EthLayer);
        assert_eq!(EthLayer::OSI_LEVEL, OsiLevel::DataLink);
    }

    #[test]
    fn create_layer() {
        let src_mac = MacAddr::new(0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff);
        let dst_mac = MacAddr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66);
        let eth_layer = EthLayer::new(src_mac, dst_mac, EtherType::IPv4, None);

        assert_eq!(
            eth_layer.src_mac,
            MacAddr::new(0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff)
        );
        assert_eq!(
            eth_layer.dst_mac,
            MacAddr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66)
        );
        assert_eq!(eth_layer.ether_type, EtherType::IPv4);
        assert_eq!(
            eth_layer,
            EthLayer {
                src_mac: MacAddr::new(0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff),
                dst_mac: MacAddr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66),
                ether_type: EtherType::IPv4,
                _802_1q_tag: None
            }
        );
    }

    #[test]
    fn convert_ether_type() {
        let ether_type = EtherType::try_from(0x0800);
        assert_eq!(ether_type, Ok(EtherType::IPv4));

        let ether_type = EtherType::try_from(0x0010);
        assert_eq!(ether_type, Err(EthError::EtherTypeIsLength));

        let ether_type = EtherType::try_from(0x9999);
        assert_eq!(ether_type, Err(EthError::UnknownEtherType));
    }

    #[test]
    fn test_802_1q_tag() {
        
        // Test default tag initialisation
        let mut tag = Q802_1Tag::default();
        assert_eq!(tag.tpid, Q802_1Tag::TPID);
        assert_eq!(tag.tic, 0b0000_0000_0000_0000);


        // Test the VID
        assert_eq!(tag.set_vid(12), Ok(()));
        assert_eq!(tag.vid(), 12);
        assert_eq!(tag.set_vid(0b1111_1111_1111_1111), Err(BitPrimitiveError::ValueOutOfRange));
        
        // Test the PCP
        let mut tag = Q802_1Tag::default();
        assert_eq!(tag.set_pcp(0b101),Ok(()));
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.set_pcp(0b1111_1111), Err(BitPrimitiveError::ValueOutOfRange));

        // Test the DEI
        let mut tag = Q802_1Tag::default();
        tag.set_drop_eligible(false);
        assert_eq!(tag.is_drop_eligible(), false);
        tag.set_drop_eligible(true);
        assert_eq!(tag.is_drop_eligible(), true);

        // Test order of operations
        let mut tag = Q802_1Tag::default();
        tag.set_vid(3456).unwrap();
        tag.set_pcp(0b101).unwrap();
        tag.set_drop_eligible(true);

        let mut tag = Q802_1Tag::default();
        tag.set_vid(3456).unwrap();
        tag.set_drop_eligible(true);
        tag.set_pcp(0b101).unwrap();

        assert_eq!(tag.vid(), 3456);
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.is_drop_eligible(), true);

        let mut tag = Q802_1Tag::default();
        tag.set_pcp(0b101).unwrap();
        tag.set_drop_eligible(true);
        tag.set_vid(3456).unwrap();

        assert_eq!(tag.vid(), 3456);
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.is_drop_eligible(), true);

        assert_eq!(tag.vid(), 3456);
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.is_drop_eligible(), true);

        let mut tag = Q802_1Tag::default();
        tag.set_drop_eligible(true);
        tag.set_vid(3456).unwrap();
        tag.set_pcp(0b101).unwrap();

        assert_eq!(tag.vid(), 3456);
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.is_drop_eligible(), true);

        let mut tag = Q802_1Tag::default();
        tag.set_drop_eligible(true);
        tag.set_pcp(0b101).unwrap();
        tag.set_vid(3456).unwrap();

        assert_eq!(tag.vid(), 3456);
        assert_eq!(tag.pcp(), 0b101);
        assert_eq!(tag.is_drop_eligible(), true);
    }
}
