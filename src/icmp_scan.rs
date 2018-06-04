

use std::net::{IpAddr, Ipv4Addr};

use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::transport_channel;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;


pub fn ping_scan(ip: String) /*-> bool */ {

}