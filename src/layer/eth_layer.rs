use super::*;

use crate::MacAddr;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EthLayer {
    pub src_mac: MacAddr,
    pub dst_mac: MacAddr,
    pub ether_type: EtherType
}

impl Layer for EthLayer {
    const NAME: &'static str = "Ethernet";
    const TYPE: LayerType = LayerType::EthLayer;
    const OSI_LEVEL: OsiLevel = OsiLevel::DataLink;
}

impl EthLayer {
    fn new(src_mac: MacAddr, dst_mac: MacAddr, ether_type: EtherType) -> Self {
     
        EthLayer {
            src_mac, 
            dst_mac, 
            ether_type
        }
    }
}

#[derive(Error, Debug,  PartialEq, Eq, Clone)]
pub enum EthError {
    
    #[error("The specified EtherType represents a length and not an actual type.")]
    EtherTypeIsLength,

    #[error("The specified EtherType is unknown.")]
    UnknownEtherType,
}


#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherType {
    IPv4 = 0x0800
}

impl TryFrom<u16> for EtherType {
    type Error = EthError;

    fn try_from(value: u16) -> Result<Self, EthError> {
        match value {
            _ if value <= 1500 => Err(EthError::EtherTypeIsLength),
            0x0800 => Ok(EtherType::IPv4),
            _ => Err(EthError::UnknownEtherType)
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
        let eth_layer = EthLayer::new(src_mac, dst_mac, EtherType::IPv4);

        assert_eq!(eth_layer.src_mac, MacAddr::new(0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff));
        assert_eq!(eth_layer.dst_mac, MacAddr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66));
        assert_eq!(eth_layer.ether_type, EtherType::IPv4);
        assert_eq!(eth_layer, EthLayer {src_mac: MacAddr::new(0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff), dst_mac: MacAddr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66), ether_type: EtherType::IPv4});
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
}