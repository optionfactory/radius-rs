// Code generated by machine generator; DO NOT EDIT.

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const PKM_SS_CERT_TYPE: AVPType = 137;
pub fn delete_pkm_ss_cert(packet: &mut Packet) {
    packet.delete(PKM_SS_CERT_TYPE);
}
pub fn add_pkm_ss_cert(packet: &mut Packet, value: &[u8]) {
    packet.extend(
        value
            .chunks(253)
            .map(|chunk| AVP::from_bytes(PKM_SS_CERT_TYPE, chunk))
            .collect(),
    );
}
pub fn lookup_pkm_ss_cert(packet: &Packet) -> Option<Vec<u8>> {
    let avps = packet.lookup_all(PKM_SS_CERT_TYPE);
    match avps.is_empty() {
        true => None,
        false => Some(avps.into_iter().fold(Vec::new(), |mut acc, v| {
            acc.extend(v.encode_bytes());
            acc
        })),
    }
}

pub const PKM_CA_CERT_TYPE: AVPType = 138;
pub fn delete_pkm_ca_cert(packet: &mut Packet) {
    packet.delete(PKM_CA_CERT_TYPE);
}
pub fn add_pkm_ca_cert(packet: &mut Packet, value: &[u8]) {
    packet.extend(
        value
            .chunks(253)
            .map(|chunk| AVP::from_bytes(PKM_CA_CERT_TYPE, chunk))
            .collect(),
    );
}
pub fn lookup_pkm_ca_cert(packet: &Packet) -> Option<Vec<u8>> {
    let avps = packet.lookup_all(PKM_CA_CERT_TYPE);
    match avps.is_empty() {
        true => None,
        false => Some(avps.into_iter().fold(Vec::new(), |mut acc, v| {
            acc.extend(v.encode_bytes());
            acc
        })),
    }
}

pub const PKM_CONFIG_SETTINGS_TYPE: AVPType = 139;
pub fn delete_pkm_config_settings(packet: &mut Packet) {
    packet.delete(PKM_CONFIG_SETTINGS_TYPE);
}
pub fn add_pkm_config_settings(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(PKM_CONFIG_SETTINGS_TYPE, value));
}
pub fn lookup_pkm_config_settings(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(PKM_CONFIG_SETTINGS_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_pkm_config_settings(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PKM_CONFIG_SETTINGS_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const PKM_CRYPTOSUITE_LIST_TYPE: AVPType = 140;
pub fn delete_pkm_cryptosuite_list(packet: &mut Packet) {
    packet.delete(PKM_CRYPTOSUITE_LIST_TYPE);
}
pub fn add_pkm_cryptosuite_list(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(PKM_CRYPTOSUITE_LIST_TYPE, value));
}
pub fn lookup_pkm_cryptosuite_list(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(PKM_CRYPTOSUITE_LIST_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_pkm_cryptosuite_list(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PKM_CRYPTOSUITE_LIST_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const PKM_SAID_TYPE: AVPType = 141;
pub fn delete_pkm_said(packet: &mut Packet) {
    packet.delete(PKM_SAID_TYPE);
}
pub fn add_pkm_said(packet: &mut Packet, value: u16) {
    packet.add(AVP::from_u16(PKM_SAID_TYPE, value));
}
pub fn lookup_pkm_said(packet: &Packet) -> Option<Result<u16, AVPError>> {
    packet.lookup(PKM_SAID_TYPE).map(|v| v.encode_u16())
}
pub fn lookup_all_pkm_said(packet: &Packet) -> Result<Vec<u16>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PKM_SAID_TYPE) {
        vec.push(avp.encode_u16()?)
    }
    Ok(vec)
}

pub const PKM_SA_DESCRIPTOR_TYPE: AVPType = 142;
pub fn delete_pkm_sa_descriptor(packet: &mut Packet) {
    packet.delete(PKM_SA_DESCRIPTOR_TYPE);
}
pub fn add_pkm_sa_descriptor(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(PKM_SA_DESCRIPTOR_TYPE, value));
}
pub fn lookup_pkm_sa_descriptor(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(PKM_SA_DESCRIPTOR_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_pkm_sa_descriptor(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PKM_SA_DESCRIPTOR_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub const PKM_AUTH_KEY_TYPE: AVPType = 143;
pub fn delete_pkm_auth_key(packet: &mut Packet) {
    packet.delete(PKM_AUTH_KEY_TYPE);
}
pub fn add_pkm_auth_key(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(PKM_AUTH_KEY_TYPE, value));
}
pub fn lookup_pkm_auth_key(packet: &Packet) -> Option<Vec<u8>> {
    packet.lookup(PKM_AUTH_KEY_TYPE).map(|v| v.encode_bytes())
}
pub fn lookup_all_pkm_auth_key(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(PKM_AUTH_KEY_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}
