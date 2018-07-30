
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::arp::MutableArpPacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperation, ArpOperations};
use pnet::datalink::{Channel, MacAddr, NetworkInterface, ParseMacAddrErr};

pub fn ping_scan(ip: String)-> bool  {
    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();
    let arp_operation: ArpOperation = ArpOperations::Request;

    println!("{:?}", ethernet_packet);
    true
}