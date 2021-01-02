// Code generated by machine generator; DO NOT EDIT.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const MOBILE_NODE_IDENTIFIER_TYPE: AVPType = 145;
pub fn delete_mobile_node_identifier(packet: &mut Packet) {
    packet.delete(MOBILE_NODE_IDENTIFIER_TYPE);
}
pub fn add_mobile_node_identifier(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(MOBILE_NODE_IDENTIFIER_TYPE, value));
}
pub fn lookup_mobile_node_identifier(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(MOBILE_NODE_IDENTIFIER_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_mobile_node_identifier(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MOBILE_NODE_IDENTIFIER_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const SERVICE_SELECTION_TYPE: AVPType = 146;
pub fn delete_service_selection(packet: &mut Packet) {
    packet.delete(SERVICE_SELECTION_TYPE);
}
pub fn add_service_selection(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(SERVICE_SELECTION_TYPE, value));
}
pub fn lookup_service_selection(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(SERVICE_SELECTION_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_service_selection(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(SERVICE_SELECTION_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_LMA_IPV6_ADDRESS_TYPE: AVPType = 147;
pub fn delete_pmip6_home_lma_ipv6_address(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_LMA_IPV6_ADDRESS_TYPE);
}
pub fn add_pmip6_home_lma_ipv6_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(PMIP6_HOME_LMA_IPV6_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_home_lma_ipv6_address(packet: &Packet) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(PMIP6_HOME_LMA_IPV6_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
pub fn lookup_all_pmip6_home_lma_ipv6_address(packet: &Packet) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_LMA_IPV6_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_LMA_IPV6_ADDRESS_TYPE: AVPType = 148;
pub fn delete_pmip6_visited_lma_ipv6_address(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_LMA_IPV6_ADDRESS_TYPE);
}
pub fn add_pmip6_visited_lma_ipv6_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(PMIP6_VISITED_LMA_IPV6_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_visited_lma_ipv6_address(
    packet: &Packet,
) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_LMA_IPV6_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
pub fn lookup_all_pmip6_visited_lma_ipv6_address(
    packet: &Packet,
) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_LMA_IPV6_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_LMA_IPV4_ADDRESS_TYPE: AVPType = 149;
pub fn delete_pmip6_home_lma_ipv4_address(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_LMA_IPV4_ADDRESS_TYPE);
}
pub fn add_pmip6_home_lma_ipv4_address(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(PMIP6_HOME_LMA_IPV4_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_home_lma_ipv4_address(packet: &Packet) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_HOME_LMA_IPV4_ADDRESS_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_home_lma_ipv4_address(packet: &Packet) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_LMA_IPV4_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_LMA_IPV4_ADDRESS_TYPE: AVPType = 150;
pub fn delete_pmip6_visited_lma_ipv4_address(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_LMA_IPV4_ADDRESS_TYPE);
}
pub fn add_pmip6_visited_lma_ipv4_address(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(PMIP6_VISITED_LMA_IPV4_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_visited_lma_ipv4_address(
    packet: &Packet,
) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_LMA_IPV4_ADDRESS_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_visited_lma_ipv4_address(
    packet: &Packet,
) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_LMA_IPV4_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_HN_PREFIX_TYPE: AVPType = 151;
pub fn delete_pmip6_home_hn_prefix(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_HN_PREFIX_TYPE);
}
pub fn add_pmip6_home_hn_prefix(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv6_prefix(PMIP6_HOME_HN_PREFIX_TYPE, value)?);
    Ok(())
}
pub fn lookup_pmip6_home_hn_prefix(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(PMIP6_HOME_HN_PREFIX_TYPE)
        .map(|v| v.encode_ipv6_prefix())
}
pub fn lookup_all_pmip6_home_hn_prefix(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_HN_PREFIX_TYPE) {
        vec.push(avp.encode_ipv6_prefix()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_HN_PREFIX_TYPE: AVPType = 152;
pub fn delete_pmip6_visited_hn_prefix(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_HN_PREFIX_TYPE);
}
pub fn add_pmip6_visited_hn_prefix(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv6_prefix(PMIP6_VISITED_HN_PREFIX_TYPE, value)?);
    Ok(())
}
pub fn lookup_pmip6_visited_hn_prefix(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_HN_PREFIX_TYPE)
        .map(|v| v.encode_ipv6_prefix())
}
pub fn lookup_all_pmip6_visited_hn_prefix(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_HN_PREFIX_TYPE) {
        vec.push(avp.encode_ipv6_prefix()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_INTERFACE_ID_TYPE: AVPType = 153;
pub fn delete_pmip6_home_interface_id(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_INTERFACE_ID_TYPE);
}
pub fn add_pmip6_home_interface_id(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    if value.len() != 8 {
        return Err(AVPError::InvalidAttributeLengthError(
            "8 bytes".to_owned(),
            value.len(),
        ));
    }
    packet.add(AVP::from_bytes(PMIP6_HOME_INTERFACE_ID_TYPE, value));
    Ok(())
}
pub fn lookup_pmip6_home_interface_id(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(PMIP6_HOME_INTERFACE_ID_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_pmip6_home_interface_id(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_INTERFACE_ID_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const PMIP6_VISITED_INTERFACE_ID_TYPE: AVPType = 154;
pub fn delete_pmip6_visited_interface_id(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_INTERFACE_ID_TYPE);
}
pub fn add_pmip6_visited_interface_id(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    if value.len() != 8 {
        return Err(AVPError::InvalidAttributeLengthError(
            "8 bytes".to_owned(),
            value.len(),
        ));
    }
    packet.add(AVP::from_bytes(PMIP6_VISITED_INTERFACE_ID_TYPE, value));
    Ok(())
}
pub fn lookup_pmip6_visited_interface_id(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(PMIP6_VISITED_INTERFACE_ID_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_pmip6_visited_interface_id(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_INTERFACE_ID_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const PMIP6_HOME_IPV4_HO_A_TYPE: AVPType = 155;
pub fn delete_pmip6_home_ipv4_ho_a(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_IPV4_HO_A_TYPE);
}
pub fn add_pmip6_home_ipv4_ho_a(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv4_prefix(PMIP6_HOME_IPV4_HO_A_TYPE, value)?);
    Ok(())
}
pub fn lookup_pmip6_home_ipv4_ho_a(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(PMIP6_HOME_IPV4_HO_A_TYPE)
        .map(|v| v.encode_ipv4_prefix())
}
pub fn lookup_all_pmip6_home_ipv4_ho_a(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_IPV4_HO_A_TYPE) {
        vec.push(avp.encode_ipv4_prefix()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_IPV4_HO_A_TYPE: AVPType = 156;
pub fn delete_pmip6_visited_ipv4_ho_a(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_IPV4_HO_A_TYPE);
}
pub fn add_pmip6_visited_ipv4_ho_a(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv4_prefix(PMIP6_VISITED_IPV4_HO_A_TYPE, value)?);
    Ok(())
}
pub fn lookup_pmip6_visited_ipv4_ho_a(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_IPV4_HO_A_TYPE)
        .map(|v| v.encode_ipv4_prefix())
}
pub fn lookup_all_pmip6_visited_ipv4_ho_a(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_IPV4_HO_A_TYPE) {
        vec.push(avp.encode_ipv4_prefix()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_DHCP4_SERVER_ADDRESS_TYPE: AVPType = 157;
pub fn delete_pmip6_home_dhcp4_server_address(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_DHCP4_SERVER_ADDRESS_TYPE);
}
pub fn add_pmip6_home_dhcp4_server_address(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(PMIP6_HOME_DHCP4_SERVER_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_home_dhcp4_server_address(
    packet: &Packet,
) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_HOME_DHCP4_SERVER_ADDRESS_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_home_dhcp4_server_address(
    packet: &Packet,
) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_DHCP4_SERVER_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_DHCP4_SERVER_ADDRESS_TYPE: AVPType = 158;
pub fn delete_pmip6_visited_dhcp4_server_address(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_DHCP4_SERVER_ADDRESS_TYPE);
}
pub fn add_pmip6_visited_dhcp4_server_address(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(
        PMIP6_VISITED_DHCP4_SERVER_ADDRESS_TYPE,
        value,
    ));
}
pub fn lookup_pmip6_visited_dhcp4_server_address(
    packet: &Packet,
) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_DHCP4_SERVER_ADDRESS_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_visited_dhcp4_server_address(
    packet: &Packet,
) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_DHCP4_SERVER_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_DHCP6_SERVER_ADDRESS_TYPE: AVPType = 159;
pub fn delete_pmip6_home_dhcp6_server_address(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_DHCP6_SERVER_ADDRESS_TYPE);
}
pub fn add_pmip6_home_dhcp6_server_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(PMIP6_HOME_DHCP6_SERVER_ADDRESS_TYPE, value));
}
pub fn lookup_pmip6_home_dhcp6_server_address(
    packet: &Packet,
) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(PMIP6_HOME_DHCP6_SERVER_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
pub fn lookup_all_pmip6_home_dhcp6_server_address(
    packet: &Packet,
) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_DHCP6_SERVER_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_DHCP6_SERVER_ADDRESS_TYPE: AVPType = 160;
pub fn delete_pmip6_visited_dhcp6_server_address(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_DHCP6_SERVER_ADDRESS_TYPE);
}
pub fn add_pmip6_visited_dhcp6_server_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(
        PMIP6_VISITED_DHCP6_SERVER_ADDRESS_TYPE,
        value,
    ));
}
pub fn lookup_pmip6_visited_dhcp6_server_address(
    packet: &Packet,
) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_DHCP6_SERVER_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
pub fn lookup_all_pmip6_visited_dhcp6_server_address(
    packet: &Packet,
) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_DHCP6_SERVER_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const PMIP6_HOME_IPV4_GATEWAY_TYPE: AVPType = 161;
pub fn delete_pmip6_home_ipv4_gateway(packet: &mut Packet) {
    packet.delete(PMIP6_HOME_IPV4_GATEWAY_TYPE);
}
pub fn add_pmip6_home_ipv4_gateway(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(PMIP6_HOME_IPV4_GATEWAY_TYPE, value));
}
pub fn lookup_pmip6_home_ipv4_gateway(packet: &Packet) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_HOME_IPV4_GATEWAY_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_home_ipv4_gateway(packet: &Packet) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_HOME_IPV4_GATEWAY_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}

pub const PMIP6_VISITED_IPV4_GATEWAY_TYPE: AVPType = 162;
pub fn delete_pmip6_visited_ipv4_gateway(packet: &mut Packet) {
    packet.delete(PMIP6_VISITED_IPV4_GATEWAY_TYPE);
}
pub fn add_pmip6_visited_ipv4_gateway(packet: &mut Packet, value: &Ipv4Addr) {
    packet.add(AVP::from_ipv4(PMIP6_VISITED_IPV4_GATEWAY_TYPE, value));
}
pub fn lookup_pmip6_visited_ipv4_gateway(packet: &Packet) -> Option<Result<Ipv4Addr, AVPError>> {
    packet
        .lookup(PMIP6_VISITED_IPV4_GATEWAY_TYPE)
        .map(|v| v.encode_ipv4())
}
pub fn lookup_all_pmip6_visited_ipv4_gateway(packet: &Packet) -> Result<Vec<Ipv4Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PMIP6_VISITED_IPV4_GATEWAY_TYPE) {
        vec.push(avp.encode_ipv4()?)
    }
    Ok(vec)
}
