use super::*;

pub struct Ipv4Layer{
    
}

impl Layer for Ipv4Layer {
    const NAME: &'static str = "Ipv4Layer";
    const TYPE: LayerType = LayerType::Ipv4Layer;
    const OSI_LEVEL:OsiLevel = OsiLevel::Network;
}