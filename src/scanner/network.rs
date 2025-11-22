
// Scan the network for active hosts
use ipnet::Ipv4Net;

pub fn scan_network() {

    // The network should be configurable; hardcoded for now
    let net: Ipv4Net = "192.168.4.0/24".parse().unwrap();
    
    for ip in net.hosts() {
        println!("Scanning IP: ==> {}", ip);    
    }
}