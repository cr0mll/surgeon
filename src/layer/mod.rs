pub mod eth_layer;
pub use eth_layer::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LayerType {
    EthLayer,
    Ipv4Layer
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OsiLevel {
    Physical,
    DataLink,
    Network,
    Transport,
    Session,
    Presentation,
    Application
}

pub trait Layer {
    const NAME: &'static str;
    const TYPE: LayerType;
    const OSI_LEVEL: OsiLevel;
}