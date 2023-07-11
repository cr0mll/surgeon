use pnet_datalink::DataLinkSender;

use crate::{MacAddr, Ipv4Addr};

pub struct NetworkInterface {
    dev: pnet_datalink::NetworkInterface,
    tx: Box<dyn pnet_datalink::DataLinkSender>,
    rx: Box<dyn pnet_datalink::DataLinkReceiver>
}

impl NetworkInterface {
    
    //fn list() -> Vec<NetworkInterfaceData> {

    //}

    //fn find_by_name(name: &str) -> Self {

    //}
}

pub struct NetworkInterfaceData {
    pub name: String,
    pub description: String,
    pub mac_addr: MacAddr,
    pub ip_addr: Ipv4Addr
}

/*impl From<pnet_datalink::NetworkInterface> for NetworkInterfaceData {
    fn from(pnet_interface: pnet_datalink::NetworkInterface) -> Self {
        let name = pnet_interface.name.clone();
        let description = pnet_interface.description.clone();
        let ips = pnet_interface.ips;
    }
}*/

impl From<pnet_datalink::NetworkInterface> for NetworkInterface {
    fn from(value: pnet_datalink::NetworkInterface) -> Self {
        let name = value.name.clone();
        
        let (mut tx, mut rx) = match pnet_datalink::channel(&value, Default::default()) {
            Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
        };

        Self {dev: value, tx, rx}
    }
}