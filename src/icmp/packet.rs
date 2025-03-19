// filepath [packet.rs](http://_vscodecontentref_/2)
//! Handles ICMP packet crafting
//!
//! This module contains the functions to craft ICMP packets.

use pnet::packet::icmp::{IcmpTypes};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::Packet;
use pnet::util::checksum;

/// Creates an ICMP echo request packet.
/// 
/// # Arguments
/// * `sequence_number` - The sequence number to set in the packet.
/// * `identifier` - The identifier to set in the packet.
/// 
/// # Returns
/// A `MutableEchoRequestPacket` containing the crafted ICMP packet.
pub fn create_icmp_packet<'a>(
    buffer: &'a mut [u8],
    sequence_number: u16, 
    identifier: u16,
) -> MutableEchoRequestPacket<'a> {
    let mut packet = MutableEchoRequestPacket::new(buffer).unwrap();

    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_sequence_number(sequence_number);
    packet.set_identifier(identifier);

    // Calculate the checksum
    let checksum = checksum(&packet.to_immutable().packet(), 1);
    packet.set_checksum(checksum);

    packet
}