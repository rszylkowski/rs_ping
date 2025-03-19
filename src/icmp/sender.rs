// filepath: [sender.rs](http://_vscodecontentref_/3)
//! Handles sending ICMP packets

use crate::icmp::packet::create_icmp_packet;
use crate::config;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{transport_channel, TransportChannelType::Layer4, TransportProtocol};
use std::net::IpAddr;

/// Sends an ICMP echo request (ping) to the specified IP address.
/// 
/// # Arguments
/// * `target_ip` - The IP address to send the ping to.
/// 
/// # Returns
/// A Result indicating success or failure.
pub fn send_ping(target_ip: &str) -> Result<(), String> {
    let target_ip: IpAddr = target_ip.parse().map_err(|_| "Invalid IP address".to_string())?;

    // Create a transport chanel for sending ICMP packets
    let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, _) = transport_channel(config::DEFAULT_BUFFER_SIZE, protocol).map_err(|e| e.to_string())?;

    // Create a buffer for the ICMP packet
    let mut buffer = [0u8; 64];

    // Create an ICMP packet
    let packet = create_icmp_packet(&mut buffer, 1, 0);

    // Send the packet
    tx.send_to(packet, target_ip).map_err(|e| e.to_string())?;

    Ok(())

}