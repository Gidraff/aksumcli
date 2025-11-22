
// Scan the network for active hosts
use ipnet::Ipv4Net;

pub fn scan_network(ip: &str) {

    // The network should be configurable; hardcoded for now
    let net: Ipv4Net = ip.parse().unwrap();
    
    for ip in net.hosts() {
        println!("Scanning IP: ==> {}", ip);    
    }
}